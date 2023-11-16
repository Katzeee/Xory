use axum::{
    http::{
        header::{ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN},
        HeaderValue, Method,
    },
    routing::MethodRouter,
    Router,
};
use common::db_conn;
use sea_orm::DatabaseConnection;
use std::env;
mod routes;
// use http::HeaderValue;
use tower_http::cors::{any, CorsLayer};
use tower_http::set_header::SetResponseHeaderLayer;

#[tokio::main]
pub async fn run() {
    let port = env::var("PORT").expect("PORT not set in .env.");
    let state = AppState {
        db: db_conn::get_db_conn().await,
    };

    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_origin(any())
        .allow_headers(any());

    let app = Router::new()
        .nest("/", routes::compose())
        .layer(cors)
        .with_state(state);

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
