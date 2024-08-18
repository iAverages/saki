use std::env;

pub struct DebugConfig {
    pub db_calls: bool,
}

pub struct Config {
    pub debug: DebugConfig,
    pub database_url: String,
}

pub async fn load_config() -> Config {
    let debug_config = DebugConfig { db_calls: true };

    Config {
        debug: debug_config,
        database_url: env::var("DATABASE_URL").expect("No database url provided"),
    }
}
