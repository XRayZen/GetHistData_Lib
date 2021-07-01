use chrono::{DateTime, Utc};

pub mod GetHistData;
pub mod Test;
#[cfg(test)]
mod tests {

}

pub fn Get_HistricRate(SymbolName:String,From:DateTime<Utc>,To:DateTime<Utc>) {
    
}

pub enum GetHistDataType
{
    Tick, OHLC
}

pub enum DataProviderType
{
    Dukascopy, TrueFX
}