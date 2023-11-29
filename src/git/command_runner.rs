use super::commands::{GitCommand, GitCommandResult};
use crate::{git::commands::PRINT_COMMAND, print::Print};
use anyhow::{anyhow, Context, Result};
use log::{debug, trace};
use std::process::Command;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct DefaultMaxCount(pub u8);

pub type GitResult = Result<GitCommandResult>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct CommandRunner();

impl CommandRunner {
    pub fn check_for_staged_files() -> GitResult {
        trace!("check_for_staged_files() called");
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
        trace!("execute_git_command() called with: {:#?}", command);

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
}
