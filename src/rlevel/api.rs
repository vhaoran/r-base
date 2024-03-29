use std::fmt::Debug;

use anyhow::anyhow;
use tracing::*;
use rusty_leveldb::LdbIterator;
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::*;

pub async fn set_str<TK, T>(key: TK, v: T) -> anyhow::Result<()>
where
    TK: AsRef<str> + std::fmt::Display,
    T: AsRef<str> + std::fmt::Display,
{
    let a = INSTANCE.get().unwrap().clone();
    let mut db = a.lock().await;
    // let key = key.to_string();
    // let v = v.to_string();
    //
    let _ = db.put(key.to_string().as_bytes(), v.to_string().as_bytes());
    Ok(())
}

pub async fn remove(key: &str) -> anyhow::Result<()> {
    let a = INSTANCE.get().unwrap().clone();
    let mut db = a.lock().await;
    //

    //
    let _ = db.delete(key.as_bytes());
    Ok(())
}

pub async fn flush() -> anyhow::Result<()> {
    let a = INSTANCE.get().unwrap().clone();
    let mut db = a.lock().await;

    let _ = db.flush();
    Ok(())
}

pub async fn get_i64<T>(key: T, default_value: Option<i64>) -> i64
where
    T: AsRef<str> + std::fmt::Display,
{
    let s = self::get_str(key).await.unwrap_or("".to_string());
    if !default_value.is_none() {
        return s.trim().parse::<i64>().unwrap_or(default_value.unwrap());
    }

    s.trim().parse::<i64>().unwrap_or(0)
}

pub async fn get_str<T>(key: T) -> anyhow::Result<String>
where
    T: AsRef<str> + std::fmt::Display,
{
    let a = INSTANCE.get().unwrap().clone();
    let mut db = a.lock().await;
    //
    // let key = key;
    //
    let l: Vec<u8> = db.get(key.to_string().as_bytes()).ok_or(anyhow!(""))?;
    let s = String::from_utf8(l)?;
    Ok(s)
}

pub async fn set_json<TK, T>(key: TK, v: T) -> anyhow::Result<()>
where
    TK: AsRef<str> + std::fmt::Display,
    T: Serialize + DeserializeOwned + Debug,
{
    let s = serde_json::to_string(&v)?;
    self::set_str(key, s).await?;
    Ok(())
}

pub async fn get_json<TK, T>(key: TK) -> anyhow::Result<T>
where
    TK: AsRef<str> + std::fmt::Display,
    T: Serialize + DeserializeOwned + Debug,
{
    let s = self::get_str(key).await?;
    let r: T = serde_json::from_str(s.as_str())?;

    Ok(r)
}

pub async fn show_all() -> anyhow::Result<Vec<String>> {
    let a = INSTANCE.get().unwrap().clone();
    let mut db = a.lock().await;

    debug!("--enter_show_all-------");

    let l: Vec<String> = Vec::new();
    let mut iter = db
        .new_iter()
        .map_err(|e| {
            error!("---new_iter-error---{}-", e.to_string());
            e
        })
        .map(|data| {
            debug!("--new_iter-ok ------");
            data
        })?;

    iter.advance();
    while let Some((k, v)) = iter.next() {
        let a = String::from_utf8(k.clone()).unwrap_or("".to_string());
        let b = String::from_utf8(v.clone()).unwrap_or("".to_string());
        debug!("---level_db: {a} : {b}-----------");
        println!("---level_db: {a} : {b}-----------");
    }
    debug!("--show_all_after_loop-------");
    println!("--show_all_after_loop-------");

    Ok(l)
}
