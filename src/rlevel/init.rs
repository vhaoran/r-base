use std::sync::Arc;
use std::{env, io};
use tokio::sync::Mutex;

use async_trait::async_trait;
use tracing::*;
use once_cell::sync::OnceCell;
use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;

use super::*;
use crate::g;
use rusty_leveldb::DB;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

// use rusty_leveldb::{CompressionType, DBIterator, LdbIterator, Options, DB};
//-----------instance--------------------------
pub(crate) static INSTANCE: OnceCell<Arc<Mutex<DB>>> = OnceCell::new();
// const SECS: i64 = 60 * 2;

//-----------install action--------------------------
pub fn init(cfg: Config) -> anyhow::Result<()> {
    let def_path = env::current_dir()?.join("level.db");
    let def_path = format!("{}", def_path.display());

    let path = cfg.path.unwrap_or(def_path);
    g::verify_mkdir_parent_of_pathfile(path.as_str())?;

    let db = {
        let mut opt = rusty_leveldb::Options::default();
        opt.create_if_missing = true;
        opt.reuse_manifest = true;
        opt.reuse_logs = true;
        // opt.max_file_size = 7024 * 1024 * 1024;
        // opt.compression_type = CompressionType::CompressionSnappy;
        DB::open(path, opt)?
    };

    let a = Arc::new(Mutex::new(db));
    match INSTANCE.set(a) {
        Ok(_) => Ok(()),
        Err(_) => Err(anyhow!("init level.db error")),
    }
}
