use anyhow::Result;
use nextterm::clean::clean;
use nextterm::oeis::load_all;

pub fn run() -> Result<()> {
    let sequences = load_all()?;
    clean(sequences);
    Ok(())
}
