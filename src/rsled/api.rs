use super::*;
use anyhow::anyhow;
use log::debug;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub async fn set_str(key: &str, v: &str) -> anyhow::Result<()> {
    let a = INSTANCE.get().unwrap().clone();
    let mut db = a.lock().await;
    //
    db.insert(key, v);
    Ok(())
}

pub async fn remove(key: &str) -> anyhow::Result<()> {
    let a = INSTANCE.get().unwrap().clone();
    let mut db = a.lock().await;
    //
    db.remove(key);
    Ok(())
}

pub async fn flush() -> anyhow::Result<()> {
    let a = INSTANCE.get().unwrap().clone();
    let mut db = a.lock().await;
    db.flush();
    Ok(())
}

pub async fn show_all() -> anyhow::Result<String> {
    let a = INSTANCE.get().unwrap().clone();
    let mut db = a.lock().await;
    //
    for r in db.iter() {
        match r {
            Ok((k, v)) => {
                let a = String::from_utf8(k.to_vec())?;
                let b = String::from_utf8(v.to_vec())?;
                println!("-----------{a}: {b}-----------",);
                debug!("-----------{a}: {b}-----------",);
            }
            _ => {}
        }
    }

    //
    Ok("".to_string())
}

pub async fn get_str(key: &str) -> anyhow::Result<String> {
    let a = INSTANCE.get().unwrap().clone();
    let mut db = a.lock().await;

    //
    let l = db.get(key)?.ok_or(anyhow!(""))?.to_vec();
    let s = String::from_utf8(l)?;
    Ok(s)
}

pub async fn get_i64(key: &str, default_value: Option<i64>) -> i64 {
    let s = self::get_str(key).await.unwrap_or("".to_string());
    if !default_value.is_none() {
        return s.trim().parse::<i64>().unwrap_or(default_value.unwrap());
    }

    s.trim().parse::<i64>().unwrap_or(0)
}

pub async fn set_json<T>(key: &str, v: T) -> anyhow::Result<()>
where
    T: Serialize + DeserializeOwned + Debug,
{
    let s = serde_json::to_string(&v)?;
    self::set_str(key, s.as_str()).await?;
    Ok(())
}

pub async fn get_json<T>(key: &str) -> anyhow::Result<T>
where
    T: Serialize + DeserializeOwned + Debug,
{
    let s = self::get_str(key).await?;
    println!("-----------str: {s}-----------",);

    let r: T = serde_json::from_str(s.as_str())?;
    Ok(r)
}
