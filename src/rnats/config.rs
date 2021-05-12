use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub host: Option<String>,
    #[serde(rename = "username")]
    pub user_name: Option<String>,
    pub pwd: Option<String>,
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
            user_name: Some("root".to_string()),
            pwd: Some("password".to_string()),
        }
    }
}
