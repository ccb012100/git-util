use super::helpers::{
    check_for_staged_files, execute_git_command, parse_for_max_count_and_execute, DefaultMaxCount,
    GitResult,
};
use crate::print::Print;
use anyhow::{anyhow, Context};
use log::debug;
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
    #[allow(unreachable_code, unused_variables)]
    pub fn aac(args: &[String]) -> GitResult {
        check_for_staged_files()?;

        // equivalent to `git add --all && git commit`
        let result: GitCommandResult = execute_git_command(GitCommand {
            subcommand: "add",
            default_args: &[&"--all"],
            user_args: &[],
        })?;

        match result {
            GitCommandResult::Success => execute_git_command(GitCommand {
                subcommand: "commit", // force color for `status` subcommand
                default_args: &[],
                user_args: args,
            }),
            GitCommandResult::Error => Err(anyhow!("git add --all returned an error")),
        }
    }

    pub fn add(args: &[String]) -> GitResult {
        if args.is_empty() {
            check_for_staged_files()?;

            // equivalent to `git add --update && git status --short`
            let result: GitCommandResult = execute_git_command(GitCommand {
                subcommand: "add",
                default_args: &[&"--update"],
                user_args: &[],
            })?;

            match result {
                GitCommandResult::Success => execute_git_command(GitCommand {
                    subcommand: "status", // force color for `status` subcommand
                    default_args: &[&"--short"],
                    user_args: &[],
                }),
                GitCommandResult::Error => Err(anyhow!("git add --update returned an error")),
            }
        } else {
            // pass through to git-add
            execute_git_command(GitCommand {
                subcommand: "add",
                default_args: &[],
                user_args: args,
            })
        }
    }

    /// list configured aliases, optionally filtering on those containing `filter`
    pub fn alias(filter: Option<&str>) -> GitResult {
        debug!("_alias_ called with: {:#?}", filter);

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

    #[allow(unreachable_code, unused_variables)]
    pub fn auc(args: &[String]) -> GitResult {
        check_for_staged_files()?;

        // equivalent to `git add --all && git commit`
        let result: GitCommandResult = execute_git_command(GitCommand {
            subcommand: "add",
            default_args: &[&"--update"],
            user_args: &[],
        })?;

        match result {
            GitCommandResult::Success => execute_git_command(GitCommand {
                subcommand: "commit", // force color for `status` subcommand
                default_args: &[],
                user_args: args,
            }),
            GitCommandResult::Error => Err(anyhow!("git add --update returned an error")),
        }
    }

    pub fn author(num: Option<u8>) -> GitResult {
        execute_git_command(GitCommand {
            subcommand: "rebase",
            default_args: &[
                &format!("HEAD~{}", num.unwrap_or(1)),
                "-x",
                "git commit --amend --no-edit --reset-author",
            ],
            user_args: &[],
        })
    }

    pub fn last(args: &[String]) -> GitResult {
        debug!("_last_ called with: {:#?}", args);

        parse_for_max_count_and_execute(
            GitCommand {
                subcommand: "log",
                default_args: &[FORCE_COLOR, "--compact-summary"],
                user_args: args,
            },
            DefaultMaxCount(1),
        )
    }

    pub fn log_oneline(args: &[String]) -> GitResult {
        debug!("_log_oneline_ called with: {:#?}", args);

        parse_for_max_count_and_execute(
            GitCommand {
                subcommand: "log",
                default_args: &[
                    FORCE_COLOR,
                    "--pretty='%C(yellow)%h %C(magenta)%as %C(blue)%aL %C(cyan)%s%C(reset)'",
                ],
                user_args: args,
            },
            DefaultMaxCount(25),
        )
    }

    pub fn restore(args: &[String]) -> GitResult {
        debug!("_restore_ called with: {:#?}", args);

        execute_git_command(GitCommand {
            subcommand: "restore",
            default_args: &[],
            user_args: args,
        })
    }

    pub fn restore_all() -> GitResult {
        debug!("_restore_all_ called");

        execute_git_command(GitCommand {
            subcommand: "restore",
            default_args: &[":/"],
            user_args: &[],
        })
    }

    pub fn show(args: &[String]) -> GitResult {
        debug!("_last_ called with: {:#?}", args);

        parse_for_max_count_and_execute(
            GitCommand {
                subcommand: "show",
                default_args: &[FORCE_COLOR, "--expand-tabs=4"],
                user_args: args,
            },
            DefaultMaxCount(1),
        )
    }

    pub fn show_files(args: &[String]) -> GitResult {
        debug!("_show_files_ called with: {:#?}", args);

        parse_for_max_count_and_execute(
            GitCommand {
                subcommand: "show",
                default_args: &["--pretty=", "--name-only"],
                user_args: args,
            },
            DefaultMaxCount(1),
        )
    }

    pub fn undo(num: Option<u8>) -> GitResult {
        debug!("undo called with: {:#?}", num);

        execute_git_command(GitCommand {
            subcommand: "reset",
            default_args: &[&format!("HEAD~{}", num.unwrap_or(1))],
            user_args: &[],
        })
    }

    pub fn unstage(args: &[String]) -> GitResult {
        debug!("_unstage_ called with: {:#?}", args);

        execute_git_command(GitCommand {
            subcommand: "restore",
            default_args: &["--staged"],
            user_args: args,
        })
    }

    pub fn unstage_all() -> GitResult {
        debug!("_update_all_ called");

        execute_git_command(GitCommand {
            subcommand: "restore",
            default_args: &["--staged", ":/"],
            user_args: &[],
        })
    }

    pub fn update(branch: &String) -> GitResult {
        debug!("_update_ called with: {:#?}", branch);

        if branch.is_empty() {
            Err(anyhow!("must provide a branch name"))
        } else {
            execute_git_command(GitCommand {
                subcommand: "fetch",
                default_args: &["origin"],
                user_args: &[format!("{0}:{0}", branch)],
            })
        }
    }
}
