use anyhow::Result;
use nextterm::oeis::{load_all, load_names};

pub fn run(id: u32) -> Result<()> {
    let sequences = load_all()?;
    let names = load_names()?;

    // there are gaps in OEIS ids
    for seq in &sequences {
        if seq.id == id {
            println!(
                "Sequence (A{} - {}): {}",
                id,
                names[&seq.id],
                seq.terms.join(", ")
            );
        }
    }

    Ok(())
}
