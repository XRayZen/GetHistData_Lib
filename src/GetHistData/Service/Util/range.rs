use chrono::{DateTime, Utc};

use crate::GetHistData::DukasCopy::{DataTypes::DukasTimeFrame, TrueDataTypes::TimeRangeType};

use super::date::date;

pub struct TimeRange_Bool {
    pub year: bool,
    pub month: bool,
    pub day: bool,
    pub hour: bool,
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
    /// 指定開始日時から最も近い利用可能な範囲の取得<br/>
    /// どれだけ近いかで取得できるヒストリカルデータの時間幅を返す<br/>
    /// 指定した開始日が現時刻とどれだけ近いかで取得できるデータの時間範囲幅が違ってくる<br/>
    pub fn GetClosestAvailableRange(
        timeframe: &DukasTimeFrame,
        startdate: &DateTime<Utc>,
    ) -> TimeRangeType {
        //指定された遡る時間が現在とどれくらい近いかで取得する時間軸が変わる
        let isCurrent = date::getIsCurrentObj(&startdate);
        //指定した開始時間が<br/>
        //今年だったら時間軸の年（年足）は使えない<br/>
        //今年の今月だったら時間軸の月（月足）は使えない<br/>
        // 今月の今日だったら時間軸の日（日足）は使えない<br/>
        // 今日の今の時間だったら時間軸の時間（時間足）は使えない<br/>
        let mut res = TimeRangeType::default();
        if (timeframe == &DukasTimeFrame::mn1) || (timeframe == &DukasTimeFrame::d1) {
            if !isCurrent.year {
                res = TimeRangeType::year;
            } else {
                if !isCurrent.month {
                    res = TimeRangeType::month;
                } else {
                    res = TimeRangeType::day;
                }
            }
        } else if timeframe == &DukasTimeFrame::h1 {
            if !isCurrent.month {
                res = TimeRangeType::month;
            } else if !isCurrent.day {
                res = TimeRangeType::day;
            } else {
                res = TimeRangeType::hour;
            }
        } else if (timeframe == &DukasTimeFrame::m30)
            & (timeframe == &DukasTimeFrame::m15)
            & (timeframe == &DukasTimeFrame::m1)
        {
            if !isCurrent.day {
                res = TimeRangeType::day;
            } else {
                res = TimeRangeType::hour;
            }
        }
        res
    }
}
