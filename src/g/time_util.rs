use chrono::prelude::*;

// unix time stamp
pub fn unix_sec() -> i64 {
    //chrono::prelude::Local::now().timestamp()
    self::now().timestamp()
}
pub fn unix_millis() -> i64 {
    //chrono::prelude::Local::now().timestamp()
    self::now().timestamp_millis()
}

//local now
pub fn now() -> DateTime<Local> {
    Local::now()
}

pub fn now_date() -> DateTime<Local> {
    self::today()
}

//2012-12-31
pub fn from_date_str(s: &str) -> Option<DateTime<Local>> {
    let mut year = 2012_i32;
    let mut month = 0_u32;
    let mut day = 0_u32;
    let l: Vec<_> = s.split(r#"-"#).collect();
    if l.len() > 0 {
        year = l.get(0).clone()?.parse::<i32>().unwrap_or(0);
        if year < 1000 || year > 5000 {
            return None;
        }
    }
    if l.len() > 1 {
        month = l.get(1).clone()?.parse::<u32>().unwrap_or(0);
        if month < 1 || month > 12 {
            return None;
        }
    }
    if l.len() > 2 {
        day = l.get(2).clone()?.parse::<u32>().unwrap_or(0);
        if day < 1 || day > 31 {
            return None;
        }
    }
    //

    let now: DateTime<Local> = Local.ymd(year, month, day).and_hms(0, 0, 0);
    Some(now)
}
//2012-12-31 12:31:48
pub fn from_datetime_str(s: &str) -> Option<DateTime<Local>> {
    //y,m,,d
    let mut year = 2012_i32;
    let mut month = 0_u32;
    let mut day = 0_u32;
    let l: Vec<_> = s.split(r#" "#).collect();
    let mut left = "";
    let mut right = "";
    if l.len() > 0 {
        left = l.get(0).clone()?;
    }
    if l.len() > 1 {
        right = l.get(1).clone()?;
    }
    if left.len() == 0 {
        return None;
    }

    let l: Vec<_> = left.split(r#"-"#).collect();
    if l.len() > 0 {
        year = l.get(0).clone()?.parse::<i32>().unwrap_or(0);
        if year < 1000 || year > 5000 {
            return None;
        }
    }
    if l.len() > 1 {
        month = l.get(1).clone()?.parse::<u32>().unwrap_or(0);
        if month < 1 || month > 12 {
            return None;
        }
    }
    if l.len() > 2 {
        day = l.get(2).clone()?.parse::<u32>().unwrap_or(0);
        if day < 1 || day > 31 {
            return None;
        }
    }
    //
    if right.len() == 0 {
        let now: DateTime<Local> = Local.ymd(year, month, day).and_hms(0, 0, 0);
        return Some(now);
    }

    //------------------------------------------
    //y,m,,d
    let mut h = 0_u32;
    let mut m = 0_u32;
    let mut sec = 0_u32;
    let l: Vec<_> = right.split(r#":"#).collect();
    if l.len() > 0 {
        h = l.get(0).clone()?.parse::<u32>().unwrap_or(0);
    }
    if l.len() > 1 {
        m = l.get(1).clone()?.parse::<u32>().unwrap_or(0);
    }
    if l.len() > 2 {
        sec = l.get(2).clone()?.parse::<u32>().unwrap_or(0);
    }

    let now: DateTime<Local> = Local.ymd(year, month, day).and_hms(h, m, sec);
    Some(now)
}

pub fn today() -> DateTime<Local> {
    let now = self::now();
    let now: DateTime<Local> = Local
        .ymd(now.year(), now.month(), now.day())
        .and_hms(0, 0, 0);
    now
}

/// 指定日期所在月的1号的时间
pub fn month_first_day(dt: DateTime<Local>) -> DateTime<Local> {
    let d = dt;
    let now: DateTime<Local> = Local.ymd(d.year(), d.month(), 1_u32).and_hms(0, 0, 0);
    now
}

pub fn prior_month_first_day(dt: DateTime<Local>) -> DateTime<Local> {
    //现在日期的当月第一天送去 15天
    let i = self::month_first_day(dt).timestamp() - 15 * 86400;
    let d = self::from_timestamp(i);
    self::month_first_day(d)
}

pub fn month_first_day_of_now() -> DateTime<Local> {
    let d = self::now();
    let now: DateTime<Local> = Local.ymd(d.year(), d.month(), 1_u32).and_hms(0, 0, 0);
    now
}

/// 上个月的第一天
pub fn prior_month_first_day_of_now() -> DateTime<Local> {
    let d = self::now();
    self::prior_month_first_day(d)
}

pub fn from_ymd(year: i32, month: u32, day: u32) -> DateTime<Local> {
    let now = self::now();
    let now: DateTime<Local> = Local.ymd(year, month, day).and_hms(0, 0, 0);
    now
}

pub fn from_ymd_hms(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    sec: u32,
) -> DateTime<Local> {
    let now = self::now();
    let now: DateTime<Local> = Local.ymd(year, month, day).and_hms(hour, minute, sec);
    now
}

pub fn yesterday() -> DateTime<Local> {
    self::add(&self::today(), -86400)
}

pub fn to_date(dt: &DateTime<Local>) -> DateTime<Local> {
    let now: DateTime<Local> = Local.ymd(dt.year(), dt.month(), dt.day()).and_hms(0, 0, 0);
    now
}

pub fn from_timestamp(timestamp: i64) -> DateTime<Local> {
    Local.timestamp(timestamp, 0)
}

pub fn add(dt: &DateTime<Local>, secs: i64) -> DateTime<Local> {
    Local.timestamp(dt.timestamp() + secs, 0)
}

//local now to str
pub fn now_str() -> String {
    self::datetime_str(self::now())
}
pub fn now_date_str() -> String {
    self::date_str(self::now())
}

pub fn now_datetime_str() -> String {
    self::datetime_str(self::now())
}

// date time to string
pub fn date_str(dt: DateTime<Local>) -> String {
    let s = format!("{:04}-{:02}-{:02}", dt.year(), dt.month(), dt.day(),);
    s
}

pub fn date_str_of_timestamp(timestamp: i64) -> String {
    self::date_str(self::from_timestamp(timestamp))
}

pub fn datetime_str_of_timestamp(timestamp: i64) -> String {
    self::datetime_str(self::from_timestamp(timestamp))
}

pub fn datetime_str(dt: DateTime<Local>) -> String {
    let s = format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        dt.year(),
        dt.month(),
        dt.day(),
        dt.hour(),
        dt.minute(),
        dt.second()
    );
    s
}

#[test]
fn a_1() {
    //---------------------
    let a = self::prior_month_first_day_of_now();
    println!("-----------上月1号： {:?}-----------", a);
    let a = self::month_first_day_of_now();
    println!("-----------this月1号： {:?}-----------", a);
}
