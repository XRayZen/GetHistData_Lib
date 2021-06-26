use chrono::{DateTime, Utc};

use super::{DataTypes::{DukasTimeFrame, PriceType}, TrueDataTypes::True_Instrument};

pub mod GetURL;
pub struct Output_URLGenerate{
    InstrumentKeyName:String,
    URL:String,
    Timeframe:DukasTimeFrame,
    PriceType:PriceType,
    pub NowDate:DateTime<Utc>,
    pub EndDate:DateTime<Utc>
}
pub struct url_generator{
    
}

impl url_generator {
    pub fn generateUrls(instrumrnt:True_Instrument,timeframe:DukasTimeFrame,
    pricetype:PriceType,startdate:DateTime<Utc>,enddate:DateTime<Utc>)->Vec<Output_URLGenerate> {
        
    }
}