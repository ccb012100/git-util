use super::commands::{GitCommand, GitCommandResult};
use crate::{git::commands::PRINT_COMMAND, print::Print};
use anyhow::{anyhow, Context, Result};
use log::debug;
use std::process::Command;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DefaultMaxCount(pub u8);

pub type GitResult = Result<GitCommandResult>;

/// parse first user arg for use as value to `--max-count=`, otherwise use `default_max_count`
pub fn parse_for_max_count_and_execute(
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

pub fn check_for_staged_files() -> GitResult {
    let output: std::process::Output = Command::new("git")
        .args(["diff", "--staged", "--name-only"])
        .output()
        .with_context(|| "Failed to execute git command")?;

    if !output.stdout.is_empty() {
        Err(anyhow!("there are already staged files!"))
    } else {
        Ok(GitCommandResult::Success)
    }
}

/// Execute `git` command with the supplied arguments
pub fn execute_git_command(command: GitCommand) -> GitResult {
    debug!("_execute_git_command_ called with: {:#?}", command);

    let mut command_args: Vec<&str> = match command.subcommand {
        "status" | "reset" => {
            // To be parsed correctly by git, "-c", "color.ui=always", and the subcommand must be passed as separate args
            vec!["-c", "color.ui=always", command.subcommand]
        }
        _ => vec![command.subcommand],
    };

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

    let x: std::process::ExitStatus = command
        .status()
        .with_context(|| format!("Failed to execute git command: {:?}", command))?;

    if x.success() {
        Ok(GitCommandResult::Success)
    } else {
        Ok(GitCommandResult::Error)
    }
}
