use cached::{
    proc_macro::cached, proc_macro::once, Cached, SizedCache, TimedCache, TimedSizedCache,
    UnboundCache,
};

use std::sync::Arc;
use std::thread::{self, sleep};
use std::time::Duration;
use tokio::sync::Mutex;

cached! {
    TIMED_SIZED: TimedSizedCache<u32, u32> = TimedSizedCache::with_size_and_lifespan(3, 2);
    fn timefac(n: u32) -> u32 = {
        sleep(Duration::new(1, 0));
        if n > 1 {
            n * timefac(n - 1)
        } else {
            n
        }
    }
}

#[test]
fn test_timed_sized_cache() {
    timefac(1);
    timefac(1);
    {
        let cache = TIMED_SIZED.lock().unwrap();
        assert_eq!(1, cache.cache_misses().unwrap());
        assert_eq!(1, cache.cache_hits().unwrap());
    }
    sleep(Duration::new(3, 0));
    timefac(1);
    {
        let cache = TIMED_SIZED.lock().unwrap();
        assert_eq!(2, cache.cache_misses().unwrap());
        assert_eq!(1, cache.cache_hits().unwrap());
    }
    {
        let mut cache = TIMED_SIZED.lock().unwrap();
        assert_eq!(2, cache.cache_set_lifespan(1).unwrap());
    }
    timefac(1);
    sleep(Duration::new(1, 0));
    timefac(1);
    {
        let cache = TIMED_SIZED.lock().unwrap();
        assert_eq!(3, cache.cache_misses().unwrap());
        assert_eq!(2, cache.cache_hits().unwrap());
    }
    {
        let mut cache = TIMED_SIZED.lock().unwrap();
        assert_eq!(1, cache.cache_set_lifespan(6).unwrap());
    }
    timefac(2);
    {
        let cache = TIMED_SIZED.lock().unwrap();
        assert_eq!(4, cache.cache_misses().unwrap());
        assert_eq!(3, cache.cache_hits().unwrap());
    }
    timefac(3);
    {
        let cache = TIMED_SIZED.lock().unwrap();
        assert_eq!(5, cache.cache_misses().unwrap());
        assert_eq!(4, cache.cache_hits().unwrap());
    }
    timefac(3);
    timefac(2);
    timefac(1);
    {
        let cache = TIMED_SIZED.lock().unwrap();
        assert_eq!(5, cache.cache_misses().unwrap());
        assert_eq!(7, cache.cache_hits().unwrap());
    }
    timefac(4);
    {
        let cache = TIMED_SIZED.lock().unwrap();
        assert_eq!(6, cache.cache_misses().unwrap());
        assert_eq!(8, cache.cache_hits().unwrap());
    }
    timefac(6);
    {
        let cache = TIMED_SIZED.lock().unwrap();
        assert_eq!(8, cache.cache_misses().unwrap());
        assert_eq!(9, cache.cache_hits().unwrap());
    }
    timefac(1);
    {
        let cache = TIMED_SIZED.lock().unwrap();
        assert_eq!(9, cache.cache_misses().unwrap());
        assert_eq!(9, cache.cache_hits().unwrap());
        assert_eq!(3, cache.cache_size());
    }
}

#[test]
fn test_timed_sized_cache_2() {
    let mut c: TimedSizedCache<u32, u32> = TimedSizedCache::with_size_and_lifespan(3, 2);
    let _ = c.cache_set(1, 1);
    let _ = c.cache_set(2, 1);
    let _ = c.cache_set(3, 1);
    let _ = c.cache_set(4, 1);
    let _ = c.cache_set(5, 1);
    let r = c.cache_get(&1);

    println!("--------1---{:?}-----------", r);
    let r = c.cache_get(&2);
    // c.cache_remove();
    println!("--------2---{:?}-----------", r);
    //
}

#[tokio::test]
async fn test_2() -> anyhow::Result<()> {
    let c: Mutex<TimedSizedCache<(u32, u32), u32>> =
        Mutex::new(TimedSizedCache::with_size_and_lifespan(30, 20));
    let mut c = c.lock().await;

    let _ = c.cache_set((1, 1), 1);
    let _ = c.cache_set((2, 2), 1);
    let _ = c.cache_set((3, 3), 1);
    let _ = c.cache_set((4, 4), 1);
    let r = c.cache_get(&(1, 1));
    println!("--------1---{:?}-----------", r);
    let r = c.cache_get(&(1, 1));
    println!("--------2---{:?}-----------", r);

    //2079.95
    Ok(())
}
