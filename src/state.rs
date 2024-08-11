use crate::mailgun::MailgunClient;
use axum::extract::FromRef;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncConnection, AsyncPgConnection,
};

#[derive(FromRef, Clone)]
pub struct State {
    pub pool: bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    pub mailgun_client: MailgunClient,
    pub for_garde: (),
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
        default_to: std::env::var("MAILGUN_TO").expect("No MAILGUN_TO in env"),
        from: std::env::var("MAILGUN_FROM").expect("No MAILGUN_FROM in env"),
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
