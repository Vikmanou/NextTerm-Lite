use anyhow::Result;
use nextterm::fetch::DATA_DIR;
use nextterm::oeis::open_gz;
use std::io::BufRead;
use std::path::Path;

pub fn run() -> Result<()> {
    let path = Path::new(DATA_DIR).join("stripped.gz");
    let reader = open_gz(&path)?;

    for line in reader.lines().take(10) {
        let line = line?;
        print!("{line}");
    }

    Ok(())
}
