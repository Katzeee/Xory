use crate::StateRoute;
use axum::Router;
mod diary;
mod user;

pub fn compose() -> StateRoute {
    Router::new()
        .nest("/diary", diary::routes())
        .nest("/user", user::routes())
}
