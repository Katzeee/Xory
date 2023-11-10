use axum::{
    extract::rejection::{JsonRejection, QueryRejection},
    response::IntoResponse,
    Json,
};
use sea_orm::DbErr;
use serde::Serialize;
use serde_json::json;

use crate::error::ReqErr;

#[derive(Debug, Serialize)]
pub struct CommonRes<T> {
    code: i32,
    data: Option<T>,
    message: String,
}

impl<T> CommonRes<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            data: data.into(),
            message: "success".into(),
        }
    }

    pub fn error(err: anyhow::Error) -> Self {
        let code = if err.downcast_ref::<ReqErr>().is_some() {
            match err.downcast_ref::<ReqErr>() {
                Some(ReqErr::ReqParamError(_)) => 1001,
                _ => 1000,
            }
        } else if err.downcast_ref::<DbErr>().is_some() {
            match err.downcast_ref::<DbErr>() {
                Some(DbErr::Exec(_)) => 2001,
                _ => 2000,
            }
        } else {
            5000 // unexpected error
        };

        Self {
            code: code,
            data: None,
            message: err.to_string(),
        }
    }
}

impl<T> IntoResponse for CommonRes<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let val = json!(self);
        Json(val).into_response()
    }
}

impl From<JsonRejection> for CommonRes<()> {
    fn from(value: JsonRejection) -> Self {
        Self {
            code: value.status().as_u16().into(),
            data: None,
            message: value.body_text(),
        }
    }
}

impl From<QueryRejection> for CommonRes<()> {
    fn from(value: QueryRejection) -> Self {
        Self {
            code: value.status().as_u16().into(),
            data: None,
            message: value.body_text(),
        }
    }
}
