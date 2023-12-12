use crate::git::{Git, GitCommand, FORCE_COLOR};
use crate::git::{GitCommandResult, GitResult};
use crate::GitConfigOpts;
use anyhow::Context;
use log::trace;
use std::{
    io::{self, Write},
    process::{ChildStdout, Stdio},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct ImmutableCommands();

impl ImmutableCommands {
    pub fn compact_summary_log(num: Option<u8>, args: &[String]) -> GitResult {
        trace!("last() called with: {:#?}, {:#?}", num, args);

        GitCommand {
            subcommand: "log",
            default_args: &[
                FORCE_COLOR,
                "--compact-summary",
                &format!("--max-count={}", num.unwrap_or(1)),
            ],
            user_args: args,
        }
        .execute_git_command()
    }

    /// list configured aliases, optionally filtering on those containing `filter`
    pub fn list_aliases(filter: Option<&str>, options: GitConfigOpts) -> GitResult {
        trace!("alias() called with: {:#?}", filter);

        let mut config_args = vec!["config"];

        Git::parse_config_options(options, &mut config_args);

        // this arg has to be last
        config_args.push("--get-regexp");
        config_args.push(r"^alias\.");

        // get Git config values that start with "alias."
        // `git --get-regexp ^alias\.`
        let git = Git::new_command_with_args("git", &config_args)
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| "Failed to execute git command")?
            .stdout
            .with_context(|| "Failed to spawn git")?;

        // strip out the initial "alias." from the config name
        // `sed 's/^alias\.//'`
        let sed = Git::new_command_with_arg("sed", r"s/^alias\.//")
            .stdin(Stdio::from(git))
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| "Failed to spawn sed")?
            .stdout
            .with_context(|| "Failed to open sed stdout")?;

        let rg: ChildStdout = match filter {
            Some(pattern) => {
                // filter on `filter`
                // `rg --fixed-strings FILTER`
                Git::new_command_with_args("rg", &["--fixed-strings", pattern])
                    .stdin(Stdio::from(sed))
                    .stdout(Stdio::piped())
                    .spawn()
                    .with_context(|| "Failed to spawn rg")?
                    .stdout
                    .with_context(|| "Failed to open ripgrep stdout")?
            }
            None => sed,
        };

        // replace the first space (which separates the alias name and value) with a semicolon
        // `sed 's/ /\;/'`
        let sed = Git::new_command_with_arg("sed", r"s/ /\;/")
            .stdin(Stdio::from(rg))
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| "Failed to spawn sed")?
            .stdout
            .with_context(|| "Failed to open sed stdout from sed pipe")?;

        // format as a table, using semicolon as the separator
        // `column --table --separator ';'`
        let column = Git::new_command_with_args("column", &["--table", "--separator", ";"])
            .stdin(Stdio::from(sed))
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| "Failed to pipe to column")?
            .wait_with_output()
            .with_context(|| "Failed to get column output")?;

        io::stdout()
            .write_all(&column.stdout)
            .with_context(|| "Failed to write column output to stdout")?;

        Ok(GitCommandResult::Success)
    }

    /// list configuration settings (excluding aliases), optionally filtering on those containing `filter`
    pub fn list_configuration_settings(filter: Option<&str>, options: GitConfigOpts) -> GitResult {
        trace!("conf() called with: {:#?}", filter);

        let mut config_args = vec!["config", "--list"];

        Git::parse_config_options(options, &mut config_args);

        // get Git config values that start with "alias."
        // `git --get-regexp ^alias\.`
        let git = Git::new_command_with_args("git", &config_args)
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| "Failed to execute git command")?
            .stdout
            .with_context(|| "Failed to spawn git")?;

        // filter out config entries that start with "alias."
        // `rg -v ^alias\.`
        let rg = Git::new_command_with_args("rg", &["--invert-match", r"^alias\."])
            .stdin(Stdio::from(git))
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| "Failed to spawn ripgrep")?
            .stdout
            .with_context(|| "Failed to open ripgrep stdout")?;

        let rg: ChildStdout = match filter {
            Some(pattern) => {
                // filter on `filter`
                // `rg --fixed-strings FILTER`
                Git::new_command_with_args("rg", &["--fixed-strings", pattern])
                    .stdin(Stdio::from(rg))
                    .stdout(Stdio::piped())
                    .spawn()
                    .with_context(|| "Failed to spawn ripgrep")?
                    .stdout
                    .with_context(|| "Failed to open ripgrep stdout")?
            }
            None => rg,
        };

        // format as a table, using equals sign as the separator
        // `column --table --separator =`
        let column = Git::new_command_with_args("column", &["--table", "--separator", "="])
            .stdin(Stdio::from(rg))
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| "Failed to pipe to column")?
            .wait_with_output()
            .with_context(|| "Failed to get column output")?;

        io::stdout()
            .write_all(&column.stdout)
            .with_context(|| "Failed to write column output to stdout")?;

        Ok(GitCommandResult::Success)
    }

    pub fn one_line_log(num: Option<u8>, args: &[String]) -> GitResult {
        trace!("log_oneline() called with: {:#?}", num);

        GitCommand {
            subcommand: "log",
            default_args: &[
                FORCE_COLOR,
                "--pretty='%C(yellow)%h %C(magenta)%as %C(blue)%aL %C(cyan)%s%C(reset)'",
                &format!("--max-count={}", num.unwrap_or(25)),
            ],
            user_args: args,
        }
        .execute_git_command()
    }

    pub fn show(num: Option<u8>, args: &[String]) -> GitResult {
        trace!("show() called with: {:#?}", num);

        GitCommand {
            subcommand: "show",
            default_args: &[
                FORCE_COLOR,
                "--expand-tabs=4",
                &format!("--max-count={}", num.unwrap_or(1)),
            ],
            user_args: args,
        }
        .execute_git_command()
    }

    pub fn show_files(num: Option<u8>) -> GitResult {
        trace!("show_files() called with: {:#?}", num);

        GitCommand {
            subcommand: "show",
            default_args: &[
                "--pretty=",
                "--name-only",
                &format!("--max-count={}", num.unwrap_or(1)),
            ],
            user_args: &[],
        }
        .execute_git_command()
    }
}
