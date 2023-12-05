use crate::git::PRINT_COMMAND;
use nu_ansi_term::{AnsiString, AnsiStrings, Color};
use std::{
    io::{stderr, IsTerminal},
    process::Command,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(crate) struct Print();

impl Print {
    /// Print `command` to `stderr` if `PRINT_COMMAND` has been set
    pub(crate) fn print_command(command: &Command) {
        if PRINT_COMMAND.load(std::sync::atomic::Ordering::SeqCst) {
            Print::stderr_purple(&format!("command: {:?}", command));
        }
    }

    /// print to `stderr` in purple
    pub(crate) fn stderr_purple(message: &str) {
        Self::stderr_color(message, Color::Purple)
    }

    /// print Error message to `stderr`
    pub(crate) fn error(message: &str) {
        let message: String = "Error: ".to_owned() + message;

        Self::stderr_color(&message, Color::Red)
    }

    fn stderr_color(message: &str, color: Color) {
        match stderr().is_terminal() {
            true => Self::stderr(color.bold().paint(message)),
            false => eprintln!("{}", message),
        }
    }

    /// print `ANSIStrings` to `stderr`
    fn stderr(message: AnsiString) {
        eprintln!("{}", AnsiStrings(&[message]));
    }
}
