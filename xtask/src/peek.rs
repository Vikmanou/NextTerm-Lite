use anyhow::Result;
use nextterm::oeis::load_names;
use nextterm::store::load;

pub fn run(id: u32) -> Result<()> {
    let sequences = load()?;
    let names = load_names()?;

    // there are gaps in OEIS ids
    for seq in &sequences {
        if seq.id == id {
            println!(
                "Sequence (A{} - {}): {:?}",
                id,
                names[&seq.id],
                seq.terms
            );
        }
    }

    Ok(())
}
