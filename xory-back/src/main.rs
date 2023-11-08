use api;
use dotenvy;

fn main() {
    dotenvy::dotenv().expect(".env file read fail.");
    api::run();
}
