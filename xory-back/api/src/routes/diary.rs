use crate::{AppState, StateRoute};
use axum::{
    extract::{Json, Query, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_extra::extract::WithRejection;
use common::response::CommonRes;
use core::diary::{DiaryAddReq, DiaryListReq, DiaryListRes};

pub fn routes() -> StateRoute {
    Router::new()
        .route("/list", get(list))
        .route("/add", post(add))
}

pub async fn list(
    state: State<AppState>,
    WithRejection(Query(req), _): WithRejection<Query<DiaryListReq>, CommonRes<()>>,
) -> impl IntoResponse {
    let res = core::diary::list(&state.db, req).await;
    match res {
        Ok(diary_list) => CommonRes::success(diary_list),
        Err(err) => CommonRes::error(err),
    }
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
