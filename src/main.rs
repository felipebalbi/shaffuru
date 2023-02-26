use clap::Parser;
use shaffuru::{run, Cli};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let perm = run(cli)?;

    println!("{perm}");

    Ok(())
}
