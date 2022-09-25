use std::io;
use std::sync::Arc;
use tokio::sync::Mutex;

use async_trait::async_trait;
use log::*;
use once_cell::sync::OnceCell;
use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;

use super::*;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

//-----------instance--------------------------
pub(crate) static INSTANCE: OnceCell<Arc<Mutex<sled::Db>>> = OnceCell::new();
// const SECS: i64 = 60 * 2;

//-----------install action--------------------------
pub fn init(cfg: Config) -> anyhow::Result<()> {
    let path = cfg.path.unwrap_or("./sled.db".to_string());
    let db = {
        let db = sled::open(path.to_string())?;
        db.insert("aaa", "aaaa");
        // db.was_recovered()
        db.flush();
        db
    };

    let a = Arc::new(Mutex::new(db));
    INSTANCE.set(a);
    Ok(())
}
