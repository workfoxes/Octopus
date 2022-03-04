use config::{Config as CConfig, ConfigError, File};
use serde::Deserialize;
use std::env;
use lazy_static::lazy_static;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Config {
    debug: bool,
    pub database: Database,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        let result_config = CConfig::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .build();
        let _new = result_config.unwrap();
        _new.try_deserialize()
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::new().unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        print!("{}", crate::config::CONFIG.database.url);
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

