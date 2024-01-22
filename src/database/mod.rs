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
    for_garde: (),
}

pub async fn new() -> State {
    dotenv::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("No DATABASE_URL in env");
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);

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
        for_garde: (),
    }
}

pub struct DatabaseConnection(
    pub bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>,
);

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
