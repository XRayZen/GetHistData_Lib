use std::default;

use super::{Buffer_Fetcher::BufferObject, Option::dukas_option, TrueDataTypes::True_Instrument, dates_normaliser::dates_normaliser, generate::Generate_TrueInstrumentData, url_generator::{self, Output_URLGenerate}};

pub struct GetHistoricRates {
    pub dukas_instruments: Vec<True_Instrument>,
}

impl GetHistoricRates {
    pub fn Ready(&mut self) {
        let res = Generate_TrueInstrumentData::Read_DukasInstrumentData();
        if res.len() > 0 {
            Generate_TrueInstrumentData::Generate();
            let r2 = Generate_TrueInstrumentData::Read_DukasInstrumentData();
            self.dukas_instruments = r2;
        }
    }
    pub fn GetHistoricRate(option: &dukas_option, TaskCount: u32) {
        let Date = dates_normaliser::TrueNormaliseDates(
            &option.instrument,
            &option.Dates.from,
            &option.Dates.to,
            &option.timefrme,
            &option.utcoffset,
        );
        let urls = url_generator::url_generator::generateUrls(
            &option.instrument,
            &option.timefrme,
            &option.priceType,
            &Date.adjustedFromDate,
            &Date.adjustedToDate,
        );
let DownloadTicks =Self::GetDownloadData(urls, TaskCount);

    }

    fn GetDownloadData(urls:Vec<Output_URLGenerate>,taskcount:u32)->Vec<BufferObject> {
        let 
    }

    /// Get a mutable reference to the get historic rates's dukas instruments.
    pub fn dukas_instruments_mut(&mut self) -> &mut Vec<True_Instrument> {
        &mut self.dukas_instruments
    }
}
