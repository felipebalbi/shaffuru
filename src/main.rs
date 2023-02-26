use clap::Parser;
use shaffuru::Cli;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let perm = Cli::parse().run()?;

    println!("{perm}");

    Ok(())
}
