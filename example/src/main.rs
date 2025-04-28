use axum::{response::{IntoResponse, Response}, Router};
pub mod routes;

use supergloo::{GlooHandler, routing::GlooRouting};


#[tokio::main]
async fn main() {
    let router = Router::new()
        .gloo_routes();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
