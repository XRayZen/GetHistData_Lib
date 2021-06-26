pub mod date;
pub mod range;
use std::{default, time::SystemTime};

use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
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

pub fn ISOStringToDateTime(isoStr: &String) -> DateTime<Utc> {
    let t = chrono::DateTime::parse_from_rfc3339(&isoStr).expect("Error!");
    let h = t.with_timezone(&Utc);
    return h;
}
