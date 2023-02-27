use crate::rotation::Rotation;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::fmt;

#[derive(Default)]
pub struct Permutation {
    seed: u64,
    length: u8,
    rotations: Vec<Rotation>,
}

impl Permutation {
    pub(crate) fn generate(seed: u64, length: u8) -> Self {
        let mut rotations: Vec<Rotation> = Vec::default();
        let mut rng = StdRng::seed_from_u64(seed);

        while rotations.len() < length.into() {
            let current: Rotation = rng.gen();
            let last = rotations.get(rotations.len().wrapping_sub(1));
            let second_to_last = rotations.get(rotations.len().wrapping_sub(2));

            if current.is_valid(last, second_to_last) {
                rotations.push(current);
            }
        }

        Self {
            seed,
            length,
            rotations,
        }
    }
}

impl fmt::Display for Permutation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Seed: {}", self.seed)?;
        writeln!(f, "Length: {}", self.length)?;

        for m in &self.rotations {
            write!(f, "{m} ")?;
        }

        Ok(())
    }
}
