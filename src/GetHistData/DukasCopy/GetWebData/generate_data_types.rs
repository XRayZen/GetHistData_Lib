use core::f64;
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct HolidayData
{
    pub  holiday_id :String,
    pub  start_day:Option<String>,
    pub  start_hour :String,
    pub  start_minute :String,
    pub  end_day :Option<String> ,
    pub  end_hour :String,
    pub  end_minute :String,
    pub  timezone :String,
    pub  updated :String,
    pub  timezone_offset :f64,
    pub  timezone_hours_offset :f64,
    pub  start :DateTime<Utc>,
    pub  end :DateTime<Utc>,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct TradingStop {
    pub WEEKLY:Option<Vec<HolidayData>> ,
    pub DAILY:Option<Vec<HolidayData>> ,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct InstrumentData {
    pub title: String,
    pub special: bool,
    pub name: String,
    pub description: String,
    pub historical_filename:Option<String>,
    pub pipValue: f64,
    pub base_currency: String,
    pub quote_currency: String,
    pub commodities_per_contract: Option<String>,
    pub tag_list: Vec<String>,
    pub history_start_tick: String,
    pub history_start_10sec: String,
    pub history_start_60sec: String,
    pub history_start_60min: String,
    pub history_start_day: String,
    pub unit:Option<String>,
    pub trading_stop:Option<HashMap<String,Vec<TradingStop>>>,
    //Option<TradingStop> ,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct GroupData {
    pub id: String,
    pub title: String,
    pub parent:Option<String> ,
    pub instruments:Option<Vec<String>> ,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct MetaDataResponse {
    pub instruments: HashMap<String, InstrumentData>,
    pub groups: HashMap<String, GroupData>,
    pub tags: Vec<String>,
}