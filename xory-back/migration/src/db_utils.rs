use crate::DATA_DIR;
use anyhow::{Context, Result};
use async_std::{
    fs::{self, File},
    io::{prelude::BufReadExt, BufReader},
    path::PathBuf,
    stream::StreamExt,
};
use sea_orm_migration::{
    prelude::*,
    sea_orm::{DatabaseBackend, Statement},
};

pub async fn init_data(manager: &SchemaManager<'_>, migration_name: &str) -> Result<()> {
    let db = manager.get_connection();
    let db_end = manager.get_database_backend();
    let dir = DATA_DIR.to_owned() + migration_name;
    let mut entries = fs::read_dir(&dir)
        .await
        .with_context(|| format!("Not any corresponding mockdata folder! No mockdata inserted."))?;
    while let Some(res) = entries.next().await {
        let path = res?.path();
        let sql_vec = get_insert_sql_string(path.clone(), db_end).await?;
        for sql in sql_vec {
            let stmt = Statement::from_string(db_end, sql).to_owned();
            match db.execute(stmt).await {
                Ok(_) => {
                    println!("Successfully init data: {}", path.to_str().unwrap());
                }
                Err(e) => {
                    println!("{}", e);
                }
            };
        }
    }
    println!("All mockdata insert succeed.");
    Ok(())
}

async fn get_insert_sql_string(path: PathBuf, db_end: DatabaseBackend) -> Result<Vec<String>> {
    let mut sql: Vec<String> = Vec::new();
    let mut sql_string = String::new();
    let file = File::open(path).await?;
    let mut buf_reader = BufReader::new(file).lines();
    while let Some(line) = buf_reader.next().await {
        let mut line = line?;
        if line.starts_with("/*!") || line.starts_with("--") {
            continue;
        }
        if line.to_lowercase().starts_with("insert") {
            line = match db_end {
                DatabaseBackend::MySql => line,
                DatabaseBackend::Postgres => line.replace('`', "\""),
                DatabaseBackend::Sqlite => line,
            }
        }
        sql_string.push_str(&line);
        if line.ends_with(';') {
            sql.push(sql_string.clone());
            sql_string.clear();
        }
    }

    Ok(sql)
}
