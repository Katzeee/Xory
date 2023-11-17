use anyhow::Result;
use chrono::Utc;
use common::{
    entity::{diary, sea_orm_active_enums::Weather},
    error::ReqErr,
};
use sea_orm::{
    entity::prelude::{DateTime, Decimal},
    sea_query::SimpleExpr,
    ActiveModelTrait,
    ActiveValue::Set,
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
};
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
    pub longtitude: Option<Decimal>,
    pub latitude: Option<Decimal>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct DiaryListReq {
    pub uid: u32,
    pub page_number: u32,
    pub page_size: u32,
    pub keywords: String,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiaryListRes {
    pub id: u64,
    pub title: String,
    pub content: Option<String>,
    pub category: u32,
    pub date: DateTime,
}

pub async fn list(
    db: &DatabaseConnection,
    diary_list_request: DiaryListReq,
) -> Result<Vec<DiaryListRes>> {
    let mut diaries = diary::Entity::find().filter(diary::Column::Uid.eq(diary_list_request.uid));
    let keywords: Vec<String> = serde_json::from_str(&diary_list_request.keywords)?;
    let keyword_expr = keywords
        .into_iter()
        .map(|keyword| {
            diary::Column::Title
                .like(format!("%{keyword}%"))
                .or(diary::Column::Content.like(format!("%{keyword}%")))
        })
        .fold(None, |acc: Option<SimpleExpr>, cur| match acc {
            Some(expr) => Some(expr.and(cur)),
            None => Some(cur),
        });
    if let Some(keyword_expr) = keyword_expr {
        diaries = diaries.filter(keyword_expr);
    }
    let category: Vec<u32> = serde_json::from_str(&diary_list_request.category)?;
    println!("{:?}", category);
    let category_expr = category
        .into_iter()
        .fold(None, |acc: Option<SimpleExpr>, cur| match acc {
            Some(expr) => Some(expr.or(diary::Column::Category.eq(cur))),
            None => Some(diary::Column::Category.eq(cur)),
        });
    if let Some(category_expr) = category_expr {
        diaries = diaries.filter(category_expr);
    }
    let diaries = diaries.order_by_asc(diary::Column::Date).all(db).await?;
    let diaries = diaries
        .into_iter()
        .map(|diary| DiaryListRes {
            id: diary.id,
            title: diary.title,
            content: diary.content,
            category: diary.category,
            date: diary.date,
        })
        .collect();
    Ok(diaries)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiaryDetailReq {
    pub id: u64,
}

pub async fn detail(
    db: &DatabaseConnection,
    diary_detail_request: DiaryDetailReq,
) -> Result<diary::Model> {
    let diary = diary::Entity::find_by_id(diary_detail_request.id)
        .one(db)
        .await?;
    diary.ok_or(ReqErr::NoResError.into())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiaryModifyReq {
    pub id: u64,
    pub date: DateTime,
    pub title: String,
    pub content: Option<String>,
    pub temperature: Option<i8>,
    pub weather: Option<Weather>,
    pub category: u32,
    pub longitude: Option<Decimal>,
    pub latitude: Option<Decimal>,
}

pub async fn modify(
    db: &DatabaseConnection,
    diary_modify_request: DiaryModifyReq,
) -> Result<diary::Model> {
    let mut diary: diary::ActiveModel = diary::Entity::find_by_id(diary_modify_request.id)
        .one(db)
        .await?
        .unwrap()
        .into();
    diary.date = Set(diary_modify_request.date);
    diary.title = Set(diary_modify_request.title);
    diary.content = Set(diary_modify_request.content);
    diary.temperature = Set(diary_modify_request.temperature);
    diary.weather = Set(diary_modify_request.weather);
    diary.category = Set(diary_modify_request.category);
    diary.longitude = Set(diary_modify_request.longitude);
    diary.latitude = Set(diary_modify_request.latitude);
    diary.date_modify = Set(Utc::now().naive_utc());
    let diary = diary.update(db).await?;
    Ok(diary)
}
