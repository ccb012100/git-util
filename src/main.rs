use anyhow::Result;
use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};
use cli::{Cli, HookSubcommands, Subcommands};
use git::{commands::GitCommands, GitConfigOpts};
use log::{debug, info, LevelFilter};
use print::Print;
use std::sync::atomic::Ordering;

use crate::git::{GitCommandResult, PRINT_COMMAND as GIT_PRINT_COMMAND};

mod cli;
mod git;
mod print;

fn main() -> ! {
    let cli = Cli::parse();

    initialize_logger(&cli.options.verbose);

    #[cfg(windows)]
    {
        log::info!("On Windows; enabling ansi support...");
        nu_ansi_term::enable_ansi_support();
    }

    debug!("parsed Cli: {:#?}", &cli);

    GIT_PRINT_COMMAND.store(cli.options.print_command, Ordering::Relaxed);

    let result = if let Some(args) = &cli.fallback {
        GitCommands::pass_through(args)
    } else if let Some(subcommand) = &cli.subcommand {
        parse_subcommand(subcommand)
    } else {
        let mut cmd = Cli::command();
        cmd.error(
            ErrorKind::MissingRequiredArgument,
            "Either FALLBACK or COMMAND must be provided!",
        )
        .exit()
    };

    match result {
        Ok(git_command) => match git_command {
            GitCommandResult::Success => std::process::exit(0),
            GitCommandResult::Error => std::process::exit(1),
        },
        Err(e) => {
            Print::error(&format!("{}", e));
            std::process::exit(1)
        }
    }
}

fn run_hook(hook: &HookSubcommands) -> Result<GitCommandResult> {
    match hook {
        HookSubcommands::Precommit {} => todo!(),
    }
}

fn initialize_logger(verbosity: &u8) {
    let log_level = match &verbosity {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        4..=std::u8::MAX => LevelFilter::Trace,
    };

    env_logger::Builder::new().filter_level(log_level).init();

    info!("logging initialized at level {}", log_level);
}

fn parse_subcommand(subcommand: &Subcommands) -> Result<GitCommandResult, anyhow::Error> {
    match subcommand {
        Subcommands::A { args } => GitCommands::add(args),
        Subcommands::Aac { args } => GitCommands::aac(args),
        Subcommands::Alias { filter, options } => GitCommands::alias(
            filter.as_deref(),
            GitConfigOpts {
                show_origin: options.show_origin,
                show_scope: options.show_scope,
            },
        ),
        Subcommands::Auc { args } => GitCommands::auc(args),
        Subcommands::Author { num } => GitCommands::author(*num),
        Subcommands::Conf { filter, options } => GitCommands::conf(
            filter.as_deref(),
            GitConfigOpts {
                show_origin: options.show_origin,
                show_scope: options.show_scope,
            },
        ),
        Subcommands::Hook { hook } => run_hook(hook),
        Subcommands::Files { num } => GitCommands::show_files(*num),
        Subcommands::L { num, args } => GitCommands::log_oneline(*num, args),
        Subcommands::Last { num, args } => GitCommands::last(*num, args),
        Subcommands::Show { num, args } => GitCommands::show(*num, args),
        Subcommands::Restore { which, args } => {
            if let Some(all) = which {
                match all {
                    cli::WhichFiles::All => GitCommands::restore_all(),
                }
            } else {
                GitCommands::restore(args)
            }
        }
        Subcommands::Undo { num } => GitCommands::undo(*num),
        Subcommands::Unstage { which, args } => {
            if let Some(which) = which {
                match which {
                    cli::WhichFiles::All => GitCommands::unstage_all(),
                }
            } else {
                GitCommands::unstage(args)
            }
        }
        Subcommands::Update { args } => GitCommands::update(args),
    }
}
