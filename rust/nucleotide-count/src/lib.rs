#![deny(clippy::all, clippy::pedantic)]

use std::collections::HashMap;

const NUCLEOTIDES: &[char] = &['A', 'G', 'C', 'T'];
fn check_nucleotide(c: char) -> Result<(), char> {
    if NUCLEOTIDES.contains(&c) {
        Ok(())
    } else {
        Err(c)
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    check_nucleotide(nucleotide)?;
    dna.chars().try_fold(0, |count, c| {
        check_nucleotide(c).and(Ok(count + if c == nucleotide { 1 } else { 0 }))
    })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    NUCLEOTIDES
        .iter()
        .map(|nucleotide| count(*nucleotide, dna).and_then(|count| Ok((*nucleotide, count))))
        .collect::<Result<HashMap<char, usize>, char>>()
}
