use polodb_core::bson::doc;
use polodb_core::{bson, Database};
use serde::{Deserialize, Serialize};
use std::path::Path;

use tracing::*;
use once_cell::sync::OnceCell;

use std::sync::Arc;
use tokio::sync::Mutex as Mu;

use super::Config;
use crate::g;
use std::time::Duration;

static INSTANCE: OnceCell<Arc<Mu<Database>>> = OnceCell::new();

pub fn init(cfg: &Config) -> anyhow::Result<()> {
    debug!("-----rpolo init start ----");
    let p = cfg.path();
    //-------------------------------------
    g::verify_mkdir_parent_of_pathfile(p.as_str())?;
    //-------------------------------------

    let db = Database::open_file(p)?;

    let a = Arc::new(Mu::new(db));
    if let Err(_) = INSTANCE.set(a) {
        error!("rpolo init error,can not set INSTANCE ");
    }

    debug!("-----rpolo init ok----");
    Ok(())
}

pub fn cnt() -> Arc<Mu<Database>> {
    INSTANCE.get().unwrap().clone()
}
