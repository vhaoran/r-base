use anyhow::anyhow;
use std::ops::Index;

const PAT: &str =
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`\"\' =~!@#$%^&*()_+;,./:[]{}\\|>?";

//转化为91进制的数
pub fn to_n91(n: i64) -> String {
    let mut negative = false;
    let mut n = n;
    if n < 0 {
        negative = true;
        n = 0 - n;
    }
    //
    let high = PAT.len();
    let md = high as i64;
    let mut s = "".to_string();
    let mut rate = n / md;
    let mut m = n % md;
    s = format!("{}", PAT.get(m as usize..(m as usize) + 1).unwrap());
    while rate >= high as i64 {
        let m = rate % md;
        s = format!("{}{}", PAT.get(m as usize..m as usize + 1).unwrap(), s);

        rate = rate / md;
    }
    if rate > 0 {
        s = format!(
            "{}{}",
            PAT.get(rate as usize..rate as usize + 1).unwrap(),
            s
        );
    }

    if negative {
        s = format!("-{}", s);
    }

    s
}

pub fn from_n91(s: &str) -> anyhow::Result<i64> {
    let high: usize = PAT.len();

    let mut negative = false;
    let mut s = s;
    if s.starts_with("-") {
        let ll = &s.as_bytes().clone()[1..];
        s = std::str::from_utf8(&ll).unwrap_or("");
        negative = true;
    }

    //
    let mut i = 0_i64;
    let list = s.as_bytes();

    let mut j = 0_usize;
    for v in list {
        let c = list.get(j).unwrap().clone() as char;
        let p = list.len() - j - 1;
        j += 1;

        //find each position value
        let n = PAT.find(c);
        if n.is_none() {
            return Err(anyhow!("error format"));
        }
        let n = n.unwrap();

        if n > 0 {
            i += (n * high.pow(p as u32)) as i64;
        }
    }

    //
    if negative {
        i = 0 - i;
    }
    Ok(i)
}

//
#[test]
fn t_n91_1() {
    {
        let i = 90_i64;
        let s = self::to_n91(i);
        let j = from_n91(s.as_str()).unwrap();
        println!("---- [{}]- [{}]- [{}]---------", i, s, j);
    }
    {
        let i = 92_i64;
        let s = self::to_n91(i);
        let j = from_n91(s.as_str()).unwrap();
        println!("---- [{}]- [{}]- [{}]---------", i, s, j);
    }
    {
        let i = 910_i64;
        let s = self::to_n91(i);
        let j = from_n91(s.as_str()).unwrap();
        println!("---- [{}]- [{}]- [{}]---------", i, s, j);
    }
    {
        let i = 150_i64;
        let s = self::to_n91(i);
        let j = from_n91(s.as_str()).unwrap();
        println!("---- [{}]- [{}]- [{}]---------", i, s, j);
    }

    for i in 1..10000 {
        let i = i * 1000000000 * (-1) as i64;
        let s = self::to_n91(i);
        let j = from_n91(s.as_str()).unwrap();
        if i != j {
            println!("--############ error: i: {} j:{}-----------", i, j);
            return;
        }
        println!("---- [{}]- [{}]- [{}]---------", i, s, j);
    }
    for i in 1..1000000 {
        {
            let i = i as i64;
            let s = self::to_n91(i);
            let j = from_n91(s.as_str()).unwrap_or(0);
            println!("---- [{}]- [{}]- [{}]---------", i, s, j);
            if i != j {
                println!("--############ error: i: {} j:{}-----------", i, j);
                return;
            }
        }
    }

    {
        let i = i64::MAX;
        let s = self::to_n91(i);
        let j = from_n91(s.as_str()).unwrap();
        println!("-MAX_i64--- [{}]- [{}]- [{}]---------", i, s, j);
    }
}
