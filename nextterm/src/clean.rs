use crate::oeis::Sequence;

const MIN_LEN: usize = 12;
const MAX_DIGITS: usize = 30; // ODO: Figure out specific implementation of headroom

pub struct CleanSequence {
    pub id: u32,
    pub terms: Vec<i128>,
}

fn long_enough(seq: &Sequence) -> bool {
    return seq.terms.len() >= MIN_LEN;
}

pub fn clean(sequences: Vec<Sequence>) -> Vec<CleanSequence> {
    let total = sequences.len();

    let mut cleaned = Vec::new();

    for mut seq in sequences {
        // cut at the first term with too many digits
        for i in 0..seq.terms.len() {
            if seq.terms[i].trim_start_matches('-').len() > MAX_DIGITS {
                seq.terms.truncate(i);
                break;
            }
        }

        if !long_enough(&seq) {
            continue;
        }

        let mut terms = Vec::new();
        for term in &seq.terms {
            terms.push(term.parse().unwrap());
        }
        cleaned.push(CleanSequence { id: seq.id, terms });
    }

    println!("Clean: {}", total);

    return cleaned;
}
