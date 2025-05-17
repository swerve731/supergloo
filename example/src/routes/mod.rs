pub mod api;


use axum::extract::Path;
use axum::response::{Response, IntoResponse};
use supergloo::{gloo_handler, GlooHandler};


// this creates the "/" route 
#[gloo_handler("get")]
async fn root() -> Response {
    "home".into_response()
} 


// this creates the "/dashboard" route
#[gloo_handler("get")]
async fn dashboard() -> Response {
    "dashboard".into_response()
}

// add PTH to the start of a section to make a path var
// this expands to /user/{name}/team/{teamname} 
#[gloo_handler("get")]
pub async fn user_PTHname_team_PTHteamname (Path((name, teamname)): Path<(String, String)>) -> impl IntoResponse{
    format!("hello, {} with team {}!", name, teamname).into_response()
}