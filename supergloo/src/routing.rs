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
            if handler.fn_name != "root" {
                path = format!("{}/{}", path, handler.fn_name);
            }
            if !path.starts_with("/") {
                path = format!("/{}", path);
            }

            if !path.ends_with("/") {
                path = format!("{}/", path);
            }
            
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