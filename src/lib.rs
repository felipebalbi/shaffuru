use clap::Parser;
use rand::{
    distributions::{Distribution, Standard},
    rngs::StdRng,
    Rng, SeedableRng,
};
use std::{error::Error, fmt};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(
    name = "Shaffuru",
    author = "Felipe Balbi <felipe@balbi.sh>",
    about = "Generate random Rubik's cube permutations",
    version = "0.1.0"
)]
pub struct Cli {
    #[arg(
        short = 's',
        long = "seed",
        value_name = "SEED",
        help = "Seed for the random number generator"
    )]
    seed: Option<u64>,

    #[arg(
        short = 'l',
        long = "length",
        value_name = "LENGTH",
	value_parser = parse_length,
	default_value = "25",
        help = "Length of generated permutation"
    )]
    length: u8,
}

pub struct Permutation {
    seed: u64,
    length: u8,
    moves: Vec<Move>,
}

impl Permutation {
    fn new() -> Self {
        Default::default()
    }
}

impl Default for Permutation {
    fn default() -> Self {
        Self {
            seed: Default::default(),
            length: Default::default(),
            moves: Default::default(),
        }
    }
}

impl fmt::Display for Permutation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Seed: {}", self.seed)?;

        for m in self.moves.iter() {
            write!(f, "{m} ")?;
        }

        Ok(())
    }
}

pub fn run(cli: Cli) -> Result<Permutation> {
    let mut perm = Permutation::new();

    perm.length = cli.length;
    perm.seed = cli.seed.unwrap_or(rand::random());
    let mut rng = StdRng::seed_from_u64(perm.seed);

    while perm.moves.len() < perm.length.into() {
        let current: Move = rng.gen();
        let last = perm.moves.last();

        match last {
            None => perm.moves.push(current),
            Some(value) => {
                // If the current face is opposite to previous face,
                // check the face before that and ensure the current
                // is not on the same plane.
                if current.face == value.face.opposite() {
                    let second_to_last = perm.moves.get(perm.moves.len().wrapping_sub(1));

                    match second_to_last {
                        Some(value) => {
                            if current.face != value.face && current.face != value.face.opposite() {
                                perm.moves.push(current);
                            }
                        }
                        None => perm.moves.push(current),
                    }
                } else if current.face != value.face {
                    // Ensure current face is not the same as the
                    // previous face
                    perm.moves.push(current);
                } else {
                }
            }
        }
    }

    Ok(perm)
}

fn parse_length(s: &str) -> std::result::Result<u8, String> {
    let length: u8 = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a valid length"))?;
    Ok(length)
}

#[derive(PartialEq)]
enum Face {
    Front,
    Back,
    Right,
    Left,
    Up,
    Down,
}

impl Face {
    fn opposite(&self) -> Self {
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

#[derive(PartialEq)]
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

#[derive(PartialEq)]
struct Move {
    face: Face,
    modifier: Modifier,
}

impl Distribution<Move> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Move {
        let face: Face = rng.gen();
        let modifier: Modifier = rng.gen();

        Move { face, modifier }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.face, self.modifier)
    }
}
