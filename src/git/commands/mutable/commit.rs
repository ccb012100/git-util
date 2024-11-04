use crate::git::{GitCommand, GitCommandResult, GitResult};
use anyhow::anyhow;
use log::trace;

/// `git add --all && git commit`
///
/// Fails if there are already staged files.
pub fn updated_and_untracked() -> GitResult {
    trace!("aac() called");

    match super::add::updated_and_untracked()? {
        GitCommandResult::Success => GitCommand::new("commit").run(),
        GitCommandResult::Error => Err(anyhow!("git add --all returned an error")),
    }
}

/// `git add --all && git commit --amend`
///
/// Fails if there are already staged files.
pub fn amend_updated_and_untracked() -> GitResult {
    trace!("commit_all_amended called");

    let result = super::add::updated_and_untracked();

    match result.is_ok() {
        true => GitCommand::new("commit")
            .with_default_args(&["--amend"])
            .run(),
        false => result,
    }
}

/// `git commit --all`
///
/// The success case is logically equivalent to `git add --update && git commit`, but differs in the failure case.
/// In the case of 2 separate **Git** commands, cancelling out of the commit (e.g. `:q!` in **Vim**) will still
/// leave the staging area updated. In this version, the staging area is not updated if the commit is cancelled.
///
/// Fails if there are already staged files.
pub fn updated() -> GitResult {
    trace!("auc() called");

    super::run_if_staging_empty(GitCommand::new("commit").with_default_args(&["--all"]))
}

/// `git commit --all --amend`
///
/// Fails if there are already staged files.
pub fn amend_updated() -> GitResult {
    trace!("commit_all_updated_files_amended() called");

    super::run_if_staging_empty(GitCommand::new("commit").with_default_args(&["--all", "--amend"]))
}

/// Changes the author on the last n commits to the current git user.
pub fn change_author(num: Option<u16>) -> GitResult {
    trace!("author() called with: {:#?}", num);

    GitCommand::new("rebase")
        .with_default_args(&[
            &format!("HEAD~{}", num.unwrap_or(1)),
            "-x",
            "git commit --amend --no-edit --reset-author",
        ])
        .run()
}

/// `git reset --mixed HEAD~NUM`
pub fn undo(num: Option<u16>) -> GitResult {
    trace!("undo() called with: {:#?}", num);

    GitCommand::new("reset")
        .with_default_args(&["--mixed", &format!("HEAD~{}", num.unwrap_or(1))])
        .run()
}
