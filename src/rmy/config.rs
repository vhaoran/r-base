use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub host: Option<String>,
    pub port: Option<u16>,
    #[serde(rename = "username")]
    pub user_name: Option<String>,
    pub pwd: Option<String>,
    pub db_name: Option<String>,
    pub max_conn: Option<u32>,
    pub min_conn: Option<u32>,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Default for Config {
    fn default() -> Self {
        Config {
            host: Some("192.168.0.99:4222".to_string()),
            port: Some(3306),
            user_name: Some("root".to_string()),
            pwd: Some("password".to_string()),
            db_name: None,
            max_conn: Some(20),
            min_conn: Some(20),
        }
    }
}
