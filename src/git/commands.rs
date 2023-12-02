use super::{
    command_runner::{CommandRunner, GitResult},
    GitConfigOpts,
};
use crate::git::{print_command, GitCommandResult};
use anyhow::{anyhow, Context};
use log::{debug, trace};
use std::{
    io::{self},
    process::{ChildStdout, Command, Stdio},
};

/// Use with `diff`, `show`, `log`, and `grep` commands to set `--color=always`.
/// This will force color, but `isatty()` will still be false.
const FORCE_COLOR: &str = "--color=always";

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GitCommand<'a> {
    pub(crate) subcommand: &'a str,
    pub(crate) default_args: &'a [&'a str],
    pub(crate) user_args: &'a [String],
}

pub(crate) struct GitCommands();

impl GitCommands {
    pub(crate) fn aac(args: &[String]) -> GitResult {
        trace!("aac() called with: {:#?}", args);
        CommandRunner::check_for_staged_files()?;

        // equivalent to `git add --all && git commit`
        let result: GitCommandResult = CommandRunner::execute_git_command(GitCommand {
            subcommand: "add",
            default_args: &["--all"],
            user_args: &[],
        })?;

        match result {
            GitCommandResult::Success => CommandRunner::execute_git_command(GitCommand {
                subcommand: "commit", // force color for `status` subcommand
                default_args: &[],
                user_args: args,
            }),
            GitCommandResult::Error => Err(anyhow!("git add --all returned an error")),
        }
    }

    pub(crate) fn add(args: &[String]) -> GitResult {
        trace!("add() called with: {:#?}", args);
        if args.is_empty() {
            CommandRunner::check_for_staged_files()?;

            // equivalent to `git add --update && git status --short`
            let result: GitCommandResult = CommandRunner::execute_git_command(GitCommand {
                subcommand: "add",
                default_args: &["--update"],
                user_args: &[],
            })?;

            match result {
                GitCommandResult::Success => CommandRunner::execute_git_command(GitCommand {
                    subcommand: "status", // force color for `status` subcommand
                    default_args: &["--short"],
                    user_args: &[],
                }),
                GitCommandResult::Error => Err(anyhow!("git add --update returned an error")),
            }
        } else {
            // pass through to git-add
            CommandRunner::execute_git_command(GitCommand {
                subcommand: "add",
                default_args: &[],
                user_args: args,
            })
        }
    }

    /// list configured aliases, optionally filtering on those containing `filter`
    pub(crate) fn alias(filter: Option<&str>, options: GitConfigOpts) -> GitResult {
        trace!("alias() called with: {:#?}", filter);

        let mut config_args = vec!["config"];

        parse_config_options(options, &mut config_args);

        // this arg has to be last
        config_args.push("--get-regexp");
        config_args.push(r"^alias\.");

        // get Git config values that start with "alias."
        // `git --get-regexp ^alias\.`
        let git = new_command_with_args("git", &config_args)
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| "Failed to execute git command")?;

        let git_output = git.stdout.with_context(|| "Failed to spawn git")?;

        // strip out the initial "alias." from the config name
        // `sed 's/^alias\.//'`
        let sed = new_command_with_arg("sed", r"s/^alias\.//")
            .stdin(Stdio::from(git_output))
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| "Failed to spawn sed")?;

        let stripped_output = sed.stdout.with_context(|| "Failed to open sed stdout")?;

        let filtered_output: ChildStdout = match filter {
            Some(pattern) => {
                // filter on `filter`
                // `rg --fixed-strings FILTER`
                new_command_with_args("rg", &["--fixed-strings", pattern])
                    .stdin(Stdio::from(stripped_output))
                    .stdout(Stdio::piped())
                    .spawn()
                    .with_context(|| "Failed to spawn sed")?
                    .stdout
                    .with_context(|| "Failed to open ripgrep stdout")?
            }
            None => stripped_output,
        };

        // replace the first space (which separates the alias name and value) with a semicolon
        // `sed 's/ /\;/'`
        let sed = new_command_with_arg("sed", r"s/ /\;/")
            .stdin(Stdio::from(filtered_output))
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| "Failed to spawn sed")?;

        let sed_output = sed
            .stdout
            .with_context(|| "Failed to open sed stdout from sed pipe")?;

        // format as a table, using semicolon as the separator
        // `column --table --separator ';'`
        let mut column = Command::new("column");
        column.arg("--table");

        print_command(&column);

        new_command_with_args("column", &["--table", "--separator", ";"])
            .stdin(Stdio::from(sed_output))
            .stdout(io::stdout())
            .spawn()
            .with_context(|| "Failed to pipe to column")?;

        Ok(GitCommandResult::Success)
    }

    /// list configuration settings (excluding aliases), optionally filtering on those containing `filter`
    pub(crate) fn conf(filter: Option<&str>, options: GitConfigOpts) -> GitResult {
        trace!("conf() called with: {:#?}", filter);

        let mut config_args = vec!["config", "--list"];

        parse_config_options(options, &mut config_args);

        // get Git config values that start with "alias."
        // `git --get-regexp ^alias\.`
        let git = new_command_with_args("git", &config_args)
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| "Failed to execute git command")?;

        let git_output = git.stdout.with_context(|| "Failed to spawn git")?;

        // filter out config entries that start with "alias."
        // `rg -v ^alias\.`
        let filtered_configs = new_command_with_args("rg", &["--invert-match", r"^alias\."])
            .stdin(Stdio::from(git_output))
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| "Failed to spawn ripgrep")?
            .stdout
            .with_context(|| "Failed to open ripgrep stdout")?;

        let filtered_output: ChildStdout = match filter {
            Some(pattern) => {
                // filter on `filter`
                // `rg --fixed-strings FILTER`
                new_command_with_args("rg", &["--fixed-strings", pattern])
                    .stdin(Stdio::from(filtered_configs))
                    .stdout(Stdio::piped())
                    .spawn()
                    .with_context(|| "Failed to spawn ripgrep")?
                    .stdout
                    .with_context(|| "Failed to open ripgrep stdout")?
            }
            None => filtered_configs,
        };

        // format as a table, using equals sign as the separator
        // `column --table --separator =`
        let mut column = Command::new("column");
        column.arg("--table");

        print_command(&column);

        new_command_with_args("column", &["--table", "--separator", "="])
            .stdin(Stdio::from(filtered_output))
            .stdout(io::stdout())
            .spawn()
            .with_context(|| "Failed to pipe to column")?;

        Ok(GitCommandResult::Success)
    }

    pub(crate) fn auc(args: &[String]) -> GitResult {
        trace!("auc() called with: {:#?}", args);
        CommandRunner::check_for_staged_files()?;

        // equivalent to `git add --all && git commit`
        let result: GitCommandResult = CommandRunner::execute_git_command(GitCommand {
            subcommand: "add",
            default_args: &["--update"],
            user_args: &[],
        })?;

        match result {
            GitCommandResult::Success => CommandRunner::execute_git_command(GitCommand {
                subcommand: "commit", // force color for `status` subcommand
                default_args: &[],
                user_args: args,
            }),
            GitCommandResult::Error => Err(anyhow!("git add --update returned an error")),
        }
    }

    pub(crate) fn author(num: Option<u8>) -> GitResult {
        trace!("author() called with: {:#?}", num);
        CommandRunner::execute_git_command(GitCommand {
            subcommand: "rebase",
            default_args: &[
                &format!("HEAD~{}", num.unwrap_or(1)),
                "-x",
                "git commit --amend --no-edit --reset-author",
            ],
            user_args: &[],
        })
    }

    pub(crate) fn last(num: Option<u8>, args: &[String]) -> GitResult {
        trace!("last() called with: {:#?}, {:#?}", num, args);

        CommandRunner::execute_git_command(GitCommand {
            subcommand: "log",
            default_args: &[
                FORCE_COLOR,
                "--compact-summary",
                &format!("--max-count={}", num.unwrap_or(1)),
            ],
            user_args: args,
        })
    }

    pub(crate) fn log_oneline(num: Option<u8>, args: &[String]) -> GitResult {
        trace!("log_oneline() called with: {:#?}", num);

        CommandRunner::execute_git_command(GitCommand {
            subcommand: "log",
            default_args: &[
                FORCE_COLOR,
                "--pretty='%C(yellow)%h %C(magenta)%as %C(blue)%aL %C(cyan)%s%C(reset)'",
                &format!("--max-count={}", num.unwrap_or(25)),
            ],
            user_args: args,
        })
    }

    pub(crate) fn pass_through(args: &[String]) -> GitResult {
        trace!("<pass_through> called with: {:#?}", args);
        debug_assert!(!args.is_empty());

        CommandRunner::execute_git_command(GitCommand {
            subcommand: &args[0],
            default_args: &[],
            user_args: if args.len() > 1 { &args[1..] } else { &[] },
        })
    }

    pub(crate) fn restore(args: &[String]) -> GitResult {
        trace!("restore() called with: {:#?}", args);

        CommandRunner::execute_git_command(GitCommand {
            subcommand: "restore",
            default_args: &[],
            user_args: args,
        })
    }

    pub(crate) fn restore_all() -> GitResult {
        trace!("restore_all() called");

        CommandRunner::execute_git_command(GitCommand {
            subcommand: "restore",
            default_args: &[":/"],
            user_args: &[],
        })
    }

    pub(crate) fn show(num: Option<u8>, args: &[String]) -> GitResult {
        trace!("show() called with: {:#?}", num);

        CommandRunner::execute_git_command(GitCommand {
            subcommand: "show",
            default_args: &[
                FORCE_COLOR,
                "--expand-tabs=4",
                &format!("--max-count={}", num.unwrap_or(1)),
            ],
            user_args: args,
        })
    }

    pub(crate) fn show_files(num: Option<u8>) -> GitResult {
        trace!("show_files() called with: {:#?}", num);

        CommandRunner::execute_git_command(GitCommand {
            subcommand: "show",
            default_args: &[
                "--pretty=",
                "--name-only",
                &format!("--max-count={}", num.unwrap_or(1)),
            ],
            user_args: &[],
        })
    }

    pub(crate) fn undo(num: Option<u8>) -> GitResult {
        trace!("undo() called with: {:#?}", num);

        CommandRunner::execute_git_command(GitCommand {
            subcommand: "reset",
            default_args: &[&format!("HEAD~{}", num.unwrap_or(1))],
            user_args: &[],
        })
    }

    pub(crate) fn unstage(args: &[String]) -> GitResult {
        trace!("unstage() called with: {:#?}", args);
        debug_assert!(!args.is_empty());

        CommandRunner::execute_git_command(GitCommand {
            subcommand: "restore",
            default_args: &["--staged"],
            user_args: args,
        })
    }

    pub(crate) fn unstage_all() -> GitResult {
        debug!("update_all() called");

        CommandRunner::execute_git_command(GitCommand {
            subcommand: "restore",
            default_args: &["--staged", ":/"],
            user_args: &[],
        })
    }

    pub(crate) fn update(branch: &String) -> GitResult {
        debug!("update() called with: {:#?}", branch);
        debug_assert!(!branch.is_empty());

        CommandRunner::execute_git_command(GitCommand {
            subcommand: "fetch",
            default_args: &["--verbose", "origin"],
            user_args: &[format!("{0}:{0}", branch)],
        })
    }
}

fn parse_config_options(options: GitConfigOpts, config_args: &mut Vec<&str>) {
    if options.show_origin {
        config_args.push("--show-origin")
    }
    if options.show_scope {
        config_args.push("--show-scope")
    }
}

/// This is mainly a convenience function so that we can print the command
fn new_command_with_args<'a>(command: &'a str, args: &'a [&'a str]) -> Command {
    let mut cmd = Command::new(command);
    cmd.args(args);
    print_command(&cmd);
    cmd
}

fn new_command_with_arg<'a>(command: &'a str, arg: &'a str) -> Command {
    let mut cmd = Command::new(command);
    cmd.arg(arg);
    print_command(&cmd);
    cmd
}
