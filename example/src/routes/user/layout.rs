use maud::{html, Markup};

use supergloo::layout::GlooLayout;

#[supergloo::gloo_layout]
pub fn layout() -> Markup {
    html! {
        body{
            nav { "user layout above" }
            main {
                (child)
            }
            footer { "user layout below" }
        }
    }
} 