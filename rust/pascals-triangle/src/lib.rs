#![deny(clippy::all, clippy::pedantic)]

use itertools::Itertools;

pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows = vec![];
        let zero = &[0_u32];
        let empty_vec = vec![];

        for _ in 0..self.0 {
            // Create an iterator that is a chain of a single 0 item, followed by the previous row (if any), followed by
            // a single 0 item. We then convert that iterator into a windowing iterator of tuples and add them together.
            rows.push(
                zero.iter()
                    .chain(rows.last().unwrap_or(&empty_vec))
                    .chain(zero)
                    .tuple_windows()
                    .map(|(l, r)| std::cmp::max(1, l + r))
                    .collect::<Vec<u32>>()
                    .to_vec(),
            );
        }
        rows
    }
}
