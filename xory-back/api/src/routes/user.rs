use crate::{AppState, StateRoute};
use axum::{
    extract::{Json, Query, State},
    http::{header, HeaderMap},
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_extra::extract::WithRejection;
use common::response::CommonRes;
use core::user::{UserAddReq, UserLoginReq};
use middleware_fn::auth::verify_token;

pub fn routes() -> StateRoute {
    Router::new()
        .layer(middleware::from_fn(verify_token))
        .route("/detail", post(detail))
        .route("/add", post(add))
        .route("/login", get(login))
}

pub async fn add(
    state: State<AppState>,
    WithRejection(Json(req), _): WithRejection<Json<UserAddReq>, CommonRes<()>>,
) -> impl IntoResponse {
    let res = core::user::add(&state.db, req).await;
    match res {
        Ok(user) => CommonRes::success(user),
        Err(err) => CommonRes::error(err),
    }
}

pub async fn login(
    state: State<AppState>,
    WithRejection(Query(req), _): WithRejection<Query<UserLoginReq>, CommonRes<()>>,
) -> impl IntoResponse {
    let res = core::user::login(&state.db, req).await;
    match res {
        Ok(token) => CommonRes::success(token),
        Err(err) => CommonRes::error(err),
    }
}

pub async fn detail(
    state: State<AppState>,
    WithRejection(Query(req), _): WithRejection<Query<UserLoginReq>, CommonRes<()>>,
) -> impl IntoResponse {
    CommonRes::success("1")
}
