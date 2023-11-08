use crate::print::Print;
use anyhow::{Context, Result};
use std::{
    io::{self, stdout, IsTerminal, Write},
    process::Command,
};

pub(crate) struct Git();

impl Git {
    fn execute_git_command(args: &[&str]) -> Result<std::process::Output> {
        Command::new("git")
            .args(args)
            .output()
            .with_context(|| "Failed to execute git command")
    }

    /// list configured aliases, filtering on `args` if supplied
    pub(crate) fn alias(args: &[String]) -> Result<()> {
        let output = Self::execute_git_command(&["config", "--get-regexp", r"^alias\."])?;

        match output.status.success() {
            true => {
                let print_fn: fn(&str) = match stdout().is_terminal() {
                    true => Print::purple,
                    false => Print::print,
                };

                let stdout = String::from_utf8(output.stdout)?;
                let stdout = stdout.lines();

                if args.is_empty() {
                    stdout
                        .map(|line| line.replace("alias.", ""))
                        .for_each(|x| print_fn(&x));
                } else {
                    let term = args.join(" ");

                    stdout
                        .filter_map(|line| {
                            if line.contains(&term) {
                                Some(line.replace("alias.", ""))
                            } else {
                                None
                            }
                        })
                        .for_each(|x| print_fn(&x));
                }
            }
            false => io::stdout().write_all(&output.stdout)?,
        }

        io::stderr().write_all(&output.stderr)?;

        Ok(())
    }
}
