use chrono::{DateTime, Duration, Utc};

use crate::GetHistData::Service::Util;

use super::{DataTypes::DukasTimeFrame,  TrueDataTypes::True_Instrument};

pub struct Adjustment_Date {
    pub adjustedFromDate: DateTime<Utc>,
    pub adjustedToDate: DateTime<Utc>,
}

impl Adjustment_Date {
    pub fn new(adjustedFromDate: DateTime<Utc>, adjustedToDate: DateTime<Utc>) -> Self {
        Self {
            adjustedFromDate,
            adjustedToDate,
        }
    }
}
pub struct dates_normaliser {}

impl dates_normaliser {
    pub fn TrueNormaliseDates(
        instrument: &True_Instrument,
        start: &DateTime<Utc>,
        end: &DateTime<Utc>,
        timeframe: &DukasTimeFrame,
        utcoffset: &i32,
    ) -> Adjustment_Date {
        let mut Start = start.clone();
        let mut End = end.clone();
        let mut minFromIsoDate = &instrument.metaData.startHourForTicks;
        match timeframe {
            DukasTimeFrame::m1 => minFromIsoDate = &instrument.metaData.startDayForMinuteCandles,
            DukasTimeFrame::m15 => minFromIsoDate = &instrument.metaData.startDayForMinuteCandles,
            DukasTimeFrame::m30 => minFromIsoDate = &instrument.metaData.startDayForMinuteCandles,
            DukasTimeFrame::h1 => minFromIsoDate = &instrument.metaData.startMonthForHourlyCandles,
            DukasTimeFrame::d1 => minFromIsoDate = &instrument.metaData.startYearForDailyCandles,
            DukasTimeFrame::mn1 => minFromIsoDate = &instrument.metaData.startYearForDailyCandles,
            _ => (),
        }
        let mut minFromDate = Util::ISOStringToDateTime(&minFromIsoDate);
        let mut  maxToDate = Utc::now();
        if utcoffset != &i32::default() {
            Start = Start + Duration::minutes(i64::from(utcoffset.clone()));
            End = End + Duration::minutes(i64::from(utcoffset.clone()));
        }
        let adjustedFromDate =Self::applyDateLimits(& Start, &minFromDate, &maxToDate);
        let adjustedToDate =Self::applyDateLimits(&End, &minFromDate, &maxToDate);
        return Adjustment_Date::new(adjustedFromDate.clone(), adjustedToDate.clone());
    }
    
}

impl dates_normaliser {
    pub fn applyDateLimits<'a>(
        date: &'a  DateTime<Utc>,
        min: &'a  DateTime<Utc>,
        max: &'a  DateTime<Utc>,
    ) -> &'a  DateTime<Utc> {
        if date < max {
            if date < min {
                min
            } else {
                date
            }
        } else {
            max
        }
    }
}
