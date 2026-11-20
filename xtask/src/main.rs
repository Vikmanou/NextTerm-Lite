use anyhow::Result;
use clap::{Parser, Subcommand};

mod build_dataset;
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
    Peek { id: u32 },
    BuildDataset,
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Hello => println!("hello from xtask"),
        Command::Fetch => fetch::run()?,
        Command::Peek { id } => peek::run(id)?,
        Command::BuildDataset => build_dataset::run()?,
    }
    Ok(())
}
