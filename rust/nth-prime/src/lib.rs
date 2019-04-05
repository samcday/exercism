#![deny(clippy::all, clippy::pedantic)]

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn nth(n: usize) -> u32 {
    (2_u32..)
        // After 2, prime numbers are always odd
        .filter(|n| *n == 2 || n.trailing_zeros() == 0)
        // Prime numbers are indivisible by all whole numbers between 2 and the square root of the number being tested.
        .filter(|n| {
            let sqrt = f64::from(*n).sqrt() as u32;
            !(2..=sqrt).any(|factor| n % factor == 0)
        })
        .nth(n)
        .unwrap()
}
