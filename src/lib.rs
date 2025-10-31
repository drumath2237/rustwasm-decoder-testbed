use wasm_bindgen::prelude::wasm_bindgen;

use crate::{types::metajson::MetaJsonType, zip_test::SogFile};

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

#[wasm_bindgen]
pub fn deserialize_metajson(json: &str) -> MetaJsonType {
    serde_json::from_str(json).unwrap()
}
