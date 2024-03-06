use std::ops::Index;

fn pat() -> Vec<u8> {
    let mut l: Vec<u8> = Vec::new();
    for i in 0..=44_u8 {
        l.push(i);
    }
    // 46-126---
    for i in 46..126 {
        l.push(i);
    }
    //
    // for i in 128..=255_u8 {
    //     l.push(i);
    // }

    l
}

fn to_char(i: u8) -> u8 {
    let l = pat();
    l.get(i as usize).unwrap().clone()
}

fn to_index(i: u8) -> u8 {
    let l = pat();
    let mut j = 0;
    for v in l {
        let v = v.clone();
        if v == i {
            return j;
        }
        j += 1;
    }
    0_u8
}

//转化为91进制的数
pub fn to_n250(n: i64) -> String {
    let mut negative = false;
    let mut n = n;
    if n < 0 {
        negative = true;
        n = 0 - n;
    }
    //
    let high = pat().len();
    let md = high as i64;
    // s = "".to_string();
    let mut rate = n / md;
    let  m = n % md;
    let mut s = format!("{}", to_char(m as u8) as char);

    while rate >= high as i64 {
        let m = rate % md;
        s = format!("{}{}", to_char(m as u8) as char, s);
        //
        rate = rate / md;
    }
    println!("-----------rate: {rate}-----------");
    if rate > 0 {
        s = format!("{}{}", to_char(rate as u8) as char, s);
    }

    if negative {
        s = format!("-{}", s);
    }

    s
}

pub fn from_n250(s: &str) -> anyhow::Result<i64> {
    let high: usize = pat().len();

    let mut negative = false;
    let mut s = s;
    if s.starts_with("-") {
        let ll = &s.as_bytes()[1..];
        s = std::str::from_utf8(&ll).unwrap_or("");
        negative = true;
    }

    //
    let mut i = 0_i64;
    let list = s.as_bytes();

    let mut j = 0_usize;
    for v in list.iter().rev() {
        let c = v.clone();
        let p = j as u32;

        //find each position value
        let n = to_index(c as u8);
        // println!("n: {n}-high: {high}-p: {p}---------");
        i += (n as usize * high.pow(p)) as i64;

        j += 1;
    }

    //
    if negative {
        i = 0 - i;
    }
    Ok(i)
}

//
#[test]
fn t_n250_1() {
    {
        let i = 150_i64;
        let s = self::to_n250(i);
        let j = from_n250(s.as_str()).unwrap();
        println!("---- [{}]- [{}]- [{}]---------", i, s, j);
    }

    for i in 0..10000 {
        let s = self::to_n250(i);
        let j = from_n250(s.as_str()).unwrap();
        println!("\n---- [{}]- [{}]- [{}]---------", i, s, j);
        if i != j {
            println!("--############ error: i: {} j:{}-----------", i, j);
            println!("--pat().len() {} -----------", pat().len());
            return;
        }
    }

    {
        let i = i64::MAX;
        let s = self::to_n250(i);
        let j = from_n250(s.as_str()).unwrap();
        println!("-MAX_i64--- [{}]- [{}]- [{}]---------", i, s, j);
    }
}

#[test]
fn a_1() {
    for i in 0..128_u8 {
        let c = to_char(i);
        let j = to_index(c);
        println!(
            "-{i}-to_char: {} to_index: {j}--mod: {}",
            c as char,
            pat().len()
        );
        if i != j {
            println!("-----------error---{i}-----pat_len: {}---", pat().len());
            break;
        }
    }
}

#[test]
fn a_5() {
    //---------------------
    let s = "ba";
    let i = from_n250(s);
    println!("-----------{:?}----------", i);

    let r = to_char(1_u8);
    println!("---------1: {r}-----------");

    //
}
