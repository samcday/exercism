#![deny(clippy::all, clippy::pedantic)]

#[derive(Debug, PartialEq)]
pub struct DNA(String);

#[derive(Debug, PartialEq)]
pub struct RNA(String);

impl DNA {
    pub fn new(dna: &str) -> Result<Self, usize> {
        dna.chars()
            .position(|c| c != 'A' && c != 'C' && c != 'G' && c != 'T')
            .map_or(Ok(()), Err)?;
        Ok(Self(dna.to_string()))
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_rna(self) -> RNA {
        RNA(self
            .0
            .chars()
            .map(|c| match c {
                'A' => 'U',
                'C' => 'G',
                'G' => 'C',
                'T' => 'A',
                _ => unreachable!(),
            })
            .collect::<String>())
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<Self, usize> {
        rna.chars()
            .position(|c| c != 'A' && c != 'C' && c != 'G' && c != 'U')
            .map_or(Ok(()), Err)?;
        Ok(Self(rna.to_string()))
    }
}
