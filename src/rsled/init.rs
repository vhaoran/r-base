use std::io;
use std::sync::Arc;
use tokio::sync::Mutex;

use async_trait::async_trait;
use log::*;
use once_cell::sync::OnceCell;
use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;

use super::*;
use crate::g;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::path::Path;

//-----------instance--------------------------
pub(crate) static INSTANCE: OnceCell<Arc<Mutex<sled::Db>>> = OnceCell::new();
// const SECS: i64 = 60 * 2;

//-----------install action--------------------------
pub fn init(cfg: Config) -> anyhow::Result<()> {
    let p = g::pwd();
    let def_path = format!("{}", Path::new(p.as_str()).join("sled.db").display());

    let path = cfg.path.unwrap_or(def_path);
    g::verify_mkdir_parent_of_pathfile(path.as_str())?;

    let db = {
        // let db = sled::open(path.to_string())?;
        // let db = sled::Config::new().path(path).open()?;
        let db = sled::Config::new()
            // .use_compression(true)
            .mode(sled::Mode::LowSpace)
            .path(path)
            .open()?;
        let _ = db.insert("aaa", "aaaa");
        // db.was_recovered()
        let _ = db.flush();
        db
    };

    let a = Arc::new(Mutex::new(db));
    let _ = INSTANCE.set(a);
    Ok(())
}
