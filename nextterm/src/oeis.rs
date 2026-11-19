use anyhow::Result;
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::fetch::DATA_DIR;

pub struct Sequence {
    pub id: u32,
    pub terms: Vec<String>,
}

pub fn open_gz(path: &Path) -> Result<BufReader<GzDecoder<File>>> {
    let file = File::open(path)?;
    let unzipped = GzDecoder::new(file);
    Ok(BufReader::new(unzipped))
}

// A000006 ,1,1,2,2,3,3,4,4,4,5,5,6,6,6,6,7,7,7,8,8,8,8,9,9,9,10,10,10,10,10,11,11,11,11,12,12,12,12,12,13,13,13,13,13,14,14,14,14,15,15,15,15,15,15,16,16,16,16,16,16,16,17,17,17,17,17,18,18,18,18,18,
pub fn parse_line(line: &str) -> Option<Sequence> {
    let (identifier, terms) = line.split_once(' ')?;

    let id = identifier.trim_start_matches('A').parse().ok()?;

    let terms = terms
        .trim_matches(',')
        .split(',')
        .map(|t| t.to_string())
        .collect();

    Some(Sequence { id, terms })
}

pub fn load_all() -> Result<Vec<Sequence>> {
    let mut sequences = Vec::new();

    let path = Path::new(DATA_DIR).join("stripped.gz");
    let reader = open_gz(&path)?;

    for line in reader.lines() {
        let line = line?;

        let sequence: Option<Sequence> = parse_line(&line);
        match sequence {
            Some(s) => {
                sequences.push(s);
            }
            None => println!("not a sequence"),
        }
    }

    Ok(sequences)
}
