use std::fmt;

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
#[derive(Debug,PartialEq, Eq,Clone)]
pub enum DukasTimeFrame {
    tick,
    m1,
    m15,
    m30,
    h1,
    d1,
    mn1,
}


#[derive(strum_macros::ToString,Clone)]
pub enum PriceType {
    bid,
    ask,
}


pub enum Format {
    array,
    json,
    csv,
}
