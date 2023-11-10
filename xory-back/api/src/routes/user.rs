use crate::{AppState, StateRoute};
use axum::{
    extract::{Json, Query, State},
    http::{header, HeaderMap, HeaderValue},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_extra::extract::WithRejection;
use common::response;
use core::{
    user::{register_user, UserRegisterRequest},
    utility::verify_token,
};
use serde::{Deserialize, Serialize};

pub fn routes() -> StateRoute {
    Router::new()
        .route("/add", post(add))
        .route("/detail", post(detail))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct A {
    a: String,
}

pub async fn add(
    state: State<AppState>,
    req: WithRejection<Json<UserRegisterRequest>, response::Res<()>>,
) -> impl IntoResponse {
    let res = register_user(&state.db, req.0 .0).await;
    match res {
        Ok(user) => response::Res::success(user),
        Err(err) => response::Res::error(err),
    }
}

pub async fn detail(
    state: State<AppState>,
    header: HeaderMap, // param: WithRejection<Json<user::Model>, response::Res<()>>,
) -> impl IntoResponse {
    let token: String = header
        .get(header::AUTHORIZATION)
        .unwrap()
        .to_str()
        .unwrap_or("")
        .into();
    verify_token(token);
    response::Res::success("1")
}
