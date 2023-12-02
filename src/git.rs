use std::{process::Command, sync::atomic::AtomicBool};

use crate::print::Print;

pub(crate) mod command_runner;
pub(crate) mod commands;

/// Flag to indicate whether or not to print the Git commands executed
pub(crate) static PRINT_COMMAND: AtomicBool = AtomicBool::new(false);

/// Outcome of running a Git command; used to set exit code at end
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(crate) enum GitCommandResult {
    Success,
    Error,
}

/// Options to the `git-config` command
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(crate) struct GitConfigOpts {
    pub(crate) show_origin: bool,
    pub(crate) show_scope: bool,
}

/// Print `command` to `stderr` if `PRINT_COMMAND` has been set
pub(crate) fn print_command(command: &Command) {
    if PRINT_COMMAND.load(std::sync::atomic::Ordering::SeqCst) {
        Print::stderr_purple(&format!("command: {:?}", command));
    }
}
