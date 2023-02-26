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

pub fn run(cli: Cli) -> Result<()> {
    let length = cli.length;
    let seed: u64 = cli.seed.unwrap_or(rand::random());
    let mut perm: Vec<Move> = Vec::default();
    let mut rng = StdRng::seed_from_u64(seed);

    println!("Seed: {seed}");

    while perm.len() < length.into() {
        let current: Move = rng.gen();
        let last = perm.last();

        match last {
            None => perm.push(current),
            Some(value) => {
                // If the current face is opposite to previous face,
                // check the face before that and ensure the current
                // is not on the same plane.
                if current.face == value.face.opposite() {
                    let second_to_last = perm.get(perm.len().wrapping_sub(1));

                    match second_to_last {
                        Some(value) => {
                            if current.face != value.face && current.face != value.face.opposite() {
                                perm.push(current);
                            }
                        }
                        None => perm.push(current),
                    }
                } else if current.face != value.face {
                    // Ensure current face is not the same as the
                    // previous face
                    perm.push(current);
                } else {
                }
            }
        }
    }

    for m in perm {
        print!("{} ", m);
    }

    println!();

    Ok(())
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
