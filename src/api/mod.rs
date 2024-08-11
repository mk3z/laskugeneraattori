use axum::{extract::DefaultBodyLimit, routing::get, routing::post, Router};
use tower_http::{limit::RequestBodyLimitLayer, trace::TraceLayer};

pub mod invoices;

pub fn app() -> Router<crate::state::State> {
    Router::new()
        .route("/health", get(health))
        .route("/invoices", post(invoices::create))
        .layer(TraceLayer::new_for_http())
        .layer(DefaultBodyLimit::disable())
        // Limit the body to 24 MiB since the email is limited to 25 MiB
        .layer(RequestBodyLimitLayer::new(24 * 1024 * 1024))
}

async fn health() {}
