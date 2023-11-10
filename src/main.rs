use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use git::Git;
use log::{debug, LevelFilter};
use std::process::ExitCode;

mod cli;
mod git;
mod print;

fn main() -> Result<ExitCode> {
    let cli = Cli::parse();

    let log_level = if cli.vv {
        LevelFilter::Debug
    } else if cli.verbose {
        LevelFilter::Info
    } else {
        LevelFilter::Warn
    };

    env_logger::Builder::new().filter_level(log_level).init();

    debug!("logging initialized");

    #[cfg(windows)]
    {
        log::info!("On Windows; enabling ansi support...");
        nu_ansi_term::enable_ansi_support();
    }

    debug!("parsed Cli: {:#?}", &cli);

    #[allow(unused_variables)]
    let result = match &cli.command {
        Commands::Alias { args } => match args.is_empty() {
            true => Git::alias(None),
            false => Git::alias(Some(args.join(" ").as_str())),
        },
        Commands::Author { args } => todo!(),
        Commands::Hook { args } => todo!(),
        Commands::Files { args } => todo!(),
        Commands::Ll { args } => Git::ll(args),
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
