use anyhow::Result;
use nextterm::fetch::{DATA_DIR, MANIFEST_PATH, download, read_manifest};
use std::path::Path;

pub fn run() -> Result<()> {
    let files = read_manifest(Path::new(MANIFEST_PATH))?;
    for file in &files {
        let dest = Path::new(DATA_DIR).join(&file.name);
        download(&file.url, &dest)?;
        println!("{} {}", file.name, file.url);
    }
    Ok(())
}
