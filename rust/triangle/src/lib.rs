#![deny(clippy::all, clippy::pedantic)]

pub struct Triangle(usize);

impl Triangle {
    pub fn build<T>(sides: [T; 3]) -> Option<Self>
    where
        T: PartialEq + Clone + Copy + core::ops::Add<Output = T> + PartialOrd<T> + From<u8>,
    {
        [
            (sides[0], (sides[1], sides[2])),
            (sides[1], (sides[0], sides[2])),
            (sides[2], (sides[0], sides[1])),
        ]
        .iter()
        .cloned()
        .try_fold(0, |acc, (l, (r1, r2))| {
            if l == T::from(0_u8) || l > r1 + r2 {
                return None;
            }
            Some(acc + if l == r1 || l == r2 { 1 } else { 0 })
        })
        .map(Self)
    }

    pub fn is_equilateral(&self) -> bool {
        self.0 == 3
    }

    pub fn is_scalene(&self) -> bool {
        self.0 == 0
    }

    pub fn is_isosceles(&self) -> bool {
        self.0 >= 2
    }
}
