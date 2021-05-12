use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub url: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            url: "amqp://root:password@w99:5672/%2f".to_string(),
        }
    }
}
