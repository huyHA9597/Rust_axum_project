use axum::{routing::get, Router};

pub fn create_route() -> Router {
    Router::new().route("/", get(|| async { "Hello World!!!" }))
}
