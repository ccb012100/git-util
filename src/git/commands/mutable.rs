use crate::git::{Git, GitCommand, GitCommandResult, GitResult};
use anyhow::anyhow;
use log::{debug, trace};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct MutableCommands();

impl MutableCommands {
    pub fn add(args: &[String]) -> GitResult {
        trace!("add() called with: {:#?}", args);
        if args.is_empty() {
            Git::verify_staging_area_is_empty()?;

            // equivalent to `git add --update && git status --short`
            let result: GitCommandResult = GitCommand {
                subcommand: "add",
                default_args: &["--update"],
                user_args: &[],
            }
            .execute_git_command()?;

            match result {
                GitCommandResult::Success => GitCommand {
                    subcommand: "status", // force color for `status` subcommand
                    default_args: &["--short"],
                    user_args: &[],
                }
                .execute_git_command(),
                GitCommandResult::Error => Err(anyhow!("git add --update returned an error")),
            }
        } else {
            // pass through to git-add
            GitCommand {
                subcommand: "add",
                default_args: &[],
                user_args: args,
            }
            .execute_git_command()
        }
    }

    pub fn add_all_and_commit(args: &[String]) -> GitResult {
        trace!("aac() called with: {:#?}", args);
        Git::verify_staging_area_is_empty()?;

        // equivalent to `git add --all && git commit`
        let result: GitCommandResult = GitCommand {
            subcommand: "add",
            default_args: &["--all"],
            user_args: &[],
        }
        .execute_git_command()?;

        match result {
            GitCommandResult::Success => GitCommand {
                subcommand: "commit", // force color for `status` subcommand
                default_args: &[],
                user_args: args,
            }
            .execute_git_command(),
            GitCommandResult::Error => Err(anyhow!("git add --all returned an error")),
        }
    }

    pub fn add_updated_files_and_commit(args: &[String]) -> GitResult {
        trace!("auc() called with: {:#?}", args);
        Git::verify_staging_area_is_empty()?;

        // equivalent to `git add --all && git commit`
        let result: GitCommandResult = GitCommand {
            subcommand: "add",
            default_args: &["--update"],
            user_args: &[],
        }
        .execute_git_command()?;

        match result {
            GitCommandResult::Success => GitCommand {
                subcommand: "commit", // force color for `status` subcommand
                default_args: &[],
                user_args: args,
            }
            .execute_git_command(),
            GitCommandResult::Error => Err(anyhow!("git add --update returned an error")),
        }
    }

    pub fn update_commit_author(num: Option<u8>) -> GitResult {
        trace!("author() called with: {:#?}", num);
        GitCommand {
            subcommand: "rebase",
            default_args: &[
                &format!("HEAD~{}", num.unwrap_or(1)),
                "-x",
                "git commit --amend --no-edit --reset-author",
            ],
            user_args: &[],
        }
        .execute_git_command()
    }

    pub fn restore(args: &[String]) -> GitResult {
        trace!("restore() called with: {:#?}", args);

        GitCommand {
            subcommand: "restore",
            default_args: &[],
            user_args: args,
        }
        .execute_git_command()
    }

    pub fn restore_all() -> GitResult {
        trace!("restore_all() called");

        GitCommand {
            subcommand: "restore",
            default_args: &[":/"],
            user_args: &[],
        }
        .execute_git_command()
    }

    pub fn undo_commits(num: Option<u8>) -> GitResult {
        trace!("undo() called with: {:#?}", num);

        GitCommand {
            subcommand: "reset",
            default_args: &[&format!("HEAD~{}", num.unwrap_or(1))],
            user_args: &[],
        }
        .execute_git_command()
    }

    pub fn unstage(args: &[String]) -> GitResult {
        trace!("unstage() called with: {:#?}", args);
        debug_assert!(!args.is_empty());

        GitCommand {
            subcommand: "restore",
            default_args: &["--staged"],
            user_args: args,
        }
        .execute_git_command()
    }

    pub fn unstage_all() -> GitResult {
        debug!("update_all() called");

        GitCommand {
            subcommand: "restore",
            default_args: &["--staged", ":/"],
            user_args: &[],
        }
        .execute_git_command()
    }

    pub fn update_branch_from_remote(branch: &String) -> GitResult {
        debug!("update() called with: {:#?}", branch);
        debug_assert!(!branch.is_empty());

        GitCommand {
            subcommand: "fetch",
            default_args: &["--verbose", "origin"],
            user_args: &[format!("{0}:{0}", branch)],
        }
        .execute_git_command()
    }
}
