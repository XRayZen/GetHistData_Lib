
use std::{
    io::{Cursor},
    sync::{Arc, Mutex},
};

use byteorder::{BigEndian, ReadBytesExt};
use chrono::{DateTime, Datelike, Duration, TimeZone, Timelike, Utc};
use lzma_rs::{lzma_decompress};
use rayon::{
    iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

use Standard_Lib::{
    datas::market::{AssetType, Symbol, Tick},
    Util::NotifyUtil::Notify,
};

use crate::GetHistData::DukasCopy::{Buffer_Fetcher::BufferObject, TrueDataTypes::True_Instrument};

pub mod Decompress;

pub fn Process(response: Vec<BufferObject>, instrument: True_Instrument) -> Symbol {
    let mut res_ticks: Vec<Vec<Tick>> = Vec::new();
    let mut counter = Arc::new(Mutex::new(0));
    let totalcount_ = response.len() as f64;
    let mut totalcount = Arc::new(Mutex::new(totalcount_));
    response
        .par_iter()
        .map(|x| {
            let t = x.Output_URLGenerate.NowDate;
            let basedatetime = Utc
                .ymd(t.year(), t.month(), t.day())
                .and_hms(t.hour(), 00, 00);
            let mut res: Vec<Tick> = Vec::new();
            if (x.get_download) & (x.Buffer.len() > 0) {
                let mut buf_c = counter.lock().unwrap();
                *buf_c += 1;
                let n = *buf_c as f64;
                let mut temp = self::DecompressDukasTickToRawTick(
                    x.Buffer.clone(),
                    instrument.metaData.decimalFactor.clone(),
                    &basedatetime,
                    &instrument.Name,
                );
                let buf_b = totalcount.lock().unwrap();
                let b = *buf_b as f64;
                //進捗を報告するが今は省略
                Notify::report_progress("汎用的なティックに変換".to_string(), &b, &n);
                for item in temp {
                    res.push(item);
                }
            }
            return res;
        })
        .collect_into_vec(&mut res_ticks);
    let mut result_ticks: Vec<Tick> = Vec::new();
    //解答されたティックをまとめる
    for item in res_ticks {
        let mut temp: Vec<Tick> = Vec::new();
        for tick in item {
            temp.push(tick);
        }
        result_ticks.append(&mut temp);
    }
    return CreateSymbolInTick( result_ticks, &instrument);
}

fn CreateSymbolInTick(mut res_Ticks: Vec<Tick>, instrument: &True_Instrument) -> Symbol {
    let mut Name = "".to_string();
    let assettype = self::convert_asset_type(&instrument);
    //インデックスなどを汎用的な名前に変換したいがまだ検討中
    match assettype {
        AssetType::Forex => Name = instrument.Key.clone(),
        AssetType::Metal => (),
        AssetType::CFD => (),
        AssetType::Stock => (),
        AssetType::Crypt => (),
        AssetType::Indicator => (),
    }
    let mut temp = Symbol::Default_Symbol();
    temp.name = Name;
    temp.ticks=tick_sort_par(res_Ticks);
    debg_view(&temp.ticks);
    return temp;
}

fn tick_sort_par(mut ticks: Vec<Tick>) -> Vec<Tick>  {
    ticks.par_sort_by_key(|x|x.time);
    return ticks;
}


fn convert_asset_type(instrument: &True_Instrument) -> AssetType {
    //今はFXのみ返す
    if instrument.Group.GroupName.contains("FX") {
        return AssetType::Forex;
    } else {
        return AssetType::CFD;
    }
}

fn decompress_lzmars(data: Vec<u8>, Name: &String, time: &DateTime<Utc>) -> Vec<u8> {
    let mut f = std::io::BufReader::new(data.as_slice());
    let mut decomp: Vec<u8> = Vec::new();
    let t = lzma_decompress(&mut f, &mut decomp);
    match t {
        Ok(_) => (),
        Err(err) => println!("Lzma Error! Name:[{}] Time:[{}] Msg:[{}]", Name, time, &err),
    }
    decomp
}

/// 圧縮されたティックを解凍して復元する
fn DecompressDukasTickToRawTick(
    data: Vec<u8>,
    decimalfactor: f64,
    basetime: &DateTime<Utc>,
    Name: &String,
) -> Vec<Tick> {
    let mut dec: Vec<u8> = Vec::new();
    let mut g = decompress_lzmars(data, &Name, &basetime);
    //t.read_to_end(&mut dec);
    dec.append(&mut g);
    let len = dec.len().clone() as u64;
    let mut cur = Cursor::new(dec);
    let mut r: Vec<Tick> = Vec::new();
    while cur.position() != len {
        let timer = cur.read_i32::<BigEndian>().unwrap() as i64;
        let time = basetime.clone() + Duration::milliseconds(timer);
        let ask = cur.read_i32::<BigEndian>().unwrap() as f64 / decimalfactor;
        let bid = cur.read_i32::<BigEndian>().unwrap() as f64 / decimalfactor;
        let ask_v = cur.read_f32::<BigEndian>().unwrap();
        let bid_v = cur.read_f32::<BigEndian>().unwrap();

        let tick = Tick::new(time, ask, bid, ask_v, bid_v);
        r.push(tick);
    }
    return r;
}

fn debg_view(ticks: &Vec<Tick>) {
    let mut count = 0;

    for item in ticks {
        println!("tick Time: {}", item.time);
        if count > 100 {
            break;
        }
        count += 1;
    }
}
