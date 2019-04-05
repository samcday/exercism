#![deny(clippy::all, clippy::pedantic)]

pub fn is_leap_year(year: u64) -> bool {
    // A leap year is every 4 years, provided the year is not also divisible by 100 and indivisble by 400.
    (year % 4 == 0) && !(year % 100 == 0 && year % 400 > 0)
}
