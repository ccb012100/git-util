use anyhow::Result;
use clap::Parser;
use cli::{Cli, HookSubcommands, Subcommands};
use git::Git;
use log::{debug, LevelFilter};
use std::process::ExitCode;

mod cli;
mod git;
mod print;

fn main() -> Result<ExitCode> {
    let cli = Cli::parse();

    let log_level = if cli.verbose {
        LevelFilter::Debug
    } else if cli.v {
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
    let result = match &cli.subcommand {
        Subcommands::Aac { args } => todo!(),
        Subcommands::Alias { args } => match args.is_empty() {
            true => Git::alias(None),
            false => Git::alias(Some(args.join(" ").as_str())),
        },
        Subcommands::Auc { args } => todo!(),
        Subcommands::Author { args } => todo!(),
        Subcommands::Hook { hook } => run_hook(hook),
        Subcommands::Files { args } => todo!(),
        Subcommands::Ll { args } => Git::ll(args),
        Subcommands::Last { args } => todo!(),
        Subcommands::Show { args } => todo!(),
        Subcommands::Restore { args } => todo!(),
        Subcommands::Undo { args } => todo!(),
        Subcommands::Unstage { args } => todo!(),
        Subcommands::Update { args } => todo!(),
    };

    #[allow(unreachable_code)]
    match result {
        Ok(_) => Ok(ExitCode::SUCCESS),
        Err(e) => Err(e),
    }
}

fn run_hook(hook: &HookSubcommands) -> Result<()> {
    match hook {
        HookSubcommands::Precommit {} => todo!(),
    }
}
