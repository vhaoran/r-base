use crate::g::*;
use chrono::prelude::*;
use std::time::Duration;

#[test]
fn pwd_1() {
    //---------------------
    let s = crate::g::get_pwd();
    println!("----dir.rs-------{}-", s);
    //
    let mut pb = std::path::PathBuf::new();
    pb.push(s);
    let _ = pb.join("./logs1/");
    let r = crate::g::verify_mk_dir(pb.to_str().unwrap().to_owned());
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
fn date_of_unix_1() {
    //---------------------
    let i = 3600;
    let mut dt: DateTime<Local> = Local.timestamp(i, 0);
    let s = crate::g::date_str(dt);
    println!("-----------a--------{}---", s);
}

#[test]
fn date_of_add() {
    //---------------------
    let i = 3600;
    let mut dt = crate::g::date::today();
    println!("-------{}--", crate::g::date::datetime_str(dt));
    let mut dt1 = crate::g::date::now();
    println!("-------{}--", crate::g::date::datetime_str(dt1));
    //
    let dt: DateTime<Local> = Local.timestamp(dt.timestamp() + 86400, 0);
    println!("---dt + 1----{}--", crate::g::date::datetime_str(dt));

    let dt1: DateTime<Local> = Local.timestamp(dt1.timestamp() + 86400, 0);
    println!("-dt1 + 1------{}--", crate::g::date::datetime_str(dt1));
}

#[test]
fn width_1() {
    //---------------------
    let l = vec![
        "中华人民a共和国长城市场",
        "社会主义",
        "中华人民共和国",
        "哪个会议室民共和国",
    ];
    for v in l {
        println!("--------old: {}-----------", v);

        println!("###--{}-----------", truncate_of_ascii_width(v, 5));
    }
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
