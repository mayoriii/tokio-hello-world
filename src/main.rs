mod schema;
use async_graphql::http::GraphiQLSource;
use schema::build_schema;

use async_graphql_poem::GraphQL;
use dotenvy::dotenv;
use poem::{IntoResponse, Route, Server, get, handler, listener::TcpListener, web::Html};
use std::env;

#[handler]
fn hello() -> String {
    format!("hello")
}

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or("8000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let schema = build_schema();

    let app = Route::new().at("/graphql", get(graphiql).post(GraphQL::new(schema)));
    Server::new(TcpListener::bind(&addr)).run(app).await
}
