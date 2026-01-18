#[macro_export]
macro_rules! cache_wrapper {
    ($K:ty,$V:ident,$CACHE_SIZE:expr,$SPAN_SECS_i64:expr) => {
        use cached::{
            proc_macro::cached, proc_macro::once, Cached, SizedCache, TimedCache, TimedSizedCache,
            UnboundCache,
        };
        use std::collections::HashMap;
        use std::sync::Arc;
        use tokio::sync::Mutex;
        use tokio::sync::OnceCell;

        const SECS: i64 = $SPAN_SECS_i64;
        // #[derive(Clone, Default, Debug)]
        // struct Wrapper<T> {
        //     pub data: T,
        //     pub expired: i64,
        // }

        //-------------------------------------
        async fn instance() -> &'static Arc<Mutex<TimedSizedCache<$K, $V>>> {
            static INSTANCE: OnceCell<Arc<Mutex<TimedSizedCache<$K, $V>>>> = OnceCell::const_new();
            INSTANCE
                .get_or_init(|| async {
                    let m =
                        TimedSizedCache::with_size_and_lifespan($CACHE_SIZE, $SPAN_SECS_i64 as u64);
                    Arc::new(Mutex::new(m))
                })
                .await
        }

        //-----------clear--------------------------
        pub async fn cache_clear() {
            let a = instance().await.clone();
            let mut m = a.lock().await;
            m.cache_clear();
        }

        //-------------------------------------
        pub async fn cache_remove(key: $K) {
            let a = instance().await.clone();
            let mut m = a.lock().await;
            m.cache_remove(&key);
        }

        pub async fn cache_get(key: $K) -> Option<$V> {
            let a = instance().await.clone();
            let mut m = a.lock().await;
            let r = m.cache_get(&key);
            if r.is_some() {
                return Some(r.unwrap().clone());
            }
            // println!(
            //     "-size: {:?} capacity: {:?}------",
            //     m.cache_size(),
            //     m.cache_capacity()
            // );

            None
        }

        pub async fn cache_contains(key: $K) -> bool {
            let a = instance().await.clone();
            let mut m = a.lock().await;
            m.cache_get(&key).is_some()
        }

        //-----------set--------------------------
        pub async fn cache_set(key: $K, v: $V) {
            let a = instance().await.clone();
            let mut m = a.lock().await;
            m.cache_set(key, v);
            // println!(
            //     "-size: {:?} capacity: {:?}------",
            //     m.cache_size(),
            //     m.cache_capacity()
            // );
        }
        pub async fn cache_count() -> usize {
            let a = instance().await.clone();
            let m = a.lock().await;
            m.cache_size()
        }
    };
}
