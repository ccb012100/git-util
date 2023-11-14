use anyhow::Result;
use clap::Parser;
use cli::{Cli, HookSubcommands, LogLevel, Subcommands};
use git::{Git, GitCommandResult};
use log::{debug, LevelFilter};
use print::Print;

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
        Subcommands::Files { args } => Git::show_files(args),
        Subcommands::Ll { args } => Git::ll(args),
        Subcommands::Last { args } => todo!(),
        Subcommands::Show { args } => todo!(),
        Subcommands::Restore { args } => Git::restore(args),
        Subcommands::Undo { args } => todo!(),
        Subcommands::Unstage { args } => Git::unstage(args),
        Subcommands::Update { args } => Git::update(args),
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
