pub mod routing;
pub mod layout;
pub mod utils;
pub use inventory;
pub use gloo_macros::{
    gloo_handler,
    gloo_layout,
};
pub use maud;



#[derive(PartialEq, Eq, Debug)]
pub enum GlooHandlerKind {
    Route,
    View(fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = maud::Markup> + Send>>),
}

pub struct GlooHandler {
    pub path: &'static str,
    pub router: fn() -> axum::routing::MethodRouter<()>,
    pub fn_name: &'static str,
    pub kind: GlooHandlerKind,
}

impl GlooHandler {
    pub fn full_mod_path(&self) -> String {
        return if self.fn_name == "base" {
            self.path.to_string()
        } else {
            format!("{}::{}", self.path, self.fn_name)
        }
    }
}

inventory::collect!(GlooHandler);
