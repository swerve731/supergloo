use maud::{html, Markup};

use supergloo::layout::GlooLayout;

#[supergloo::gloo_layout]
pub fn layout() -> Markup {
    html! {
        body{
            nav { "Nav" }
            main {
                (child)
            }
            footer { "Footer" }
        }
    }
} 