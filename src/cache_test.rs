use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Default, Debug)]
struct Wrapper<T> {
    pub data: T,
    pub expired: i64,
}

// impl Cnt {}

//
fn instance() -> &'static Arc<Mutex<HashMap<i64, Wrapper<i64>>>> {
    static INSTANCE: OnceCell<Arc<Mutex<HashMap<i64, Wrapper<i64>>>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut m = HashMap::new();
        Arc::new(Mutex::new(m))
    })
}

pub async fn cache_clear() {
    let a = self::instance().clone();
    let mut m = a.lock().await;
    m.clear();
}

pub async fn cache_get(key: i64, v: i64, live_secs: i64) -> Option<i64> {
    let a = self::instance().clone();
    let mut m = a.lock().await;
    if m.contains_key(&key) {
        let wrapper = m.get(&key).unwrap();
        //
        let now = crate::g::unix_sec();
        if now > wrapper.expired {
            m.remove(&key);
            return None;
        }
        return Some(wrapper.data.clone());
    }
    None
}

pub async fn cache_set(key: i64, v: i64, live_secs: i64) {
    let a = self::instance().clone();
    let mut m = a.lock().await;
    m.insert(
        key,
        Wrapper {
            data: v,
            expired: crate::g::unix_sec() + live_secs,
        },
    );
}

#[tokio::test]
async fn test_c_1() -> anyhow::Result<()> {
    //
    tokio::spawn(async move {
        for i in 0..100 {
            self::cache_set(i, i * 100, 30).await;
        }
    });

    tokio::spawn(async move {
        for i in 0..100 {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            let a = self::cache_get(i, i * 100, 30).await;
            println!("-----------{:?}-----------", a);
        }
    });

    println!("-----------wait 10 secs-----------");
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    Ok(())
}
