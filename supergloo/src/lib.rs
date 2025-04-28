
pub use inventory;

pub use gloo_macros::gloo_handler;


pub struct GlooHandler {
    pub path: &'static str,
    pub router: fn() -> axum::routing::MethodRouter<()>,
    pub fn_name: &'static str,
}

inventory::collect!(GlooHandler);