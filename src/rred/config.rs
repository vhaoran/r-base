use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub host: String,
    pub port: u32,
    /// The database number to use.  This is usually `0`.
    pub db: i64,
    /// Optionally a username that should be used for connection.
    pub user_name: Option<String>,
    /// Optionally a password that should be used for connection.
    pub password: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: "192.168.0.99".to_string(),
            port: 6379u32,
            db: 0,
            user_name: None,
            password: None,
        }
    }
}
