use super::range::TimeRange_Bool;
use crate::GetHistData::DukasCopy::TrueDataTypes::TimeRangeType;
use chrono::{DateTime, Datelike, Duration, TimeZone, Timelike, Utc};

pub struct temp_Res_isCurrentdate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
}

impl temp_Res_isCurrentdate {
    pub fn new(year: i32, month: u32, day: u32, hour: u32) -> Self {
        Self {
            year,
            month,
            day,
            hour,
        }
    }
}
pub struct date {}

impl date {
    pub fn getIsCurrentObj(&startDate: &DateTime<Utc>) -> TimeRange_Bool {
        let dateUTC_Res = Self::getYMDH(&startDate);
        let dateUTCNow_Res = Self::getYMDH(&Utc::now());

        let isCurrentYear = if dateUTC_Res.year == dateUTCNow_Res.year {
            true
        } else {
            false
        };
        let isCurrentMonth = if (isCurrentYear) & (dateUTC_Res.month == dateUTCNow_Res.month) {
            true
        } else {
            false
        };
        let isCurrentDay = if (isCurrentMonth) & (dateUTC_Res.day == dateUTCNow_Res.day) {
            true
        } else {
            false
        };
        let isCurrentHour = if (isCurrentDay) & (dateUTC_Res.hour == dateUTCNow_Res.hour) {
            true
        } else {
            false
        };
        TimeRange_Bool::new(isCurrentYear, isCurrentMonth, isCurrentDay, isCurrentHour)
    }
}
impl date {
    pub fn getYMDH(ConvertUTC: &DateTime<Utc>) -> temp_Res_isCurrentdate {
        temp_Res_isCurrentdate::new(
            ConvertUTC.year(),
            ConvertUTC.month(),
            ConvertUTC.day(),
            ConvertUTC.hour(),
        )
    }
    pub fn IsCurrentRange(rangeType: &TimeRangeType, datetime: &DateTime<Utc>) -> bool {
        let IsCurrent = Self::getIsCurrentObj(&datetime);
        match rangeType {
            TimeRangeType::day => IsCurrent.day,
            TimeRangeType::month => IsCurrent.month,
            TimeRangeType::year => IsCurrent.year,
            _ => false,
        }
    }
    pub fn GetStartOfUtc(date: &DateTime<Utc>, period: &TimeRangeType) -> DateTime<Utc> {
        let ymdh = Self::getYMDH(date);
        let mut startOfUtc = Utc::now();
        match period {
            TimeRangeType::hour => {
                startOfUtc = Utc
                    .ymd(ymdh.year, ymdh.month, ymdh.day)
                    .and_hms(ymdh.hour, 00, 00)
            }
            TimeRangeType::day => {
                startOfUtc = Utc.ymd(ymdh.year, ymdh.month, ymdh.day)
                .and_hms(00, 00, 00)
            }
            TimeRangeType::month => {
                startOfUtc = Utc.ymd(ymdh.year, ymdh.month, 1)
                .and_hms(00, 00, 00)
            }
            TimeRangeType::year => startOfUtc = Utc.ymd(ymdh.year, 1, 1).and_hms(0, 00, 00),
        }
        startOfUtc
    }
    pub fn GetStartOfUtc_offset(
        date: &DateTime<Utc>,
        period: &TimeRangeType,
        offset: i32,
    ) -> DateTime<Utc> {
        let ymdh = Self::getYMDH(date);
        match period {
            TimeRangeType::hour => {
                Utc.ymd(ymdh.year, ymdh.month, ymdh.day)
                    .and_hms(ymdh.hour, 00, 00)
                    + Duration::hours(offset.into())
            }
            TimeRangeType::day => {
                Utc.ymd(ymdh.year, ymdh.month, ymdh.day)
                .and_hms(00, 00, 00)
                    + Duration::days(offset.into())
            }
            //月に加算できるのがこれしかないがこれでいいのか疑問
            TimeRangeType::month => {
                let mut temp = Utc.ymd(ymdh.year, ymdh.month, 1)
                .and_hms(00, 00, 00);
                Standard_Lib::Util::TimeUtil::add_onemonth(&temp)
            }
            TimeRangeType::year => {
                let y = ymdh.year;
                Utc.ymd(y + 1, ymdh.month, 1).and_hms(00, 00, 00)
            }
        }
    }
}
