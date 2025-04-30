
pub fn route_path_from_mod_path(mod_path: &str) -> String {

    let mut path = mod_path.split("::").collect::<Vec<_>>();
    for i in 0..path.len() {
        if path[i] == "routes" {
            path.remove(i);
            break;
        } else {
            path[i] = ""
        }
    }

    for i in 0..path.len() {
        if path[i] == "layout" {
            path.remove(i);
            break;
        } else {
            continue;
        }
    }

    let mut path = path.join("/").replace("::", "/");
    if !path.starts_with("/") {
        path = format!("/{}", path);
    }
    if path != "/" && path.ends_with('/') {
        path.pop();
    }
    path

}

// Helper function to box the future returned by an async fn
// This is needed because the type `async fn() -> maud::Markup` is not directly storable
// The function needs to be async to be an axum method router and is a better user experience
// in the enum without knowing the exact future type, which is unnameable.
// We erase the specific future type using dynamic dispatch (`dyn Future`).
pub fn box_view_fn(f: impl std::future::Future<Output = maud::Markup> + Send + 'static)
    -> std::pin::Pin<Box<dyn std::future::Future<Output = maud::Markup> + Send>> {
    Box::pin(f)
}