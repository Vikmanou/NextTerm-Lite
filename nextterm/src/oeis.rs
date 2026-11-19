use anyhow::Result;
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn open_gz(path: &Path) -> Result<BufReader<GzDecoder<File>>> {
    let file = File::open(path)?;
    let unzipped = GzDecoder::new(file);
    Ok(BufReader::new(unzipped))
}
