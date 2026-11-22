use anyhow::Result;
use nextterm::lookup::find;
use nextterm::oeis::load_names;
use nextterm::store::load;
use nextterm::{lookup, poly};

pub fn run(terms: Vec<i128>) -> Result<()> {
    let sequences = load()?;
    let names = load_names()?;

    // poly with fallback to lookup
    match poly::predict(&terms) {
        Some(next) => println!("{}  (poly)", next),
        None => match lookup::predict(&sequences, &terms) {
            Some(next) => println!("{}  (lookup)", next),
            None => println!("no answer"),
        },
    }

    let found = find(&sequences, &terms);
    println!("{} matches", found.len());

    for (id, next) in found {
        println!("A{}  next {}  {}", id, next, names[&id]);
    }

    Ok(())
}
