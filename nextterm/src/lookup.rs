use crate::clean::CleanSequence;

pub fn find(sequences: &Vec<CleanSequence>, terms: &Vec<i128>) -> Vec<(u32, i128)> {
    let mut found = Vec::new();

    for seq in sequences {
        // needs at least one more term than we were given, to have a "next"
        if seq.terms.len() > terms.len() && seq.terms.starts_with(terms) {
            found.push((seq.id, seq.terms[terms.len()]));
        }
    }

    return found;
}

pub fn predict(sequences: &Vec<CleanSequence>, terms: &Vec<i128>) -> Option<i128> {
    let found = find(sequences, terms);

    if found.is_empty() {
        return None;
    }

    let (_id, next) = found[0];
    return Some(next);
}
