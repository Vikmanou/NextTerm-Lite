use anyhow::Result;
use nextterm::lookup::find;
use nextterm::oeis::load_names;
use nextterm::store::load;

pub fn run(terms: Vec<i128>) -> Result<()> {
    let sequences = load()?;
    let names = load_names()?;

    let found = find(&sequences, &terms);
    println!("{} matches", found.len());

    for (id, next) in found {
        println!("A{}  next {}  {}", id, next, names[&id]);
    }

    Ok(())
}
