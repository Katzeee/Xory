use axum::{
    routing::{get, MethodRouter},
    Router,
};

pub fn routes() -> Router {
    Router::new().route("/add", add())
}

pub fn add() -> MethodRouter {
    get(|| async { "User add" })
}
