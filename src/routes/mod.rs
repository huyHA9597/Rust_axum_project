mod hello_world;
mod mirror_body_string;

use axum::{
    routing::{delete, get, patch, post, put},
    Router,
};
use hello_world::hello_world;
use mirror_body_string::mirror_body_string;

pub fn create_route() -> Router {
    Router::new()
        .route("/hello_world", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
}
