use anyhow::Ok;
use anyhow::Result;
use common::entity::{diary, sea_orm_active_enums::Weather};
use sea_orm::entity::prelude::DateTime;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiaryAddReq {
    pub date: DateTime,
    pub title: String,
    pub content: Option<String>,
    pub temperature: Option<i8>,
    pub weather: Option<Weather>,
    pub category: u32,
    pub uid: u32,
    pub longtitude: Option<f32>,
    pub latitude: Option<f32>,
}

pub async fn add(db: &DatabaseConnection, diary_add_request: DiaryAddReq) -> Result<diary::Model> {
    let diary = diary::ActiveModel {
        date: Set(diary_add_request.date),
        title: Set(diary_add_request.title),
        content: Set(diary_add_request.content),
        temperature: Set(diary_add_request.temperature),
        weather: Set(diary_add_request.weather),
        category: Set(diary_add_request.category),
        uid: Set(diary_add_request.uid),
        longitude: Set(diary_add_request.longtitude),
        latitude: Set(diary_add_request.latitude),

        ..Default::default()
    };
    let diary = diary.insert(db).await?;
    Ok(diary)
}
