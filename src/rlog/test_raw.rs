use serde_json::*;
use tracing::*;

use super::Config;

#[test]
fn log_1() {
    let a: Config = Default::default();
    let c = a.clone();

    let s = serde_json::to_string(&c);
    println!("----test_log.rs----{:?}----", s);
    //
    let r: Config = serde_json::from_str(s.unwrap().as_str()).unwrap();
    println!("---from_str:-{:?}----", r);
}

#[test]
fn log_2() {
    let mut cfg: Config = Default::default();
    cfg.size = 1;
    let _ = super::init(&cfg);

    for i in 0..100 {
        debug!(" pk {i}");
    }
    println!("-----------ok-----------",);
}
