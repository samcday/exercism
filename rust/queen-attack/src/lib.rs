#![deny(clippy::all, clippy::pedantic)]

#[derive(Clone, Copy, Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        // Slightly cheating, Range::contains is currently unstable, but will be stable in 1.35.
        if rank >= 0 && rank < 8 && file >= 0 && file < 8 {
            Some(Self { rank, file })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Self) -> bool {
        self.0.rank == other.0.rank
            || self.0.file == other.0.file
            || (self.0.file + self.0.rank) == (other.0.file + other.0.rank)
            || (self.0.file - self.0.rank) == (other.0.file - other.0.rank)
    }
}
