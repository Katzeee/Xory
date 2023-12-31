use api;
use dotenvy;
use tracing;

fn main() {
    dotenvy::dotenv().expect(".env file read fail.");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .pretty()
        .init();
    api::run();
}
