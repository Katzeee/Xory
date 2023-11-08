use axum::{
    routing::{get, MethodRouter},
    Router,
};

pub fn routes() -> Router {
    Router::new().route("/list", list())
}

pub fn list() -> MethodRouter {
    get(|| async { "Diary list" })
}
