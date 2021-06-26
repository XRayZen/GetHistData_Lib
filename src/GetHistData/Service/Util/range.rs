use chrono::{DateTime, Utc};

use crate::GetHistData::DukasCopy::DataTypes::DukasTimeFrame;

pub struct TimeRange_Bool {
   pub  year: bool,
   pub  month: bool,
   pub  day: bool,
   pub  hour: bool,
}

impl TimeRange_Bool {
    pub fn new(year: bool, month: bool, day: bool, hour: bool) -> Self {
        Self {
            year,
            month,
            day,
            hour,
        }
    }
}

pub struct range {}

impl range {
    pub fn GetClosestAvailableRange(timeframe: DukasTimeFrame, startdate: DateTime<Utc>) {}
}
