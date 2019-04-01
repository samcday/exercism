pub fn primes() -> impl Iterator<Item = u64> {
    (2u64..)
        // After 2, prime numbers are always odd
        .filter(|n| *n == 2 || n.trailing_zeros() == 0)
        // Prime numbers are indivisible by all whole numbers between 2 and the square root of the number being tested.
        .filter(|n| {
            let sqrt = (*n as f64).sqrt() as u64;
            !(2..=sqrt).any(|factor| n % factor == 0)
        })
}

pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = vec![];
    while n > 1 {
        let prime = primes().filter(|prime| n % prime == 0).next().unwrap();
        n = n / prime;
        factors.push(prime);
    }
    factors
}
