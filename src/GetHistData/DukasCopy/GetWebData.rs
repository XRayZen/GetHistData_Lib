use std::{u8};

use reqwest::{header};

pub mod generate_data_types;

pub struct GetWeb {}

impl GetWeb {
    ///reqwestのblockingでhttpでテキストを取得する
    pub fn GetWebText(Uri: &String, referer: &String) -> Result<String, reqwest::Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::REFERER,
            header::HeaderValue::from_str(&referer).unwrap(),
        );
        let cl = reqwest::blocking::Client::new();
        cl.get(Uri).headers(headers.clone()).send()?.text()
    }
    ///バイナリをダウンロードするが大きいやつでプログレスが必要なら先にファイルサイズを読んで・・の処理が必要
    pub fn GetWebBytes(Uri: &String, referer: &String) -> Result<Vec<u8>, reqwest::Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::REFERER,
            header::HeaderValue::from_str(&referer).unwrap(),
        );
        let cl = reqwest::blocking::Client::new();
        let j = cl
            .get(Uri)
            .headers(headers.clone())
            .send()?
            .bytes()?
            .to_vec();
        Ok(j)
    }
}
