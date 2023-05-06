use clap::Parser;
use permutation::Permutation;

mod permutation;
mod rotation;

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
    #[must_use]
    pub fn run(&self) -> Permutation {
        Permutation::generate(self.seed.unwrap_or_else(rand::random), self.length)
    }
}

/// # Errors
///
/// Will return `Err` if `length` is larger than 255.
fn parse_length(s: &str) -> std::result::Result<u8, String> {
    let length: u8 = s
        .parse()
        .map_err(|_| format!("length must be less than 256"))?;
    Ok(length)
}
