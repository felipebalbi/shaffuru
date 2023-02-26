use crate::rotation::Rotation;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::fmt;

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

            match second_to_last {
                None => match last {
                    // Very first rotation. Just push it onto the rotations list
                    None => rotations.push(current.clone()),
                    // The second rotation. It must be different from the
                    // previous face
                    Some(value) => {
                        if current.face != value.face {
                            rotations.push(current.clone());
                        }
                    }
                },
                // Rotations list has at least 2 rotations. Current must be
                // different from previous and choose a face from a
                // different plane than the second to last face.
                Some(value) => {
                    if current.face != value.face
                        && current.face != value.face.opposite()
                        && current.face != last.unwrap().face
                    {
                        rotations.push(current.clone());
                    }
                }
            }
        }

        Self {
            seed,
            length,
            rotations,
        }
    }
}

impl Default for Permutation {
    fn default() -> Self {
        Self {
            seed: Default::default(),
            length: Default::default(),
            rotations: Default::default(),
        }
    }
}

impl fmt::Display for Permutation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Seed: {}", self.seed)?;
        writeln!(f, "Length: {}", self.length)?;

        for m in self.rotations.iter() {
            write!(f, "{m} ")?;
        }

        Ok(())
    }
}
