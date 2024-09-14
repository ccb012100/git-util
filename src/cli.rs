use self::subcommands::Subcommands;
use crate::git::{Git, GitResult, DRY_RUN, PRINT_COMMANDS};
use clap::{
    arg,
    builder::{styling::AnsiColor, Styles},
    command,
    error::ErrorKind,
    Args, CommandFactory, Parser,
};
use log::{info, LevelFilter};
use std::sync::atomic::Ordering;

mod subcommands;

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Green.on_default())
    .literal(AnsiColor::Green.on_default())
    .placeholder(AnsiColor::Green.on_default());

#[derive(Parser, Debug)]
#[command(styles=STYLES)]
#[command(about, version, arg_required_else_help = true)]
pub struct Cli {
    #[clap(flatten)]
    pub options: CliOptions,

    /// A catch-all for passing straight through to the native `git` binary; required if [COMMAND] is not specified.
    #[arg(allow_hyphen_values = true)]
    pub fallback: Option<Vec<String>>,

    /// Required if [FALLBACK] is not specified
    #[command(subcommand)]
    pub subcommand: Option<Subcommands>,
}

#[derive(Args, Debug, Clone, Copy)]
pub struct CliOptions {
    /// Set verbosity; adding multiple times increases the verbosity level (>=4, i.e. `-vvvv`, sets maximum verbosity).
    #[arg(
        long,
        short = 'v',
        action = clap::ArgAction::Count,
    )]
    pub verbose: u8,

    /// Set logging level - if set, overrides `verbose`
    #[arg(
        long,
        visible_alias("log"),
        visible_alias("level"),
        value_name = "LEVEL",
        global = true
    )]
    pub log_level: Option<LevelFilter>,

    /// Print the `std::process::Command`s that are executed
    #[arg(long, short = 'p')]
    pub print_command: bool,

    /// Print the `std::process::Command`s that will be executed, but do not run
    #[arg(long, short = 'd')]
    pub dry_run: bool,
}

#[derive(Args, Debug, Clone, Copy)]
pub struct GitConfigOpts {
    /// Show the value's scope.
    #[arg(long, short = 's', action = clap::ArgAction::Set, default_value_t = false)]
    pub show_scope: bool,

    /// Show the value's origin.
    #[arg(long, short = 'o', action = clap::ArgAction::Set, default_value_t = false)]
    pub show_origin: bool,
}

impl Cli {
    pub fn run_subcommand(&self) -> GitResult {
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

    pub fn initialize_logger(&self) {
        let level = match self.options.log_level {
            Some(logging_level) => logging_level,
            None => match self.options.verbose {
                0 => LevelFilter::Error,
                1 => LevelFilter::Warn,
                2 => LevelFilter::Info,
                3 => LevelFilter::Debug,
                4..=std::u8::MAX => LevelFilter::Trace,
            },
        };

        env_logger::Builder::new().filter_level(level).init();

        info!("ℹ️ logging initialized at level {}", level);
    }
}
