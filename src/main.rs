use dotenvy::dotenv;
use poem::{Route, Server, get, handler, listener::TcpListener};
use std::env;

#[handler]
fn hello() -> String {
    format!("hello")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or("8000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let app = Route::new().at("/", get(hello));
    Server::new(TcpListener::bind(&addr)).run(app).await
}
