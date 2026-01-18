pub fn utf16_len(s: &str) -> isize {
    s.chars().count() as isize
}

pub fn uuid_str() -> String {
    // use uuid::{Uuid, UuidVersion};
    // let uuid1 = Uuid::new(UuidVersion::Random).unwrap();
    // let s = uuid1.simple().to_string();

    let s = uuid::Uuid::new_v4().to_string();
    s
}

pub fn uuid_short_str() -> String {
    // use uuid::{Uuid, UuidVersion};
    // let uuid1 = Uuid::new(UuidVersion::Random).unwrap();
    // let s = uuid1.simple().to_string();

    let s = self::uuid_str();
    s.replace("-", "")
}

// 半角字符的宽度
pub fn truncate_of_ascii_width(s: &str, high: usize) -> String {
    let i = self::ascii_width(s);
    if i <= high {
        return s.to_string();
    }
    //
    let mut l = "".to_string();
    // let mut old = "".to_string();
    for v in s.chars() {
        let old = l.clone();
        l.push(v);
        if self::ascii_width(l.as_str()) > high {
            return old;
        }

        if self::ascii_width(l.as_str()) == high {
            return l;
        }
    }

    s.to_string()
}

pub fn fix_bit_len(s: &str, bit_len: usize) -> String {
    if s.len() >= bit_len {
        return s.to_string();
    }

    let mut s = format!("{s}");
    for _i in s.len()..bit_len {
        s = format!("{s} ");
    }

    s
}

pub fn ascii_width(s: &str) -> usize {
    let l = s.chars();
    let mut count = 0usize;
    for v in l {
        // if (v >= 'a' && v <= 'z') || (v >= 'A' && v <= 'Z') || "/|,.;".contains(v) {
        if v.is_ascii() {
            count += 1;
            continue;
        }
        count += 2;
    }

    count
}

pub fn rm_sign(s: &str) -> String {
    let mut r = s.to_string();
    let l = vec![
        "\n", "\r", "\t", "\\", "/", ",", ".", "?", "#", "*", "@", " ", "\"
    ", "\'", "{", "}",
        "[", "]", "<", ">", "_", "-", "+", "-", "*",
    ];
    for v in l {
        r = r.replace(v, "")
    }

    r
}

pub fn truncate_n_bytes(s: &str, high: usize) -> String {
    let i = s.bytes().len();
    if i <= high {
        return s.to_string();
    }
    //
    let mut l = "".to_string();
    // let mut old = "".to_string();
    for v in s.chars() {
        let old = l.clone();
        l.push(v);
        if l.bytes().len() > high {
            return old;
        }

        if l.bytes().len() == high {
            return l;
        }
    }

    s.to_string()
}

pub fn truncate_fill_n_chats(s: &str, high: usize) -> String {
    let i = s.chars().count();
    if i <= high {
        return s.to_string();
    }
    //
    let mut l: Vec<String> = Vec::new();
    for c in s.chars() {
        l.push(format!("{c}"));
    }
    //-------------------------------------
    let l = l.iter().as_slice();
    //
    let h = high / 2;
    let a = &l[..h];
    let a: String = a
        .iter()
        .map(|x| format!("{x}"))
        .reduce(|x, y| format!("{x}{y}"))
        .unwrap_or("".to_string());
    let b = &l[(i - h)..];
    let b: String = b
        .iter()
        .map(|x| format!("{x}"))
        .reduce(|x, y| format!("{x}{y}"))
        .unwrap_or("".to_string());

    format!("{a}**{b}",)
}
