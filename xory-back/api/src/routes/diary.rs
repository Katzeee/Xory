use axum::{routing::get, Router};
use crate::{StateMethodRouter, StateRoute};

pub fn routes() -> StateRoute {
    Router::new().route("/list", list())
}

pub fn list() -> StateMethodRouter {
    get(|| async { "Diary list" })
}
