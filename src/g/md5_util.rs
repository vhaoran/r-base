pub fn md5_of_file(path: &str) -> anyhow::Result<String> {
    let r = std::fs::read(path)?;
    // println!("---------after read-------------");
    // let s = std::str::from_utf8(r.as_slice())?;
    // println!("--------after---from_utf8-----------");
    Ok(self::md5(r.as_slice()))
}

pub fn md5<T: AsRef<[u8]>>(s: T) -> String {
    //crate: md5 0.7.0
    let r = md5::compute(s);
    //
    let s = format!("{:?}", r);
    s
}

#[test]
fn md5_1() {
    //---------------------
    let s = self::md5("abcdefg".to_string());
    println!("------abcdefg _>-----{}------len: {}-----", s, s.len());

    let old_s = "xxx".to_string();
    let s = md5(old_s.clone());
    println!("-----{old_s} -> {s}-------{}----", s.len());
}
