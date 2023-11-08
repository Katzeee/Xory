use axum::Router;
use std::env;

mod routes;

#[tokio::main]
pub async fn run() {
    let port = env::var("PORT").expect("PORT not set in .env.");
    // build our application with a single route
    let app = Router::new().nest("/", routes::compose());

    // run it with hyper on localhost:3000
    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
