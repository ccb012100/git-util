use clap::{arg, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(about, version, arg_required_else_help = true)]
pub(crate) struct Cli {
    /// Set logging level; if only the flag is supplied, it will set LogLevel::Debug
    #[arg(long, global = true, value_name = "LogLevel")]
    #[arg(default_value_t = LogLevel::Warn)]
    #[arg(default_missing_value = "debug", num_args = 0..=1, require_equals = true)]
    #[clap(value_enum)]
    pub(crate) verbose: LogLevel,

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
        /// Command arguments
        args: Vec<String>,
    },
    /// Reset last commit or last n commits and keeps undone changes in working directory
    Undo {
        /// Command arguments
        args: Vec<String>,
    },
    /// Move staged files back to staging area; alias for `git-restore --staged`
    #[clap(alias = "u")]
    Unstage {
        /// Command arguments
        args: Vec<String>,
    },
    /// Update local branch from origin without checking it out
    #[clap(alias = "unwind")]
    Update {
        /// Command arguments
        args: Vec<String>,
    },
}

#[derive(Subcommand, Debug)]
pub(crate) enum HookSubcommands {
    /// Precommit hook
    Precommit {},
}

#[derive(ValueEnum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
