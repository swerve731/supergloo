use axum::{response::{IntoResponse, Response}, Router};
use gloo_macros::gloo_routes;
pub mod routes;

use supergloo::GlooHandler;


#[tokio::main]
async fn main() {
    let router = Router::new()
        .merge(gloo_routes!());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
