use crate::g;
use core::fmt::Debug;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::path::Path;
use tracing::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KVStore<K, V>
where
    K: Hash + Eq,
{
    pub store: HashMap<K, (i64, V)>,
    pub size: usize,
    // if data
    pub changed: bool,
    last_rm_expire: i64,

    pub path_of_disk: String,
    /// secs
    pub flush_internal: u64,
}

impl<K, V> KVStore<K, V>
where
    K: Hash + Eq + Clone + Serialize + DeserializeOwned,
    V: Serialize + Debug + Clone + DeserializeOwned,
{
    pub fn new() -> Self {
        KVStore {
            store: HashMap::<K, (i64, V)>::new(),
            size: 10000,
            changed: false,
            last_rm_expire: 0,
            path_of_disk: "".to_string(),
            flush_internal: 0,
        }
    }

    // flush data to disk
    pub fn flush_to_disk(&self) -> anyhow::Result<()> {
        debug!("--enter flush to disk-------");

        let s = serde_yml::to_string(&self.store).map_err(|e| {
            error!("---flush_error---{}-", e.to_string());
            e
        })?;
        let p = Path::new(self.path_of_disk.as_str());
        let _ = fs::write(p, s.as_str()).map_err(|e| {
            error!("---flush_error---{}-", e.to_string());
            e
        })?;
        debug!("--flush_to_disk_ok-------");

        Ok(())
    }

    pub fn load_of_disk(&mut self) -> anyhow::Result<()> {
        let p = Path::new(self.path_of_disk.as_str());
        let s = fs::read_to_string(p)?;
        self.store = serde_yml::from_str(s.as_str())?;
        Ok(())
    }

    /// remove expired data
    pub fn rm_expired(&mut self) -> anyhow::Result<()> {
        let now = g::unix_sec();
        self.store.retain(|_, v| v.0 > now);
        Ok(())
    }

    /// set (k,v)
    pub fn set(&mut self, k: K, v: V, offset: u64) -> anyhow::Result<()> {
        let i = g::unix_sec() + offset as i64;
        let _ = self.store.insert(k, (i, v));

        self.auto_rm_expired();
        Ok(())
    }

    fn auto_rm_expired(&mut self) {
        let now = g::unix_sec();
        if now - 10 > self.last_rm_expire {
            let _ = self.rm_expired();
            self.last_rm_expire = now;
        }
    }

    /// get key value,if not expired
    pub fn get(&self, k: K) -> anyhow::Result<V> {
        let now = g::unix_sec();
        match self.store.get(&k) {
            Some(v) => {
                if v.0 < now {
                    // self.auto_rm_expired();
                    Err(anyhow!("has expired",))
                } else {
                    Ok(v.1.clone())
                }
            }
            _ => Err(anyhow!("not exist",)),
        }
    }
    /// rm of key
    pub fn rm(&mut self, k: K) -> anyhow::Result<()> {
        if self.store.contains_key(&k) {
            self.store.remove(&k);
        }
        Ok(())
    }
    /// remove all key-values,and not flush
    pub fn clear(&mut self) -> anyhow::Result<()> {
        self.store.clear();
        Ok(())
    }

    pub fn get_copy_all(&mut self) -> HashMap<K, V> {
        let _ = self.rm_expired();
        self.store
            .iter()
            .map(|(k, v)| (k.clone(), v.1.clone()))
            .collect()
    }
}

#[test]
fn aaa() {
    //---------------------
    let mut a = KVStore::<(i64, i64), i64> {
        store: HashMap::new(),
        size: 10,
        changed: false,
        last_rm_expire: 0,
        path_of_disk: "kv-store.txt".to_string(),
        flush_internal: 0,
    };
    let _ = a.set((3, 3), 33, 10);
    let _ = a.set((4, 3), 44, 10);
    let _ = a.set((5, 3), 55, 10000);
    //
    let _ = a.flush_to_disk();

    println!("-----------{a:#?}-----------",);
    println!("-----------to_disk ok-----------",);
}

#[test]
fn aabb() {
    //---------------------
    let mut a = KVStore::<(i64, i64), i64> {
        store: HashMap::new(),
        size: 10,
        changed: false,
        last_rm_expire: 0,
        path_of_disk: "kv-store.txt".to_string(),
        flush_internal: 0,
    };
    let _ = a.load_of_disk();

    println!("-----------{a:#?}-----------",);
    println!("-----------from_disk ok-----------",);
}
