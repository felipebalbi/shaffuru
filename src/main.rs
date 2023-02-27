use clap::Parser;
use shaffuru::Cli;

fn main() {
    let perm = Cli::parse().run();
    println!("{perm}");
}
