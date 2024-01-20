use core::fmt;

/// Environment variables used by the **git-util** application
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(crate) enum GitUtilEnvVars {
    /// Pipe-delimited list of strings that are not allowed in the commit diffs
    DisallowedStrings,
    /// The email that is used for commits
    UserEmail,
}

/// Environment variables used by **Git**
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(crate) enum GitEnvVars {
    #[allow(dead_code)]
    AuthorDate,
    AuthorEmail,
    #[allow(dead_code)]
    AuthorName,
    #[allow(dead_code)]
    ExecPath,
    #[allow(dead_code)]
    IndexFile,
    #[allow(dead_code)]
    Prefix,
}

impl fmt::Display for GitEnvVars {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitEnvVars::AuthorDate => write!(f, "GIT_AUTHOR_DATE"),
            GitEnvVars::AuthorEmail => write!(f, "GIT_AUTHOR_EMAIL"),
            GitEnvVars::AuthorName => write!(f, "GIT_AUTHOR_NAME"),
            GitEnvVars::ExecPath => write!(f, "GIT_EXEC_PATH"),
            GitEnvVars::IndexFile => write!(f, "GIT_INDEX_FILE"),
            GitEnvVars::Prefix => write!(f, "GIT_PREFIX"),
        }
    }
}

impl From<GitEnvVars> for String {
    fn from(value: GitEnvVars) -> Self {
        value.to_string()
    }
}

impl fmt::Display for GitUtilEnvVars {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitUtilEnvVars::DisallowedStrings => write!(f,"GIT_UTIL_DISALLOWED_STRINGS"),
            GitUtilEnvVars::UserEmail => write!(f, "GIT_UTIL_USER_EMAIL"),
        }
    }
}

impl From<GitUtilEnvVars> for String {
    fn from(value: GitUtilEnvVars) -> Self {
        value.to_string()
    }
}
