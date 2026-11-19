use anyhow::Result;
use nextterm::oeis::load_all;

pub fn run(id: u32) -> Result<()> {
    let sequences = load_all()?;

    // there are gaps in OEIS ids
    for seq in &sequences {
        if seq.id == id {
            println!("Sequence: {}", seq.terms.join(", "));
        }
    }

    Ok(())
}
