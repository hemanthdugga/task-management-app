use std::env;

use axum::{Router, routing::get, serve};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/", get(index));
    let port = env::var("PORT").expect("PORT environment variable must be set");
    let address = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&address)
        .await
        .expect("failed to bind to network port address");
    serve(listener, app).await.expect("failed to start the server");
    Ok(())
}

async fn index() -> &'static str {
    "Hello, World!"
}
