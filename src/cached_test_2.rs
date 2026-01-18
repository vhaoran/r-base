use cached::{proc_macro::cached, TimedSizedCache};

use std::thread::sleep;
use std::time::Duration;

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
