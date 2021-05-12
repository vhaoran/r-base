use log::*;
use once_cell::sync::OnceCell;
use tokio::sync::Mutex;

use lapin::{
    message::DeliveryResult, options::*, publisher_confirm::Confirmation, types::FieldTable,
    BasicProperties, Connection, ConnectionProperties,
};
use std::sync::Arc;
// use tokio::sync::Mutex ;

use log::info;
// use nats::connect;
use tokio_amqp::*;

use super::Config;

static INSTANCE: OnceCell<Mutex<Arc<Connection>>> = OnceCell::new();

pub async fn init(cfg: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let r = self::init_instance(cfg.clone()).await;
    //
    self::watch_spawn(cfg.clone()).await;
    Ok(())
}

pub async fn init_instance(cfg: Config) -> Result<(), Box<dyn std::error::Error>> {
    // let addr = "amqp://root:password@192.168.0.99:5672/%2f";
    let addr = cfg.url.as_str();
    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;

    let a = Arc::new(conn);
    let m = Mutex::new(a.clone());

    //first set
    if let Err(e) = INSTANCE.set(m) {
        //second set
        let i = INSTANCE.get().unwrap();
        let mut m = i.lock().await;
        *m = a;
        return Err(crate::err("set error"));
    }

    Ok(())
}

pub async fn conn() -> Arc<Connection> {
    let a = INSTANCE.get().unwrap();
    let m = a.lock().await;
    m.clone()
}

async fn watch_spawn(cfg: Config) {
    tokio::spawn(async move {
        loop {
            let cfg = cfg.clone();
            // debug!("--loop_check_rabbit_connection-------");
            // println!("--loop_check_rabbit_connection-------");

            let cnt = self::conn().await;
            let stat = cnt.status();
            if stat.closing() || stat.closed() || stat.errored() {
                debug!("--cnt is error,after 5 seconds will  reconnecting...");
                println!("--cnt is error,after 5 seconds will  reconnecting...");
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                self::init_instance(cfg.clone()).await;
                continue;
            }
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    });
}
