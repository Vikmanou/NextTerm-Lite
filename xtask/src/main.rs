use anyhow::Result;
use clap::{Parser, Subcommand};

mod build_dataset;
mod fetch;
mod peek;
mod predict;

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
    Peek { id: u32 },
    BuildDataset,
    Predict { terms: Vec<i128> },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Hello => println!("hello from xtask"),
        Command::Fetch => fetch::run()?,
        Command::Peek { id } => peek::run(id)?,
        Command::BuildDataset => build_dataset::run()?,
        Command::Predict { terms } => predict::run(terms)?,
    }
    Ok(())
}
