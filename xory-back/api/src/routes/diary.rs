use crate::{AppState, StateMethodRouter, StateRoute};
use axum::{
    extract::{Json, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_extra::extract::WithRejection;
use common::response::CommonRes;
use core::diary::DiaryAddReq;

pub fn routes() -> StateRoute {
    Router::new()
        .route("/list", list())
        .route("/add", post(add))
}

pub fn list() -> StateMethodRouter {
    get(|| async { "Diary list" })
}

pub async fn add(
    state: State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<DiaryAddReq>, CommonRes<()>>,
) -> impl IntoResponse {
    let res = core::diary::add(&state.db, req).await;
    match res {
        Ok(diary) => CommonRes::success(diary),
        Err(err) => CommonRes::error(err),
    }
}
