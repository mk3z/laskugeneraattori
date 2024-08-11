use crate::state::State;
use axum::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};

mod invoices;

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
