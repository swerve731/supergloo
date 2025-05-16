# Supergloo

Supergloo is a Rust library designed to simplify route creation in [Axum](https://github.com/tokio-rs/axum) web applications. It automatically generates routes based on your handler functions' module paths and names, reducing boilerplate code. The routing is similar to svelte where the route is decided by the module path and function name. layouts will render on all children that are in their dir. use gloo_handler("view") to glue layouts to the handler. gloo_handler can take any http request type (get || post || options || delete etc...) example: gloo_handler("post").

## Features

- **Automatic Route Generation**: Define your handlers in modules, and Supergloo automatically maps them to URL paths.
- **Convention-Based Routing**: Follows a simple convention: `crate::routes::path::to::handler_fn` becomes `/path/to/handler_fn/`.
- **Macro-Based**: Uses the `#[gloo_handler]` attribute macro to mark functions as route handlers.
- **Simple Integration**: Easily merge generated routes into your main Axum router.

## Installation

Add `supergloo` and its companion macro crate `gloo_macros` to your `Cargo.toml`:

```toml
[dependencies]
axum = "0.7" # Or your desired version
tokio = { version = "1", features = ["full"] } # Needed for the Axum runtime
supergloo = "*" # Or use git/crates.io version
gloo_macros = "*" # Or use git/crates.io version
```

## Usage

### Define Handlers

Create your Axum handler functions within modules, typically under a `src/routes/` directory. Mark each handler function with the `#[gloo_handler]` attribute. The attribute can optionally take the HTTP method as a string (e.g., "post", "put"). If omitted, it defaults to "get".

```rust
// src/routes/mod.rs
use axum::response::{IntoResponse, Response};
use gloo_macros::gloo_handler;

// This creates the route: GET /
#[gloo_handler]
async fn base() -> Response {
    "Welcome home!".into_response()
}

// This creates the route: GET /dashboard/
#[gloo_handler("get")]
async fn dashboard() -> Response {
    "User Dashboard".into_response()
}

pub mod api {
    // src/routes/api/mod.rs
    use axum::response::{IntoResponse, Response};
    use gloo_macros::gloo_handler;

    #[gloo_handler]
    async fn base() -> Response {
        "API Base".into_response()
    }

    pub mod users {
        // src/routes/api/users.rs
        use axum::response::{IntoResponse, Response};
        use gloo_macros::gloo_handler;

        #[gloo_handler]
        async fn base() -> Response {
            "List users".into_response()
        }

        #[gloo_handler("post")]
        async fn create() -> Response {
            "Create user".into_response()
        }
    }
}
```

### Routing Convention

Supergloo uses the `module_path!()` macro to determine the base path.

- Removes the leading parts of the path up to and including the first `routes` segment. For example, `my_crate::routes::api::users` becomes `api::users`.
- Segments are joined by `/`.
- If the function name is `base`, it maps to the module's path (e.g., `routes::api::base` -> `/api/`).
- If the function name is not `base`, the function name is appended (e.g., `routes::api::users::create` -> `/api/users/create/`).
- Paths are normalized to start and end with `/`.

### Integrate Routes

In your `main.rs` or wherever you set up your Axum application, import the necessary items and use the `.gloo_routes()` method on your `axum::Router`. Make sure to import your routes module to ensure the handlers are discovered.

```rust
// src/main.rs
use axum::Router;
use supergloo::routing::GlooRouting;

mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .gloo_routes()
        .await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
```

### Run

Compile and run your application:

```bash
cargo run
```

Then access the routes:

- http://127.0.0.1:3000/
- http://127.0.0.1:3000/dashboard/
- http://127.0.0.1:3000/api/
- http://127.0.0.1:3000/api/users/
- http://127.0.0.1:3000/api/users/create/

## How It Works

Supergloo uses the [`inventory`](https://crates.io/crates/inventory) crate behind the scenes.

- The `#[gloo_handler]` macro wraps your function and registers a `GlooHandler` struct (containing the module path, function name, and a function pointer to create the Axum `MethodRouter`) with `inventory::submit!`.
- `.gloo_routes()` iterates through all registered `GlooHandler` instances using `inventory::iter`.
- For each handler, it processes the `module_path` and `fn_name` to construct the final URL path according to the conventions described.
- Creates the specific `axum::routing::MethodRouter` (e.g., `get(handler_fn)`) and adds it to a new router.
- Merges this router containing all discovered routes into the router you called `.gloo_routes()` on.

## License

This project is licensed under the [MIT License](LICENSE) or [Apache License 2.0](LICENSE-APACHE).

## Future Features


Config based layering, future versions will have a config file where you can define your cors rules and static directories to serve

Cargo generate template, a template that has all the necessary default configuration of cors and routing to start your app


I plan on working on deep integrations focused at Maud and HTMX
