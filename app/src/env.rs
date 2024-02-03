use dotenvy::dotenv;
use std::env;

pub fn load_config() {
    dotenv().ok();
}

pub fn db_url() -> String {
    env::var("DATABASE_URL").expect("DB URL is not set in.env file")
}
