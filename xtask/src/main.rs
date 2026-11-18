use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about = "NextTerm")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Hello
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Hello => println!("hello from xtask"),
    }
    Ok(())
}