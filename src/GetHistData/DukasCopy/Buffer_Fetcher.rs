use super::url_generator::Output_URLGenerate;

pub struct BufferObject {
    pub url: String,
    pub Buffer: Vec<u8>,
    pub Output_URLGenerate: Output_URLGenerate,
}

impl BufferObject {
    pub fn new(url: String, Buffer: Vec<u8>, Output_URLGenerate: Output_URLGenerate) -> Self {
        Self {
            url,
            Buffer,
            Output_URLGenerate,
        }
    }
}
