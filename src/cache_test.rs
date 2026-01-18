use std::collections::HashMap;
use std::sync::Arc;

use once_cell::sync::OnceCell;
use tokio::sync::Mutex;

#[allow(dead_code)]
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
        let m = HashMap::new();
        Arc::new(Mutex::new(m))
    })
}

#[allow(dead_code)]
pub async fn cache_clear() {
    let a = self::instance().clone();
    let mut m = a.lock().await;
    m.clear();
}

#[allow(dead_code)]
pub async fn cache_get(key: i64) -> Option<i64> {
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

#[allow(dead_code)]
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
