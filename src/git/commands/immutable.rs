use crate::commands::Commands;
use crate::git::{GitCommandResult, GitResult};
use crate::GitConfigOpts;
use crate::{
    commands::RipgrepOptions,
    git::{Git, GitCommand, FORCE_COLOR},
};
use anyhow::Context;
use log::trace;
use std::{
    io::{self, Write},
    process::{ChildStdout, Output},
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
        let git = Commands::pipe_from_git(&config_args)?;

        // strip out the initial "alias." from the config name
        let sed = Commands::pipe_to_sed(git, r"s/^alias\.//")?;

        let rg: ChildStdout = match filter {
            Some(pattern) => {
                // filter on `pattern`
                Commands::pipe_to_ripgrep(sed, pattern, &[RipgrepOptions::FixedStrings])?
            }
            None => sed,
        };

        // replace the first space (which separates the alias name and value) with a semicolon
        let sed = Commands::pipe_to_sed(rg, r"s/ /\;/")?;

        let column: Output = Commands::pipe_to_column(sed, ';')?;

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
        let git = Commands::pipe_from_git(&config_args)?;

        // filter out config entries that start with "alias."
        // `rg -v ^alias\.`
        let rg = Commands::pipe_to_ripgrep(git, r"^alias\.", &[RipgrepOptions::InvertMatch])?;

        let rg: ChildStdout = match filter {
            Some(pattern) => {
                // filter on `pattern`
                Commands::pipe_to_ripgrep(rg, pattern, &[RipgrepOptions::FixedStrings])?
            }
            None => rg,
        };

        let column: Output = Commands::pipe_to_column(rg, '=')?;

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
