use crate::oeis::Sequence;

const MIN_LEN: usize = 12;

pub struct CleanSequence {
    pub id: u32,
    pub terms: Vec<i128>,
}

fn long_enough(seq: &Sequence) -> bool {
    return seq.terms.len() >= MIN_LEN;
}

pub fn clean(sequences: Vec<Sequence>) {
    let mut count = 0;
    for seq in sequences {
        if long_enough(&seq) {
            count += 1;
        }
    }

    println!("{} clean", count);

    // TODO: Return result
}
