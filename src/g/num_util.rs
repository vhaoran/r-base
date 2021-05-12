use rand::prelude::*;

pub fn random() -> i64 {
    let mut i = rand::random::<i64>();
    if i < 0 {
        i = 0 - i;
    }
    i
}

pub fn random_u64() -> u64 {
    let mut i = rand::random::<i64>();
    if i < 0 {
        i = 0 - i;
    }
    i as u64
}

pub fn thousand_sep_i64(i: i64) -> String {
    use num_format::{Locale, ToFormattedString};
    let s = i.to_formatted_string(&Locale::en);
    s
}

// pub fn thousand_sep_f64(i: f64) -> String {
//     use num_format::{Locale, ToFormattedString};
//     let s = i.to_formatted_string(&Locale::en);
//     s
// }

#[test]
fn rand_1() {
    //---------------------
    for i in 0..10 {
        println!("----------- ({}) -----------", self::random());
    }
}

#[test]
fn ss_1() {
    //---------------------
    println!(
        "-----------{}-----------",
        self::thousand_sep_i64(123455678)
    );
}

/// 對於數字類的字符中，如果有小數點，則截取掉0
pub fn cut_tail_zero(n_str: String) -> String {
    let mut s = n_str.clone();
    if !s.contains(".") {
        return s;
    }

    let mut dot_found = false;
    let mut r = "".to_string();
    for v in s.as_bytes().iter().rev() {
        let c = v.clone() as char;
        if r.len() == 0 && !dot_found {
            if c == '0' {
                continue;
            }
            if c == '.' {
                dot_found = true;
                continue;
            }
        }
        //
        r = format!("{c}{r}");
    }

    r
}

#[test]
fn a_1_1() {
    //---------------------
    let s = "123.00".to_string();
    println!("-----------{}-----------", cut_tail_zero(s));
    let s = "10.00".to_string();
    println!("-----------{}-----------", cut_tail_zero(s));
}
