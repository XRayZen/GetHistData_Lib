
use rayon::prelude::*;
use std::{
    sync::{Arc, Mutex},
};
use Standard_Lib::{
    datas::market::Symbol,
    Util::NotifyUtil::{failed_or_sucess, Notify},
};

use crate::GetHistData::Service::Util;

use super::{
    dates_normaliser::dates_normaliser,
    generate::Generate_TrueInstrumentData,
    url_generator::{self, Output_URLGenerate},
    Buffer_Fetcher::BufferObject,
    GetWebData::GetWeb,
    Option::dukas_option,
    Process::ProcessData::Process,
    TrueDataTypes::True_Instrument,
};

pub struct GetHistoricRates {
    pub dukas_instruments: Vec<True_Instrument>,
}

impl GetHistoricRates {
    pub fn new() -> Self {
        let dukas_instruments=Vec::default();
        Self { dukas_instruments } }

    pub fn Ready(&mut self) {
        let res = Generate_TrueInstrumentData::Read_DukasInstrumentData();
        if res.len() == 0 {
            Generate_TrueInstrumentData::Generate();
            let r2 = Generate_TrueInstrumentData::Read_DukasInstrumentData();
            self.dukas_instruments = r2;
        } else {
            self.dukas_instruments = res;
        }
    }

pub fn Find_SymbolName(&mut self,name:String,)->True_Instrument {
    let inst=self.dukas_instruments.iter().
    find(|x|x.Key.contains(name.as_str()));
    match inst {
        Some(data) => return data.clone(),
        None => (),
    }
    return True_Instrument::default();
}

    pub fn GetHistoricRate(&mut self,option: &dukas_option, TaskCount: u32) -> Symbol {
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
        if !DownloadTicks.is_empty() {
            let min = DownloadTicks
                .par_iter()
                .min_by_key(|n| n.Output_URLGenerate.NowDate);
            let max = DownloadTicks
                .par_iter()
                .max_by_key(|n| n.Output_URLGenerate.NowDate);
            //TODO:近いうちにノッティファイにリポートログ機能を追加する
            //Notify::
            //Debug用にローカルにティックを保存しておく
            Util::save_json_to_curdir_tick(
                "Test_RawTickData.json".to_string(),
                DownloadTicks.clone(),
            );
            let pro_res = Process(DownloadTicks, option.instrument.clone());
            return pro_res;
        }
        return Symbol::Default_Symbol();
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
        result.par_sort();
        result.retain(|x| x.get_download == true);
        return result;
    }

    fn task_download(
        url: &Output_URLGenerate,
        counter: &Arc<Mutex<i32>>,
        urls_count: &Arc<Mutex<i32>>,
    ) -> BufferObject {
        let mut success = false;
        let referer = "https://freeserv.dukascopy.com/".to_string();
        let donwnload_res = GetWeb::GetWebBytes(&url.URL, &referer);
        let mut data: Vec<u8> = Vec::new();
        match donwnload_res {
            Ok(txt) => {
                if !txt.is_empty() {
                    data = txt;
                    success = true;
                } else {
                    success = false;
                }
            }
            Err(error) => {
                success = false;
                println!("GetDownloadData Error! : {}", error);
            }
        }
        if success {
            Notify::report_on_download(
                failed_or_sucess::sucess,
                &"DukasCopy".to_string(),
                &url.URL,
                &url.NowDate,
            );
        }
        let mut count = *counter.lock().unwrap();
        count += 1;
        let count_ = count as f64;
        let total_c = *urls_count.lock().unwrap();
        let total_c_ = total_c as f64;
        Notify::report_progress(
            "ヒストリカルデータのダウンロード".to_string(),
            &total_c_,
            &count_,
        );
        let r = BufferObject::new(success, url.URL.clone(), data, url.clone());
        return r;
    }

    /// Get a mutable reference to the get historic rates's dukas instruments.
    pub fn dukas_instruments_mut(&mut self) -> &mut Vec<True_Instrument> {
        &mut self.dukas_instruments
    }
}
