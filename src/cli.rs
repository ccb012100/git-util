use self::subcommands::Subcommands;
use crate::git::{Git, GitResult, PRINT_COMMANDS, DRY_RUN};
use clap::{arg, command, error::ErrorKind, Args, CommandFactory, Parser};
use log::{info, LevelFilter};
use std::sync::atomic::Ordering;

mod subcommands;

#[derive(Parser, Debug)]
#[command(about, version, arg_required_else_help = true)]
pub(crate) struct Cli {
    #[clap(flatten)]
    pub(crate) options: CliOptions,

    /// A catch-all for passing straight through to the native `git` binary; required if [COMMAND] is not specified.
    #[arg(allow_hyphen_values = true)]
    pub(crate) fallback: Option<Vec<String>>,

    /// Required if [FALLBACK] is not specified
    #[command(subcommand)]
    pub(crate) subcommand: Option<Subcommands>,
}

#[derive(Args, Debug, Clone, Copy)]
pub(crate) struct CliOptions {
    /// Set verbosity; adding multiple times increases the verbosity level (>=4, i.e. `-vvvv`, sets maximum verbosity).
    #[arg(
        long,
        short = 'v',
        action = clap::ArgAction::Count,
    )]
    pub(crate) verbose: u8,

    /// Print the `std::process::Command`s that are executed
    #[arg(long, short = 'p')]
    pub(crate) print_command: bool,

    /// Print the `std::process::Command`s that will be executed, but do not run
    #[arg(long, short = 'd')]
    pub(crate) dry_run: bool,
}

#[derive(Args, Debug, Clone, Copy)]
pub(crate) struct GitConfigOpts {
    /// Show the value's scope.
    #[arg(long, short = 's', action = clap::ArgAction::Set, default_value_t = false)]
    pub(crate) show_scope: bool,

    /// Show the value's origin.
    #[arg(long, short = 'o', action = clap::ArgAction::Set, default_value_t = false)]
    pub(crate) show_origin: bool,
}

impl Cli {
    pub(crate) fn run_subcommand(&self) -> GitResult {
        // global flags
        PRINT_COMMANDS.store(self.options.print_command, Ordering::Relaxed);
        DRY_RUN.store(self.options.dry_run, Ordering::Relaxed);

        if let Some(args) = &self.fallback {
            Git::pass_through(args)
        } else if let Some(subcommand) = &self.subcommand {
            subcommand.run()
        } else {
            Cli::command()
                .error(
                    ErrorKind::MissingRequiredArgument,
                    "Either FALLBACK or COMMAND must be provided!",
                )
                .exit()
        }
    }

    pub(crate) fn initialize_logger(&self) {
        let log_level = match self.options.verbose {
            0 => LevelFilter::Error,
            1 => LevelFilter::Warn,
            2 => LevelFilter::Info,
            3 => LevelFilter::Debug,
            4..=std::u8::MAX => LevelFilter::Trace,
        };

        env_logger::Builder::new().filter_level(log_level).init();

        info!("logging initialized at level {}", log_level);
    }
}
