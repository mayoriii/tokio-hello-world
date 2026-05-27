use axum::{Router, routing::get};
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or("8000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let app = Router::new().route("/", get(|| async { "Hello, World" }));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
