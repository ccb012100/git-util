use std::{
    env::{self, VarError},
    fmt::Display,
};

use anyhow::anyhow;
use log::{debug, info};

use crate::git::{
    env_vars::{GitEnvVars, GitUtilEnvVars},
    GitResult,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct PrecommitHook {}

impl PrecommitHook {
    pub fn run() -> GitResult {
        info!("Running pre-commit hook");

        match env::var(String::from(GitUtilEnvVars::ValidUserEmail)) {
            Ok(valid_user_email) => match env::var(String::from(GitEnvVars::AuthorEmail)) {
                Ok(email) => {
                    if email != valid_user_email {
                        return Err(anyhow!(
                            "{} value is \"{}\". Expected: \"{}\"",
                            GitEnvVars::AuthorEmail,
                            email,
                            valid_user_email
                        ));
                    }
                }
                Err(err) => return get_env_var_error(&GitEnvVars::AuthorEmail.to_string(), &err),
            },
            Err(err) => return get_env_var_error(&GitUtilEnvVars::ValidUserEmail, &err),
        }

        match env::var(String::from(GitUtilEnvVars::DisallowedStrings)) {
            Ok(disallowed_strings) => {
                debug!(
                    "{}=\"{}\"",
                    GitUtilEnvVars::DisallowedStrings,
                    disallowed_strings
                );
                let disallowed_strings: Vec<&str> = disallowed_strings.split(';').collect();
                debug!("parsed {}: {:#?}", GitUtilEnvVars::DisallowedStrings, disallowed_strings);
                // TODO: check for disallowed strings in the diffs
                todo!()
            }
            Err(err) => {
                if err.to_string() == "environment variable not found" {
                    debug!("{} not found; skipping", GitUtilEnvVars::DisallowedStrings)
                } else {
                    // env var exists, but there's some other problem with it
                    return get_env_var_error(&GitUtilEnvVars::DisallowedStrings, &err);
                }
            }
        }
        todo!()
    }
}

fn get_env_var_error<T: Display>(env_var: &T, err: &VarError) -> GitResult {
    Err(anyhow!("failed to get env variable {}: {}", env_var, err))
}
