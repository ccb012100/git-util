use crate::print::Print;
use anyhow::{Context, Result};
use std::{
    io::{self, stdout, IsTerminal, Write},
    process::Command,
};

pub(crate) struct Git();

impl Git {
    /// list configured aliases, optionally filtering on those containing `filter`
    pub(crate) fn alias(filter: Option<&str>) -> Result<()> {
        let output = {
            Command::new("git")
                .args(["config", "--get-regexp", r"^alias\."])
                .output()
                .with_context(|| "Failed to execute git command")
        }?;

        match output.status.success() {
            true => {
                let print_fn: fn(&str) = match stdout().is_terminal() {
                    true => Print::purple,
                    false => Print::print,
                };

                let stdout = String::from_utf8(output.stdout)?;
                let stdout = stdout.lines();

                match filter {
                    Some(f) => {
                        stdout
                            .filter_map(|line| {
                                if line.contains(f) {
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

        Ok(())
    }

    pub(crate) fn ll(args: &[String]) -> Result<()> {
        Self::parse_for_max_count_and_execute(
            "log",
            &[
                "--color=always", // this will force color, but isatty() will still be false
                "--pretty='%C(yellow)%h %C(magenta)%as %C(blue)%aL %C(cyan)%s%C(reset)'",
            ],
            args,
            Some(25),
        )
    }

    /// parse first user arg for use as value to `--max-count=`, otherwise use `default_max_count`
    fn parse_for_max_count_and_execute(
        command: &str,
        default_args: &[&str],
        user_args: &[String],
        default_max_count: Option<u8>,
    ) -> Result<()> {
        let default_max_count: u8 = default_max_count.unwrap_or(5);

        let (max_count, user_args): (u8, &[String]) = match user_args.is_empty() {
            true => (default_max_count, &user_args),
            false => match user_args[0].parse::<u8>() {
                Ok(num) => (num, &user_args[1..]),
                Err(_) => (default_max_count, user_args),
            },
        };

        let max_count = &format!("--max-count={}", max_count);

        // add --max_count argument to the end of original list of default_args
        let default_args: Vec<&str> = {
            let mut args: Vec<&str> = default_args.to_vec();
            args.push(max_count);
            args
        };

        Self::execute_git_command(command, &default_args, user_args)
    }

    /// Execute `git` command with the supplied arguments
    fn execute_git_command(
        command: &str,
        default_args: &[&str],
        user_args: &[String],
    ) -> Result<()> {
        let mut command_args: Vec<&str> = Vec::new();
        command_args.push(command);

        if !default_args.is_empty() {
            default_args.iter().for_each(|arg| command_args.push(arg));
        }
        if !user_args.is_empty() {
            user_args.iter().for_each(|arg| command_args.push(arg));
        }

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

        Ok(())
    }
}
