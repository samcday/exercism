#![deny(clippy::all, clippy::pedantic)]

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Clone, Copy, Debug)]
enum Frame {
    Strike,
    Spare(u16, u16),
    Open(u16, u16),
    Last(u16, u16, u16),
}

impl Frame {
    fn score<I: Iterator<Item = Self>>(self, following: I) -> u16 {
        let following = following.flat_map(|frame| match frame {
            Frame::Strike => vec![10],
            Frame::Spare(first, second) | Frame::Open(first, second) | Frame::Last(first, second, _) => {
                vec![first, second]
            }
        });

        match self {
            Frame::Strike => 10 + following.take(2).sum::<u16>(),
            Frame::Spare(_, _) => 10 + following.take(1).sum::<u16>(),
            Frame::Open(first, second) => first + second,
            Frame::Last(first, second, third) => first + second + third,
        }
    }
}

#[derive(Debug)]
pub struct BowlingGame {
    frames: Vec<Frame>,
    rolls: Vec<u16>,
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: vec![],
            rolls: vec![],
        }
    }

    fn record_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
        self.rolls.clear();
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.frames.len() == 10 {
            return Err(Error::GameComplete);
        }
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.rolls.push(pins);

        let frame_total = self.rolls.iter().sum::<u16>();
        let frame_rolls = self.rolls.len();

        if self.frames.len() == 9 {
            let first_two = self.rolls.iter().take(2).sum::<u16>();
            let last_two = self.rolls.iter().skip(1).take(2).sum::<u16>();

            if (first_two != 10 && first_two != 20) && last_two > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }

            if frame_rolls == 3 {
                self.record_frame(Frame::Last(self.rolls[0], self.rolls[1], self.rolls[2]))
            } else if frame_rolls == 2 && frame_total < 10 {
                self.record_frame(Frame::Last(self.rolls[0], self.rolls[1], 0))
            }
        } else {
            if frame_total > 10 && self.frames.len() < 9 {
                return Err(Error::NotEnoughPinsLeft);
            }

            if frame_total == 10 && frame_rolls == 1 {
                self.record_frame(Frame::Strike);
            } else if frame_rolls == 2 {
                self.record_frame(if frame_total == 10 {
                    Frame::Spare(self.rolls[0], self.rolls[1])
                } else {
                    Frame::Open(self.rolls[0], self.rolls[1])
                });
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames.len() == 10 {
            let mut score = 0;
            for (idx, frame) in self.frames.iter().enumerate() {
                score += frame.score(self.frames[idx..].iter().skip(1).cloned());
            }
            Some(score)
        } else {
            None
        }
    }
}
