use anyhow::{Context, Result};
use std::process::{ChildStdout, Command, Output, Stdio};

use crate::print::Print;

pub(crate) struct Commands();

pub(crate) enum RipgrepOptions {
    InvertMatch,
    FixedStrings,
}

impl Commands {
    /// This is mainly a convenience function so that we can print the command
    pub(crate) fn new_command_with_arg(command: &str, arg: &str) -> Command {
        let mut cmd = Command::new(command);
        cmd.arg(arg);
        Print::print_command(&cmd);
        cmd
    }

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

    /// filter `input` on fixed string `pattern` with ripgrep
    ///
    /// `rg --fixed-strings PATTERN`
    pub(crate) fn pipe_to_ripgrep(
        input: ChildStdout,
        pattern: &str,
        options: &[RipgrepOptions],
    ) -> Result<ChildStdout> {
        let mut rg_options: Vec<&str> = Vec::new();

        for opt in options {
            match opt {
                RipgrepOptions::InvertMatch => rg_options.push("--invert-match"),
                RipgrepOptions::FixedStrings => rg_options.push("--fixed-strings"),
            }
        }

        rg_options.push(pattern);

        Self::new_command_with_args("rg", &rg_options)
            .stdin(Stdio::from(input))
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| "Failed to spawn ripgrep")?
            .stdout
            .with_context(|| "Failed to open ripgrep stdout")
    }

    /// Call `sed` with `pattern`
    ///
    /// `sed PATTERN`
    pub(crate) fn pipe_to_sed(input: ChildStdout, pattern: &str) -> Result<ChildStdout> {
        Self::new_command_with_arg("sed", pattern)
            .stdin(Stdio::from(input))
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| "Failed to spawn sed")?
            .stdout
            .with_context(|| "Failed to open sed stdout from sed pipe")
    }

    /// Call `git` with arguments from `args`
    ///
    /// `git ARGS`
    pub(crate) fn pipe_from_git(args: &[&str]) -> Result<ChildStdout> {
        Self::new_command_with_args("git", args)
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| "Failed to execute git command")?
            .stdout
            .with_context(|| "Failed to spawn git")
    }
}
