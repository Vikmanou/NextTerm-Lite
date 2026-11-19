use anyhow::Result;
use nextterm::fetch::DATA_DIR;
use nextterm::oeis::open_gz;
use std::io::BufRead;
use std::path::Path;

pub struct Sequence {
    pub id: u32,
    pub terms: Vec<String>,
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

pub fn run() -> Result<()> {
    let path = Path::new(DATA_DIR).join("stripped.gz");
    let reader = open_gz(&path)?;

    let mut count = 0;
    for line in reader.lines() {
        let line = line?;
        if !line.starts_with("#") {
            count += 1;
        }

        if count < 10 {
            println!("{line}");

            let sequence: Option<Sequence> = parse_line(&line);
            match sequence {
                Some(s) => println!("{}", s.id),
                None => println!("not a sequence"),
            }
        }
    }

    println!("{count} entries");

    Ok(())
}
