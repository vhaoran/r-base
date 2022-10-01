use crate::g;

#[test]
fn a_1() {
    use rocksdb::{Options, DB};
    // NB: db is automatically closed at end of lifetime
    let path = "./rk.db";
    {
        let db = DB::open_default(path).unwrap();
        db.put(b"my key", b"my value").unwrap();
        match db.get(b"my key") {
            Ok(Some(value)) => println!("retrieved value {}", String::from_utf8(value).unwrap()),
            Ok(None) => println!("value not found"),
            Err(e) => println!("operational problem encountered: {}", e),
        }
        db.delete(b"my key").unwrap();
    }
    // let _ = DB::destroy(&Options::default(), path);
}

#[test]
fn a_2() {
    use rocksdb::{Options, DB};
    // NB: db is automatically closed at end of lifetime
    let path = "./rk.db";
    {
        // let db = DB::open_default(path).unwrap();
        let mut opt = Options::default();
        opt.set_db_write_buffer_size(10 * 1024 * 1024);
        let db = DB::open(&opt, path).unwrap();

        let dt0 = g::now().timestamp_millis();
        for i in 0..100_000_i64 {
            let k = format!("key_{i}");
            let v = format!("value_is_{i}");
            db.put(k.as_str(), v.as_str()).unwrap();
            match db.get(k.as_str()) {
                Ok(Some(value)) => {
                    // println!("retrieved value {}", String::from_utf8(value).unwrap())
                }
                Ok(None) => {
                    println!("value not found");
                }
                Err(e) => {
                    println!("operational problem encountered: {}", e);
                }
            }
        }
        println!(
            "-----------millis: {}-----------",
            g::now().timestamp_millis() - dt0
        );

        // db.delete(b"my key").unwrap();
    }
    // let _ = DB::destroy(&Options::default(), path);
}
