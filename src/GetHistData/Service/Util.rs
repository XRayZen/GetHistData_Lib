pub mod date;
pub mod range;

use std::error::{self, Error};

use chrono::{DateTime, Duration, TimeZone, Utc};
use regex::Regex;

use crate::GetHistData::DukasCopy::{Buffer_Fetcher, TrueDataTypes::True_Instrument};

use super::FileService;

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
    //println!("time txt is {}", &time);
    let res = UnixTimeSecToDateTime(time.parse().unwrap());
    //println!("datetime is {}", &res);
    let t = res.format("%+").to_string();
    t
}

pub fn UnixTimeSecToDateTime(value: i64) -> DateTime<Utc> {
    //let dt = Utc.timestamp(value, 00);
    let dt = Utc
        .datetime_from_str("1970/01/01 00:00:00", "%Y/%m/%d %H:%M:%S")
        .unwrap();
    dt + Duration::milliseconds(value)
}

pub fn read_json_to_currentdir_trueinstrument() -> Result<Vec<True_Instrument>, Box<dyn Error>> {
    let dir = std::env::current_dir()
        .unwrap()
        .as_path()
        .to_str()
        .unwrap()
        .to_string();
    let text = FileService::FileService::read_text(&"instrument-list.json".to_string(), &dir)?;
    let res: Vec<True_Instrument> = serde_json::from_str(&text)?;
    Ok(res)
}

pub fn savejson_to_curdir_trueinstrument(
    filename: String,
    data: Vec<True_Instrument>,
) -> Result<(), Box<dyn Error>> {
    let dir = std::env::current_dir()
        .unwrap()
        .as_path()
        .to_str()
        .unwrap()
        .to_string();
    let d = serde_json::to_string(&data)?;
    FileService::FileService::save_text(&filename, &dir, d)?;
    Ok(())
}
pub fn save_json_to_curdir_tick(filename: String,data: Vec<Buffer_Fetcher::BufferObject>,) {
    let dir = std::env::current_dir()
    .unwrap()
    .as_path()
    .to_str()
    .unwrap()
    .to_string();
let d = serde_json::to_string(&data).unwrap();
FileService::FileService::save_text(&filename, &dir, d).unwrap();

}
pub fn read_json_to_curdir_tick(filename: String,)->Vec<Buffer_Fetcher::BufferObject> {
    let dir = std::env::current_dir()
    .unwrap()
    .as_path()
    .to_str()
    .unwrap()
    .to_string();
    let text = FileService::FileService::read_text(&filename, &dir).unwrap();
    let d=serde_json::from_str(text.as_str()).unwrap();
    d
}