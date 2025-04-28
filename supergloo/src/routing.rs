use crate::GlooHandler;



pub trait GlooRouting {
    fn gloo_routes() -> axum::Router;
}



impl GlooRouting for axum::Router {
    fn gloo_routes() -> axum::Router {
        let mut router = axum::Router::new();
        for handler in inventory::iter::<GlooHandler> {
            let mut path = handler.path.split("::").collect::<Vec<_>>();
            for i in 0..path.len() {
                if path[i] == "routes" {
                    path.remove(i);
                    break;
                } else {
                    path[i] = ""
                }
            }
            let mut path = path.join("/").replace("::", "/");
            if handler.fn_name != "base" {
                path = format!("{}/{}", path, handler.fn_name);
            }
            if !path.starts_with("/") {
                path = format!("/{}", path);
            }
            router = router.route(&path, (handler.router)());
        }
        router
    }
}