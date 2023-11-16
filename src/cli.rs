use clap::{arg, command, Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(about, version, arg_required_else_help = true)]
pub(crate) struct Cli {
    /// Set logging level; if only the flag is supplied, it will set LogLevel::Debug
    #[arg(long, global = true, value_name = "LogLevel")]
    #[arg(default_value_t = LogLevel::Warn)]
    #[arg(default_missing_value = "debug", num_args = 0..=1, require_equals = true)]
    #[clap(value_enum)]
    pub(crate) verbose: LogLevel,

    /// Print the Git command executed
    #[arg(long, global = true)]
    pub(crate) print_command: bool,

    #[command(subcommand)]
    pub(crate) subcommand: Subcommands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Subcommands {
    /// Add updated and untracked files and then commit
    Aac {
        /// Command arguments
        args: Vec<String>,
    },
    /// List configured aliases
    Alias {
        /// Command arguments
        args: Vec<String>,
    },
    /// Add updated files and then commit
    #[clap(alias = "ac")]
    Auc {
        /// Command arguments
        args: Vec<String>,
    },
    /// Reset author for last n commits
    Author {
        /// Command arguments
        args: Vec<String>,
    },
    /// Call git hook
    Hook {
        #[command(subcommand)]
        hook: HookSubcommands,
    },
    /// List files changed in last n commits
    #[clap(alias = "shf")]
    Files {
        /// Command arguments
        args: Vec<String>,
    },
    /// git-log, formatted to 1 line per commit
    Ll {
        /// Command arguments
        args: Vec<String>,
    },
    /// git-log compact summary (commit message and list of changed files)
    #[clap(alias = "la")]
    Last {
        /// Command arguments
        args: Vec<String>,
    },
    /// git-show
    #[clap(alias = "sh")]
    Show {
        /// Command arguments
        args: Vec<String>,
    },
    /// git-restore
    #[clap(alias = "rest")]
    Restore {
        #[command(flatten)]
        files: FileOperations,
    },
    /// Reset last commit or last n commits and keeps undone changes in working directory
    Undo {
        /// number of commits to undo
        num: u8,
    },
    /// Move staged files back to staging area; alias for `git-restore --staged`
    #[clap(alias = "u")]
    Unstage {
        #[command(flatten)]
        files: FileOperations,
    },
    /// Update local branch from origin without checking it out
    #[clap(alias = "unwind")]
    Update {
        /// Command arguments
        args: String,
    },
}

#[derive(Debug, Clone, Args)]
#[group(required = true, multiple = false)]
pub(crate) struct FileOperations {
    pub(crate) which: Option<WhichFiles>,

    /// Command arguments
    pub(crate) args: Vec<String>,
}

#[derive(Subcommand, Debug, Clone, Copy)]
pub(crate) enum HookSubcommands {
    /// Precommit hook
    Precommit {},
}

#[derive(Debug, ValueEnum, Clone, Copy)]
pub(crate) enum WhichFiles {
    All,
}

#[derive(ValueEnum, Debug, Copy, Clone)]
pub(crate) enum LogLevel {
    /// Info
    Info,
    /// Debug
    Debug,
    /// Warn
    Warn,
    /// Error
    Error,
    /// Trace
    Trace,
    /// Off
    Off,
}
