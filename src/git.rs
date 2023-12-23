use anyhow::{Context, Result};
use log::{debug, trace};
use std::{
    io::{stdout, IsTerminal},
    process::Command,
    sync::atomic::AtomicBool,
};

use crate::commands::Commands;

pub(crate) mod commands;
pub(crate) mod env_vars;
pub(crate) mod hooks;

pub(crate) type GitResult = Result<GitCommandResult>;
pub(crate) struct Git();

/// Flag to indicate whether or not to print the commands executed
pub(crate) static PRINT_COMMANDS: AtomicBool = AtomicBool::new(false);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(super) struct DefaultMaxCount(pub u8);

/// Represents a call to the Git CLI in the form:
///
/// `git SUBCOMMAND [DEFAULT_ARGS]`
#[derive(Debug, PartialEq, Eq)]
pub(super) struct GitCommand<'a> {
    pub(crate) subcommand: &'a str,
    pub(crate) default_args: &'a [&'a str],
    pub(crate) user_args: &'a [String],
}

/// Outcome of running a Git command; used to set exit code at end
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(crate) enum GitCommandResult {
    Success,
    Error,
}

/// Options to the `git-config` command
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(crate) struct GitConfigOpts {
    pub(crate) show_origin: bool,
    pub(crate) show_scope: bool,
}

impl Git {
    pub(crate) fn parse_config_options(options: GitConfigOpts, config_args: &mut Vec<&str>) {
        if options.show_origin {
            config_args.push("--show-origin")
        }
        if options.show_scope {
            config_args.push("--show-scope")
        }
    }

    pub(crate) fn pass_through(args: &[String]) -> GitResult {
        trace!("<pass_through> called with: {:#?}", args);
        debug_assert!(!args.is_empty());

        let command = GitCommand {
            subcommand: &args[0],
            default_args: &[],
            user_args: if args.len() > 1 { &args[1..] } else { &[] },
        };

        command.execute_git_command()
    }

    /// Return success if
    pub(crate) fn verify_staging_area_is_empty() -> GitResult {
        trace!("check_for_staged_files() called");
        let output: std::process::Output =
            Commands::new_command_with_args("git", &["diff", "--staged", "--name-only"])
                .output()
                .with_context(|| "Failed to execute git command")?;

        if output.stdout.is_empty() {
            Ok(GitCommandResult::Success)
        } else {
            Ok(GitCommandResult::Error)
        }
    }
}

impl GitCommand<'_> {
    /// Construct and then execute a `std::process:Command` that calls `git`
    pub(crate) fn execute_git_command(&self) -> GitResult {
        trace!("execute_git_command() called with: {:#?}", self);

        if self.construct_git_command().status()?.success() {
            Ok(GitCommandResult::Success)
        } else {
            Ok(GitCommandResult::Error)
        }
    }

    /// Construct a `std::process:Command` that calls `git`
    pub(crate) fn construct_git_command(&self) -> Command {
        trace!("construct_git_command() called with: {:#?}", self);

        let mut command_args: Vec<&str> = match stdout().is_terminal() {
            /* Force color on subcommands that support it.
             * Note that this will force color, but `isatty()` will still be false. */
            true => vec!["-c", "color.ui=always", self.subcommand],
            false => vec![self.subcommand],
        };

        if !self.default_args.is_empty() {
            self.default_args
                .iter()
                .for_each(|arg| command_args.push(arg));
        }

        if !self.user_args.is_empty() {
            self.user_args.iter().for_each(|arg| command_args.push(arg));
        }

        debug!("parsed command args: {:#?}", command_args);

        Commands::new_command_with_args("git", &command_args)
    }
}
