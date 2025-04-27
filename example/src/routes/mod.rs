pub mod api;


use axum::response::{Response, IntoResponse};
use gloo_macros::gloo_handler;
use supergloo::GlooHandler;


// this creates the "/" route 
#[gloo_handler("get")]
async fn base() -> Response {
    "home".into_response()
} 


// this creates the "/dashboard" route
#[gloo_handler("get")]
async fn dashboard() -> Response {
    "dashboard".into_response()
}