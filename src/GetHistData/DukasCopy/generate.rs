use regex::Regex;

use crate::GetHistData::Service::Util::{self, generateTrueIdNane};

use super::{
    GetWebData::{self, generate_data_types::MetaDataResponse},
    TrueDataTypes::{True_GroupData, True_Instrument, True_InstrumentMetaData},
};

pub struct Generate_TrueInstrumentData {}

impl Generate_TrueInstrumentData {
    pub fn Generate() {
        let obj = Self::GetMetaData();
        let generateInstrumentGroupList = Self::generateInstrumentGroupData(&obj);
        let geneInstList = Self::GenerateInstrumentList(&obj, &generateInstrumentGroupList);
    }
    pub fn generateInstrumentGroupData(obj: &MetaDataResponse) -> Vec<True_GroupData> {
        let mut res_itme: Vec<True_GroupData> = Vec::new();
        for item in obj.groups.iter() {
            if item.1.instruments.iter().len() > 0 {
                //すべての空白を-で置き換える
                let group_id = Regex::new(r"[^A-Za-z0-9]+")
                    .unwrap()
                    .replace(&item.0, "-")
                    .to_string();

                let mut group_name = String::new();
                if item.1.id == "" {
                    group_name = item.1.title.to_string();
                } else {
                    group_name = item.1.id.to_string();
                }
                let pre_group_name = item.0.clone();
                let mut list_instruments: Vec<String> = Vec::new();

                match item.1.instruments.clone() {
                    Some(data) => {
                        for item_instrument in data {
                            list_instruments
                                .push(generateTrueIdNane(&"".to_string(), &item_instrument));
                        }
                        let Save = True_GroupData::new(
                            group_id,
                            group_name,
                            pre_group_name,
                            Vec::new(), //ここは後で入れるから空にしておく
                            list_instruments,
                        );
                        res_itme.push(Save);
                    }
                    None => (),
                };
            }
        }
        res_itme
    }

    pub fn GenerateInstrumentList(
        obj: &MetaDataResponse,
        groupDates: &Vec<True_GroupData>,
    ) -> Vec<True_Instrument> {
        let mut Res: Vec<True_Instrument> = Vec::default();
        for item in obj.instruments.iter() {
            let mut inst = item.1.clone();
            let mut Hist_Filename = String::new();
            match inst.historical_filename.clone() {
                Some(txt) => Hist_Filename = txt,
                None => (),
            }
            let groupdata = groupDates.clone();
            let mut key = Util::generateTrueIdNane(&Hist_Filename, item.0);
            let group_option = groupdata
                .iter()
                .find(|&x| x.Group_Instruments.contains(&key));
            let mut group = True_GroupData::default();
            match group_option.clone() {
                Some(data) => {
                    group = data.clone();
                    group.Tags = item.1.tag_list.clone();
                }
                None => (),
            }

            let decimal = 10.0 / &inst.pipValue.clone();
            let metadata = True_InstrumentMetaData::new(
                decimal,
                Util::ToISOString(&inst.history_start_tick),
                Util::ToISOString(&inst.history_start_60sec),
                Util::ToISOString(&inst.history_start_60min),
                Util::ToISOString(&inst.history_start_day),
            );

            let res = True_Instrument::new(
                "dukasCopy".to_string(),
                key.clone(),
                inst.name.clone(),
                inst.description.clone(),
                Hist_Filename,
                group,
                metadata,
            );
            Res.push(res);
        }
        return Res;
    }

    /**
    現時点（2021年6月26日）では最新のjsonをうまくデシリアライズできないため
    ローカルにある古いjsonファイルを読みこんで使う
    が・・、最新のでもいくつかの定義を更新してリプレースで[]をnullにするとかスキップデシリアライズで
    でうまく処理できる可能性があるがここまでに時間を使いすぎたため次にjsonが効かなくなったら
    書き換える
    */
    pub fn GetMetaData() -> MetaDataResponse {
        let URI =
            "https://freeserv.dukascopy.com/2.0/index.php?path=common%2Finstruments".to_string();
        let referer = "https://freeserv.dukascopy.com/".to_string();
        let res = GetWebData::GetWeb::GetWebText(&URI, &referer);
        let mut text = String::new();
        match res {
            Ok(txt) => text = txt,
            Err(err) => println!("Error! {}", &err),
        }
        text.replace("jsonp(", "");

        let mut data = String::new();
        let path = <String as std::str::FromStr>::from_str(
            "C:/Trade_System/AppDevelop/Rust TradeCenter/raw-meta-data-2021-01-02.json",
        )
        .unwrap();
        let mut res = std::fs::File::open(path).unwrap();
        std::io::Read::read_to_string(&mut res, &mut data).unwrap();
        Self::Convertjson_metadatares(&data)
    }

    fn Convertjson_metadatares(txt: &String) -> MetaDataResponse {
        let res = serde_json::from_str(&txt).unwrap();
        res
    }
}
