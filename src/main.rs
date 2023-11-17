use anyhow::Result;
use clap::Parser;
use cli::{Cli, HookSubcommands, LogLevel, Subcommands};
use git::commands::{GitCommands, GitCommandResult, PRINT_COMMAND as GIT_PRINT_COMMAND};
use log::{debug, LevelFilter};
use print::Print;
use std::sync::atomic::Ordering;

mod cli;
mod git;
mod print;

fn main() -> ! {
    let cli = Cli::parse();

    let log_level = match cli.verbose {
        LogLevel::Debug => LevelFilter::Debug,
        LogLevel::Error => LevelFilter::Error,
        LogLevel::Info => LevelFilter::Info,
        LogLevel::Warn => LevelFilter::Warn,
        LogLevel::Off => LevelFilter::Off,
        LogLevel::Trace => LevelFilter::Trace,
    };

    env_logger::Builder::new().filter_level(log_level).init();

    debug!("logging initialized");

    #[cfg(windows)]
    {
        log::info!("On Windows; enabling ansi support...");
        nu_ansi_term::enable_ansi_support();
    }

    debug!("parsed Cli: {:#?}", &cli);

    GIT_PRINT_COMMAND.store(cli.print_command, Ordering::Relaxed);

    #[allow(unused_variables)]
    let result = match &cli.subcommand {
        Subcommands::A { args } => GitCommands::add(args),
        Subcommands::Aac { args } => GitCommands::aac(args),
        Subcommands::Alias { args } => match args.is_empty() {
            true => GitCommands::alias(None),
            false => GitCommands::alias(Some(args.join(" ").as_str())),
        },
        Subcommands::Auc { args } => GitCommands::auc(args),
        Subcommands::Author { num } => GitCommands::author(*num),
        Subcommands::Hook { hook } => run_hook(hook),
        Subcommands::Files { args } => GitCommands::show_files(args),
        Subcommands::L { args } => GitCommands::log_oneline(args),
        Subcommands::Last { args } => GitCommands::last(args),
        Subcommands::Show { args } => GitCommands::show(args),
        Subcommands::Restore { files } => {
            if let Some(which) = files.which {
                match which {
                    cli::WhichFiles::All => GitCommands::restore_all(),
                }
            } else {
                GitCommands::restore(&files.args)
            }
        }
        Subcommands::Undo { num } => GitCommands::undo(*num),
        Subcommands::Unstage { files } => {
            if let Some(which) = files.which {
                match which {
                    cli::WhichFiles::All => GitCommands::unstage_all(),
                }
            } else {
                GitCommands::unstage(&files.args)
            }
        }
        Subcommands::Update { args } => GitCommands::update(args),
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
