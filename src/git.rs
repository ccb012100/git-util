use crate::print::Print;
use anyhow::{anyhow, Context, Result};
use log::debug;
use std::{
    io::{self, StdoutLock, Write},
    process::Command,
    sync::atomic::AtomicBool,
};

pub(crate) static PRINT_COMMAND: AtomicBool = AtomicBool::new(false);

pub(crate) struct Git();

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct DefaultMaxCount(u8);

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GitCommand<'a> {
    subcommand: &'a str,
    default_args: &'a [&'a str],
    user_args: &'a [String],
}

/// Outcome of running a Git command; used to set exit code at end
#[derive(Debug)]
pub(crate) enum GitCommandResult {
    Success,
    Error,
}

type GitResult = Result<GitCommandResult>;

/// this will force color, but isatty() will still be false
const COLOR: &str = "--color=always"; // this will force color, but isatty() will still be false

impl Git {
    /// list configured aliases, optionally filtering on those containing `filter`
    pub(crate) fn alias(filter: Option<&str>) -> GitResult {
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

    pub(crate) fn last(args: &[String]) -> GitResult {
        debug!("_last_ called with: {:#?}", args);

        parse_for_max_count_and_execute(
            GitCommand {
                subcommand: "log",
                default_args: &[COLOR, "--compact-summary"],
                user_args: args,
            },
            DefaultMaxCount(1),
        )
    }

    pub(crate) fn ll(args: &[String]) -> GitResult {
        debug!("_ll_ called with: {:#?}", args);

        parse_for_max_count_and_execute(
            GitCommand {
                subcommand: "log",
                default_args: &[
                    COLOR,
                    "--pretty='%C(yellow)%h %C(magenta)%as %C(blue)%aL %C(cyan)%s%C(reset)'",
                ],
                user_args: args,
            },
            DefaultMaxCount(25),
        )
    }

    pub(crate) fn restore(args: &[String]) -> GitResult {
        debug!("_restore_ called with: {:#?}", args);

        execute_git_command(GitCommand {
            subcommand: "restore",
            default_args: &[],
            user_args: args,
        })
    }

    pub(crate) fn restore_all() -> GitResult {
        debug!("_restore_all_ called");

        execute_git_command(GitCommand {
            subcommand: "restore",
            default_args: &[":/"],
            user_args: &[],
        })
    }

    pub(crate) fn show(args: &[String]) -> GitResult {
        debug!("_last_ called with: {:#?}", args);

        parse_for_max_count_and_execute(
            GitCommand {
                subcommand: "show",
                default_args: &[COLOR, "--expand-tabs=4"],
                user_args: args,
            },
            DefaultMaxCount(1),
        )
    }

    pub(crate) fn show_files(args: &[String]) -> GitResult {
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

    pub(crate) fn undo(num: u8) -> GitResult {
        debug!("undo called with: {:#?}", num);

        execute_git_command(GitCommand {
            subcommand: "reset",
            default_args: &[&format!("HEAD~{}", num)],
            user_args: &[],
        })
    }

    pub(crate) fn unstage(args: &[String]) -> GitResult {
        debug!("_unstage_ called with: {:#?}", args);

        execute_git_command(GitCommand {
            subcommand: "restore",
            default_args: &["--staged"],
            user_args: args,
        })
    }

    pub(crate) fn unstage_all() -> GitResult {
        debug!("_update_all_ called");

        execute_git_command(GitCommand {
            subcommand: "restore",
            default_args: &["--staged", ":/"],
            user_args: &[],
        })
    }

    pub(crate) fn update(branch: &String) -> GitResult {
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

/// parse first user arg for use as value to `--max-count=`, otherwise use `default_max_count`
fn parse_for_max_count_and_execute(
    command: GitCommand,
    default_max_count: DefaultMaxCount,
) -> Result<GitCommandResult> {
    debug!(
        "_parse_for_max_count_and_execute_ called with: {:#?}\n{:#?}",
        command, default_max_count
    );

    let (max_count, user_args): (u8, &[String]) = match command.user_args.is_empty() {
        true => (default_max_count.0, &command.user_args),
        false => match command.user_args[0].parse::<u8>() {
            Ok(num) => (num, &command.user_args[1..]),
            Err(_) => (default_max_count.0, command.user_args),
        },
    };

    let max_count: &String = &format!("--max-count={}", max_count);

    // add --max_count argument to the end of original list of default_args
    let default_args: Vec<&str> = {
        let mut args: Vec<&str> = command.default_args.to_vec();
        args.push(max_count);
        args
    };

    execute_git_command(GitCommand {
        subcommand: command.subcommand,
        default_args: &default_args,
        user_args,
    })
}

/// Execute `git` command with the supplied arguments
fn execute_git_command(command: GitCommand) -> GitResult {
    debug!("_execute_git_command_ called with: {:#?}", command);

    let mut command_args: Vec<&str> = vec![command.subcommand];

    if !command.default_args.is_empty() {
        command
            .default_args
            .iter()
            .for_each(|arg| command_args.push(arg));
    }

    if !command.user_args.is_empty() {
        command
            .user_args
            .iter()
            .for_each(|arg| command_args.push(arg));
    }

    debug!("parsed command args: {:#?}", command_args);

    let mut command = Command::new("git");
    command.args(&command_args);

    if PRINT_COMMAND.load(std::sync::atomic::Ordering::SeqCst) {
        Print::blue_stderr(&format!("command: {:?}", command));
    }

    let output: std::process::Output = command.output().with_context(|| {
        format!(
            "Failed to execute git command with args \"{}\"",
            command_args.join(" ")
        )
    })?;

    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    match output.status.success() {
        true => Ok(GitCommandResult::Success),
        false => Ok(GitCommandResult::Error),
    }
}
