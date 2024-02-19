// impl Cnt {}

//
// extern crate redis_rs;

use async_trait::async_trait;
use tracing::*;
use once_cell::sync::OnceCell;

use redisclient::RedisClient;
use redisclient::RedisError;
use redisclient::RedisResult;

use std::net::TcpStream;
use std::sync::Arc;
use tokio::sync::Mutex as Mu;

use super::Config;

pub static INSTANCE: OnceCell<Arc<Mu<RedisClient>>> = OnceCell::new();

pub fn create_cnt(cfg: Config) -> anyhow::Result<RedisClient> {
    println!(".############# CCC create redis pool..............");
    debug!(".############# CCC create redis pool..............");

    let host = cfg.host.as_str();
    let port = cfg.port;
    let addr = format!("{}:{}", host, port);

    let cfg = redisclient::config::RedisConfig {
        address: addr,
        database: 0,
        username: None,
        password: None,
        pool_capacity: 10,
    };

    let c = RedisClient::with_config(cfg)?;

    Ok(c)
}

pub fn init(cfg: &Config) -> anyhow::Result<()> {
    debug!("-----rred(is) init start----");

    let conn = self::create_cnt(cfg.clone())?;
    let a = Arc::new(Mu::new(conn));
    if let Err(_) = INSTANCE.set(a) {
        panic!("redis init error");
    }

    debug!("-----rred(is) init ok----");

    Ok(())
}
