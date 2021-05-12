use super::*;
use redisclient::error::RedisError;

pub async fn get_bool<T>(key: T) -> bool
where
    T: AsRef<str> + std::fmt::Display,
{
    // let key = key.to_string*);
    let s = self::get(key.to_string().as_str())
        .await
        .unwrap_or("".to_string());
    if s.len() == 0 {
        return false;
    }
    match s.as_str() {
        "1" => true,
        "t" | "T" => true,
        "true" | "True" | "TRUE" => true,
        _ => false,
    }
}

pub async fn get_i64<T>(key: T, default_value: Option<i64>) -> i64
where
    T: AsRef<str> + std::fmt::Display,
{
    self::get(key.to_string().as_str())
        .await
        .unwrap_or("".to_string())
        .parse::<i64>()
        .unwrap_or(default_value.unwrap_or(0_i64))
}

pub async fn get_f64<T>(key: T, default_value: Option<f64>) -> f64
where
    T: AsRef<str> + std::fmt::Display,
{
    self::get(key.to_string().as_str())
        .await
        .unwrap_or("".to_string())
        .parse::<f64>()
        .unwrap_or(default_value.unwrap_or(0_f64))
}

pub async fn get<T>(key: T) -> Result<String, Box<dyn std::error::Error>>
where
    T: AsRef<str> + std::fmt::Display,
{
    let a = INSTANCE.get().unwrap().clone();
    let pool = a.lock().await;
    let mut c = pool.get().await.unwrap();

    // let mut c = c.lock().await;
    //
    let s: String = c.get(key.to_string())?;
    Ok(s.to_string())
}

pub async fn set<K, V>(key: K, v: V) -> Result<(), Box<dyn std::error::Error>>
where
    K: AsRef<str> + std::fmt::Display,
    V: AsRef<str> + std::fmt::Display,
{
    let a = INSTANCE.get().unwrap().clone();
    let pool = a.lock().await;
    let mut c = pool.get().await.unwrap();

    //
    let a = c.simple_set(key.to_string(), v.to_string())?;
    Ok(a)
}

pub async fn set_x<K, V>(
    key: K,
    v: V,
    expire_secs: usize,
) -> Result<bool, Box<dyn std::error::Error>>
where
    K: AsRef<str> + std::fmt::Display,
    V: AsRef<str> + std::fmt::Display,
{
    let a = INSTANCE.get().unwrap().clone();
    let pool = a.lock().await;
    let mut c = pool.get().await.unwrap();

    let _a = c.simple_set(key.to_string().as_str(), v.to_string().as_str())?;
    let _a = c.expire(key.to_string().as_str(), expire_secs)?;
    Ok(true)
}

pub async fn incr<T>(key: T) -> Result<i64, Box<dyn std::error::Error>>
where
    T: AsRef<str> + std::fmt::Display,
{
    let a = INSTANCE.get().unwrap().clone();
    let pool = a.lock().await;
    let mut c = pool.get().await.unwrap();

    let i = c.incr(key.to_string().as_str())?;
    Ok(i)
}

pub async fn incr_by<T>(key: T, i: i64) -> Result<i64, Box<dyn std::error::Error>>
where
    T: AsRef<str> + std::fmt::Display,
{
    let a = INSTANCE.get().unwrap().clone();
    let pool = a.lock().await;
    let mut c = pool.get().await.unwrap();

    let r = c.incrby(key.to_string().as_str(), i)?;
    Ok(r)
}

pub async fn expire<T>(key: T, expire_secs: usize) -> Result<(), Box<dyn std::error::Error>>
where
    T: AsRef<str> + std::fmt::Display,
{
    let a = INSTANCE.get().unwrap().clone();
    let pool = a.lock().await;
    let mut c = pool.get().await.unwrap();

    let _a = c.expire(key.to_string().as_str(), expire_secs)?;
    Ok(())
}

pub async fn del<T>(key: T) -> Result<(), Box<dyn std::error::Error>>
where
    T: AsRef<str> + std::fmt::Display,
{
    let a = INSTANCE.get().unwrap().clone();
    let pool = a.lock().await;
    let mut c = pool.get().await.unwrap();

    let mut l: Vec<String> = Vec::new();
    l.push(key.to_string());

    let _a = c.del(l)?;
    Ok(())
}

pub async fn del_many(keys: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    if keys.len() == 0 {
        return Err(crate::err("no keys to delete...."));
    }

    let a = INSTANCE.get().unwrap().clone();
    let pool = a.lock().await;
    let mut c = pool.get().await.unwrap();

    let _a = c.del(keys)?;
    Ok(())
}
