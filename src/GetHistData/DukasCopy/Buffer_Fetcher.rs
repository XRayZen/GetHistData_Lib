use super::url_generator::Output_URLGenerate;
use serde::{Deserialize, Serialize};

#[derive(Clone,PartialEq, Eq, PartialOrd, Ord,Serialize,Deserialize)]
pub struct BufferObject {
    pub get_download:bool,
    pub url: String,
    pub Buffer: Vec<u8>,
    pub Output_URLGenerate: Output_URLGenerate,
}

impl BufferObject {
    pub fn new(get_download: bool, url: String, Buffer: Vec<u8>, Output_URLGenerate: Output_URLGenerate) -> Self { Self { get_download, url, Buffer, Output_URLGenerate } }
}

