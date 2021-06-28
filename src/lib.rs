pub mod GetHistData;
#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};

    use crate::GetHistData::DukasCopy::{self, DataTypes::DukasTimeFrame, Get_HistoricRates::GetHistoricRates, Option::dukas_option, TrueDataTypes::True_Instrument, dates_normaliser::dates_normaliser, generate::Generate_TrueInstrumentData, url_generator::url_generator};

    #[test]
    fn it_works() {
        test();

        assert_eq!(2 + 2, 4);
    }
    
    fn test() {
        Generate_TrueInstrumentData::Generate();
        let inst = Generate_TrueInstrumentData::Read_DukasInstrumentData();
        let s = inst.iter().find(|&x| x.Key.contains("USDJPY"));
        let mut  inst=True_Instrument::default();
        match s {
            Some(tst) => inst=tst.clone(),
            None => (),
        }
        let option = dukas_option::new(
            inst,
            Utc::now()-Duration::days(80),
            Utc::now(),
            DukasTimeFrame::tick,
            DukasCopy::DataTypes::Price_Type::ask,
            0,
        );
        let Date =dates_normaliser::TrueNormaliseDates(
            &option.instrument,
            &option.Dates.from,
            &option.Dates.to,
            &option.timefrme,
            &option.utcoffset,
        );
        let urls = url_generator::generateUrls(
            &option.instrument,
            &option.timefrme,
            &option.price_type,
            &Date.adjustedFromDate,
            &Date.adjustedToDate,
        );
        let DownloadTicks =GetHistoricRates::GetDownloadData(urls, 10);
    
    }
}
