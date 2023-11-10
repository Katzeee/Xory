use anyhow::Error;
use axum::{response::IntoResponse, Json};
use serde::Serialize;
use serde_json::json;

use crate::error::ReqErr;

#[derive(Debug, Serialize)]
pub struct Res<T> {
    code: i32,
    data: Option<T>,
    message: String,
}

impl<T> Res<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            data: data.into(),
            message: "success".into(),
        }
    }

    pub fn error(e: Error) -> Self {
        let code = if e.downcast_ref::<ReqErr>().is_some() {
            match e.downcast_ref::<ReqErr>() {
                _ => 400,
            }
        } else {
            500 // unexpected error
        };

        Self {
            code: code,
            data: Option::None,
            message: e.to_string(),
        }
    }
}

impl<T> IntoResponse for Res<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let val = json!(self);
        Json(val).into_response()
    }
}
