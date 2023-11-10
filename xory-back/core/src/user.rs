use anyhow::Result;
use common::entity::user;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub comment: Option<String>,
    pub wechat: Option<String>,
    pub phone_number: Option<String>,
    pub group_id: i32,
    pub avatar: Option<String>,
}

pub async fn register_user(
    db: &DatabaseConnection,
    user_register_request: UserRegisterRequest,
) -> Result<user::Model> {
    let user = user::ActiveModel {
        email: Set(user_register_request.email),
        username: Set(user_register_request.username),
        password: Set(user_register_request.password),
        comment: Set(user_register_request.comment),
        wechat: Set(user_register_request.wechat),
        phone_number: Set(user_register_request.phone_number),
        group_id: Set(user_register_request.group_id),
        avatar: Set(user_register_request.avatar),
        ..Default::default()
    };

    let user = user.insert(db).await?;
    Ok(user)
}
