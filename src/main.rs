use std::{fs::File, io::Read};

use zip::result::ZipError;

use crate::{types::metajson::MetaJsonType, zip_test::SogFile};

mod zip_test;

mod types;

fn main() -> Result<(), ZipError> {
    let mut file = File::open("./sample_data/pizza.sog").unwrap();
    let mut bytes = Vec::new();
    let _size = file.read_to_end(&mut bytes).unwrap();
    let files = zip_test::zip_test(&bytes)?;

    let sog_file = files
        .iter()
        .find(|f| match f {
            SogFile::MetaJson(_) => true,
            SogFile::Image {
                data: _,
                filename: _,
            } => false,
        })
        .ok_or(ZipError::FileNotFound)?;

    match sog_file {
        SogFile::MetaJson(json_str) => {
            let json: MetaJsonType = serde_json::from_str(&json_str).unwrap();
            println!("{:?}", json.means.files)
        }
        SogFile::Image {
            filename: _,
            data: _,
        } => println!("no"),
    }

    Ok(())
}
