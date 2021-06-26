pub mod date;
pub mod range;

use chrono::{DateTime, Duration, FixedOffset, NaiveDateTime, TimeZone, Utc};
use regex::Regex;

pub fn generateTrueIdNane(histname: &String, key: &String) -> String {
    if !histname.is_empty() {
        return histname.clone();
    }
    let res = Regex::new(r"[^A-Za-z0-9]+")
        .unwrap()
        .replace(&key, "")
        .to_string();
    return res;
}
/// ISO8601形式の文字列→DateTime型
pub fn ISOStringToDateTime(isoStr: &String) -> DateTime<Utc> {
    let t = chrono::DateTime::parse_from_rfc3339(&isoStr).expect("Error!");
    let h = t.with_timezone(&Utc);
    return h;
}
///ISO8601形式の文字列へ変換
pub fn ToISOString(time: &String) -> String {
    println!("time txt is {}", &time);
    let res = UnixTimeSecToDateTime(time.parse().unwrap());
    println!("datetime is {}", &res);
    let t = res.format("%+").to_string();
    //2017-11-02T00:00:00Z 正しいもの
    t
}

pub fn UnixTimeSecToDateTime(value: i64) -> DateTime<Utc> {
    //let dt = Utc.timestamp(value, 00);
    let mut dates: DateTime<FixedOffset>=
    DateTime::parse_from_str("1970/01/01 00:00:00", "%Y/%m/%d %H:%M:%S").unwrap();
    let dt = DateTime::parse_from_str("1970/01/01 00:00:00", "%Y/%m/%d %H:%M:%S");
    match dt {
        Ok(date) => dates = date + Duration::milliseconds(value),
        Err(error) => println!("Unixtime error {}", error),
    }
   let dt2= dates.with_timezone(&Utc);
   dt2
}
