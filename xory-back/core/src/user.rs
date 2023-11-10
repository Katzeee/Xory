use anyhow::Result;
use common::{entity::user, error::ReqErr};
use middleware_fn::auth::{create_token, Claims};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegisterReq {
    pub email: String,
    pub username: String,
    pub password: String,
    pub comment: Option<String>,
    pub wechat: Option<String>,
    pub phone_number: Option<String>,
    pub group_id: i32,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginReq {
    pub email: String,
    pub password: String,
}

pub async fn login(db: &DatabaseConnection, user_login_request: UserLoginReq) -> Result<String> {
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(&user_login_request.email))
        .one(db)
        .await?;
    match user {
        Some(user) => {
            let claims = Claims {
                id: user.uid,
                email: user.email,
                ..Default::default()
            };
            Ok(create_token(claims).await)
        }
        None => Err(ReqErr::LoginError.into()),
    }
}

pub async fn register_user(
    db: &DatabaseConnection,
    user_register_request: UserRegisterReq,
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
