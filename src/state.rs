use crate::mailgun::MailgunClient;
use axum::extract::FromRef;

#[derive(FromRef, Clone)]
pub struct State {
    pub mailgun_client: MailgunClient,
    pub for_garde: (),
}

pub async fn new() -> State {
    dotenv::dotenv().ok();

    let mailgun_client = MailgunClient {
        client: reqwest::Client::new(),
        url: std::env::var("MAILGUN_URL").expect("No MAILGUN_URL in env"),
        api_user: std::env::var("MAILGUN_USER").expect("No MAILGUN_USER in env"),
        api_key: std::env::var("MAILGUN_PASSWORD").expect("No MAILGUN_PASSWORD in env"),
        default_to: std::env::var("MAILGUN_TO").expect("No MAILGUN_TO in env"),
        from: std::env::var("MAILGUN_FROM").expect("No MAILGUN_FROM in env"),
    };

    State {
        mailgun_client,
        for_garde: (),
    }
}
