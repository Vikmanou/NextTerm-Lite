use anyhow::Result;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::clean::CleanSequence;

pub const PROCESSED_DIR: &str = "data/processed";

pub fn save(sequences: &Vec<CleanSequence>) -> Result<()> {
    std::fs::create_dir_all(PROCESSED_DIR)?;

    let path = Path::new(PROCESSED_DIR).join("seqs.bin");
    let mut file = BufWriter::new(File::create(path)?);
    bincode::serde::encode_into_std_write(sequences, &mut file, bincode::config::standard())?;

    Ok(())
}
