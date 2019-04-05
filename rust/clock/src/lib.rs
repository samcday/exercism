#![deny(clippy::all, clippy::pedantic)]

use std::fmt::{Display, Formatter, Result};

const MINUTES_IN_DAY: i32 = 60 * 24;

#[derive(Debug, PartialEq)]
pub struct Clock(i32);

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:02}:{:02}", self.0 / 60, self.0 % 60)
    }
}

impl Clock {
    fn from_minutes(mut minutes: i32) -> Self {
        minutes %= MINUTES_IN_DAY;
        while minutes < 0 {
            minutes = MINUTES_IN_DAY - minutes.abs();
        }
        Self(minutes)
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::from_minutes(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::from_minutes(self.0 + minutes)
    }
}
