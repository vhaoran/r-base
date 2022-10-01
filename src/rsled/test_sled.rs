use std::sync::Arc;

use sled::IVec;

#[test]
fn s_1() {
    //---------------------
    // let tree = sled::open("./sled.db").expect("open");
    let tree = sled::Config::new()
        .path("./test_sled.db".to_string())
        .segment_size(2048)
        .flush_every_ms(Some(10_000))
        .open()
        .unwrap();

    // insert and get, similar to std's BTreeMap
    let _ = tree.insert("KEY1", "VAL1");
    let r = tree.get(&"KEY1").unwrap().unwrap();
    //
    // r.to_vec();
    let s = String::from_utf8(r.to_vec());
    println!("-----------{:?}-----------", s);
}

#[test]
fn fib_1() {
    fn fib(n: u64) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => fib(n - 1) + fib(n - 2),
        }
    }

    for i in 30..60_u64 {
        println!("----------fib({i}) = {}-----------", fib(i));
    }
}
