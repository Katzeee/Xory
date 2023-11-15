use api;
use dotenvy;
use tracing;

fn main() {
    dotenvy::dotenv().expect(".env file read fail.");
    tracing_subscriber::fmt().init();
    api::run();
}
