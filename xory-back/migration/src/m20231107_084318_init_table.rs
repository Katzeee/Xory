use crate::db_utils::init_data;
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
    Salt,
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
enum DiaryTag {
    Table,
    Tid,
    Name,
}

#[derive(DeriveIden)]
enum UserToDiaryTag {
    Table,
    Uid,
    Tid,
}

#[derive(DeriveIden)]
enum Diary {
    Table,
    Did,
    Date,
    Title,
    Content,
    Temperature,
    Weather,
    Mood,
    DateCreate,
    DateModify,
    Uid,
    Longitude,
    Latitude,
}

#[derive(DeriveIden)]
enum DiaryToDiaryTag {
    Table,
    Did,
    Tid,
}

#[derive(DeriveIden)]
enum MoodTypes {
    Enum,
    Elated,
    Content,
    Neutral,
    Displeased,
    Miserable,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Set default for those data not get from front end
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserGroup::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserGroup::Id)
                            .unsigned()
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
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::Email)
                            .string_len(50)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::Username).string_len(50).not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Salt).string().not_null())
                    .col(
                        ColumnDef::new(User::RegisterTime)
                            .timestamp()
                            .default(Expr::current_timestamp())
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
                            .unsigned()
                            .not_null()
                            .default(2u32),
                    )
                    .col(
                        ColumnDef::new(User::DiaryCount)
                            .integer()
                            .not_null()
                            .default(0u32),
                    )
                    .col(ColumnDef::new(User::Avatar).string().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_user_group_group_id")
                            .from_col(User::GroupId)
                            .to(UserGroup::Table, UserGroup::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(DiaryTag::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DiaryTag::Tid)
                            .unsigned()
                            .not_null()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(DiaryTag::Name).string_len(50).not_null())
                    .primary_key(
                        IndexCreateStatement::new()
                            .col(DiaryTag::Tid)
                            .index_type(IndexType::Hash),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                INSERT INTO `diary_tag` VALUES (null, 'uncategorized'); 
                INSERT INTO `diary_tag` VALUES (null, 'study');
                INSERT INTO `diary_tag` VALUES (null, 'work');
                INSERT INTO `diary_tag` VALUES (null, 'entertainment');
                INSERT INTO `diary_tag` VALUES (null, 'sports');
                INSERT INTO `diary_tag` VALUES (null, 'memo'); 
                "#,
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Diary::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Diary::Did)
                            .big_unsigned()
                            .not_null()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Diary::Date).date_time().not_null())
                    .col(ColumnDef::new(Diary::Title).string().not_null())
                    .col(ColumnDef::new(Diary::Content).text().null())
                    .col(ColumnDef::new(Diary::Temperature).tiny_integer().null())
                    .col(ColumnDef::new(Diary::Mood).enumeration(
                        MoodTypes::Enum,
                        [
                            MoodTypes::Elated,
                            MoodTypes::Content,
                            MoodTypes::Neutral,
                            MoodTypes::Displeased,
                            MoodTypes::Miserable,
                        ],
                    ))
                    .col(ColumnDef::new(Diary::Weather).small_unsigned().default(0))
                    .col(
                        ColumnDef::new(Diary::DateCreate)
                            .date_time()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Diary::DateModify)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Diary::Uid).unsigned().not_null())
                    .col(ColumnDef::new(Diary::Longitude).decimal_len(9, 6).null())
                    .col(ColumnDef::new(Diary::Latitude).decimal_len(9, 6).null())
                    .primary_key(
                        IndexCreateStatement::new()
                            .col(Diary::Did)
                            .index_type(IndexType::Hash),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_diary_user_uid")
                            .from_col(Diary::Uid)
                            .to(User::Table, User::Uid)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserToDiaryTag::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserToDiaryTag::Uid).unsigned().not_null())
                    .col(ColumnDef::new(UserToDiaryTag::Tid).unsigned().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_user_to_diary_tag_user_uid")
                            .from_col(UserToDiaryTag::Uid)
                            .to(User::Table, User::Uid)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_user_to_diary_tag_diary_tag_tid")
                            .from_col(UserToDiaryTag::Tid)
                            .to(DiaryTag::Table, DiaryTag::Tid)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        IndexCreateStatement::new()
                            .col(UserToDiaryTag::Uid)
                            .col(UserToDiaryTag::Tid),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(DiaryToDiaryTag::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DiaryToDiaryTag::Did)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(DiaryToDiaryTag::Tid).unsigned().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_diary_to_diary_tag_diary_did")
                            .from_col(DiaryToDiaryTag::Did)
                            .to(Diary::Table, Diary::Did)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_diary_to_diary_tag_diary_tag_tid")
                            .from_col(DiaryToDiaryTag::Tid)
                            .to(DiaryTag::Table, DiaryTag::Tid)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        IndexCreateStatement::new()
                            .col(DiaryToDiaryTag::Did)
                            .col(DiaryToDiaryTag::Tid),
                    )
                    .to_owned(),
            )
            .await?;

        match init_data(manager, Migration.name()).await {
            Ok(_) => Ok(()),
            Err(err) => {
                println!("{:?}", err);
                panic!()
            }
        }
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Diary::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserGroup::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(DiaryTag::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserToDiaryTag::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(DiaryToDiaryTag::Table).to_owned())
            .await?;
        Ok(())
    }
}
