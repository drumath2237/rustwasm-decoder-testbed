use std::io::Cursor;
use std::io::Read;

use zip::{ZipArchive, result::ZipError};

pub struct RawFile {
    pub name: String,
    pub data: Vec<u8>,
}

pub enum SogFile {
    MetaJson(String),
    Image { filename: String, data: Vec<u8> },
}

pub fn zip_test(zip_bytes: &[u8]) -> Result<Vec<SogFile>, ZipError> {
    let cursor = Cursor::new(zip_bytes);

    let mut archive = ZipArchive::new(cursor)?;
    let mut files = Vec::new();

    for i in 0..archive.len() {
        let mut zip_file = archive.by_index_raw(i)?;
        // raw_files.push(RawFile { name, data: buf });
        let name = zip_file.name().to_owned();

        let sog = if name == "meta.json" {
            let mut buf = String::new();
            zip_file.read_to_string(&mut buf)?;
            SogFile::MetaJson(buf)
        } else {
            let mut buf = Vec::new();
            let _size = zip_file.read_to_end(&mut buf)?;
            SogFile::Image {
                filename: name,
                data: buf,
            }
        };

        files.push(sog);
    }

    Ok(files)
}
