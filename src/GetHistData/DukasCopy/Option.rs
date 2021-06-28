use chrono::{DateTime, Utc};

use super::{
    DataTypes::{DukasTimeFrame, Price_Type},
    TrueDataTypes::True_Instrument,
};

pub struct Dates {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

impl Dates {
    pub fn new(from: DateTime<Utc>, to: DateTime<Utc>) -> Self { Self { from, to } }
}
pub struct dukas_option {
    pub instrument: True_Instrument,
    pub Dates: Dates,
    pub timefrme: DukasTimeFrame,
    pub price_type: Price_Type,
    pub utcoffset: i32,
}

impl dukas_option {
    pub fn new(
        instrument: True_Instrument,
        from:DateTime<Utc>,
        to:DateTime<Utc>,
        timefrme: DukasTimeFrame,
        price_type: Price_Type,
        utcoffset: i32,
    ) -> Self {
        let date=Dates::new(from, to);
        Self{
            instrument,
            Dates:date,
            timefrme,
            price_type,
            utcoffset
        }
    }
}
