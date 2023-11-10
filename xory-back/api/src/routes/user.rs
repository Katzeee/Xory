use crate::{AppState, StateRoute};
use axum::{
    extract::{Json, Query, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_extra::extract::WithRejection;
use common::{entity::user, error::ReqErr, response};
use core::user::{register_user, UserRegisterRequest};
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};

pub fn routes() -> StateRoute {
    Router::new()
        .route("/add", post(add))
        .route("/register", post(register))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct A {
    a: String,
}

pub async fn add(
    state: State<AppState>,
    req: WithRejection<Json<UserRegisterRequest>, response::Res<()>>,
) -> impl IntoResponse {
    // let data = format!("{}", param.a);
    // response::Res::success(data)
    let res = register_user(&state.db, req.0 .0).await;
    match res {
        Ok(user) => response::Res::success(user),
        Err(err) => response::Res::error(err),
    }
}

pub async fn register(
    param: WithRejection<Json<user::Model>, response::Res<()>>,
) -> impl IntoResponse {
    response::Res::success(param.0 .0)
}
