use crate::{AppState, StateRoute};
use axum::{
    extract::{Json, Query, State},
    http::{header, HeaderMap, HeaderValue},
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_extra::extract::WithRejection;
use common::response;
use core::user::{register_user, UserRegisterReq};
use middleware_fn::auth::verify_token;
use serde::{Deserialize, Serialize};

pub fn routes() -> StateRoute {
    Router::new()
        .route("/detail", post(detail))
        .route("/add", post(add))
        .layer(middleware::from_fn(verify_token))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct A {
    a: String,
}

pub async fn add(
    state: State<AppState>,
    req: WithRejection<Json<UserRegisterReq>, response::Res<()>>,
) -> impl IntoResponse {
    let res = register_user(&state.db, req.0 .0).await;
    match res {
        Ok(user) => response::Res::success(user),
        Err(err) => response::Res::error(err),
    }
}

pub async fn login(
    state: State<AppState>,
    req: WithRejection<Json<UserRegisterReq>, response::Res<()>>,
) -> impl IntoResponse {
}

pub async fn detail(
    state: State<AppState>,
    header: HeaderMap, // param: WithRejection<Json<user::Model>, response::Res<()>>,
) -> impl IntoResponse {
    let token = header.get(header::AUTHORIZATION);

    // verify_token(token);
    response::Res::success("1")
}
