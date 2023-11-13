use api;
use dotenvy;
use tracing;

fn main() {
    dotenvy::dotenv().expect(".env file read fail.");
    tracing_subscriber::fmt()
        // .with_max_level(tracing::level::DEBUG)
        // .with_test_write()
        .init();
    api::run();
}
