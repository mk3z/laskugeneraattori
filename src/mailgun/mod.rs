use crate::state::State;
use axum::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};

mod invoices;

#[derive(Clone, Debug)]
pub struct MailgunClient {
    client: reqwest::Client,
    url: String,
    api_user: String,
    api_key: String,
    default_to: String,
    from: String,
}

impl From<crate::MailgunConfig> for MailgunClient {
    fn from(config: crate::MailgunConfig) -> Self {
        Self {
            client: reqwest::Client::new(),
            url: config.url,
            api_user: config.user,
            api_key: config.password,
            default_to: config.to,
            from: config.from,
        }
    }
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
