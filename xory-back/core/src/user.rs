use anyhow::Result;
use bcrypt::{hash, DEFAULT_COST};
use common::{entity::user, error::ReqErr};
use middleware_fn::auth::{create_token, Claims};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAddReq {
    pub email: String,
    pub username: String,
    pub password: String,
    pub comment: Option<String>,
    pub wechat: Option<String>,
    pub phone_number: Option<String>,
    pub group_id: u32,
    pub avatar: Option<String>,
}

pub async fn add(
    db: &DatabaseConnection,
    user_register_request: UserAddReq,
) -> Result<user::Model> {
    let salt: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    let password = hash(user_register_request.password + &salt, DEFAULT_COST)?;
    let user = user::ActiveModel {
        email: Set(user_register_request.email),
        username: Set(user_register_request.username),
        password: Set(password),
        salt: Set(salt),
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
