use axum::Router;
pub mod routes;

use supergloo::routing::GlooRouting;


#[tokio::main]
async fn main() {
    let router = Router::new()
        .gloo_routes().await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
