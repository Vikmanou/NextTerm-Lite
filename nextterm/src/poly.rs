pub fn predict(terms: &Vec<i128>) -> Option<i128> {
    if terms.len() < 3 {
        return None;
    }

    if terms.iter().all(|t| *t == terms[0]) {
        return Some(terms[0]);
    }

    let mut diffs = Vec::new();
    for i in 0..terms.len() - 1 {
        diffs.push(terms[i + 1].checked_sub(terms[i])?);
    }

    let next_diff = predict(&diffs)?;
    return terms[terms.len() - 1].checked_add(next_diff);
}
