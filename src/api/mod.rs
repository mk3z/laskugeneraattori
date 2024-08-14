use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use tower_http::{cors::CorsLayer, limit::RequestBodyLimitLayer, trace::TraceLayer};

pub mod invoices;

pub fn app() -> Router<crate::state::State> {
    let cors_layer = CorsLayer::new().allow_origin([
        "https://tietokilta.fi".parse().unwrap(),
        "http://localhost:3000".parse().unwrap(),
    ]);

    Router::new()
        .route("/health", get(health))
        .route("/invoices", post(invoices::create))
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer)
        .layer(DefaultBodyLimit::disable())
        // Limit the body to 24 MiB since the email is limited to 25 MiB
        .layer(RequestBodyLimitLayer::new(24 * 1024 * 1024))
}

async fn health() {}
