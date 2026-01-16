#[macro_export]
macro_rules! cache_wrapper {
    ($K:ty,$V:ident,$CACHE_SIZE:expr,$SPAN_SECS_i64:expr) => {
        use cached::{
            proc_macro::cached, proc_macro::once, Cached, SizedCache, TimedCache, TimedSizedCache,
            UnboundCache,
        };
        use tokio::sync::OnceCell;
        use std::collections::HashMap;
        use std::sync::Arc;
        use tokio::sync::Mutex;

        const SECS: i64 = $SPAN_SECS_i64;
        // #[derive(Clone, Default, Debug)]
        // struct Wrapper<T> {
        //     pub data: T,
        //     pub expired: i64,
        // }

        //-------------------------------------
        async fn instance() -> &'static Arc<Mutex<TimedSizedCache<$K, $V>>> {
            static INSTANCE: OnceCell<Arc<Mutex<TimedSizedCache<$K, $V>>>> = OnceCell::const_new();
            INSTANCE.get_or_init(|| async {
                let  m =
                    TimedSizedCache::with_size_and_lifespan($CACHE_SIZE, $SPAN_SECS_i64 as u64);
                Arc::new(Mutex::new(m))
            }).await
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

mod test {
    #[tokio::test]
    async fn test_1() -> anyhow::Result<()> {
        cache_wrapper!(i64, i64, 1000_usize, 600_i64);

        //---------------------
        for i in 0..10_i64 {
            // let key = format!("{i}_key");
            cache_set(i, i % 10).await;
        }

        for i in 0..10_i64 {
            // let key = format!("{i}_key");
            let r = cache_get(i).await;
            println!("-----{i} = {:?}-----------", r);
        }
        for i in 0..10_i64 {
            // let key = format!("{i}_key");
            let r = cache_contains(i).await;
            println!("---contains: {i} = {:?}-----------", r);
        }

        //-----------sl--------------------------

        Ok(())
    }

    #[tokio::test]
    async fn benchmark_1() -> anyhow::Result<()> {
        cache_wrapper!(i64, i64, 1000_usize, 600_i64);

        for _ in 0..100 {
            tokio::spawn(async move {
                println!("----after sleep-----");

                for i in 0..1000000_i64 {
                    // let key = format!("{i}_key");
                    cache_set(i, i % 10).await;
                }

                for i in 0..1000000_i64 {
                    // let key = format!("{i}_key");
                    // let r = cache_get(i).await;
                    let r = cache_remove(i).await;
                    println!("-----{i} = {:?}-----------", r);
                    println!("-----count: {} ----------", cache_count().await);
                }
            });
        }

        loop {
            println!("-----------aa-----------");
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }

        // Ok(())
    }
}
