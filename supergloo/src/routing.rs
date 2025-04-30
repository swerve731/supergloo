// /home/nioe/projects/supergloo/supergloo/src/routing.rs
use axum::{routing::get, response::Html}; // Import Html response type

use crate::{layout::GlooLayout, utils, GlooHandler, GlooHandlerKind}; // Import GlooHandlerKind


#[async_trait::async_trait]
pub trait GlooRouting {
    async fn gloo_routes(self) -> axum::Router;
}


#[async_trait::async_trait]
impl GlooRouting for axum::Router {
    async fn gloo_routes(self) -> axum::Router {
        let mut router = axum::Router::new();

        // create a router with all the handlers and generate the paths
        for handler in inventory::iter::<GlooHandler> {
            let mut path = utils::route_path_from_mod_path(&handler.full_mod_path());

            // If the function name is 'base', use the directory path directly
            if handler.fn_name == "root" {
                 path = utils::route_path_from_mod_path(handler.path);
            } else {
                 path = format!("{}/{}", utils::route_path_from_mod_path(handler.path), handler.fn_name);
            }
            // Ensure path starts with / and doesn't end with / unless it's the root
            if !path.starts_with('/') {
                path = format!("/{}", path);
            }


            dbg!(&path);
            // dbg!(&handler.path);
            // dbg!(&handler.kind);

            // Determine the actual Axum handler based on GlooHandlerKind
            let route_handler = match &handler.kind {
                GlooHandlerKind::View(view_fn_wrapper) => {
                    let mut used_layouts: Vec<fn(maud::Markup) -> maud::Markup> = vec![]; // Explicit type
                    for layout in inventory::iter::<GlooLayout> {

                        // dbg!(layout);
                        let layout_path_str = utils::route_path_from_mod_path(layout.path);
                        // dbg!(&layout_path_str);
                        // Handle root layout case
                        if layout_path_str == "/" {
                             // Apply root layout if the handler path is also effectively root or starts with it
                             // (This logic might need refinement based on desired root layout behavior)
                             if path == "/" || path.starts_with('/') { // Apply root layout broadly
                                used_layouts.push(layout.layout_fn);
                             }
                             continue; // Move to next layout
                        }

                        // Skip empty layout paths if not root
                        // if layout_path_str.is_empty() || layout_path_str == "/" {
                         //   continue;
                        //}

                        let layout_path_segments: Vec<&str> = layout_path_str.trim_matches('/').split('/').collect();
                        let handler_path_segments: Vec<&str> = path.trim_matches('/').split('/').collect();

                        // dbg!(&layout_path_segments);
                        // dbg!(&handler_path_segments);

                        // Check if the handler path starts with the layout path
                        if handler_path_segments.len() >= layout_path_segments.len() &&
                           handler_path_segments[..layout_path_segments.len()] == layout_path_segments[..] {
                            used_layouts.push(layout.layout_fn);
                        }
                    }

                    // Sort layouts by path depth (descending) so most specific applies first
                    // This requires storing path along with layout_fn temporarily or adjusting collection
                    // For now, we apply in collected order, which might not be specific enough.
                    // A better approach would store (path_depth, layout_fn) and sort.

                    // Create the final async handler
                    let final_handler = get(move || async move {
                        // Call the wrapper to get the boxed future, then await it
                        let mut markup = view_fn_wrapper().await;

                        // Apply layouts in reverse order of collection (innermost first)
                        // Or sort them by specificity if path info was stored
                        for layout in used_layouts.iter().rev() { // Apply layouts
                            markup = layout(markup);
                        }
                        Html(markup.into_string()) // Wrap markup in Html response
                    });
                    final_handler // Return the MethodRouter created by get()
                }
                GlooHandlerKind::Route => {
                    // For non-view routes, just use the router function provided
                    (handler.router)()
                }
            };

            router = router.route(&path, route_handler); // Add the route to the router

        }

        self.merge(router)
    }
}
