use core::f32;

use chrono::{DateTime, Utc};

use super::{DataTypes::{DukasTimeFrame, PriceType}, TrueDataTypes::True_Instrument};

pub struct Dates{
    pub from:DateTime<Utc>,
    pub to:DateTime<Utc>
}
pub struct dukas_option{
    pub instrument:True_Instrument,
    pub Dates:Dates,
    pub timefrme:DukasTimeFrame,
    pub priceType:PriceType,
    pub utcoffset:i32
}

impl dukas_option {
    pub fn new(instrument: True_Instrument, Dates: Dates, timefrme: DukasTimeFrame, priceType: PriceType, utcoffset: i32) -> Self { Self { instrument, Dates, timefrme, priceType, utcoffset } }
}