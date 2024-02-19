#![allow(unused_qualifications)]

use crate::res::init;
use crate::storages::KVStore;
use tracing::*;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

//
type KType = i64;
type VType = i64;
type IType = Arc<Mutex<KVStore<KType, VType>>>;
const PATH_DISK: &str = "xx.yml";
const FLUSH_INTERNAL: u64 = 10;
const CAPACITY: usize = 10;

fn instance() -> &'static IType {
    static INSTANCE: OnceCell<IType> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut m = KVStore::<KType, VType>::new();
        m.path_of_disk = PATH_DISK.to_string();
        m.flush_internal = FLUSH_INTERNAL;
        m.size = CAPACITY;
        m.changed = false;
        let _ = m.load_of_disk();
        Arc::new(Mutex::new(m))
    })
}

pub async fn set(k: KType, v: VType, offset: u64) -> anyhow::Result<()> {
    let a = self::instance().clone();
    let mut m = a.lock().await;
    m.set(k, v, offset)
}
pub async fn rm(k: KType) -> anyhow::Result<()> {
    let a = self::instance().clone();
    let mut m = a.lock().await;
    m.rm(k)
}
pub async fn clear() -> anyhow::Result<()> {
    let a = self::instance().clone();
    let mut m = a.lock().await;
    m.clear()
}

pub async fn get(k: KType) -> anyhow::Result<VType> {
    let a = self::instance().clone();
    let m = a.lock().await;
    m.get(k)
}

pub async fn get_copy() -> HashMap<KType, VType> {
    let a = self::instance().clone();
    let mut m = a.lock().await;
    m.get_copy_all()
}

pub async fn flush_to_disk() -> anyhow::Result<()> {
    let a = {
        let a = self::instance().clone();
        let m = a.lock().await;
        m.clone()
    };
    a.flush_to_disk()
}

#[tokio::test]
async fn test_xxx() -> anyhow::Result<()> {
    crate::init_module_n(None, true, false).await?;
    for i in 0..10_i64 {
        let _ = self::set(i, i * 11, 55).await;
        println!("-----------before sleep: {i}-----------",);
    }
    let r = self::get_copy().await;
    let _ = self::flush_to_disk().await;
    println!("-----------{r:#?}-----------",);

    Ok(())
}
