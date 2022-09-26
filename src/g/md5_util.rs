fn md5_of_file(path: &str) -> anyhow::Result<String> {
    let r = std::fs::read(path)?;
    // println!("---------after read-------------");
    // let s = std::str::from_utf8(r.as_slice())?;
    // println!("--------after---from_utf8-----------");

    Ok(self::md5(r.as_slice()))
}

fn md5<T: AsRef<[u8]>>(s: T) -> String {
    let r = md5::compute(s);
    //
    let s = format!("{:?}", r);
    s
}

#[test]
fn md5_1() {
    //---------------------
    let s = self::md5("abcdefg");
    println!("-----------{}-----------", s);
    let s = self::md5_of_file("./a.mp3");
    println!("-----------{:?}-----------", s);
}
