use crate::git::{
    commands::immutable::ImmutableCommands, Git, GitCommand, GitCommandResult, GitResult,
};
use anyhow::anyhow;
use log::trace;

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
pub fn updated_and_untracked() -> GitResult {
    trace!("add_all() called");

    let result = super::run_if_staging_empty(GitCommand::new("add").with_default_args(&["--all"]));

    if result.is_err() {
        Err(anyhow!("git add --all returned an error"))
    } else {
        ImmutableCommands::status_short()
    }
}

/// `git add --all`
pub fn updated_and_untracked_forced() -> GitResult {
    trace!("add_all() called");

    let result = GitCommand::new("add").with_default_args(&["--all"]).run();

    if result.is_err() {
        Err(anyhow!("git add --all returned an error"))
    } else {
        ImmutableCommands::status_short()
    }
}

/// `git add --update && git status --short`
///
/// Fails if there are already staged files.
pub fn updated() -> GitResult {
    trace!("add_updated() called");

    if let GitCommandResult::Error = Git::verify_staging_area_is_empty()? {
        return Err(anyhow!(
            "Can not add updated files to staging area; there are already staged files!"
        ));
    }

    // Equivalent to `git add --update && git status --short`
    let result =
        super::run_if_staging_empty(GitCommand::new("add").with_default_args(&["--update"]));

    if result.is_err() {
        Err(anyhow!("git add --update returned an error"))
    } else {
        ImmutableCommands::status_short()
    }
}

/// `git add --update && git status --short`
pub fn updated_forced() -> GitResult {
    trace!("add_updated_forced() called");

    // Equivalent to `git add --update && git status --short`
    let result = GitCommand::new("add")
        .with_default_args(&["--update"])
        .run();

    if result.is_err() {
        Err(anyhow!("git add --update returned an error"))
    } else {
        ImmutableCommands::status_short()
    }
}
