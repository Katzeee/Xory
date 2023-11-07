use std::intrinsics::mir::Discriminant;

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum UserGroup {
    Table,
    Id,
    Name,
    Description,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Uid,
    Email,
    Username,
    Password,
    RegisterTime,
    LastVisitTime,
    Comment,
    Wechat,
    PhoneNumber,
    GroupId,
    DiaryCount,
    Avatar,
}

#[derive(DeriveIden)]
enum DiaryCategory {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum Diary {
    Table,
    Id,
    Date,
    Title,
    Content,
    Temperature,
    Weather,
    Category,
    DateCreate,
    DateModify,
    Uid,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserGroup::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserGroup::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserGroup::Name).string_len(50).not_null())
                    .col(ColumnDef::new(UserGroup::Description).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                INSERT INTO `user_group` VALUES (1, 'admin', null); 
                INSERT INTO `user_group` VALUES (2, 'user', null);
                "#,
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Uid)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Email).string_len(50).not_null())
                    .col(ColumnDef::new(User::Username).string_len(50).not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(
                        ColumnDef::new(User::RegisterTime)
                            .timestamp()
                            // .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::LastVisitTime)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::Comment).string().null())
                    .col(ColumnDef::new(User::Wechat).string().null())
                    .col(ColumnDef::new(User::PhoneNumber).string_len(20).null())
                    .col(
                        ColumnDef::new(User::GroupId)
                            .integer()
                            .not_null()
                            .default(2i32),
                    )
                    .col(
                        ColumnDef::new(User::DiaryCount)
                            .integer()
                            .not_null()
                            .default(0i32),
                    )
                    .col(ColumnDef::new(User::Avatar).string().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("group_id")
                            .from_col(User::GroupId)
                            .to(UserGroup::Table, UserGroup::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(DiaryCategory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DiaryCategory::Id)
                            .integer()
                            .not_null()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(DiaryCategory::Name)
                            .string_len(50)
                            .not_null(),
                    )
                    .primary_key(
                        IndexCreateStatement::new()
                            .col(DiaryCategory::Id)
                            .index_type(IndexType::BTree),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                INSERT INTO `diary_category` VALUES (null, 'memo'); 
                INSERT INTO `diary_category` VALUES (null, 'study');
                INSERT INTO `diary_category` VALUES (null, 'work');
                INSERT INTO `diary_category` VALUES (null, 'entertainment');
                "#,
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Diary::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Diary::Id)
                            .integer()
                            .not_null()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Diary::Date).date_time().not_null())
                    .col(ColumnDef::new(Diary::Title).string().not_null())
                    .col(ColumnDef::new(Diary::Content).string().null())
                    .col(ColumnDef::new(Diary::Temperature).tiny_integer().null())
                    .col(
                        ColumnDef::new(Diary::Weather), // .enumeration(, vec!["sunny", "cloudy"])
                                                        // .default("sunny"),
                    )
                    .col(ColumnDef::new(Diary::Category).integer().not_null())
                    .col(ColumnDef::new(Diary::DateCreate).date_time().not_null())
                    .col(
                        ColumnDef::new(Diary::DateModify)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Diary::Uid).integer().not_null())
                    .primary_key(
                        IndexCreateStatement::new()
                            .col(Diary::Id)
                            .index_type(IndexType::BTree),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from(User::Table, User::Uid)
                            .to_col(Diary::Uid)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserGroup::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(DiaryCategory::Table).to_owned())
            .await?;
        Ok(())
    }
}
