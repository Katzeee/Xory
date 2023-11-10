use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::{env, time::Duration};

pub async fn get_db_conn() -> DatabaseConnection {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in env file.");
    let opt = ConnectOptions::new(db_url)
        .max_connections(20)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .to_owned();

    let db: DatabaseConnection = Database::connect(opt)
        .await
        .expect("Create db connection failed.");

    db
}

pub async fn close_db_conn(db: DatabaseConnection) -> Result<(), DbErr> {
    println!("close db conn");
    db.close().await
}
