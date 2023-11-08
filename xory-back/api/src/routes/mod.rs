use axum::Router;
mod diary;
mod user;

pub fn compose() -> Router {
    Router::new()
        .nest("/diary", diary::routes())
        .nest("/user", user::routes())
}
