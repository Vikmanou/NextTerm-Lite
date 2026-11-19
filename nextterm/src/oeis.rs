use anyhow::Result;
use flate2::read::GzDecoder;
use std::collections::HashMap;
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
    let id = line[1..7].parse().ok()?;

    let terms = line[8..]
        .trim_matches(',')
        .split(',')
        .map(|t| t.to_string())
        .collect();

    Some(Sequence { id, terms })
}

pub fn load_names() -> Result<HashMap<u32, String>> {
    let mut names = HashMap::new();

    let path = Path::new(DATA_DIR).join("names.gz");
    let reader = open_gz(&path)?;

    for line in reader.lines() {
        let line = line?;

        if line.starts_with('#') {
            continue;
        };

        let id = line[1..7].parse()?;
        let name = line[8..].to_string();

        names.insert(id, name);
    }

    Ok(names)
}

pub fn load_all() -> Result<Vec<Sequence>> {
    let mut sequences = Vec::new();

    let path = Path::new(DATA_DIR).join("stripped.gz");
    let reader = open_gz(&path)?;

    for line in reader.lines() {
        let line = line?;

        if line.starts_with('#') {
            continue;
        }

        match parse_line(&line) {
            Some(s) => sequences.push(s),
            None => {}
        }
    }

    Ok(sequences)
}
