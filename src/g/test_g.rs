use std::time::Duration;

use chrono::prelude::*;

use crate::g::*;

#[test]
fn pwd_1() {
    //---------------------
    let s = crate::g::pwd();
    println!("----dir.rs-------{}-", s);
    //
    let mut pb = std::path::PathBuf::new();
    pb.push(s);
    let _ = pb.join("./logs1/");
    let r = crate::g::verify_mkdir(pb.to_str().unwrap().to_owned());
    println!("----dir.rs---a---{:?}--", r);
}

#[test]
fn t_unix_sec() {
    //---------------------
    let a = super::unix_sec();
    println!("----------------------");
    println!("unix_sec: {:?}", a);
}

#[test]
fn utf16_len_1() {
    let s = "中国good";
    println!("-----------a-----------");
    println!("{} len is  {:?}", s, utf16_len(s));
}

#[test]
fn now_test() {
    //---------------------
    println!("-----------a-----------");
    println!(" {:?}", super::now_str());
    println!(" {:?}", super::now());
    println!(" {}", super::now());
    println!(" {:?}", super::date_str(super::now()));
}

#[test]
fn uuid_1() {
    //---------------------
    let s = self::uuid_str();
    println!("-----------{}-----------", s);
}

#[test]
fn t_date_1() {
    //
    let s = "2010-11-12";
    let r = self::from_date_str(s);
    println!("-----------{:#?}-----------", r);

    //----------------------------
    let s = "2010-11-12 13:14:15";
    let r = self::from_datetime_str(s);
    println!("-----------{:#?}-----------", r);

    let r = self::from_ymd(1999, 12, 13);
    println!("-----------{:#?}-----------", r);
}

#[test]
fn truncate_n_1() {
    //---------------------
    let s = "中國人民解放軍社會主義";
    let s = truncate_n_bytes(s, 5);
    println!("-------{}----len: {}-----------", s, s.bytes().len());
}
