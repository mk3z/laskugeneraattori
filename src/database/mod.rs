use axum::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncConnection, AsyncPgConnection,
};

mod invoices;

#[derive(FromRef, Clone)]
pub struct State {
    pool: bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    mailgun_client: MailgunClient,
    for_garde: (),
}

pub async fn new() -> State {
    dotenv::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("No DATABASE_URL in env");
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);

    let mailgun_client = MailgunClient {
        client: reqwest::Client::new(),
        url: std::env::var("MAILGUN_URL").expect("No MAILGUN_URL in env"),
        api_user: std::env::var("MAILGUN_USER").expect("No MAILGUN_USER in env"),
        api_key: std::env::var("MAILGUN_PASSWORD").expect("No MAILGUN_PASSWORD in env"),
        default_to: std::env::var("MAILGUN_TO").unwrap_or(String::from("")),
        from: std::env::var("MAILGUN_FROM").unwrap_or(String::from("")),
    };

    State {
        pool: if cfg!(test) {
            let pool = bb8::Pool::builder()
                .max_size(1)
                .build(config)
                .await
                .expect("Failed to build database pool");

            let mut conn = pool.get_owned().await.expect("Failed to get connection");
            conn.begin_test_transaction()
                .await
                .expect("Failed to begin test transaction");
            pool
        } else {
            bb8::Pool::builder()
                .build(config)
                .await
                .expect("Failed to build database pool")
        },
        mailgun_client,
        for_garde: (),
    }
}

pub struct DatabaseConnection(
    pub bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>,
);

#[derive(Clone)]
pub struct MailgunClient {
    pub client: reqwest::Client,
    pub url: String,
    pub api_user: String,
    pub api_key: String,
    pub default_to: String,
    pub from: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for MailgunClient
where
    S: Send + Sync,
    State: FromRef<S>,
{
    type Rejection = crate::error::Error;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = State::from_ref(state);
        Ok(state.mailgun_client)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    S: Send + Sync,
    State: FromRef<S>,
{
    type Rejection = crate::error::Error;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = State::from_ref(state);
        let conn = state.pool.get_owned().await?;

        Ok(Self(conn))
    }
}
