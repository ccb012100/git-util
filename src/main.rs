use anyhow::Result;
use clap::{error::ErrorKind, CommandFactory, Parser};
use cli::{Cli, HookSubcommands, Subcommands};
use git::{
    commands::{immutable::ImmutableCommands, mutable::MutableCommands},
    hooks::pre_commit::PreCommitHook,
    GitConfigOpts, GitResult,
};
use git::{Git, GitCommandResult, PRINT_COMMAND as GIT_PRINT_COMMAND};
use log::{debug, info, LevelFilter};
use print::Print;
use std::sync::atomic::Ordering;

mod cli;
mod git;
mod print;
mod commands;

fn main() -> ! {
    let cli = Cli::parse();

    initialize_logger(&cli.options.verbose);

    #[cfg(windows)]
    {
        log::info!("On Windows; enabling ansi support...");
        nu_ansi_term::enable_ansi_support().unwrap();
    }

    debug!("parsed Cli: {:#?}", &cli);

    GIT_PRINT_COMMAND.store(cli.options.print_command, Ordering::Relaxed);

    let result = if let Some(args) = &cli.fallback {
        Git::pass_through(args)
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

fn run_hook(hook: &HookSubcommands) -> GitResult {
    match hook {
        HookSubcommands::PreCommit {} => PreCommitHook::run(),
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
        Subcommands::A { args } => MutableCommands::add(args),
        Subcommands::Aac { args } => MutableCommands::add_all_and_commit(args),
        Subcommands::Alias { filter, options } => ImmutableCommands::list_aliases(
            filter.as_deref(),
            GitConfigOpts {
                show_origin: options.show_origin,
                show_scope: options.show_scope,
            },
        ),
        Subcommands::Auc { args } => MutableCommands::add_updated_files_and_commit(args),
        Subcommands::Author { num } => MutableCommands::update_commit_author(*num),
        Subcommands::Conf { filter, options } => ImmutableCommands::list_configuration_settings(
            filter.as_deref(),
            GitConfigOpts {
                show_origin: options.show_origin,
                show_scope: options.show_scope,
            },
        ),
        Subcommands::Hook { hook } => run_hook(hook),
        Subcommands::Files { num } => ImmutableCommands::show_files(*num),
        Subcommands::L { num, args } => ImmutableCommands::one_line_log(*num, args),
        Subcommands::Last { num, args } => ImmutableCommands::compact_summary_log(*num, args),
        Subcommands::Show { num, args } => ImmutableCommands::show(*num, args),
        Subcommands::Restore { which, args } => {
            if let Some(all) = which {
                match all {
                    cli::WhichFiles::All => MutableCommands::restore_all(),
                }
            } else {
                MutableCommands::restore(args)
            }
        }
        Subcommands::Undo { num } => MutableCommands::undo_commits(*num),
        Subcommands::Unstage { which, args } => {
            if let Some(which) = which {
                match which {
                    cli::WhichFiles::All => MutableCommands::unstage_all(),
                }
            } else {
                MutableCommands::unstage(args)
            }
        }
        Subcommands::Update { args } => MutableCommands::update_branch_from_remote(args),
    }
}
