use once_cell::sync::OnceCell;
use tracing::*;

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex as Mu;

use tracing::*;

use super::Config;
use async_nats::{Client, ConnectOptions};

static INSTANCE: OnceCell<Arc<Client>> = OnceCell::new();

pub async fn init(cfg: &Config) -> anyhow::Result<()> {
    let urls = cfg.hosts().to_owned();
    let user_name = cfg.user_name().to_owned();
    let pwd = cfg.pwd().to_owned();

    let conn = ConnectOptions::new()
        .user_and_password(user_name.clone(), pwd.clone())
        .ping_interval(Duration::from_secs(cfg.ping_secs()))
        .max_reconnects(cfg.max_reconnects())
        .connect(urls.as_slice())
        .await?;
    // let conn = ::Options::with_user_pass(user_name.as_str(), pwd.as_str())
    //     .with_name("My Rust NATS App")
    //     // .max_reconnects(3)
    //     .max_reconnects(3)
    //     .reconnect_buffer_size(64 * 1024)
    //     .disconnect_callback(|| debug!("***********connection has been lost**********"))
    //     .reconnect_callback(|| debug!(".....connecting........"))
    //     .connect(host.as_str())?;

    let a = Arc::new(conn);
    if let Err(_) = INSTANCE.set(a) {
        error!("nats init error");
    }

    Ok(())
}

pub fn cnt() -> Arc<Client> {
    INSTANCE.get().unwrap().clone()
}
