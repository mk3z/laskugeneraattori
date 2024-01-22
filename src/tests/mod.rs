use crate::api::app;
use axum::body::Body;
use axum::http::request::Request;
use axum::http::StatusCode;
use tower::ServiceExt;

mod invoices;

#[tokio::test]
async fn health() {
    let app = app().with_state(crate::database::new().await);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
