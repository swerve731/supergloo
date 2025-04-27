use axum::response::{IntoResponse, Response};
use gloo_macros::gloo_handler;
use supergloo::GlooHandler;



#[gloo_handler("get")]
async fn dashboard() -> Response {
    "base".into_response()
} 