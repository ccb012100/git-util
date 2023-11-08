use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use git::Git;
use std::process::ExitCode;

mod cli;
mod git;
mod print;

fn main() -> Result<ExitCode> {
    #[cfg(windows)]
    ansi_term::enable_ansi_support();

    let cli = Cli::parse();

    #[allow(unused_variables)]
    let result = match &cli.command {
        Commands::Alias { args } => Git::alias(args),
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
    };

    #[allow(unreachable_code)]
    match result {
        Ok(_) => Ok(ExitCode::SUCCESS),
        Err(e) => Err(e),
    }
}
