use serde::{Deserialize, Serialize};

use std::default::Default;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub url: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            url: "http://elastic:password@192.168.0.201:9200".to_string(),
        }
    }
}
