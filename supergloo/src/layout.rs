
#[derive(Clone, Debug)]
pub struct GlooLayout {
    pub layout_fn: fn(child: maud::Markup) -> maud::Markup,
    pub path: &'static str,
}

inventory::collect!(GlooLayout);


