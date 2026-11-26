use axum::{Router, http::StatusCode, routing::get};

pub fn router() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(|| async { StatusCode::OK }))
}
