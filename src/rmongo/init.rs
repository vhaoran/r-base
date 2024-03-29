use once_cell::sync::OnceCell;
use tracing::*;

use std::sync::Arc;
use tokio::sync::Mutex as Mu;

use super::Config;
use mongodb::options::{Acknowledgment, Credential, ReadConcern, TransactionOptions, WriteConcern};
use mongodb::{options::ClientOptions, Client, ClientSession};
use std::time::Duration;

static INSTANCE: OnceCell<Arc<Client>> = OnceCell::new();

pub async fn init(cfg: &Config) -> anyhow::Result<()> {
    debug!("-----rmongo init start ----");

    let auth = if cfg.password.is_some() || cfg.user_name.is_some() {
        Some(
            Credential::builder()
                .username("root".to_string())
                .password("password".to_string())
                .build(),
        )
    } else {
        None
    };

    let mut opt = ClientOptions::parse(cfg.url.as_str()).await?;
    opt.min_pool_size = cfg.min_pool_size;
    opt.max_pool_size = cfg.max_pool_size;
    opt.max_idle_time = Some(Duration::from_secs(cfg.max_idle_time.unwrap_or(1800) as u64));
    opt.retry_writes = Some(false);
    opt.retry_reads = Some(false);
    opt.credential = auth;
    opt.write_concern = Some(WriteConcern::builder().w(Acknowledgment::Nodes(1)).build());
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

pub async fn get_tx() -> anyhow::Result<ClientSession> {
    let cnt = self::cnt();
    let mut session = cnt.start_session(None).await?;
    let options = TransactionOptions::builder()
        .read_concern(ReadConcern::majority())
        .write_concern(WriteConcern::builder().w(Acknowledgment::Majority).build())
        .build();
    session.start_transaction(options).await?;
    Ok(session)
}
