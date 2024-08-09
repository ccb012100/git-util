use crate::git::{Git, GitCommand, GitCommandResult, GitResult};
use anyhow::anyhow;
use log::{debug, trace};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct MutableCommands();

impl MutableCommands {
    /// `git add ARGS`
    pub fn add(args: &[String]) -> GitResult {
        trace!("add() called with: {:#?}", args);
        if args.is_empty() {
            return Err(anyhow!("Must supply arguments"));
        }

        GitCommand {
            subcommand: "add",
            default_args: &[],
            user_args: args,
        }
        .execute_git_command()
    }

    /// `git add --all`
    ///
    /// Fails if there are already staged files
    pub fn add_all() -> GitResult {
        trace!("add_all called");

        if let GitCommandResult::Error = Git::verify_staging_area_is_empty()? {
            return Err(anyhow!(
                "Can not add updated and untracked files to staging area; there are already staged files!"
            ));
        }

        // equivalent to `git add --all && git commit`
        GitCommand {
            subcommand: "add",
            default_args: &["--all"],
            user_args: &[],
        }
        .execute_git_command()
    }

    /// `git add --update && git status --short`
    ///
    /// Fails if there are already staged files.
    pub fn add_updated() -> GitResult {
        trace!("add_updated() called");

        if let GitCommandResult::Error = Git::verify_staging_area_is_empty()? {
            return Err(anyhow!(
                "Can not add updated files to staging area; there are already staged files!"
            ));
        }

        // equivalent to `git add --update && git status --short`
        let result: GitCommandResult = GitCommand {
            subcommand: "add",
            default_args: &["--update"],
            user_args: &[],
        }
        .execute_git_command()?;

        match result {
            GitCommandResult::Success => GitCommand {
                subcommand: "status",
                default_args: &["--short"],
                user_args: &[],
            }
            .execute_git_command(),
            GitCommandResult::Error => Err(anyhow!("git add --update returned an error")),
        }
    }

    /// `git add --all && git commit`
    ///
    /// Fails if there are already staged files
    pub fn add_all_and_commit(args: &[String]) -> GitResult {
        trace!("aac() called with: {:#?}", args);

        match Self::add_all()? {
            GitCommandResult::Success => GitCommand {
                subcommand: "commit",
                default_args: &[],
                user_args: args,
            }
            .execute_git_command(),
            GitCommandResult::Error => Err(anyhow!("git add --all returned an error")),
        }
    }

    /// `git commit --all`
    ///
    /// The success case is logically equivalent to `git add --update && git commit`, but differs in the failure case.
    /// In the case of 2 separate **Git** commands, cancelling out of the commit (e.g. `:q!` in **Vim**) will still
    /// leave the staging area updated. In this version, the staging area is not updated if the commit is cancelled.
    ///
    /// Fails if there are already staged files.
    pub fn commit_all_updated_files(args: &[String]) -> GitResult {
        trace!("auc() called with: {:#?}", args);

        if let GitCommandResult::Error = Git::verify_staging_area_is_empty()? {
            return Err(anyhow!("There are already staged files!"));
        }

        GitCommand {
            subcommand: "commit",
            default_args: &["--all"],
            user_args: args,
        }
        .execute_git_command()
    }

    /// Changes the author on the last n commits to the current git user
    pub fn update_commit_author(num: Option<u16>) -> GitResult {
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

    /// Wrapper around `git-restore`
    pub fn restore(args: &[String]) -> GitResult {
        trace!("restore() called with: {:#?}", args);

        GitCommand {
            subcommand: "restore",
            default_args: &[],
            user_args: args,
        }
        .execute_git_command()
    }

    /// `git restore :/`
    pub fn restore_all() -> GitResult {
        trace!("restore_all() called");

        GitCommand {
            subcommand: "restore",
            default_args: &[":/"],
            user_args: &[],
        }
        .execute_git_command()
    }

    /// `git reset --mixed HEAD~NUM`
    pub fn undo_commits(num: Option<u16>) -> GitResult {
        trace!("undo() called with: {:#?}", num);

        GitCommand {
            subcommand: "reset",
            default_args: &["--mixed", &format!("HEAD~{}", num.unwrap_or(1))],
            user_args: &[],
        }
        .execute_git_command()
    }

    /// `git restore --staged ARGS`
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

    /// `git restore --staged :/`
    pub fn unstage_all() -> GitResult {
        debug!("update_all() called");

        GitCommand {
            subcommand: "restore",
            default_args: &["--staged", ":/"],
            user_args: &[],
        }
        .execute_git_command()
    }

    // `git fetch --verbose origin:BRANCH`
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
