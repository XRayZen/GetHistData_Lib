use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read, Write},
    path,
};

pub struct FileService {}

impl FileService {
    pub fn read_text(file_name: &String, path: &String) -> Result<String, Box<dyn Error>> {
        let mut  b = String::from(path);
        b.push(path::MAIN_SEPARATOR);
        b.push_str(file_name);
        let file = File::open(b)?;
        let mut  reader = BufReader::new(file);
        let mut buf = String::new();
        reader.read_to_string(&mut buf);
        Ok(buf)
    }
    pub fn save_text(file_name: &String, path: &String, data: String) -> Result<(), Box<dyn Error>> {
        let mut  b = String::from(path);
        b.push(path::MAIN_SEPARATOR);
        b.push_str(file_name);
        let mut  file = File::create(b)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}
