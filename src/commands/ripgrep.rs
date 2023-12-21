use anyhow::{anyhow, Result};
use std::process::{ChildStdout, Command, ExitStatus, Stdio};

use super::Commands;

pub(crate) struct Ripgrep {}

pub(crate) enum RipgrepOptions {
    Context(u8),
    FixedStrings,
    IgnoreCase,
    InvertMatch,
}

impl Ripgrep {
    /// Pipe from `input` to ripgrep to stdin
    ///
    /// `INPUT | rg | ...`
    pub(crate) fn double_ended_pipe(
        input: ChildStdout,
        pattern: &str,
        options: Option<&[RipgrepOptions]>,
    ) -> Result<ChildStdout> {
        if let Some(opts) = options {
            Commands::double_ended_pipe("rg", input, &parse_options(opts, pattern))
        } else {
            Commands::double_ended_pipe("rg", input, &parse_options(&[], pattern))
        }
    }

    /// Pipe `input` to ripgrep
    ///
    /// `INPUT | rg OPTIONS PATTERN`
    pub(crate) fn pipe_to_ripgrep(
        input: ChildStdout,
        pattern: &str,
        options: Option<&[RipgrepOptions]>,
    ) -> Result<ExitStatus> {
        let args = match options {
            Some(opts) => parse_options(opts, pattern),
            None => vec![pattern],
        };

        let mut command: Command = Commands::new_command_with_args("rg", &args);

        match command.stdin(Stdio::from(input)).status() {
            Ok(status) => Ok(status),
            Err(err) => Err(anyhow!(err)),
        }
    }
}

/// Parse `options` in a `Vec<&str>` that can be used in a `std::process::Command`
fn parse_options<'a>(options: &'a [RipgrepOptions], pattern: &'a str) -> Vec<&'a str> {
    let mut args: Vec<&str> = Vec::new();

    for opt in options {
        args.push(match opt {
            RipgrepOptions::Context(num) => {
                // TODO: get this borrow issue worked out
                //context = &format!("-C{num}"),
                match num {
                    1 => "--context=1",
                    2 => "--context=2",
                    3 => "--context=3",
                    4 => "--context=4",
                    5 => "--context=5",
                    _ => todo!(),
                }
            }
            RipgrepOptions::FixedStrings => "--fixed-strings",
            RipgrepOptions::IgnoreCase => "--ignore-case",
            RipgrepOptions::InvertMatch => "--invert-match",
        })
    }

    // add pattern last
    args.push(pattern);

    args
}
