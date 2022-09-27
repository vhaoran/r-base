use serde::{Deserialize, Serialize};
use std::default::Default;
use std::time::Duration;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub url: String,
    pub user_name: Option<String>,
    pub password: Option<String>,
    pub max_idle_time: Option<u32>,
    pub max_pool_size: Option<u32>,
    pub min_pool_size: Option<u32>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            url: "mongodb://root:password@192.168.0.99:27017".to_string(),
            max_idle_time: Some(1800),
            min_pool_size: Some(5),
            max_pool_size: Some(10),
            user_name: None,
            password: None,
        }
    }
}
