#![deny(clippy::all, clippy::pedantic)]

use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub struct School(BTreeMap<u32, BTreeSet<String>>);

impl School {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.0
            .entry(grade)
            .or_insert_with(BTreeSet::new)
            .insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.0.keys().cloned().collect()
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.0.get(&grade).map(|v| v.iter().cloned().collect())
    }
}
