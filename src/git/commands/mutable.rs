use crate::git::{Git, GitCommand, GitCommandResult, GitResult};
use anyhow::anyhow;
use log::{debug, trace};

pub mod add;
pub mod commit;
pub mod index;

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
