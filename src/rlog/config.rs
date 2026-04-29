// use  serde::*;
use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub level: String,
    pub path: String,
    pub file_name: String,
    pub size: u64,
    pub roll_count: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            level: "debug".to_string(),
            path: "./logs/".to_string(),
            file_name: "sys.log".to_string(),
            roll_count: 3,
            size: 512u64,
        }
    }
}
impl Config {
    pub fn level_str(&self) -> String {
        let s = self.level.clone();
        let all = "/error/debug/info/warn/trace";
        if !all.contains(s.as_str()) {
            return "info".to_string();
        }
        s
    }
}
