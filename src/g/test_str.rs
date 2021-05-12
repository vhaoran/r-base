#[test]
fn t_str_1() {
    //---------------------
    let s = "abcdefg";
    let r = s.find("a");
    println!("-----------{:?}-----------", r);
    let r = s.chars().nth(0_usize).clone().unwrap();
    println!("-----------{:?}-----------", r);
}