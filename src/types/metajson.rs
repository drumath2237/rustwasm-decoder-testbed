use serde::Deserialize;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct MetaJsonType {
    pub version: i32,
    pub count: u32,
    pub antialias: Option<bool>,
    pub means: Means,
    pub scales: Scales,
    pub quats: Quats,
    pub sh0: Sh0,
    #[serde(rename(deserialize = "shN"))]
    pub sh_n: Option<ShN>,
}

#[derive(Debug, Deserialize, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct Means {
    pub mins: Vec<f32>,
    pub maxs: Vec<f32>,
    pub files: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct Scales {
    pub codebook: Vec<f32>,
    pub files: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct Quats {
    pub files: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct Sh0 {
    pub codebook: Vec<f32>,
    pub files: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct ShN {
    pub count: i32,
    pub bands: i32,
    pub codebook: Vec<f32>,
    pub files: Vec<String>,
}
