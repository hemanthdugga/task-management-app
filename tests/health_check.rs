use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use task_management_app::router;
use tower::ServiceExt;

#[tokio::test]
async fn health_check() {
    let response = router()
        .oneshot(
            Request::builder()
                .uri("/health")
                .method("GET")
                .body(Body::empty())
                .expect("failed to build request"),
        )
        .await
        .expect("failed to call router");
    assert_eq!(response.status(), StatusCode::OK)
}
