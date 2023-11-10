use anyhow::Result;
use common::entity::user;
use sea_orm::{ActiveModelTrait, DatabaseConnection};

pub async fn register_user(db: &DatabaseConnection) -> Result<user::Model> {
    let user = user::ActiveModel {
        ..Default::default()
    };

    let user: user::Model = user.insert(db).await?;
    Ok(user)
}
