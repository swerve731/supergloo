use gloo_macros::gloo_handler;
use maud::{html, Markup};
use supergloo::GlooHandler;
pub mod layout;
pub mod user;
// this creates the "/" route 
#[gloo_handler("view")]
async fn root() -> Markup {
    html! {
        h1 { "Hello, world!" }
    }

} 

// this creates the "/dashboard" route
#[gloo_handler("view")]
async fn dashboard() -> Markup {
    html! {
        h1 { "Dashboard!" }
    }
}

