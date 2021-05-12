use rusty_leveldb::{DBIterator, LdbIterator, Options, DB};

#[test]
fn rl_1() {
    let opt = rusty_leveldb::Options::default();
    let mut db = DB::open("mydatabase", opt).unwrap();

    db.put(b"Hello", b"World").unwrap();
    let r = db.get("Hello".as_bytes()).unwrap();
    let r = String::from_utf8(r);
    println!("-----------{:?}-----------", r);
}
