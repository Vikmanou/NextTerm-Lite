use anyhow::Result;
use nextterm::oeis::load_all;

pub fn run() -> Result<()> {
    let sequences = load_all()?;
    for seq in &sequences {
        println!("{}", seq.id);
    }

    Ok(())
}
