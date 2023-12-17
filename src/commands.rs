use crate::print::Print;
use anyhow::{Context, Result};
use std::process::{ChildStdout, Command, Output, Stdio};

pub(crate) mod ripgrep;

pub(crate) struct Commands();

impl Commands {
    /// This is mainly a convenience function so that we can print the command
    pub(crate) fn new_command_with_args(command: &str, args: &[&str]) -> Command {
        let mut cmd = Command::new(command);
        cmd.args(args);
        Print::print_command(&cmd);
        cmd
    }

    /// format `input` as a table, using `separator` as the separator
    ///
    /// `column --table --separator 'SEPARATOR'`
    pub(crate) fn pipe_to_column(input: ChildStdout, separator: char) -> Result<Output> {
        Self::new_command_with_args(
            "column",
            &["--table", "--separator", separator.to_string().as_str()],
        )
        .stdin(Stdio::from(input))
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| "Failed to pipe to column")?
        .wait_with_output()
        .with_context(|| "Failed to get column output")
    }

    /// Pipe `input` to `command` with `arg` and pipe `command` to stdin
    ///
    /// `INPUT | COMMAND ARG | ...`
    pub(crate) fn double_ended_pipe(
        command: &str,
        input: ChildStdout,
        args: &[&str],
    ) -> Result<ChildStdout> {
        Self::new_command_with_args(command, args)
            .stdin(Stdio::from(input))
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| format!("Failed to spawn {command}"))?
            .stdout
            .with_context(|| format!("Failed to open stdout from {command} pipe"))
    }

    /// Call `command` with arguments from `args` and pipe output to stdin
    ///
    /// `COMMAND ARGS | ...`
    pub(crate) fn pipe_from_command(command: &str, args: &[&str]) -> Result<ChildStdout> {
        Self::new_command_with_args(command, args)
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| format!("Failed to execute {command} command"))?
            .stdout
            .with_context(|| format!("Failed to spawn {command}"))
    }
}
