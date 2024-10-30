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

pub fn random_u32() -> u32 {
    let mut i = rand::random::<i32>();
    if i < 0 {
        i = 0 - i;
    }
    i as u32
}

pub fn thousand_sep_i64(i: i64) -> String {
    use num_format::{Locale, ToFormattedString};
    let s = i.to_formatted_string(&Locale::en);
    s
}

/// s is a str like "123.456"
pub fn comma_float(f: f64, dot_count: u64) -> String {
    let s = match dot_count {
        0 => format!("{f:.0}"),
        1 => format!("{f:.1}"),
        2 => format!("{f:.2}"),
        3 => format!("{f:.3}"),
        4 => format!("{f:.4}"),
        5 => format!("{f:.5}"),
        6 => format!("{f:.6}"),
        7 => format!("{f:.7}"),
        8 => format!("{f:.8}"),
        9 => format!("{f:.9}"),
        10 => format!("{f:.10}"),
        11 => format!("{f:.11}"),
        12 => format!("{f:.12}"),
        13 => format!("{f:.13}"),
        14 => format!("{f:.14}"),
        15 => format!("{f:.15}"),
        16 => format!("{f:.16}"),
        17 => format!("{f:.17}"),
        18 => format!("{f:.18}"),
        19 => format!("{f:.19}"),
        20 => format!("{f:.20}"),
        _ => format!("{f:.2}"),
    };

    self::comma_float_str(s)
}

pub fn comma_float_str<T>(float: T) -> String
where
    T: AsRef<str> + std::fmt::Display,
{
    let float = float.to_string();
    let float = float.as_str();

    fn comma_no_dot(float: &str) -> String {
        let mut s = "".to_string();
        let mut sub = "".to_string();
        for v in float.chars().rev() {
            sub = format!("{sub}{v}");

            if sub.len() == 3 {
                s = format!("{s}{sub},");
                sub = "".to_string();
            }
        }
        s = format!("{s}{sub}",);

        // e.g: 123456789.00
        let mut l: String = s.chars().rev().collect();
        if l.starts_with(",") {
            l = l[1..].to_string()
        }
        l
    }

    match float.split_once('.') {
        Some((l, r)) => {
            let l = comma_no_dot(l);
            if r.len() > 0 {
                format!("{l}.{r}")
            } else {
                l
            }
        }
        _ => comma_no_dot(float),
    }
}

#[test]
fn rand_1() {
    //---------------------
    for _ in 0..10 {
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
    let s = n_str.clone();
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

#[test]
fn a_2() {
    //---------------------
    let i = 1234576_i64;
    let r = self::thousand_sep_i64(i);
    println!("-----------{r}-----------",);
}

#[test]
fn f_1() {
    //---------------------
    let r = self::comma_float_str("12345678.32478");
    println!("-----------{r:?}-----------",);
    let r = self::comma_float_str("112345678.32478");
    println!("-----------{r:?}-----------",);
    let r = self::comma_float_str("678.32478");
    println!("-----------{r:?}-----------",);
    let r = self::comma_float_str("78.328");
    println!("-----------{r:?}-----------",);
}
