use axum::{routing::get, Router};
use crate::{StateMethodRouter, StateRoute};

pub fn routes() -> StateRoute {
    Router::new().route("/add", add())
}

pub fn add() -> StateMethodRouter {
    get(|| async { "User add" })
}
