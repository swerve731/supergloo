use gloo_macros::gloo_handler;
use maud::{html, Markup};
use supergloo::GlooHandler;

#[gloo_handler("view")]
async fn root() -> Markup {
    html! {
        h1 { "Register" }
    }
}


#[gloo_handler("view")]
async fn other() -> Markup {
    html! {
        h1 { "Other view" }
    }
}