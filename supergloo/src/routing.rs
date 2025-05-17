use crate::{config::GlooConfig, GlooHandler};


#[async_trait::async_trait]
pub trait GlooRouting {
    fn gloo_routes(self) -> axum::Router;
    async fn run_from_config(self, path: &str) -> Result<(), crate::error::Error>;
}


#[async_trait::async_trait]
impl GlooRouting for axum::Router {
    fn gloo_routes(self) -> axum::Router {
        let mut router = axum::Router::new();
        for handler in inventory::iter::<GlooHandler> {
            let path = parse_route_path(&handler.path, &handler.fn_name);
            dbg!(&path);
            router = router.route(&path, (handler.router)());
        }

        self.merge(router)

    }

    async fn run_from_config(self, path: &str) -> Result<(), crate::error::Error> {
        let c = GlooConfig::from_toml(path)?;

        let host = format!("{}:{}", c.host_address, c.port);

        let listener = tokio::net::TcpListener::bind(&host).await?;

        axum::serve(listener, self)
            .await?;

        Ok(())
    }
}


fn parse_route_path(mod_path: &str, fn_name: &str) -> String {
    let mut path = mod_path.split("::").collect::<Vec<_>>();
    for i in 0..path.len() {
        if path[i] == "routes" {
            path.remove(i);
            break;
        } else {
            path[i] = ""
        }
    }

    let mut path = path.join("/").replace("::", "/");
    
    let fn_path = fn_name
        .split("_")
        .map(|s| {
            if s.starts_with("PTH") {
                let path_var = format!("{{{}}}", s[3..].to_string());
                path_var
            } else if s == "root" {
                "".to_string()
            } else  {
                s.to_string()
            }

        })
        .collect::<Vec<String>>()
        .join("/");
    
    path = format!("{}/{}", path, fn_path);

    if !path.starts_with("/") {
        path = format!("/{}", path);
    }

    if !path.ends_with("/") {
        path = format!("{}/", path);
    }
    
    path

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_route_path() {
        let mod_path = "routes";
        let fn_name = "root";
        let path = parse_route_path(mod_path, fn_name);
        assert_eq!(path, "/");



        let mod_path = "routes::test::deep::routing::with::more::stuff";
        let fn_name = "test_more_long_function_name_PTHtest_more_stuff";
        let path = parse_route_path(mod_path, fn_name);
        assert_eq!(path, "/test/deep/routing/with/more/stuff/test/more/long/function/name/{test}/more/stuff/");


        let mod_path = "routes";
        let fn_name = "hello_PTHname";
        let path = parse_route_path(mod_path, fn_name);
        assert_eq!(path, "/hello/{name}/");


        let mod_path = "routes::user";
        let fn_name = "PTHfirstname_PTHlastname";
        let path = parse_route_path(mod_path, fn_name);
        assert_eq!(path, "/user/{firstname}/{lastname}/");

    }
}