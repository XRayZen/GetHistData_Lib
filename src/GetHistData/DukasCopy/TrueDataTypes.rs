use core::f32;

pub struct True_Instrument {
    pub DataProviderName: String,
    pub Key: String,
    pub Name: String,
    pub Description: String,
    pub historicalFileName: String,
    pub Group: True_GroupData,
    pub metaData: True_InstrumentMetaData,
}

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

pub struct True_InstrumentMetaData {
    pub decimalFactor: f32,
    pub startHourForTicks: String,
    pub startDayForMinuteCandles: String,
    pub startMonthForHourlyCandles: String,
    pub startYearForDailyCandles: String,
}

pub enum TimeRangeType {
    hour,
    day,
    month,
    year,
}
