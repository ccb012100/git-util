use clap::{arg, command, Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(about, version, arg_required_else_help = true)]
pub(crate) struct Cli {
    #[clap(flatten)]
    pub(crate) options: CliOptions,

    /// Catch-all for passing straight through to the native `git` binary; required if [COMMAND] is not specified
    #[arg(allow_hyphen_values = true)]
    pub(crate) fallback: Option<Vec<String>>,

    /// Required if [FALLBACK] is not specified
    #[command(subcommand)]
    pub(crate) subcommand: Option<Subcommands>,
}

#[derive(Args, Debug, Clone, Copy)]
pub(crate) struct CliOptions {
    /// Set verbosity
    #[arg(
        long,
        short = 'v',
        action = clap::ArgAction::Count,
    )]
    pub(crate) verbose: u8,

    /// Print the Git command that is executed
    #[arg(long, short = 'p')]
    pub(crate) print_command: bool,
}

#[derive(Args, Debug, Clone, Copy)]
pub(crate) struct GitConfigOpts {
    /// show value's scope
    #[arg(long, short = 's', action = clap::ArgAction::Set)]
    pub(crate) show_scope: bool,

    /// show value's origin
    #[arg(long, short = 'o', action = clap::ArgAction::Set)]
    pub(crate) show_origin: bool,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Subcommands {
    /// Wrapper around git-add
    #[command(allow_hyphen_values = true)]
    A {
        /// Command arguments
        args: Vec<String>,
    },
    /// Add updated and untracked files and then commit
    #[command(allow_hyphen_values = true)]
    Aac {
        /// Command arguments
        args: Vec<String>,
    },
    /// List configured aliases
    Alias {
        /// text to filter on
        filter: Option<String>,

        #[clap(flatten)]
        options: GitConfigOpts,
    },
    /// Add updated files and then commit
    #[clap(alias = "ac")]
    #[command(allow_hyphen_values = true)]
    Auc {
        /// Command arguments
        args: Vec<String>,
    },
    /// Reset author for last n commits
    Author {
        /// Number of commits to reset (else defaults to 1)
        num: Option<u8>,
    },
    /// List config settings (excluding aliases)
    Conf {
        /// Text to filter on
        filter: Option<String>,

        #[clap(flatten)]
        options: GitConfigOpts,
    },
    /// Call git hook
    Hook {
        #[command(subcommand)]
        hook: HookSubcommands,
    },
    /// List files changed in last n commits
    #[clap(alias = "shf")]
    Files {
        /// number of commits to list files for (else defaults to 1)
        num: Option<u8>,
    },
    /// git-log, formatted to 1 line per commit
    #[command(allow_hyphen_values = true)]
    L {
        /// number of commits to list (else defaults to 25)
        num: Option<u8>,

        /// Command arguments
        args: Vec<String>,
    },
    /// git-log, compact summary (commit message and list of changed files)
    #[clap(alias = "la")]
    #[command(allow_hyphen_values = true)]
    Last {
        /// number of commits to list (else defaults to 10)
        num: Option<u8>,

        /// Command arguments
        args: Vec<String>,
    },
    /// git-restore
    #[clap(alias = "rest")]
    #[command(allow_hyphen_values = true)]
    Restore {
        /// which files to operate on
        #[command(subcommand)]
        which: Option<WhichFiles>,

        /// Command arguments
        args: Vec<String>,
    },
    /// git-show
    #[command(allow_hyphen_values = true)]
    #[clap(alias = "sh")]
    Show {
        /// number of commits to show (else defaults to 1)
        num: Option<u8>,

        /// Command arguments
        args: Vec<String>,
    },
    /// Reset last commit or last n commits and keeps undone changes in working directory
    Undo {
        /// number of commits to undo (else defaults to 1)
        num: Option<u8>,
    },
    /// Move staged files back to staging area; alias for `git-restore --staged`
    #[clap(alias = "u")]
    #[command(allow_hyphen_values = true)]
    Unstage {
        /// which files to operate on
        #[command(subcommand)]
        which: Option<WhichFiles>,

        /// Command arguments
        args: Vec<String>,
    },
    /// Update local branch from origin without checking it out
    #[clap(alias = "unwind")]
    #[command(allow_hyphen_values = true)]
    Update {
        /// Command arguments
        args: String,
    },
}

#[derive(Subcommand, Debug, Clone, Copy)]
pub(crate) enum HookSubcommands {
    /// Precommit hook
    Precommit {},
}

#[derive(Subcommand, Debug, Clone, Copy)]
pub(crate) enum WhichFiles {
    All,
}
