use anyhow::Result;
use clap::{Parser, Subcommand};

mod fetch;

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
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Hello => println!("hello from xtask"),
        Command::Fetch => fetch::run()?,
    }
    Ok(())
}
