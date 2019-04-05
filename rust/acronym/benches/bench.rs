#![feature(test)]

extern crate test;

use acronym;

#[bench]
fn bench_mine(b: &mut test::Bencher) {
    b.iter(|| {
        acronym::abbreviate("");
        acronym::abbreviate("Portable Network Graphics");
        acronym::abbreviate("Ruby on Rails");
        acronym::abbreviate("HyperText Markup Language");
        acronym::abbreviate("First In, First Out");
        acronym::abbreviate("PHP: Hypertext Preprocessor");
        acronym::abbreviate("GNU Image Manipulation Program");
        acronym::abbreviate("Complementary metal-oxide semiconductor");
    });
}
