use anyhow::Result;
use common::entity::{diary, sea_orm_active_enums::Weather};
use sea_orm::{
    entity::prelude::{DateTime, Decimal},
    sea_query::SimpleExpr,
    ActiveModelTrait,
    ActiveValue::Set,
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
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
    pub page_number: u32,
    pub page_size: u32,
    pub keywords: String,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiaryListRes {
    pub title: String,
    pub content: Option<String>,
    pub category: u32,
}

pub async fn list(
    db: &DatabaseConnection,
    diary_list_request: DiaryListReq,
) -> Result<Vec<DiaryListRes>> {
    let keywords: Vec<String> = serde_json::from_str(&diary_list_request.keywords)?;
    println!("{:?}", keywords);
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

    let mut diaries = diary::Entity::find();
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

    // println!("{}", diaries.debug_print());

    let diaries = diaries.all(db).await?;

    let diaries = diaries
        .into_iter()
        .map(|diary| DiaryListRes {
            title: diary.title,
            content: diary.content,
            category: diary.category,
        })
        .collect();

    Ok(diaries)
}
