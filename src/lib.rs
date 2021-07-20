use chrono::{DateTime, Utc};
use DB_Lib::SubMod::DBCtrl::{
    Convert::DeConvertTick,
    FindRead::{ReadTickDataCheck, ReadTickSelectTime},
    Insert::SaveTick,
};
use GetHistData::GetTickDukas;
use Standard_Lib::datas::market::Tick;

mod GetHistData;
pub mod Test;
#[cfg(test)]
mod tests {}

//バックテスターなどがデータがないか最初に来るメソッド
pub async fn Get_HistricRate(
    SymbolName: &str,
    From: &DateTime<Utc>,
    To: &DateTime<Utc>,
    providerType: &DataProviderType,
    dataType: &GetHistDataType,
) -> Vec<Tick> {
    match providerType {
        DataProviderType::Dukascopy => match dataType {
            GetHistDataType::Tick => {
                if ReadTickDataCheck(SymbolName, To, From).await {
                    let res = ReadTickSelectTime(SymbolName, To, From).await;
                    match res {
                        Ok(item) => {
                            let data = DeConvertTick(item);
                            return data;
                        }
                        Err(err) => {
                            println!("{}", &err);
                            return Vec::new();
                        }
                    }
                } else {
                    //要求したデータがデータベースに存在しないのならダウンロードする
                    let res = GetTickDukas(SymbolName, From, To);
                    match res {
                        Ok(item) => {
                            let save_res = SaveTick(SymbolName, &item).await;
                            match save_res {
                                Ok(insert_c) => {
                                    println!("insert count:{}", insert_c);
                                }
                                Err(err) => {
                                    println!("{}", &err);
                                }
                            }
                            return item.ticks;
                        }
                        Err(err) => {
                            println!("{}", &err);
                            return Vec::new();
                        }
                    }
                }
            }
            GetHistDataType::OHLC => {
                //TODO:バーをリクエストされた場合はティックをリサンプルするか　素直にバーをプロバイダーにリクエストするか検討
                return Vec::new();
            }
        },
        DataProviderType::TrueFX => {
            //TODO:TrueFXは解析がされていないためまだ未実装
            return Vec::new();
        }
    }
}

pub enum GetHistDataType {
    Tick,
    OHLC,
}

pub enum DataProviderType {
    Dukascopy,
    TrueFX,
}
