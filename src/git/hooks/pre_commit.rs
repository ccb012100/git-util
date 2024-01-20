use anyhow::{anyhow, Context};
use log::{debug, info};
use std::{
    env::{self, VarError},
    fmt::Display,
    io::{self, Write},
};

use crate::{
    git::{
        env_vars::{GitEnvVars, GitUtilEnvVars},
        GitCommand, GitCommandResult, GitResult,
    },
    print::Print,
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
                let diff_changes_output: std::process::Output = GitCommand {
                    subcommand: "diff-index",
                    default_args: &["-p", "-M", "--cached", "HEAD"],
                    user_args: &[],
                }
                .construct_git_command()
                .output()
                .with_context(|| "Failed to execute 'git diff-index' command")?;

                match diff_changes_output.status.success() {
                    true => {
                        let stdout = String::from_utf8(diff_changes_output.stdout)?;
                        let stdout = stdout.lines();

                        let blocked_strings: Vec<String> = disallowed_strings
                            .split('|')
                            .map(|ds| ds.to_lowercase())
                            .collect();

                        // filter down to code additions only
                        for line in stdout.filter(|line| line.starts_with('+')) {
                            for blocked in &blocked_strings {
                                if line.to_lowercase().contains(blocked) {
                                    Print::stderr_purple(&format!(
                                        "Disallowed addition:\n\n{line}"
                                    ));

                                    return Err(anyhow!(
                                        "Disallowed string found in commit changes!"
                                    ));
                                }
                            }
                        }
                        debug!("No disallowed changes found");
                    }
                    false => io::stdout().write_all(&diff_changes_output.stdout)?,
                }

                io::stderr().write_all(&diff_changes_output.stderr)?;
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
