use dotenvy::dotenv;
use std::env;

pub fn load_config() {
    dotenv().ok();
    println!("loading configuration");
}

pub fn db_url() -> String {
    let db_url = env::var("DATABASE_URL").expect("DB URL is not set in.env file");
    println!("initializing database {}",db_url);
    db_url
}
