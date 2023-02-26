use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
enum Face {
    Front,
    Back,
    Right,
    Left,
    Up,
    Down,
}

impl Face {
    pub(crate) fn opposite(&self) -> Self {
        match self {
            Face::Front => Face::Back,
            Face::Back => Face::Front,
            Face::Right => Face::Left,
            Face::Left => Face::Right,
            Face::Up => Face::Down,
            Face::Down => Face::Up,
        }
    }
}

impl Distribution<Face> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Face {
        match rng.gen_range(0..=5) {
            0 => Face::Front,
            1 => Face::Back,
            2 => Face::Right,
            3 => Face::Left,
            4 => Face::Up,
            _ => Face::Down,
        }
    }
}

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let face = match self {
            Face::Front => "F",
            Face::Back => "B",
            Face::Right => "R",
            Face::Left => "L",
            Face::Up => "U",
            Face::Down => "D",
        };

        write!(f, "{face}")
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Modifier {
    Empty,
    Prime,
    Two,
}

impl Distribution<Modifier> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Modifier {
        match rng.gen_range(0..=2) {
            0 => Modifier::Empty,
            1 => Modifier::Prime,
            _ => Modifier::Two,
        }
    }
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let modifier = match self {
            Modifier::Empty => "",
            Modifier::Two => "2",
            Modifier::Prime => "\'",
        };

        write!(f, "{modifier}")
    }
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Rotation {
    face: Face,
    modifier: Modifier,
}

impl Rotation {
    pub(crate) fn is_valid(
        &self,
        last: Option<&Rotation>,
        second_to_last: Option<&Rotation>,
    ) -> bool {
        if last.is_none() && second_to_last.is_none() {
            true
        } else if second_to_last.is_none() {
            self.face != last.unwrap().face
        } else {
            self.face != second_to_last.unwrap().face
                && self.face != second_to_last.unwrap().face.opposite()
                && self.face != last.unwrap().face
        }
    }
}

impl Distribution<Rotation> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rotation {
        let face: Face = rng.gen();
        let modifier: Modifier = rng.gen();

        Rotation { face, modifier }
    }
}

impl fmt::Display for Rotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.face, self.modifier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opposite_faces() {
        let face = Face::Up;
        assert_eq!(face.opposite(), Face::Down);

        let face = Face::Down;
        assert_eq!(face.opposite(), Face::Up);

        let face = Face::Right;
        assert_eq!(face.opposite(), Face::Left);

        let face = Face::Left;
        assert_eq!(face.opposite(), Face::Right);

        let face = Face::Front;
        assert_eq!(face.opposite(), Face::Back);

        let face = Face::Back;
        assert_eq!(face.opposite(), Face::Front);
    }

    #[test]
    fn every_rotation_is_valid_if_empty() {
        for _ in 0..100 {
            let r: Rotation = rand::random();
            assert!(r.is_valid(None, None));
        }
    }

    #[test]
    fn valid_rotations_different_faces() {
        let r1 = Rotation {
            face: Face::Up,
            modifier: rand::random(),
        };
        let r2 = Rotation {
            face: Face::Left,
            modifier: rand::random(),
        };
        assert!(r1.is_valid(Some(&r2), None));

        let r1 = Rotation {
            face: Face::Down,
            modifier: rand::random(),
        };
        let r2 = Rotation {
            face: Face::Left,
            modifier: rand::random(),
        };
        assert!(r1.is_valid(Some(&r2), None));

        let r1 = Rotation {
            face: Face::Right,
            modifier: rand::random(),
        };
        let r2 = Rotation {
            face: Face::Left,
            modifier: rand::random(),
        };
        assert!(r1.is_valid(Some(&r2), None));

        let r1 = Rotation {
            face: Face::Back,
            modifier: rand::random(),
        };
        let r2 = Rotation {
            face: Face::Left,
            modifier: rand::random(),
        };
        assert!(r1.is_valid(Some(&r2), None));

        let r1 = Rotation {
            face: Face::Front,
            modifier: rand::random(),
        };
        let r2 = Rotation {
            face: Face::Left,
            modifier: rand::random(),
        };
        assert!(r1.is_valid(Some(&r2), None));
    }

    #[test]
    fn invalid_self_solving_rotations() {
        let r1 = Rotation {
            face: Face::Up,
            modifier: Modifier::Empty,
        };
        let r2 = Rotation {
            face: Face::Up,
            modifier: Modifier::Prime,
        };
        assert!(!r1.is_valid(Some(&r2), None));

        let r1 = Rotation {
            face: Face::Right,
            modifier: Modifier::Prime,
        };
        let r2 = Rotation {
            face: Face::Right,
            modifier: Modifier::Empty,
        };
        assert!(!r1.is_valid(Some(&r2), None));

        let r1 = Rotation {
            face: Face::Back,
            modifier: Modifier::Two,
        };
        let r2 = Rotation {
            face: Face::Back,
            modifier: Modifier::Two,
        };
        assert!(!r1.is_valid(Some(&r2), None));
    }

    #[test]
    fn invalid_same_plane_rotation() {
        let r1 = Rotation {
            face: Face::Up,
            modifier: Modifier::Empty,
        };
        let r2 = Rotation {
            face: Face::Right,
            modifier: Modifier::Empty,
        };
        let r3 = Rotation {
            face: Face::Up,
            modifier: Modifier::Prime,
        };
        assert!(!r1.is_valid(Some(&r2), Some(&r3)));

        let r1 = Rotation {
            face: Face::Up,
            modifier: Modifier::Empty,
        };
        let r2 = Rotation {
            face: Face::Right,
            modifier: Modifier::Empty,
        };
        let r3 = Rotation {
            face: Face::Down,
            modifier: Modifier::Empty,
        };
        assert!(!r1.is_valid(Some(&r2), Some(&r3)));
    }
}
