use anyhow::{Context, Ok, Result};
use log::{debug, trace};
use std::{
    io::{stdout, IsTerminal},
    process::Command,
    sync::atomic::AtomicBool,
};

use crate::{commands::Commands, print::Print};

pub mod commands;
pub mod env_vars;
pub mod hooks;

pub type GitResult = Result<GitCommandResult>;
pub struct Git();

/// Flag used to indicate whether or not to print the commands executed.
pub static PRINT_COMMANDS: AtomicBool = AtomicBool::new(false);

/// Flag used to indicate whether subcommand is a dry run
pub static DRY_RUN: AtomicBool = AtomicBool::new(false);

/// Represents a call to the Git CLI in the form: `git SUBCOMMAND [DEFAULT_ARGS] [USER_ARGS]`
#[derive(Debug, PartialEq, Eq)]
pub struct GitCommand<'a> {
    subcommand: &'a str,
    default_args: &'a [&'a str],
    user_args: &'a [String],
}

/// The outcome of running a Git command; used to set exit code at end.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum GitCommandResult {
    Success,
    Error,
}

/// The options to the `git-config` command.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct GitConfigOpts {
    pub show_origin: bool,
    pub show_scope: bool,
}

impl Git {
    pub fn pass_through(args: &[String]) -> GitResult {
        trace!("<pass_through> called with: {:#?}", args);
        debug_assert!(!args.is_empty());

        let command = GitCommand {
            subcommand: &args[0],
            default_args: &[],
            user_args: if args.len() > 1 { &args[1..] } else { &[] },
        };

        command.run()
    }

    fn parse_config_options(options: GitConfigOpts, config_args: &mut Vec<&str>) {
        if options.show_origin {
            config_args.push("--show-origin")
        }
        if options.show_scope {
            config_args.push("--show-scope")
        }
    }

    /// Return `Success` if nothing is printed to stdout when `git diff --staged --name-only` is run.
    fn verify_staging_area_is_empty() -> GitResult {
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

    /// Return `Success` if there are no unstaged changes in the work tree.
    ///
    /// The staging area can be empty or populated.
    fn verify_no_unstaged_changes() -> GitResult {
        trace!("check_for_staged_files() called");
        let output: std::process::Output =
            Commands::new_command_with_args("git", &["status", "--porcelain"])
                .output()
                .expect("git command should execute");

        if output.stdout.is_empty() {
            Ok(GitCommandResult::Success)
        } else {
            let outlines = core::str::from_utf8(&output.stdout)
                .expect("git output should be valid UTF-8")
                .split('\n');

            /*
             * Each path starts with 'XY', where X is the status of the index,
             * and Y is the status of the work tree. If XY is ' *' or '**', then
             * we have unstaged changes (in other words, '* ' is the only valid state of XY)
             */
            let status_codes = ['M', 'A', 'D', 'C', 'R', 'U', '?'];

            for line in outlines {
                if line.is_empty() {
                    // the output ends with a blank line
                    continue;
                }

                let mut chars = line.chars();

                let x = chars
                    .next()
                    .expect("git status entry should be in valid porcelain format");

                let y = chars
                    .next()
                    .expect("git status entry should be in valid porcelain format");

                debug!("'xy' = '{x}{y}'");

                if status_codes.contains(&x) && y == ' ' {
                    continue;
                }

                if x == ' ' || status_codes.contains(&y) {
                    return Ok(GitCommandResult::Error);
                }

                if !status_codes.contains(&x) {
                    panic!(
                        "Invalid status code value {:?} in status entry: {:?}",
                        x, line
                    );
                } else if y != ' ' {
                    panic!(
                        "Invalid status code value {:?} in status entry: {:?}",
                        y, line
                    );
                } else {
                    log::error!("Invalid status codes. Status entry: {:?}", line);
                    panic!("This should be unreachable! Status entry: {:?}", line);
                }
            }

            Ok(GitCommandResult::Success)
        }
    }
}

impl GitCommand<'_> {
    fn new(subcommand: &str) -> GitCommand<'_> {
        GitCommand {
            subcommand,
            default_args: &[],
            user_args: &[],
        }
    }

    /// same as `self`, but with `defaults_args` set to `args`
    fn with_default_args<'a>(&'a self, args: &'a [&'a str]) -> GitCommand<'a> {
        GitCommand {
            subcommand: self.subcommand,
            default_args: args,
            user_args: self.user_args,
        }
    }

    /// same as `self`, but with `user_args` set to `args`
    fn with_user_args<'a>(&'a self, args: &'a [String]) -> GitCommand<'a> {
        GitCommand {
            subcommand: self.subcommand,
            default_args: self.default_args,
            user_args: args,
        }
    }

    /// Construct and then execute a `std::process:Command` that calls `git` with the **Git Subcommand** represented by `self`.
    fn run(&self) -> GitResult {
        trace!("run() called with: {:#?}", self);

        if DRY_RUN.load(std::sync::atomic::Ordering::SeqCst) {
            Print::stderr_purple(&format!(
                "command that would be run: `{}`",
                self.construct_git_command_string()
            ));
            Ok(GitCommandResult::Success)
        } else if self.construct_git_command().status()?.success() {
            Ok(GitCommandResult::Success)
        } else {
            Ok(GitCommandResult::Error)
        }
    }

    /// Construct a `std::process:Command` that calls `git` using the **Git Subcommand** represented by `self`.
    fn construct_git_command_string(&self) -> String {
        trace!("construct_git_command() called with: {:#?}", self);

        let command_args = self.parse_command_args();

        format!("git {} {}", self.subcommand, command_args.join(" "))
    }

    /// Construct a `std::process:Command` that calls `git` using the **Git Subcommand** represented by `self`.
    fn construct_git_command(&self) -> Command {
        trace!("construct_git_command() called with: {:#?}", self);

        let command_args = self.parse_command_args();

        Commands::new_command_with_args("git", &command_args)
    }

    fn parse_command_args(&self) -> Vec<&str> {
        trace!("parse_command_args() called with: {:#?}", self);

        let mut command_args: Vec<&str> = if stdout().is_terminal() {
            vec!["-c", "color.ui=always", self.subcommand]
        } else {
            vec![self.subcommand]
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

        command_args
    }
}
