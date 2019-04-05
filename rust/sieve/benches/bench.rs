#![feature(test)]

extern crate test;

use test::Bencher;

#[bench]
fn bench_mine(b: &mut test::Bencher) {
    b.iter(|| sieve::primes_up_to(1000))
}
