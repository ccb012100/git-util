use crate::git::{GitCommand, GitResult};
use anyhow::anyhow;
use log::{debug, trace};

/// Wrapper around `git-restore`
pub fn restore(args: &[String]) -> GitResult {
    trace!("restore() called with: {:#?}", args);

    GitCommand::new("restore").with_user_args(args).run()
}

/// `git restore :/`
pub fn restore_all() -> GitResult {
    trace!("restore_all() called");

    GitCommand::new("restore").with_default_args(&[":/"]).run()
}

/// `git restore --staged ARGS`
pub fn unstage(args: &[String]) -> GitResult {
    trace!("unstage() called with: {:#?}", args);

    if args.is_empty() {
        return Err(anyhow!("Must supply arguments"));
    }

    GitCommand::new("restore")
        .with_default_args(&["--staged"])
        .with_user_args(args)
        .run()
}

/// `git restore --staged :/`
pub fn unstage_all() -> GitResult {
    debug!("update_all() called");

    GitCommand::new("restore")
        .with_default_args(&["--staged", ":/"])
        .run()
}
