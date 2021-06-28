use Standard_Lib::datas::market::Symbol;

use crate::GetHistData::DukasCopy::{Buffer_Fetcher::BufferObject, TrueDataTypes::True_Instrument};

pub mod Decompress;

pub fn Process(response:Vec<BufferObject>,instrument:True_Instrument)->Symbol {
    
}