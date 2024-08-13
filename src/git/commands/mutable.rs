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

        GitCommand::new("add").run()
    }

    /// `git add --all`
    ///
    /// Fails if there are already staged files.
    pub fn add_updated_untracked() -> GitResult {
        trace!("add_all() called");

        Self::run_if_staging_empty(GitCommand::new("add").with_default_args(&["--all"]))
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

        // Equivalent to `git add --update && git status --short`
        let result =
            Self::run_if_staging_empty(GitCommand::new("add").with_default_args(&["--update"]))?;

        match result {
            GitCommandResult::Success => GitCommand::new("status")
                .with_default_args(&["--short"])
                .run(),
            GitCommandResult::Error => Err(anyhow!("git add --update returned an error")),
        }
    }

    /// `git add --all && git commit`
    ///
    /// Fails if there are already staged files.
    pub fn commit_updated_untracked() -> GitResult {
        trace!("aac() called");

        match Self::add_updated_untracked()? {
            GitCommandResult::Success => GitCommand::new("commit").run(),
            GitCommandResult::Error => Err(anyhow!("git add --all returned an error")),
        }
    }

    /// `git add --all && git commit --amend`
    ///
    /// Fails if there are already staged files.
    pub fn commit_updated_and_untracked_amend() -> GitResult {
        trace!("commit_all_amended called");

        match Self::add_updated_untracked()? {
            GitCommandResult::Success => GitCommand::new("commit")
                .with_default_args(&["--amend"])
                .run(),
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
    pub fn commit_updated() -> GitResult {
        trace!("auc() called");

        Self::run_if_staging_empty(GitCommand::new("commit").with_default_args(&["--all"]))
    }

    /// `git commit --all --amend`
    ///
    /// Fails if there are already staged files.
    pub fn commit_updated_amend() -> GitResult {
        trace!("commit_all_updated_files_amended() called");

        Self::run_if_staging_empty(
            GitCommand::new("commit").with_default_args(&["--all", "--amend"]),
        )
    }

    /// Changes the author on the last n commits to the current git user.
    pub fn update_commit_author(num: Option<u16>) -> GitResult {
        trace!("author() called with: {:#?}", num);

        GitCommand::new("rebase")
            .with_default_args(&[
                &format!("HEAD~{}", num.unwrap_or(1)),
                "-x",
                "git commit --amend --no-edit --reset-author",
            ])
            .run()
    }

    /// Wrapper around `git-restore`
    pub fn restore(args: &[String]) -> GitResult {
        trace!("restore() called with: {:#?}", args);

        GitCommand::new("restore").run()
    }

    /// `git restore :/`
    pub fn restore_all() -> GitResult {
        trace!("restore_all() called");

        GitCommand::new("restore").with_default_args(&[":/"]).run()
    }

    /// `git reset --mixed HEAD~NUM`
    pub fn undo_commits(num: Option<u16>) -> GitResult {
        trace!("undo() called with: {:#?}", num);

        GitCommand::new("reset")
            .with_default_args(&["--mixed", &format!("HEAD~{}", num.unwrap_or(1))])
            .run()
    }

    /// `git restore --staged ARGS`
    pub fn unstage(args: &[String]) -> GitResult {
        trace!("unstage() called with: {:#?}", args);

        if args.is_empty() {
            return Err(anyhow!("Must supply arguments"));
        }

        GitCommand::new("restore")
            .with_default_args(&["--staged"])
            .run()
    }

    /// `git restore --staged :/`
    pub fn unstage_all() -> GitResult {
        debug!("update_all() called");

        GitCommand::new("restore")
            .with_default_args(&["--staged", ":/"])
            .run()
    }

    // `git fetch --verbose origin:BRANCH`
    pub fn update_branch_from_remote(branch: &String) -> GitResult {
        debug!("update() called with: {:#?}", branch);

        if branch.is_empty() {
            return Err(anyhow!("Must supply branch name"));
        }

        GitCommand::new("fetch")
            .with_default_args(&["--verbose", "origin"])
            .with_user_args(&[format!("{0}:{0}", branch)])
            .run()
    }

    /// Run `command` if the staging area is empty.
    ///
    /// Fails if there are already staged files.
    fn run_if_staging_empty(command: GitCommand) -> GitResult {
        trace!("run_if_staging_empty() called");

        if let GitCommandResult::Error = Git::verify_staging_area_is_empty()? {
            return Err(anyhow!("There are already files in the staging area!"));
        }

        command.run()
    }
}
