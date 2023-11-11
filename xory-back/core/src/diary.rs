use common::entity::sea_orm_active_enums::Weather;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiaryAddReq {
    pub id: i32,
    pub title: String,
    pub content: Option<String>,
    pub temperature: Option<i8>,
    pub weather: Option<Weather>,
    pub category: i32,
    pub uid: u32,
}
