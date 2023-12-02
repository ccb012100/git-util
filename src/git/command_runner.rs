use super::{commands::GitCommand, GitCommandResult, GitConfigOpts};
use crate::git::print_command;
use anyhow::{anyhow, Context, Result};
use log::{debug, trace};
use std::process::Command;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(crate) struct DefaultMaxCount(pub u8);

pub(crate) type GitResult = Result<GitCommandResult>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(crate) struct CommandRunner();

impl CommandRunner {
    pub(crate) fn check_for_staged_files() -> GitResult {
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
    pub(crate) fn execute_git_command(command: GitCommand) -> GitResult {
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

        let mut command = Self::new_command_with_args("git", &command_args);

        let x: std::process::ExitStatus = command
            .status()
            .with_context(|| format!("Failed to execute git command: {:?}", command))?;

        if x.success() {
            Ok(GitCommandResult::Success)
        } else {
            Ok(GitCommandResult::Error)
        }
    }

    pub(crate) fn parse_config_options(options: GitConfigOpts, config_args: &mut Vec<&str>) {
        if options.show_origin {
            config_args.push("--show-origin")
        }
        if options.show_scope {
            config_args.push("--show-scope")
        }
    }

    /// This is mainly a convenience function so that we can print the command
    pub(crate) fn new_command_with_args<'a>(command: &'a str, args: &'a [&'a str]) -> Command {
        let mut cmd = Command::new(command);
        cmd.args(args);
        print_command(&cmd);
        cmd
    }

    pub(crate) fn new_command_with_arg<'a>(command: &'a str, arg: &'a str) -> Command {
        let mut cmd = Command::new(command);
        cmd.arg(arg);
        print_command(&cmd);
        cmd
    }
}
