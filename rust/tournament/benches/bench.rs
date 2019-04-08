#![feature(test)]

extern crate test;

#[bench]
fn mine(b: &mut test::Bencher) {
    let input = "Courageous Californians;Devastating Donkeys;win\n".to_string()
        + "Allegoric Alaskans;Blithering Badgers;win\n"
        + "Devastating Donkeys;Allegoric Alaskans;loss\n"
        + "Courageous Californians;Blithering Badgers;win\n"
        + "Blithering Badgers;Devastating Donkeys;draw\n"
        + "Allegoric Alaskans;Courageous Californians;draw";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  3 |  2 |  1 |  0 |  7\n"
        + "Courageous Californians        |  3 |  2 |  1 |  0 |  7\n"
        + "Blithering Badgers             |  3 |  0 |  1 |  2 |  1\n"
        + "Devastating Donkeys            |  3 |  0 |  1 |  2 |  1";

    b.iter(|| assert_eq!(tournament::tally(&input), expected))
}
