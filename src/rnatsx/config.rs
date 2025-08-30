use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub hosts: Vec<String>,
    #[serde(default)]
    #[serde(rename = "username")]
    pub user_name: String,
    #[serde(default)]
    pub pwd: String,
    #[serde(default)]
    pub ping_secs: u64,
    #[serde(default)]
    pub max_reconnects: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hosts: vec![],
            user_name: "root".to_string(),
            pwd: "password".to_string(),
            ping_secs: 10,
            max_reconnects: 0,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn hosts(&self) -> &Vec<String> {
        &self.hosts
    }
    pub fn user_name(&self) -> &String {
        &self.user_name
    }
    pub fn pwd(&self) -> &String {
        &self.pwd
    }
    pub fn max_reconnects(&self) -> usize {
        self.max_reconnects
    }
    pub fn ping_secs(&self) -> u64 {
        self.ping_secs
    }


}
