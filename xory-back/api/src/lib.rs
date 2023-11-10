use axum::{routing::MethodRouter, Router};
use common::db_conn;
use sea_orm::DatabaseConnection;
use std::env;
mod routes;

#[tokio::main]
pub async fn run() {
    let port = env::var("PORT").expect("PORT not set in .env.");
    let state = AppState {
        db: db_conn::get_db_conn().await,
    };

    let app = Router::new().nest("/", routes::compose()).with_state(state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Clone)]
pub struct AppState {
    db: DatabaseConnection,
}

type StateRoute = Router<AppState>;
type StateMethodRouter = MethodRouter<AppState>;
