use serde::Deserialize;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct Means {
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
    pub files: [String; 2],
}

#[derive(Debug, Deserialize)]
pub struct Scales {
    pub codebook: Vec<f32>,
    pub files: [String; 1],
}

#[derive(Debug, Deserialize)]
pub struct Quats {
    pub files: [String; 1],
}

#[derive(Debug, Deserialize)]
pub struct Sh0 {
    pub codebook: Vec<f32>,
    pub files: [String; 1],
}

#[derive(Debug, Deserialize)]
pub struct ShN {
    pub count: i32,
    pub bands: i32,
    pub codebook: Vec<f32>,
    pub files: [String; 2],
}
