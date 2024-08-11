use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use std::net::SocketAddr;

mod api;
mod error;
mod mailgun;
mod merge;
mod state;

mod pdfgen;

#[cfg(test)]
mod tests;

#[macro_use]
extern crate tracing;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "laskugeneraattori=debug,tower_http=debug,axum::rejection=trace".into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = state::new().await;

    let ip = if std::env::var("EXPOSE").unwrap_or("0".into()) == "1" {
        [0, 0, 0, 0]
    } else {
        [127, 0, 0, 1]
    };

    let addr = SocketAddr::from((
        ip,
        std::env::var("PORT")
            .map(|p| p.parse::<u16>().unwrap())
            .unwrap_or(3000),
    ));
    debug!("Listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TcpListener");

    axum::serve(listener, api::app().with_state(state))
        .await
        .expect("Failed to start server");
}
