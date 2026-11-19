use anyhow::Result;
use clap::{Parser, Subcommand};

mod fetch;
mod peek;

#[derive(Parser)]
#[command(about = "NextTerm")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Hello,
    Fetch,
    Peek,
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Hello => println!("hello from xtask"),
        Command::Fetch => fetch::run()?,
        Command::Peek => peek::run()?,
    }
    Ok(())
}
