use chrono::{DateTime, Utc};

use crate::GetHistData::Service::Util::{
    date::{self, date},
    range::range,
};

use super::{
    DataTypes::{DukasTimeFrame, PriceType},
    TrueDataTypes::{TimeRangeType, True_Instrument},
};

pub mod GetURL;

#[derive(Clone)]
pub struct Output_URLGenerate {
   pub  InstrumentKeyName: String,
   pub  URL: String,
   pub  Timeframe: DukasTimeFrame,
   pub  PriceType: PriceType,
    pub NowDate: DateTime<Utc>,
    pub EndDate: DateTime<Utc>,
}

impl Output_URLGenerate {
    pub fn new(
        InstrumentKeyName: String,
        URL: String,
        Timeframe: DukasTimeFrame,
        PriceType: PriceType,
        NowDate: DateTime<Utc>,
        EndDate: DateTime<Utc>,
    ) -> Self {
        Self {
            InstrumentKeyName,
            URL,
            Timeframe,
            PriceType,
            NowDate,
            EndDate,
        }
    }
}
pub struct url_generator {}

impl url_generator {
    pub fn generateUrls(
        instrumrnt: &True_Instrument,
        timeframe: &DukasTimeFrame,
        pricetype: &PriceType,
        startdate: &DateTime<Utc>,
        enddate: &DateTime<Utc>,
    ) -> Vec<Output_URLGenerate> {
        let rangetype = range::GetClosestAvailableRange(&timeframe, &startdate);
        let dateLimit = Self::getDateLimit(&startdate, &enddate, &timeframe);
        let constructUrls = Self::getConstructor(
            &instrumrnt,
            &pricetype,
            &rangetype,
            &startdate,
            &enddate,
            &timeframe,
        );
        return constructUrls;
    }

    fn getConstructor(
        instrument: &True_Instrument,
        pricetype: &PriceType,
        rangetype: &TimeRangeType,
        startdate: &DateTime<Utc>,
        enddate: &DateTime<Utc>,
        timeframe: &DukasTimeFrame,
    ) -> Vec<Output_URLGenerate> {
        let mut dates: Vec<DateTime<Utc>> = Vec::new();
        let mut tempStartDate = date::date::GetStartOfUtc(&startdate, &rangetype);

        while (&tempStartDate < enddate) == false {
            dates.push(tempStartDate.clone());
            tempStartDate = date::date::GetStartOfUtc(&startdate, rangetype);
        }

        let mut temp_all: Vec<Output_URLGenerate> = Vec::new();
        for item in dates {
            let lastitem = dates.last().unwrap().clone();
            if (lastitem == item) & date::date::IsCurrentRange(&rangetype, &item) {
                let ranges = vec![
                    TimeRangeType::year,
                    TimeRangeType::month,
                    TimeRangeType::day,
                    TimeRangeType::hour,
                ];
                let lookupIndex = ranges
                    .iter()
                    .position(|&x| &x == rangetype)
                    .unwrap()
                    .clone();
                let lowerRangeData = ranges[lookupIndex + 1];
                let temp = Self::getConstructor(
                    &instrument,
                    &pricetype,
                    &lowerRangeData,
                    &item,
                    &enddate,
                    &timeframe,
                );
                for item_ in temp {
                    temp_all.push(Output_URLGenerate::new(
                        instrument.Key.clone(),
                        item_.URL,
                        timeframe.clone(),
                        pricetype.clone(),
                        item.clone(),
                        enddate.clone(),
                    ));
                }
            } else {
                let temp = Self::GetURL(&instrument, &item, &rangetype, &pricetype);
                temp_all.push(Output_URLGenerate::new(
                    instrument.Key.clone(),
                    temp,
                    timeframe.clone(),
                    pricetype.clone(),
                    item.clone(),
                    enddate.clone(),
                ));
            }
        }
        return temp_all;
    }

    fn getDateLimit(
        startdate: &DateTime<Utc>,
        enddate: &DateTime<Utc>,
        timeframe: &DukasTimeFrame,
    ) -> DateTime<Utc> {
        let nowdate = Utc::now();
        let mut adjustedEndDate = Utc::now();
        if enddate < &nowdate {
            adjustedEndDate = enddate.clone();
        } else {
            adjustedEndDate = nowdate;
        }
        let mut dateLimit = adjustedEndDate;

        if (timeframe == &DukasTimeFrame::tick)
            || (timeframe == &DukasTimeFrame::m1)
            || (timeframe == &DukasTimeFrame::m15)
            || (timeframe == &DukasTimeFrame::m30)
        {
            if enddate.timestamp_millis() - startdate.timestamp_millis() <= 3600000 {
                dateLimit = date::date::GetStartOfUtc_offset(
                    &dateLimit,
                    &super::TrueDataTypes::TimeRangeType::hour,
                    1,
                );
            } else {
                dateLimit = date::date::GetStartOfUtc(&dateLimit, &TimeRangeType::hour);
            }
        } else if timeframe == &DukasTimeFrame::h1 {
            dateLimit = date::date::GetStartOfUtc(&dateLimit, &TimeRangeType::hour);
        } else if timeframe == &DukasTimeFrame::d1 {
            dateLimit = date::date::GetStartOfUtc(&dateLimit, &TimeRangeType::day);
        } else if timeframe == &DukasTimeFrame::mn1 {
            dateLimit = date::date::GetStartOfUtc(&dateLimit, &TimeRangeType::month);
        }
        dateLimit
    }

    fn GetURL(
        instrument: &True_Instrument,
        date: &DateTime<Utc>,
        rangetype: &TimeRangeType,
        pricetype: &PriceType,
    ) -> String {
        let URL_ROOT = "https://datafeed.dukascopy.com/datafeed";
        let ymdh = date::date::getYMDH(&date);

        let url = String::new();
        url.push_str(URL_ROOT.clone());
        url.push_str("/");
        url.push_str(&instrument.Key.clone().as_str());
        url.push_str("/");
        url.push_str(&ymdh.year.clone().to_string().as_str());
        url.push_str("/");
        // var url = URL_ROOT + "/" + instrument.Key + "/" + ymdh.year + "/";に相当
        let month = format!("{:0>2}", ymdh.month - 1);
        let day = format!("{:0>2}", ymdh.day);
        let hour = format!("{:0>2}", ymdh.hour);
        match rangetype {
            TimeRangeType::hour => {
                url.push_str(&month.clone());
                url.push_str("/");
                url.push_str(&day.clone());
                url.push_str("/");
                url.push_str(&hour.clone());
                url.push_str("h_ticks.bi5");
            }
            TimeRangeType::day => {
                url.push_str(&month.clone());
                url.push_str("/");
                url.push_str(&day.clone());
                url.push_str("/");
                url.push_str(pricetype.clone().to_string().to_uppercase().as_str());
                url.push_str("_candles_min_1.bi5");
            }
            TimeRangeType::month => {
                url.push_str(&month.clone());
                url.push_str("/");
                url.push_str(pricetype.clone().to_string().to_uppercase().as_str());
                url.push_str("_candles_hour_1.bi5");
            }
            TimeRangeType::year => {
                url.push_str(pricetype.clone().to_string().to_uppercase().as_str());
                url.push_str("_candles_day_1.bi5");
            }
        }

        println!("Generate Url : {}", &url);
        return url;
    }
}
