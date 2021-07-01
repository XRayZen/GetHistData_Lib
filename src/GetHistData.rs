

use chrono::{DateTime, Utc};
use Standard_Lib::datas::market::Symbol;

use self::DukasCopy::{DataTypes::DukasTimeFrame, Get_HistoricRates::GetHistoricRates, Option::dukas_option, TrueDataTypes::True_Instrument};

pub mod DukasCopy;
pub mod Service;
pub mod TrueFX;

pub fn GetTickDukas(symbolname: String, From: DateTime<Utc>, To: DateTime<Utc>) -> Result<Symbol,String> {
    let mut t = GetHistoricRates::new();
    t.Ready();
    let inst = t.Find_SymbolName(symbolname.clone());
    if inst==True_Instrument::default() {
        ("Not Found SymbolName! Find is {}",symbolname);
    }
    let option = dukas_option::new(
        inst,
        From,
        To,
        DukasTimeFrame::tick,
        DukasCopy::DataTypes::Price_Type::ask,
        0,
    );
    let sym = t.GetHistoricRate(&option, 20);
    Ok(sym)
}
