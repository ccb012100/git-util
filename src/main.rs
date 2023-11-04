use anyhow::{Result, Ok};
use clap::Parser;
use cli::{Cli, Commands};
use std::process::ExitCode;

mod cli;
mod output;

fn main() -> Result<ExitCode> {
    let cli = Cli::parse();

    #[allow(unused_variables)]
    match &cli.command {
        Commands::Alias { args } => todo!(),
        Commands::Author { args } => todo!(),
        Commands::Hook { args } => todo!(),
        Commands::Files { args } => todo!(),
        Commands::L { args } => todo!(),
        Commands::Last { args } => todo!(),
        Commands::Show { args } => todo!(),
        Commands::Restore { args } => todo!(),
        Commands::Undo { args } => todo!(),
        Commands::Unstage { args } => todo!(),
        Commands::Update { args } => todo!(),
    }

    #[allow(unreachable_code)]
    Ok(ExitCode::SUCCESS)
}
