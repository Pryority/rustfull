use dotenv::dotenv;
use std::env;

pub fn url() -> String {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    database_url
}

pub fn setup_environment() {
    dotenv().ok();
    env_logger::init();
}
