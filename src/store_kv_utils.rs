#[macro_export]
macro_rules! store_kv {
    ($KType:tt,$VType:ident,$path_disk:expr,$capacity:expr) => {
        use once_cell::sync::OnceCell;
        use std::collections::HashMap;
        use std::sync::Arc;
        use tokio::sync::Mutex;
        use $crate::storages::KVStore;

        //
        type KType = $KType;
        type VType = $VType;
        type IType = Arc<Mutex<KVStore<$KType, $VType>>>;
        // const PATH_DISK: &str = ;
        // const FLUSH_INTERNAL: u64 = $;
        // const CAPACITY: usize = ;

        fn instance() -> &'static IType {
            static INSTANCE: OnceCell<IType> = OnceCell::new();
            INSTANCE.get_or_init(|| {
                let mut m = KVStore::<KType, VType>::new();
                m.path_of_disk = $path_disk.to_string();
                // m.flush_internal = FLUSH_INTERNAL;
                m.size = $capacity;
                m.changed = false;
                let _ = m.load_of_disk();
                Arc::new(Mutex::new(m))
            })
        }

        pub async fn set(k: KType, v: VType, offset: u64) -> anyhow::Result<()> {
            let a = instance().clone();
            let mut m = a.lock().await;
            m.set(k, v, offset)
        }

        pub async fn rm(k: KType) -> anyhow::Result<()> {
            let a = instance().clone();
            let mut m = a.lock().await;
            m.rm(k)
        }
        pub async fn clear() -> anyhow::Result<()> {
            let a = instance().clone();
            let mut m = a.lock().await;
            m.clear()
        }
        pub async fn count() -> usize {
            let a = instance().clone();
            let mut m = a.lock().await;
            m.store.len()
        }

        pub async fn get(k: KType) -> anyhow::Result<VType> {
            let a = instance().clone();
            let m = a.lock().await;
            m.get(k)
        }

        pub async fn get_copy() -> HashMap<KType, VType> {
            let a = instance().clone();
            let mut m = a.lock().await;
            m.get_copy_all()
        }

        pub async fn flush_to_disk() -> anyhow::Result<()> {
            let a = {
                let a = instance().clone();
                let m = a.lock().await;
                m.clone()
            };
            a.flush_to_disk()
        }
    };
}
