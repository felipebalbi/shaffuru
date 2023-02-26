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
            Face::Down => Face::Right,
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
