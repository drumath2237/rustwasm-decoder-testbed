// use std::{fs::File, io::Read};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::zip_test::SogFile;
// use zip::result::ZipError;

// use crate::{types::metajson::MetaJsonType, zip_test::SogFile};

mod zip_test;

mod types;

#[wasm_bindgen(getter_with_clone)]
pub struct FileInSog {
    pub name: String,
    pub data: Vec<u8>,
}

#[wasm_bindgen]
pub fn zip_test_wasm(data: Vec<u8>) -> Vec<FileInSog> {
    let sog_files = zip_test::zip_test(&data).unwrap();
    sog_files
        .iter()
        .map(|file| match file {
            SogFile::MetaJson(_meta) => FileInSog {
                name: "meta.json".to_string(),
                data: [].to_vec(),
            },
            SogFile::Image { filename, data } => FileInSog {
                name: filename.to_string(),
                data: data.to_vec(),
            },
        })
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut file = File::open("./sample_data/pizza.sog").unwrap();
    // let mut bytes = Vec::new();
    // let _size = file.read_to_end(&mut bytes).unwrap();
    // let files = zip_test::zip_test(&bytes)?;

    // let sog_file = files
    //     .iter()
    //     .find(|f| match f {
    //         SogFile::MetaJson(_) => true,
    //         SogFile::Image {
    //             data: _,
    //             filename: _,
    //         } => false,
    //     })
    //     .ok_or(ZipError::FileNotFound)?;

    // match sog_file {
    //     SogFile::MetaJson(json_str) => {
    //         let json: MetaJsonType = serde_json::from_str(&json_str).unwrap();
    //         println!("{:?}", json.means.files)
    //     }
    //     SogFile::Image {
    //         filename: _,
    //         data: _,
    //     } => println!("no"),
    // }

    Ok(())
}
