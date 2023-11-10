use clap::{arg, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(about, version, arg_required_else_help = true)]
pub(crate) struct Cli {
    /// Enable INFO logging
    #[arg(long)]
    #[arg(default_value_t = false)]
    pub(crate) verbose: bool,

    /// Enable DEBUG logging
    #[arg(long)]
    pub(crate) vv: bool,

    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// List configured aliases
    Alias {
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
        /// Command arguments
        args: Vec<String>,
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
