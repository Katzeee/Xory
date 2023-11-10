use crate::{StateMethodRouter, StateRoute};
use anyhow::Result;
use axum::{routing::get, Router};
use common::{error::ReqErr, res};

pub fn routes() -> StateRoute {
    Router::new().route("/add", add())
}

pub fn add() -> StateMethodRouter {
    get(|| async {
        let err: Result<String> = Err(ReqErr::ReqParamError("I don't know".into()).into());
        // let err: Result<String> = err.into();
        match err {
            Ok(data) => res::Res::success(data),
            Err(err) => res::Res::error(err),
        }
    })
}