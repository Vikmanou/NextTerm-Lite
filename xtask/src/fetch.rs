use anyhow::Result;
use nextterm::fetch::{MANIFEST_PATH, read_manifest};
use std::path::Path;

pub fn run() -> Result<()> {
    let files = read_manifest(Path::new(MANIFEST_PATH))?;
    for file in &files {
        println!("{} {}", file.name, file.url);
    }
    Ok(())
}
