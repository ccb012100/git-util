use super::command_runner::{CommandRunner, GitResult};
use crate::print::Print;
use anyhow::{anyhow, Context};
use log::{debug, trace};
use std::{
    io::{self, StdoutLock, Write},
    process::Command,
    sync::atomic::AtomicBool,
};

/// Flag to indicate whether or not to print the Git commands executed
pub static PRINT_COMMAND: AtomicBool = AtomicBool::new(false);

pub struct GitCommands();

#[derive(Debug, PartialEq, Eq)]
pub struct GitCommand<'a> {
    pub subcommand: &'a str,
    pub default_args: &'a [&'a str],
    pub user_args: &'a [String],
}

/// Outcome of running a Git command; used to set exit code at end
#[derive(Debug)]
pub enum GitCommandResult {
    Success,
    Error,
}

/// Use with diff, show, log, grep commands to set `--color=always`.
/// This will force color, but `isatty()` will still be false.
const FORCE_COLOR: &str = "--color=always";

impl GitCommands {
    pub fn aac(args: &[String]) -> GitResult {
        trace!("aac() called with: {:#?}", args);
        CommandRunner::check_for_staged_files()?;

        // equivalent to `git add --all && git commit`
        let result: GitCommandResult = CommandRunner::execute_git_command(GitCommand {
            subcommand: "add",
            default_args: &[&"--all"],
            user_args: &[],
        })?;

        match result {
            GitCommandResult::Success => CommandRunner::execute_git_command(GitCommand {
                subcommand: "commit", // force color for `status` subcommand
                default_args: &[],
                user_args: args,
            }),
            GitCommandResult::Error => Err(anyhow!("git add --all returned an error")),
        }
    }

    pub fn add(args: &[String]) -> GitResult {
        trace!("add() called with: {:#?}", args);
        if args.is_empty() {
            CommandRunner::check_for_staged_files()?;

            // equivalent to `git add --update && git status --short`
            let result: GitCommandResult = CommandRunner::execute_git_command(GitCommand {
                subcommand: "add",
                default_args: &[&"--update"],
                user_args: &[],
            })?;

            match result {
                GitCommandResult::Success => CommandRunner::execute_git_command(GitCommand {
                    subcommand: "status", // force color for `status` subcommand
                    default_args: &[&"--short"],
                    user_args: &[],
                }),
                GitCommandResult::Error => Err(anyhow!("git add --update returned an error")),
            }
        } else {
            // pass through to git-add
            CommandRunner::execute_git_command(GitCommand {
                subcommand: "add",
                default_args: &[],
                user_args: args,
            })
        }
    }

    /// list configured aliases, optionally filtering on those containing `filter`
    pub fn alias(filter: Option<&str>) -> GitResult {
        trace!("alias() called with: {:#?}", filter);

        let output = {
            Command::new("git")
                .args(["config", "--get-regexp", r"^alias\."])
                .output()
                .with_context(|| "Failed to execute git command")
        }?;

        match output.status.success() {
            true => {
                let output = String::from_utf8(output.stdout)?;
                let output_lines = output.lines();

                let mut lock: io::StdoutLock<'_> = io::stdout().lock();

                let print_fn: fn(&str, &mut StdoutLock) = Print::blue_stdout;

                match filter {
                    Some(f) => {
                        let term_upper = f.to_uppercase();

                        output_lines
                            .filter_map(|line| {
                                if line.to_uppercase().contains(&term_upper) {
                                    Some(line.replace("alias.", ""))
                                } else {
                                    None
                                }
                            })
                            .for_each(|x| print_fn(&x, &mut lock));
                    }
                    None => {
                        output_lines
                            .map(|line| line.replace("alias.", ""))
                            .for_each(|x| print_fn(&x, &mut lock));
                    }
                }
            }
            false => io::stdout().write_all(&output.stdout)?,
        }

        io::stderr().write_all(&output.stderr)?;

        match output.status.success() {
            true => Ok(GitCommandResult::Success),
            false => Ok(GitCommandResult::Error),
        }
    }

    pub fn auc(args: &[String]) -> GitResult {
        trace!("auc() called with: {:#?}", args);
        CommandRunner::check_for_staged_files()?;

        // equivalent to `git add --all && git commit`
        let result: GitCommandResult = CommandRunner::execute_git_command(GitCommand {
            subcommand: "add",
            default_args: &[&"--update"],
            user_args: &[],
        })?;

        match result {
            GitCommandResult::Success => CommandRunner::execute_git_command(GitCommand {
                subcommand: "commit", // force color for `status` subcommand
                default_args: &[],
                user_args: args,
            }),
            GitCommandResult::Error => Err(anyhow!("git add --update returned an error")),
        }
    }

    pub fn author(num: Option<u8>) -> GitResult {
        trace!("author() called with: {:#?}", num);
        CommandRunner::execute_git_command(GitCommand {
            subcommand: "rebase",
            default_args: &[
                &format!("HEAD~{}", num.unwrap_or(1)),
                "-x",
                "git commit --amend --no-edit --reset-author",
            ],
            user_args: &[],
        })
    }

    pub fn last(num: Option<u8>, args: &[String]) -> GitResult {
        trace!("last() called with: {:#?}, {:#?}", num, args);

        CommandRunner::execute_git_command(GitCommand {
            subcommand: "log",
            default_args: &[
                FORCE_COLOR,
                "--compact-summary",
                &format!("--max-count={}", num.unwrap_or(1)),
            ],
            user_args: args,
        })
    }

    pub fn log_oneline(num: Option<u8>, args: &[String]) -> GitResult {
        trace!("log_oneline() called with: {:#?}", num);

        CommandRunner::execute_git_command(GitCommand {
            subcommand: "log",
            default_args: &[
                FORCE_COLOR,
                "--pretty='%C(yellow)%h %C(magenta)%as %C(blue)%aL %C(cyan)%s%C(reset)'",
                &format!("--max-count={}", num.unwrap_or(25)),
            ],
            user_args: args,
        })
    }

    pub fn pass_through(args: &[String]) -> GitResult {
        trace!("<pass_through> called with: {:#?}", args);
        debug_assert!(!args.is_empty());

        CommandRunner::execute_git_command(GitCommand {
            subcommand: &args[0],
            default_args: &[],
            user_args: if args.len() > 1 { &args[1..] } else { &[] },
        })
    }

    pub fn restore(args: &[String]) -> GitResult {
        trace!("restore() called with: {:#?}", args);

        CommandRunner::execute_git_command(GitCommand {
            subcommand: "restore",
            default_args: &[],
            user_args: args,
        })
    }

    pub fn restore_all() -> GitResult {
        trace!("restore_all() called");

        CommandRunner::execute_git_command(GitCommand {
            subcommand: "restore",
            default_args: &[":/"],
            user_args: &[],
        })
    }

    pub fn show(num: Option<u8>, args: &[String]) -> GitResult {
        trace!("show() called with: {:#?}", num);

        CommandRunner::execute_git_command(GitCommand {
            subcommand: "show",
            default_args: &[
                FORCE_COLOR,
                "--expand-tabs=4",
                &format!("--max-count={}", num.unwrap_or(1)),
            ],
            user_args: args,
        })
    }

    pub fn show_files(num: Option<u8>) -> GitResult {
        trace!("show_files() called with: {:#?}", num);

        CommandRunner::execute_git_command(GitCommand {
            subcommand: "show",
            default_args: &[
                "--pretty=",
                "--name-only",
                &format!("--max-count={}", num.unwrap_or(1)),
            ],
            user_args: &[],
        })
    }

    pub fn undo(num: Option<u8>) -> GitResult {
        trace!("undo() called with: {:#?}", num);

        CommandRunner::execute_git_command(GitCommand {
            subcommand: "reset",
            default_args: &[&format!("HEAD~{}", num.unwrap_or(1))],
            user_args: &[],
        })
    }

    pub fn unstage(args: &[String]) -> GitResult {
        trace!("unstage() called with: {:#?}", args);
        debug_assert!(!args.is_empty());

        CommandRunner::execute_git_command(GitCommand {
            subcommand: "restore",
            default_args: &["--staged"],
            user_args: args,
        })
    }

    pub fn unstage_all() -> GitResult {
        debug!("update_all() called");

        CommandRunner::execute_git_command(GitCommand {
            subcommand: "restore",
            default_args: &["--staged", ":/"],
            user_args: &[],
        })
    }

    pub fn update(branch: &String) -> GitResult {
        debug!("update() called with: {:#?}", branch);
        debug_assert!(!branch.is_empty());

        CommandRunner::execute_git_command(GitCommand {
            subcommand: "fetch",
            default_args: &["origin"],
            user_args: &[format!("{0}:{0}", branch)],
        })
    }
}
