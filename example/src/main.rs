use axum::Router;
pub mod routes;

use supergloo::routing::GlooRouting;


#[tokio::main]
async fn main() {
    let _app = Router::new()
        .gloo_routes()
        .run_from_config("config.toml")
        .await
        .unwrap();
}
