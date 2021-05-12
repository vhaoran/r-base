use log::*;
use once_cell::sync::OnceCell;

use std::sync::Arc;
use tokio::sync::Mutex as Mu;

use super::Config;
use mongodb::{options::ClientOptions, Client};
use std::time::Duration;

static INSTANCE: OnceCell<Arc<Client>> = OnceCell::new();

pub async fn init(cfg: &Config) -> Result<(), Box<dyn std::error::Error>> {
    debug!("-----rmongo init start ----");

    let mut opt = ClientOptions::parse(cfg.url.as_str()).await?;
    opt.min_pool_size = cfg.min_pool_size;
    opt.max_pool_size = cfg.max_pool_size;
    opt.max_idle_time = Some(Duration::from_secs(cfg.max_idle_time.unwrap_or(1800) as u64));
    opt.app_name = Some("telgram".to_string());

    let c = Client::with_options(opt)?;

    let a = Arc::new(c);
    if let Err(_) = INSTANCE.set(a) {
        error!("mongo init error,can not set INSTANCE ");
    }

    debug!("-----rmongo init ok----");

    Ok(())
}

pub fn cnt() -> Arc<Client> {
    INSTANCE.get().unwrap().clone()
}
