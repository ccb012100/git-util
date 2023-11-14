use crate::print::Print;
use anyhow::{anyhow, Context, Result};
use log::debug;
use std::{
    io::{self, Write},
    process::Command,
};

pub(crate) struct Git();

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct DefaultMaxCount(u8);

/// Outcome of running a Git command; used to set exit code at end
#[derive(Debug)]
pub(crate) enum GitCommandResult {
    Success,
    Error,
}

type GitResult = Result<GitCommandResult>;

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
                let print_fn: fn(&str) = Print::blue;

                let stdout = String::from_utf8(output.stdout)?;
                let stdout = stdout.lines();

                match filter {
                    Some(f) => {
                        let term_upper = f.to_uppercase();

                        stdout
                            .filter_map(|line| {
                                if line.to_uppercase().contains(&term_upper) {
                                    Some(line.replace("alias.", ""))
                                } else {
                                    None
                                }
                            })
                            .for_each(|x| print_fn(&x));
                    }
                    None => {
                        stdout
                            .map(|line| line.replace("alias.", ""))
                            .for_each(|x| print_fn(&x));
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

    pub(crate) fn ll(args: &[String]) -> Result<GitCommandResult> {
        debug!("_ll_ called with: {:#?}", args);
        parse_for_max_count_and_execute(
            "log",
            &[
                "--color=always", // this will force color, but isatty() will still be false
                "--pretty='%C(yellow)%h %C(magenta)%as %C(blue)%aL %C(cyan)%s%C(reset)'",
            ],
            args,
            DefaultMaxCount(25),
        )
    }

    pub(crate) fn restore(args: &[String]) -> GitResult {
        debug!("_restore_ called with: {:#?}", args);

        parse_for_all_files_and_execute("restore", &[], args)
    }

    pub(crate) fn show_files(args: &[String]) -> GitResult {
        debug!("_show_files_ called with: {:#?}", args);

        parse_for_max_count_and_execute(
            "show",
            &["--pretty=", "--name-only"],
            args,
            DefaultMaxCount(1),
        )
    }

    pub(crate) fn update(branch: &String) -> GitResult {
        debug!("_update_ called with: {:#?}", branch);

        if branch.is_empty() {
            Err(anyhow!("must provide a branch name"))
        } else {
            execute_git_command("fetch", &["origin"], &[format!("{0}:{0}", branch)])
        }
    }

    pub(crate) fn unstage(args: &[String]) -> GitResult {
        debug!("_unstage_ called with: {:#?}", args);

        parse_for_all_files_and_execute("restore", &["--staged"], args)
    }
}

/// parse first arg for specifier 'all' to operate on all files
pub(crate) fn parse_for_all_files_and_execute(
    command: &str,
    default_args: &[&str],
    user_args: &[String],
) -> GitResult {
    debug!(
        "_parse_for_all_and_execute_ called with: {:#?}\n{:#?}\n{:#?}",
        command, default_args, user_args
    );

    if user_args.is_empty() {
        Err(anyhow!("must provide \"all\" or filename(s)"))
    } else if user_args[0].to_uppercase() == "ALL" {
        if user_args.len() > 1 {
            Err(anyhow!("the argument \"all\" should be used alone"))
        } else {
            let default_args = &[default_args, &[":/"]].concat();
            execute_git_command("restore", default_args, &user_args[1..])
        }
    } else {
        execute_git_command("restore", &[], user_args)
    }
}

/// parse first user arg for use as value to `--max-count=`, otherwise use `default_max_count`
fn parse_for_max_count_and_execute(
    command: &str,
    default_args: &[&str],
    user_args: &[String],
    default_max_count: DefaultMaxCount,
) -> Result<GitCommandResult> {
    debug!(
        "_parse_for_max_count_and_execute_ called with: {:#?}\n{:#?}\n{:#?}\n{:#?}",
        command, default_args, user_args, default_max_count
    );

    let (max_count, user_args): (u8, &[String]) = match user_args.is_empty() {
        true => (default_max_count.0, &user_args),
        false => match user_args[0].parse::<u8>() {
            Ok(num) => (num, &user_args[1..]),
            Err(_) => (default_max_count.0, user_args),
        },
    };

    let max_count = &format!("--max-count={}", max_count);

    // add --max_count argument to the end of original list of default_args
    let default_args: Vec<&str> = {
        let mut args: Vec<&str> = default_args.to_vec();
        args.push(max_count);
        args
    };

    execute_git_command(command, &default_args, user_args)
}

/// Execute `git` command with the supplied arguments
fn execute_git_command(command: &str, default_args: &[&str], user_args: &[String]) -> GitResult {
    debug!(
        "_execute_git_command_ called with: {:#?}\n{:#?}\n{:#?}",
        command, default_args, user_args
    );

    let mut command_args: Vec<&str> = Vec::new();
    command_args.push(command);

    if !default_args.is_empty() {
        default_args.iter().for_each(|arg| command_args.push(arg));
    }
    if !user_args.is_empty() {
        user_args.iter().for_each(|arg| command_args.push(arg));
    }

    debug!("parsed command_args: {:#?}", command_args);

    let output = Command::new("git")
        .args(&command_args)
        .output()
        .with_context(|| {
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
