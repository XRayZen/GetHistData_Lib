
use serde::{Deserialize, Serialize};
pub struct InstrumentMetaData
{
    pub  name :String,
    pub  description :String,
    pub  decimalFactor :String,
    pub  startHourForTicks :String,
    pub  startDayForMinuteCandles :String,
    pub  startMonthForHourlyCandles: String,
    pub  startYearForDailyCandles :String,
}

pub struct Instrument {
    pub DataProviderName: String,
    pub Key: String,
    pub Name: String,
    pub Description: String,
    pub historicalFileName: String,
}
#[derive(Serialize,Deserialize,Debug,PartialEq, Eq,Clone,PartialOrd, Ord)]
pub enum DukasTimeFrame {
    tick,
    m1,
    m15,
    m30,
    h1,
    d1,
    mn1,
}


#[derive(Serialize,Deserialize,strum_macros::ToString,Clone,PartialEq, Eq, PartialOrd, Ord)]
pub enum Price_Type {
    bid,
    ask,
}


pub enum Format {
    array,
    json,
    csv,
}
