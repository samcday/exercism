#![deny(clippy::all, clippy::pedantic)]

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}
use Direction::{East, North, South, West};

const DIRECTIONS: [Direction; 4] = [North, East, South, West];

pub struct Robot {
    x: i32,
    y: i32,
    facing: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, facing: Direction) -> Self {
        Self { x, y, facing }
    }

    pub fn turn_right(mut self) -> Self {
        self.facing = DIRECTIONS[(self.facing as usize + 1) % 4];
        self
    }

    pub fn turn_left(mut self) -> Self {
        self.facing = DIRECTIONS[(self.facing as usize).checked_sub(1).unwrap_or(3) % 4];
        self
    }

    pub fn advance(mut self) -> Self {
        match self.facing {
            North => self.y += 1,
            East => self.x += 1,
            South => self.y -= 1,
            West => self.x -= 1,
        }
        self
    }

    pub fn instructions(mut self, instructions: &str) -> Self {
        for c in instructions.chars() {
            self = match c {
                'A' => self.advance(),
                'L' => self.turn_left(),
                'R' => self.turn_right(),
                _ => self,
            };
        }
        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.facing
    }
}
