use crate::g;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::env;
use std::path::Path;
use std::time::Duration;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            path: Some("./polo.db".to_string()),
        }
    }
}

impl Config {
    pub fn path(&self) -> String {
        let p = g::pwd();
        let def_path = format!("{}", Path::new(p.as_str()).join("polo.db").display());
        match self.path.clone() {
            Some(v) => v,
            _ => def_path,
        }
    }
}
