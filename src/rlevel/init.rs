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

use rusty_leveldb::{DBIterator, LdbIterator, Options, DB};
//-----------instance--------------------------
pub(crate) static INSTANCE: OnceCell<Arc<Mutex<DB>>> = OnceCell::new();
// const SECS: i64 = 60 * 2;

//-----------install action--------------------------
pub fn init(cfg: Config) -> anyhow::Result<()> {
    let path = cfg.path.unwrap_or("./level_db".to_string());
    let db = {
        let mut opt = rusty_leveldb::Options::default();
        opt.create_if_missing = true;
        opt.reuse_manifest = true;
        opt.reuse_logs = true;

        DB::open(path, opt)?
    };

    let a = Arc::new(Mutex::new(db));
    INSTANCE.set(a);
    Ok(())
}
