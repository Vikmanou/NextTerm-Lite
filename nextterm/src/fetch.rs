use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const MANIFEST_PATH: &str = "data/oeis/manifest.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFile {
    pub name: String,
    pub url: String,
    pub sha256: String,
}

pub fn read_manifest(path: &Path) -> Result<Vec<DataFile>> {
    let text = std::fs::read_to_string(path)?;
    let files = serde_json::from_str(&text)?;
    Ok(files)
}
