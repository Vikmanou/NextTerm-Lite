use anyhow::Result;
use nextterm::lookup::find;
use nextterm::oeis::load_names;
use nextterm::predictor::{default_chain, predict_chain};
use nextterm::store::load;

pub fn run(terms: Vec<i128>) -> Result<()> {
    let sequences = load()?;
    let names = load_names()?;

    let found = find(&sequences, &terms);
    let chain = default_chain(sequences);

    match predict_chain(&chain, &terms) {
        Some((name, next)) => println!("{}  ({})", next, name),
        None => println!("no answer"),
    }

    println!("{} matches", found.len());

    for (id, next) in found {
        println!("A{}  next {}  {}", id, next, names[&id]);
    }

    return Ok(());
}
