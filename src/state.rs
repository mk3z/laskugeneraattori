use crate::mailgun::MailgunClient;
use axum::extract::FromRef;

#[derive(FromRef, Clone)]
pub struct State {
    pub mailgun_client: MailgunClient,
    pub for_garde: (),
}

pub async fn new() -> State {
    dotenv::dotenv().ok();

    State {
        mailgun_client: MailgunClient::from(crate::CONFIG.mailgun.clone()),
        for_garde: (),
    }
}
