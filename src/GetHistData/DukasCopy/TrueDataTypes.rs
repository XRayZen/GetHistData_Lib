use core::f32;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,Default,Clone,PartialEq, PartialOrd)]
pub struct True_Instrument {
    pub DataProviderName: String,
    pub Key: String,
    pub Name: String,
    pub Description: String,
    pub historicalFileName: String,
    pub Group: True_GroupData,
    pub metaData: True_InstrumentMetaData,
}

impl True_Instrument {
    pub fn new(DataProviderName: String, Key: String, Name: String, Description: String, historicalFileName: String, Group: True_GroupData, metaData: True_InstrumentMetaData) -> Self { Self { DataProviderName, Key, Name, Description, historicalFileName, Group, metaData } }
}

#[derive(Serialize,Deserialize,Debug,Default,Clone,PartialEq, PartialOrd)]
pub struct True_GroupData {
    pub GroupID: String,
    pub GroupName: String,
    pub Pre_GroupName: String,
    pub Tags: Vec<String>,
    pub Group_Instruments: Vec<String>,
}
impl True_GroupData {
    pub fn new(
        GroupID: String,
        GroupName: String,
        Pre_GroupName: String,
        Tags: Vec<String>,
        Group_Instruments: Vec<String>,
    ) -> Self {
        Self {
            GroupID,
            GroupName,
            Pre_GroupName,
            Tags,
            Group_Instruments,
        }
    }
}
#[derive(Serialize,Deserialize,Debug,Default,Clone,PartialEq, PartialOrd)]
pub struct True_InstrumentMetaData {
    pub decimalFactor: f64,
    pub startHourForTicks: String,
    pub startDayForMinuteCandles: String,
    pub startMonthForHourlyCandles: String,
    pub startYearForDailyCandles: String,
}

impl True_InstrumentMetaData {
    pub fn new(decimalFactor: f64, startHourForTicks: String, startDayForMinuteCandles: String, startMonthForHourlyCandles: String, startYearForDailyCandles: String) -> Self { Self { decimalFactor, startHourForTicks, startDayForMinuteCandles, startMonthForHourlyCandles, startYearForDailyCandles } }
}
#[derive(Serialize,Deserialize,Debug,PartialEq, Eq)]
pub enum TimeRangeType {
    hour,
    day,
    month,
    year,
}

impl TimeRangeType {
    pub fn default()->Self{
     self::TimeRangeType::hour
    }
}
