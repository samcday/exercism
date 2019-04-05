#![deny(clippy::all, clippy::pedantic)]

pub fn primes_up_to(upper_bound: usize) -> Vec<usize> {
    let mut marked = (0..=upper_bound).map(|_| true).collect::<Vec<bool>>();

    (2..=upper_bound)
        .filter(|num| {
            if !marked[*num] {
                return false;
            }
            let mut multiple = *num;
            while multiple <= upper_bound {
                marked[multiple] = false;
                multiple += num;
            }
            true
        })
        .collect::<Vec<usize>>()
}
