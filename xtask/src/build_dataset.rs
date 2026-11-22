use anyhow::Result;
use nextterm::clean::clean;
use nextterm::oeis::load_all;
use nextterm::store::save;

pub fn run() -> Result<()> {
    let sequences = load_all()?;
    let cleaned = clean(sequences);
    save(&cleaned)?;
    return Ok(());
}
