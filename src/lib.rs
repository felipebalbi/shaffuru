mod permutation;
mod rotation;

use clap::Parser;
use permutation::Permutation;
use std::error::Error;

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
        help = "Seed for the random number generator."
    )]
    seed: Option<u64>,

    #[arg(
        short = 'l',
        long = "length",
        value_name = "LENGTH",
	value_parser = parse_length,
	default_value = "25",
        help = "Length of generated permutation. Maximum length is 255 moves.",
    )]
    length: u8,
}

impl Cli {
    pub fn run(&self) -> Result<Permutation> {
        Ok(Permutation::generate(
            self.seed.unwrap_or(rand::random()),
            self.length,
        ))
    }
}

fn parse_length(s: &str) -> std::result::Result<u8, String> {
    let length: u8 = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a valid length"))?;
    Ok(length)
}
