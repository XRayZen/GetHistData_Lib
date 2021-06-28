
use rayon::{prelude::*};
use std::{sync::{Arc, Mutex},};
use Standard_Lib::Util::NotifyUtil::{failed_or_sucess, Notify};



use super::{
    dates_normaliser::dates_normaliser,
    generate::Generate_TrueInstrumentData,
    url_generator::{self, Output_URLGenerate},
    Buffer_Fetcher::BufferObject,
    GetWebData::GetWeb,
    Option::dukas_option,
    TrueDataTypes::True_Instrument,
};

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
            &option.price_type,
            &Date.adjustedFromDate,
            &Date.adjustedToDate,
        );
        let DownloadTicks = Self::GetDownloadData(urls, TaskCount);

    }

    fn GetDownloadData(urls: Vec<Output_URLGenerate>, taskcount: u32) -> Vec<BufferObject> {
        let mut result: Vec<BufferObject> = Vec::new();
        let counter = Arc::new(Mutex::new(0));
        let urls_count = Arc::new(Mutex::new(urls.len() as i32));
        urls.par_iter()
            .map(|x| {
                return Self::task_download(&x, &counter, &urls_count);
            })
            .collect_into_vec(&mut result);
        return result;
    }

    fn task_download(
        url: &Output_URLGenerate,
        counter: &Arc<Mutex<i32>>,
        urls_count: &Arc<Mutex<i32>>,
    ) -> BufferObject {
        let mut fail = false;
        let mut data: Vec<u8> = Vec::new();
        let referer = "https://freeserv.dukascopy.com/".to_string();
        let donwnload_res = GetWeb::GetWebBytes(&url.URL, &referer);
        match donwnload_res {
            Ok(txt) => {
                let mut count = counter.lock().unwrap();
                *count += 1;
                let total_c = urls_count.lock().unwrap();
                Notify::report_on_download(
                    failed_or_sucess::sucess,
                    &"DukasCopy".to_string(),
                    &url.URL,
                    &url.NowDate,
                );
                Notify::ReportProgress(
                    "ヒストリカルデータのダウンロード".to_string(),
                    *total_c,
                    *count,
                );
                data = txt;
            }
            Err(error) => {
                fail = true;
                println!("GetDownloadData Error! : {}", error);
            }
        }
        let r = BufferObject::new(fail, url.URL.clone(), data, url.clone());
        return r;
    }

    /// Get a mutable reference to the get historic rates's dukas instruments.
    pub fn dukas_instruments_mut(&mut self) -> &mut Vec<True_Instrument> {
        &mut self.dukas_instruments
    }
}
