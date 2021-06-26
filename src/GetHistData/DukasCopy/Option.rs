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