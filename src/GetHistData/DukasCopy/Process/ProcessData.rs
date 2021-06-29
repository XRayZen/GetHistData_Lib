use core::f32;
use std::{
    io::{BufReader, Cursor, Read, Seek},
    sync::{Arc, Mutex},
};

use byteorder::{BigEndian, ReadBytesExt};
use chrono::{DateTime, Datelike, Duration, TimeZone, Timelike, Utc};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use xz2::read::XzDecoder;
use Standard_Lib::{Util::NotifyUtil::Notify, datas::market::{AssetType, BarData, Symbol, Tick}};

use crate::GetHistData::DukasCopy::{Buffer_Fetcher::BufferObject, TrueDataTypes::True_Instrument};

pub mod Decompress;

pub fn Process(response: Vec<BufferObject>, instrument: True_Instrument) -> Symbol {
    let raw_res: Vec<BufferObject> = Vec::new();
    let mut res_ticks: Vec<Vec<Tick>> = Vec::new();
    let mut counter = Arc::new(Mutex::new(0));
    let totalcount = response.len() as f64;
    response
        .par_iter()
        .map(|x| {
            let t = x.Output_URLGenerate.NowDate;
            let basedatetime = Utc
                .ymd(t.year(), t.month(), t.day())
                .and_hms(t.hour(), 00, 00);
            let mut res: Vec<Tick> = Vec::new();
            if x.get_download {
                let mut buf_c = counter.lock().unwrap();
                *buf_c += 1;
                let mut temp = self::DecompressDukasTickToRawTick(
                    x.Buffer,
                    instrument.metaData.decimalFactor.clone(),
                    &basedatetime,
                );
                let n = *buf_c as f64;
                //進捗を報告するが今は省略
                Notify::report_progress("汎用的なティックに変換".to_string(), &totalcount, &n);
                res.append(&mut temp);
            }
            return res;
        })
        .collect_into_vec(&mut res_ticks);
    let mut result_ticks: Vec<Tick> = Vec::new();
    //解答されたティックをまとめる
    for item in res_ticks {
        result_ticks.append(&mut item);
    }
    return CreateSymbolInTick(result_ticks, &instrument);
}

fn CreateSymbolInTick(res_Ticks: Vec<Tick>, instrument: &True_Instrument) -> Symbol {
    let mut Name = "".to_string();
    let assettype=self::ConvertAssetType(&instrument);
    //インデックスなどを汎用的な名前に変換したいがまだ検討中
    match assettype {
        AssetType::Forex => Name=instrument.Key.clone(),
        AssetType::Metal => (),
        AssetType::CFD => (),
        AssetType::Stock => (),
        AssetType::Crypt => (),
        AssetType::Indicator => (),
    }
    let mut  temp=Symbol::default();
    temp.name=Name;temp.ticks=res_Ticks;
    return temp;
}

fn ConvertAssetType(instrument:&True_Instrument)->AssetType {
    //今はFXのみ返す
    if instrument.Group.GroupName.contains("FX") {
        return AssetType::Forex;
    }else {
        return AssetType::CFD;
    }
}

/// 圧縮されたティックを解凍して復元する
pub fn DecompressDukasTickToRawTick(
    data: Vec<u8>,
    decimalfactor: f64,
    basetime: &DateTime<Utc>,
) -> Vec<Tick> {
    let t = XzDecoder::new(data.as_slice());
    let mut dec: Vec<u8> = Vec::new();
    t.read_to_end(&mut dec);
    let cur = Cursor::new(dec);
    let r: Vec<Tick> = Vec::new();
    while cur.position() != dec.len() as u64 {
        let timer = cur.read_i32::<BigEndian>().unwrap() as i64;
        let time = basetime.clone() + Duration::milliseconds(timer);
        let ask = cur.read_i32::<BigEndian>().unwrap() as f64 / decimalfactor;
        let bid = cur.read_i32::<BigEndian>().unwrap() as f64 / decimalfactor;
        let ask_v = cur.read_f64::<BigEndian>().unwrap() as f32;
        let bid_v = cur.read_f64::<BigEndian>().unwrap() as f32;

        let tick = Tick::new(time, ask, bid, ask_v, bid_v);
        r.push(tick);
    }

    return r;
}
