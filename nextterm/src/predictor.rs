use crate::clean::CleanSequence;
use crate::lookup;
use crate::poly;

pub enum Predictor {
    Poly,
    Lookup(Vec<CleanSequence>),
}

impl Predictor {
    pub fn name(&self) -> &'static str {
        match self {
            Predictor::Poly => return "poly",
            Predictor::Lookup(_) => return "lookup",
        }
    }

    pub fn predict(&self, terms: &Vec<i128>) -> Option<i128> {
        match self {
            Predictor::Poly => return poly::predict(terms),
            Predictor::Lookup(sequences) => return lookup::predict(sequences, terms),
        }
    }
}

pub fn default_chain(sequences: Vec<CleanSequence>) -> Vec<Predictor> {
    return vec![Predictor::Poly, Predictor::Lookup(sequences)];
}

pub fn predict_chain(chain: &Vec<Predictor>, terms: &Vec<i128>) -> Option<(&'static str, i128)> {
    for predictor in chain {
        match predictor.predict(terms) {
            Some(next) => return Some((predictor.name(), next)),
            None => {}
        }
    }

    return None;
}
