use log::*;
use once_cell::sync::OnceCell;

use std::sync::Arc;
use tokio::sync::Mutex as Mu;

use log::info;

use super::Config;
use nats::Connection;

static INSTANCE: OnceCell<Arc<Connection>> = OnceCell::new();

pub async fn init(cfg: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let host = cfg.host.clone().unwrap_or("192.168.0.99:4222".to_string());
    let user_name = cfg.user_name.clone().unwrap_or("root".to_string());
    let pwd = cfg.pwd.clone().unwrap_or("password".to_string());

    let conn = nats::Options::with_user_pass(user_name.as_str(), pwd.as_str())
        .with_name("My Rust NATS App")
        .max_reconnects(20000)
        .reconnect_buffer_size(60 * 1024)
        .disconnect_callback(|| println!("***********connection has been lost**********"))
        .reconnect_callback(|| println!(".....connecting........"))
        .connect(host.as_str())?;

    let a = Arc::new(conn);
    if let Err(_) = INSTANCE.set(a) {
        panic!("nats init error");
    }

    Ok(())
}

pub fn cnt() -> Arc<Connection> {
    INSTANCE.get().unwrap().clone()
}
