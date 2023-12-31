use anyhow::Result;
use chrono::Utc;
use common::{
    entity::{diary, diary_to_diary_tag, sea_orm_active_enums::Mood},
    error::ReqErr,
};
use sea_orm::{
    entity::prelude::{DateTime, Decimal},
    sea_query::SimpleExpr,
    ActiveModelTrait,
    ActiveValue::Set,
    ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, QueryOrder,
    TransactionTrait,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiaryAddReq {
    pub date: DateTime,
    pub title: String,
    pub content: Option<String>,
    pub temperature: Option<i8>,
    pub weather: Option<u16>,
    pub mood: Option<Mood>,
    pub tags: Vec<u32>,
    pub uid: u32,
    pub longtitude: Option<Decimal>,
    pub latitude: Option<Decimal>,
}

pub async fn add(
    db: &DatabaseConnection,
    diary_add_request: DiaryAddReq,
) -> Result<DiaryDetailRes> {
    let diary = diary::ActiveModel {
        date: Set(diary_add_request.date),
        title: Set(diary_add_request.title),
        content: Set(diary_add_request.content),
        temperature: Set(diary_add_request.temperature),
        weather: Set(diary_add_request.weather),
        mood: Set(diary_add_request.mood),
        uid: Set(diary_add_request.uid),
        longitude: Set(diary_add_request.longtitude),
        latitude: Set(diary_add_request.latitude),
        ..Default::default()
    };
    let txn = db.begin().await?;
    let diary = diary.insert(&txn).await?;
    if !diary_add_request.tags.is_empty() {
        let diary_to_diary_tags = diary_add_request
            .tags
            .iter()
            .map(|tag| diary_to_diary_tag::ActiveModel {
                did: Set(diary.did),
                tid: Set(tag.to_owned()),
            })
            .collect::<Vec<diary_to_diary_tag::ActiveModel>>();
        diary_to_diary_tag::Entity::insert_many(diary_to_diary_tags)
            .exec(&txn)
            .await?;
    }
    txn.commit().await?;
    Ok(DiaryDetailRes::from_diary_and_tags(
        diary,
        diary_add_request.tags,
    ))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiaryListReq {
    pub uid: u32,
    pub page_number: u32,
    pub page_size: u32,
    pub keywords: String,
    pub tags: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiaryListRes {
    pub did: u64,
    pub title: String,
    pub content: Option<String>,
    pub tags: Vec<u32>,
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
                .contains(&keyword)
                .or(diary::Column::Content.contains(&keyword))
        })
        .fold(None, |acc: Option<SimpleExpr>, cur| match acc {
            Some(expr) => Some(expr.and(cur)),
            None => Some(cur),
        });
    if let Some(keyword_expr) = keyword_expr {
        diaries = diaries.filter(keyword_expr);
    }
    let tags: Vec<u32> = serde_json::from_str(&diary_list_request.tags)?;
    println!("{:?}", tags);
    let tag_expr = tags
        .into_iter()
        .fold(None, |acc: Option<SimpleExpr>, cur| match acc {
            Some(expr) => Some(expr.or(diary_to_diary_tag::Column::Tid.eq(cur))),
            None => Some(diary_to_diary_tag::Column::Tid.eq(cur)),
        });
    if let Some(tag_expr) = tag_expr {
        diaries = diaries.filter(tag_expr);
    }
    let diaries = diaries
        .order_by_asc(diary::Column::Date)
        .find_with_related(diary_to_diary_tag::Entity)
        .all(db)
        .await?;
    let diaries = diaries
        .into_iter()
        .map(|(diary, diary_tag)| DiaryListRes {
            did: diary.did,
            title: diary.title,
            content: diary.content,
            tags: diary_tag.into_iter().map(|tag| tag.tid).collect(),
            date: diary.date,
        })
        .collect();
    Ok(diaries)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiaryDetailReq {
    pub did: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiaryDetailRes {
    pub did: u64,
    pub date: DateTime,
    pub title: String,
    pub content: Option<String>,
    pub temperature: Option<i8>,
    pub weather: Option<u16>,
    pub mood: Option<Mood>,
    pub date_create: DateTime,
    pub date_modify: DateTime,
    pub uid: u32,
    pub longitude: Option<Decimal>,
    pub latitude: Option<Decimal>,
    pub tags: Vec<u32>,
}

impl DiaryDetailRes {
    fn from_diary_and_tags(diary: diary::Model, tags: Vec<u32>) -> Self {
        Self {
            did: diary.did,
            date: diary.date,
            title: diary.title,
            content: diary.content,
            temperature: diary.temperature,
            weather: diary.weather,
            mood: diary.mood,
            date_create: diary.date_create,
            date_modify: diary.date_modify,
            uid: diary.uid,
            longitude: diary.longitude,
            latitude: diary.latitude,
            tags: tags,
        }
    }
}

pub async fn detail(
    db: &DatabaseConnection,
    diary_detail_request: DiaryDetailReq,
) -> Result<DiaryDetailRes> {
    let diary = diary::Entity::find_by_id(diary_detail_request.did)
        .find_with_related(diary_to_diary_tag::Entity)
        .all(db)
        .await?;
    if diary.is_empty() {
        Err(ReqErr::NoResError.into())
    } else {
        let (diary, tags) = diary[0].to_owned();
        Ok(DiaryDetailRes::from_diary_and_tags(
            diary,
            tags.into_iter().map(|tag| tag.tid).collect(),
        ))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiaryModifyReq {
    pub did: u64,
    pub date: DateTime,
    pub title: String,
    pub content: Option<String>,
    pub temperature: Option<i8>,
    pub weather: Option<u16>,
    pub mood: Option<Mood>,
    pub add_tags: Vec<u32>,
    pub rm_tags: Vec<u32>,
    pub longitude: Option<Decimal>,
    pub latitude: Option<Decimal>,
}

pub async fn modify(
    db: &DatabaseConnection,
    diary_modify_request: DiaryModifyReq,
) -> Result<DiaryDetailRes> {
    let mut diary: diary::ActiveModel = diary::Entity::find_by_id(diary_modify_request.did)
        .one(db)
        .await?
        .unwrap()
        .into();
    diary.date = Set(diary_modify_request.date);
    diary.title = Set(diary_modify_request.title);
    diary.content = Set(diary_modify_request.content);
    diary.temperature = Set(diary_modify_request.temperature);
    diary.weather = Set(diary_modify_request.weather);
    diary.mood = Set(diary_modify_request.mood);
    diary.longitude = Set(diary_modify_request.longitude);
    diary.latitude = Set(diary_modify_request.latitude);
    diary.date_modify = Set(Utc::now().naive_utc());
    let txn = db.begin().await?;
    let diary = diary.update(&txn).await?;
    if !diary_modify_request.rm_tags.is_empty() {
        let diary_to_diary_tags_rm_expr =
            diary_modify_request
                .rm_tags
                .into_iter()
                .fold(None, |expr: Option<SimpleExpr>, tag| match expr {
                    Some(expr) => Some(
                        expr.or(diary_to_diary_tag::Column::Did
                            .eq(diary_modify_request.did)
                            .and(diary_to_diary_tag::Column::Tid.eq(tag))),
                    ),
                    None => Some(
                        diary_to_diary_tag::Column::Did
                            .eq(diary_modify_request.did)
                            .and(diary_to_diary_tag::Column::Tid.eq(tag)),
                    ),
                });
        diary_to_diary_tag::Entity::delete_many()
            .filter(diary_to_diary_tags_rm_expr.unwrap())
            .exec(&txn)
            .await?;
    }

    if !diary_modify_request.add_tags.is_empty() {
        let diary_to_diary_tags_add = diary_modify_request
            .add_tags
            .into_iter()
            .map(|tag| diary_to_diary_tag::ActiveModel {
                did: Set(diary_modify_request.did),
                tid: Set(tag),
            })
            .collect::<Vec<diary_to_diary_tag::ActiveModel>>();
        diary_to_diary_tag::Entity::insert_many(diary_to_diary_tags_add)
            .exec(&txn)
            .await?;
    }
    txn.commit().await?;
    let tags = diary
        .find_related(diary_to_diary_tag::Entity)
        .all(db)
        .await?;
    Ok(DiaryDetailRes::from_diary_and_tags(
        diary,
        tags.into_iter().map(|tag| tag.tid).collect(),
    ))
}
