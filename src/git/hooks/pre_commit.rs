use anyhow::anyhow;
use log::{debug, info};
use std::{
    env::{self, VarError},
    fmt::Display,
    process::ExitStatus,
};

use crate::{
    commands::{
        ripgrep::{Ripgrep, RipgrepOptions},
        Commands,
    },
    git::{
        env_vars::{GitEnvVars, GitUtilEnvVars},
        GitCommandResult, GitResult,
    },
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct PreCommitHook {}

impl PreCommitHook {
    /// Run pre-commit hook
    ///
    /// 1. Commit email is checked against the env value `$GIT_UTIL_USER_EMAIL`
    ///     - fails if the commit email does not match
    /// 2. If env value `$GIT_UTIL_DISALLOWED_STRINGS` is set, the diff changes are checked for matches to the disallowed strings
    ///     - fails if any added changes in the diff contain a match for any of the disallowed strings
    pub fn run() -> GitResult {
        info!("Running pre-commit hook");

        match env::var(String::from(GitUtilEnvVars::UserEmail)) {
            Ok(allowed_email) => match env::var(String::from(GitEnvVars::AuthorEmail)) {
                Ok(commit_email) => {
                    if commit_email != allowed_email {
                        return Err(anyhow!(
                            "Invalid commit email; {} value is \"{}\". Expected: \"{}\"",
                            GitEnvVars::AuthorEmail,
                            commit_email,
                            allowed_email
                        ));
                    }
                }
                Err(err) => return get_env_var_error(&GitEnvVars::AuthorEmail.to_string(), &err),
            },
            Err(err) => return get_env_var_error(&GitUtilEnvVars::UserEmail, &err),
        }

        match env::var(String::from(GitUtilEnvVars::DisallowedStrings)) {
            Ok(disallowed_strings) => {
                debug!(
                    "{}=\"{}\"",
                    GitUtilEnvVars::DisallowedStrings,
                    disallowed_strings
                );

                // get diff for impending commit
                let diff_changes =
                    Commands::pipe_from_command("git", &["diff-index", "-p", "-M", "--cached", "HEAD"])?;

                // filter down to code additions only
                let diff_added = Ripgrep::double_ended_pipe(diff_changes, r"^+", None)?;

                // filter for additions that match on any of the disallowed strings
                let blocked: ExitStatus = Ripgrep::pipe_to_ripgrep(
                    diff_added,
                    &disallowed_strings,
                    Some(&[RipgrepOptions::IgnoreCase, RipgrepOptions::Context(2)]),
                )?;

                // if blocked succeeds, that means a match was found for the disallowed strings
                if blocked.success() {
                    return Err(anyhow!("Disallowed string found in commit changes!"));
                }
            }
            Err(err) => {
                if err.to_string() == "environment variable not found" {
                    debug!(
                        "{} not found; skipping check",
                        GitUtilEnvVars::DisallowedStrings
                    )
                } else {
                    // env var exists, but there's some other problem with it
                    return get_env_var_error(&GitUtilEnvVars::DisallowedStrings, &err);
                }
            }
        }

        Ok(GitCommandResult::Success)
    }
}

/// Add detail to `&VarError` returned from `std::env::var` call
fn get_env_var_error<T: Display>(env_var: &T, err: &VarError) -> GitResult {
    Err(anyhow!("failed to get env variable {}: {}", env_var, err))
}
