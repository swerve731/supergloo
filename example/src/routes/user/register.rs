use gloo_macros::gloo_handler;
use maud::{html, Markup};
use supergloo::GlooHandler;

#[gloo_handler("view")]
async fn base() -> Markup {
    html! {
        h1 { "Register" }
    }
}

