use anyhow::anyhow;
use log::debug;
use log::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;

use super::*;

pub async fn count() -> usize {
    let a = INSTANCE.get().unwrap().clone();
    let db = a.lock().await;
    db.len()
}

pub async fn set_str(key: &str, v: &str) -> anyhow::Result<()> {
    let a = INSTANCE.get().unwrap().clone();
    let db = a.lock().await;
    //
    let _ = db.insert(key, v);
    Ok(())
}

pub async fn remove(key: &str) -> anyhow::Result<()> {
    let a = INSTANCE.get().unwrap().clone();
    let db = a.lock().await;
    //
    let _ = db.remove(key);
    Ok(())
}

pub async fn flush() -> anyhow::Result<()> {
    let a = INSTANCE.get().unwrap().clone();
    let db = a.lock().await;
    let _ = db.flush();
    Ok(())
}

pub async fn show_all() -> anyhow::Result<String> {
    let a = INSTANCE.get().unwrap().clone();
    let db = a.lock().await;
    //
    for r in db.iter() {
        match r {
            Ok((k, v)) => {
                let a = String::from_utf8(k.to_vec())?;
                let b = String::from_utf8(v.to_vec())?;
                debug!("---------show_all_each_-{a}: {b}-----------",);
            }
            _ => {}
        }
    }

    debug!("--show_all_len: {}-------", db.iter().count());
    Ok("".to_string())
}

pub async fn pop_min() -> anyhow::Result<(String, String)> {
    let a = INSTANCE.get().unwrap().clone();
    let db = a.lock().await;
    match db.pop_min() {
        Ok(Some((k, v))) => {
            let k = String::from_utf8(k.to_vec())?;
            let v = String::from_utf8(v.to_vec())?;
            Ok((k, v))
        }
        _ => Err(anyhow!("no data",)),
    }
}

pub async fn pop_min_n(n: Option<usize>) -> anyhow::Result<Vec<(String, String)>> {
    let n = n.unwrap_or(1);

    let a = INSTANCE.get().unwrap().clone();
    let db = a.lock().await;

    let mut r = Vec::<(String, String)>::new();
    for _ in 0..n {
        match db.pop_min() {
            Ok(Some((k, v))) => {
                let k = String::from_utf8(k.to_vec())?;
                let v = String::from_utf8(v.to_vec())?;
                r.push((k, v));
            }
            _ => break,
        }
    }
    Ok(r)
}

pub async fn all_keys() -> Vec<String> {
    let a = INSTANCE.get().unwrap().clone();
    let db = a.lock().await;
    //
    let mut l: Vec<String> = Vec::new();
    for r in db.iter() {
        match r {
            Ok((k, _v)) => match String::from_utf8(k.to_vec()) {
                Ok(a) => {
                    l.push(a);
                }
                _ => {}
            },
            _ => {}
        }
    }

    l
}

pub async fn all_values() -> HashMap<String, String> {
    let a = INSTANCE.get().unwrap().clone();
    let db = a.lock().await;
    //
    let mut h: HashMap<String, String> = HashMap::new();
    for r in db.iter() {
        match r {
            Ok((k, v)) => match (String::from_utf8(k.to_vec()), String::from_utf8(v.to_vec())) {
                (Ok(k_str), Ok(v_str)) => {
                    h.insert(k_str, v_str);
                }
                _ => {}
            },
            _ => {}
        }
    }

    h
}

pub async fn clear()  {
    let a = INSTANCE.get().unwrap().clone();
    let db = a.lock().await;
    let _ = db.clear();
    let _ = db.flush();
}

pub async fn get_clone() -> anyhow::Result<HashMap<String, String>> {
    let a = INSTANCE.get().unwrap().clone();
    let db = a.lock().await;
    //
    let mut m = HashMap::<String, String>::new();
    for r in db.iter() {
        match r {
            Ok((k, v)) => {
                let k = String::from_utf8(k.to_vec())?;
                let v = String::from_utf8(v.to_vec())?;
                m.insert(k, v);
            }
            _ => {}
        }
    }

    Ok(m)
}

pub async fn get_str(key: &str) -> anyhow::Result<String> {
    let a = INSTANCE.get().unwrap().clone();
    let db = a.lock().await;

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

pub async fn export_file(p: &str) -> anyhow::Result<()> {
    let h = self::all_values().await;
    //
    let s = serde_json::to_string(&h)?;
    let _ = std::fs::write(p, s)?;
    debug!("--export ok--total: {}-----", h.len());

    Ok(())
}

pub async fn export_txt_file(p: &str) -> anyhow::Result<()> {
    let h = self::all_values().await;
    //
    let mut s = "".to_string();
    for (k, v) in &h {
        let sub = format!("{k} : {v}");
        s = format!("{s}\n{sub}");
    }
    let _ = std::fs::write(p, s)?;
    debug!("--export ok--total: {}-----", h.len());

    Ok(())
}

pub async fn import_file(p: &str) -> anyhow::Result<()> {
    debug!("--start import-------");
    let s = std::fs::read_to_string(p)?;
    let h: HashMap<String, String> = serde_json::from_str(s.as_str()).map_err(|e| {
        error!("---serde_json_error---{}-", e.to_string());
        e
    })?;
    for (k, v) in &h {
        let k = k.clone();
        let v = v.clone();

        let _ = self::set_str(k.as_str(), v.as_str()).await;
    }
    let _ = self::flush().await;

    debug!("--import ok,total: {}-------", h.len());
    Ok(())
}
