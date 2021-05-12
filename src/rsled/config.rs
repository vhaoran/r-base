use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            path: Some("./sled-storage.db".to_string()),
        }
    }
}
