use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::path::Path;

pub const MANIFEST_PATH: &str = "data/oeis/manifest.json";
pub const DATA_DIR: &str = "data/oeis";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFile {
    pub name: String,
    pub url: String,
}

pub fn read_manifest(path: &Path) -> Result<Vec<DataFile>> {
    let text = std::fs::read_to_string(path)?;
    let files = serde_json::from_str(&text)?;
    Ok(files)
}

pub fn download(url: &str, dest: &Path) -> Result<()> {
    println!("{}", url);

    let mut response = ureq::get(url).header("User-Agent", "nextterm").call()?;

    let mut file = File::create(&dest)?;
    io::copy(&mut response.body_mut().as_reader(), &mut file)?;

    println!("Done");

    Ok(())
}
