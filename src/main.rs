use axum::serve;
use task_management_app::router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = router();
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    serve(listener, app).await?;
    Ok(())
}
