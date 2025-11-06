use chrono::prelude::*;
use chrono::LocalResult;
// unix time stamp
pub fn unix_sec() -> i64 {
    //chrono::prelude::Local::now().timestamp()
    self::now().timestamp()
}

pub fn minutes_str(secs: i64) -> String {
    if secs <= 0 {
        return "0:00".to_string();
    }
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let sec = (secs % 3600) % 60;
    format!("{h}:{m}:{sec}")
}

pub fn unix_millis() -> i64 {
    //chrono::prelude::Local::now().timestamp()
    self::now().timestamp_millis()
}

//local now
pub fn now() -> DateTime<Local> {
    Local::now()
}

/// monday:1   friday: 5; sunday:7
pub fn week_n_of_timestamp(dt: i64) -> i64 {
    let dt = self::from_timestamp(dt);
    week_n(dt)
}
pub fn week_n(dt: DateTime<Local>) -> i64 {
    match dt.weekday() {
        /*
                Mon = 0,
        /// Tuesday.
        Tue = 1,
        /// Wednesday.
        Wed = 2,
        /// Thursday.
        Thu = 3,
        /// Friday.
        Fri = 4,
        /// Saturday.
        Sat = 5,
        /// Sunday.
        Sun = 6,
            */
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
        Weekday::Sun => 7,
    }
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

    // let now: DateTime<Local> = Local.ymd(year, month, day).and_hms(0, 0, 0);
    let r: LocalResult<DateTime<Local>> = Local.with_ymd_and_hms(year, month, day, 0, 0, 0);
    match r {
        LocalResult::Single(v) => Some(v),
        _ => None,
    }

    // let r: LocalResult<Date<Local>> = Local.ymd_opt(year, month, day);
    // match r {
    //     LocalResult::Single(v) => {
    //         DateTime<Local>::
    //     },
    //     _ => None,
    // }

    // Some(now)
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
        // let now: DateTime<Local> = Local.ymd(year, month, day).and_hms(0, 0, 0);
        // return Some(now);

        let r: LocalResult<DateTime<Local>> = Local.with_ymd_and_hms(year, month, day, 0, 0, 0);
        return match r {
            LocalResult::Single(v) => Some(v),
            _ => None,
        };
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

    // let now: DateTime<Local> = Local.ymd(year, month, day).and_hms(h, m, sec);
    // Some(now)
    let r: LocalResult<DateTime<Local>> = Local.with_ymd_and_hms(year, month, day, h, m, sec);
    return match r {
        LocalResult::Single(v) => Some(v),
        _ => None,
    };
}

pub fn today() -> DateTime<Local> {
    let now = self::now();
    // let now: DateTime<Local> = Local
    //     .ymd(now.year(), now.month(), now.day())
    //     .and_hms(0, 0, 0);
    // now

    let r: LocalResult<DateTime<Local>> =
        Local.with_ymd_and_hms(now.year(), now.month(), now.day(), 0, 0, 0);
    // return match r {
    //     LocalResult::Single(v) => Some(v),
    //     _ => None,
    // };
    r.unwrap()
}

/// 指定日期所在月的1号的时间
pub fn month_first_day(dt: DateTime<Local>) -> DateTime<Local> {
    let d = dt;
    // let now: DateTime<Local> = Local.ymd(d.year(), d.month(), 1_u32).and_hms(0, 0, 0);
    // now
    let r: LocalResult<DateTime<Local>> = Local.with_ymd_and_hms(d.year(), d.month(), 1, 0, 0, 0);
    // return match r {
    //     LocalResult::Single(v) => Some(v),
    //     _ => None,
    // };
    r.unwrap()
}

pub fn prior_month_first_day(dt: DateTime<Local>) -> DateTime<Local> {
    //现在日期的当月第一天送去 15天
    let i = self::month_first_day(dt).timestamp() - 15 * 86400;
    let d = self::from_timestamp(i);
    self::month_first_day(d)
}

pub fn month_first_day_of_now() -> DateTime<Local> {
    let d = self::now();
    // let now: DateTime<Local> = Local.ymd(d.year(), d.month(), 1_u32).and_hms(0, 0, 0);
    // now
    let r: LocalResult<DateTime<Local>> = Local.with_ymd_and_hms(d.year(), d.month(), 1, 0, 0, 0);
    // return match r {
    //     LocalResult::Single(v) => Some(v),
    //     _ => None,
    // };
    r.unwrap()
}

/// 上个月的第一天
pub fn prior_month_first_day_of_now() -> DateTime<Local> {
    let d = self::now();
    self::prior_month_first_day(d)
}

/// align to yyyy-mm-dd 00:00::00
pub fn timestamp_align_day(i: i64) -> i64 {
    let dt = self::from_timestamp(i);
    let dt = self::from_ymd(dt.year(), dt.month(), dt.day());
    dt.timestamp()
}
pub fn timestamp_align_hour(i: i64) -> i64 {
    i - i % 3600
}

pub fn timestamp_align_minute(i: i64) -> i64 {
    i - i % 60
}
pub fn timestamp_align_next_hour(i: i64) -> i64 {
    i - i % 3600 + 3600
}

pub fn from_ymd(year: i32, month: u32, day: u32) -> DateTime<Local> {
    // let now = self::now();
    // let now: DateTime<Local> = Local.ymd(year, month, day).and_hms(0, 0, 0);
    // now
    let r: LocalResult<DateTime<Local>> = Local.with_ymd_and_hms(year, month, day, 0, 0, 0);
    // return match r {
    //     LocalResult::Single(v) => Some(v),
    //     _ => None,
    // };

    r.unwrap()
}

pub fn from_ymd_hms(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    sec: u32,
) -> DateTime<Local> {
    // let now = self::now();
    // let now: DateTime<Local> = Local.ymd(year, month, day).and_hms(hour, minute, sec);
    // now

    let r: LocalResult<DateTime<Local>> =
        Local.with_ymd_and_hms(year, month, day, hour, minute, sec);
    // return match r {
    //     LocalResult::Single(v) => Some(v),
    //     _ => None,
    // };
    r.unwrap()
}

pub fn yesterday() -> DateTime<Local> {
    self::add(&self::today(), -86400)
}

pub fn tomorrow() -> DateTime<Local> {
    self::add(&self::today(), 86400)
}

pub fn next_day(i: i64) -> DateTime<Local> {
    let dt = from_timestamp(i);
    let dt = to_date(&dt);
    self::add(&dt, 86400)
}

pub fn prior_day(i: i64) -> DateTime<Local> {
    let dt = from_timestamp(i);
    let dt = to_date(&dt);
    self::add(&dt, -86400)
}

pub fn to_date(dt: &DateTime<Local>) -> DateTime<Local> {
    // let now: DateTime<Local> = Local.ymd(dt.year(), dt.month(), dt.day()).and_hms(0, 0, 0);
    // now
    let r: LocalResult<DateTime<Local>> =
        Local.with_ymd_and_hms(dt.year(), dt.month(), dt.day(), 0, 0, 0);
    // return match r {
    //     LocalResult::Single(v) => Some(v),
    //     _ => None,
    // };
    r.unwrap()
}

pub fn from_timestamp(timestamp: i64) -> DateTime<Local> {
    let r = Local.timestamp_opt(timestamp, 0);
    r.unwrap()
}

pub fn add(dt: &DateTime<Local>, secs: i64) -> DateTime<Local> {
    // Local.timestamp(dt.timestamp() + secs, 0)
    let r = Local.timestamp_opt(dt.timestamp() + secs, 0);
    r.unwrap()
}

//local now to str
pub fn now_str() -> String {
    self::datetime_str(self::now())
}
pub fn now_str_only_number() -> String {
    self::datetime_str_only_num(self::now())
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

/// output is a date_time_str
/// similar: 2022-12-31 12:38:46
pub fn timestamp_str(timestamp: i64) -> String {
    if timestamp == 0 {
        return "".to_string();
    }

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

pub fn datetime_str_only_num(dt: DateTime<Local>) -> String {
    let s = format!(
        "{:04}{:02}{:02}_{:02}{:02}{:02}",
        dt.year(),
        dt.month(),
        dt.day(),
        dt.hour(),
        dt.minute(),
        dt.second(),
    );
    s
}

pub fn time_duration_str(i: i64) -> String {
    if i < 0 {
        return "".to_string();
    }
    let h = i / 3600_i64;
    let m = i % 3600_i64 / 60_i64;
    let sec = i % 3600_i64 % 60_i64;

    format!("{h}:{m}:{sec}")
}

#[test]
fn a_1() {
    //---------------------
    let a = 3700;
    println!("-----------{}-----------", self::minutes_str(a));
}
