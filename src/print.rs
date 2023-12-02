use std::io::{stderr, IsTerminal};

use nu_ansi_term::{AnsiString, AnsiStrings, Color};

pub(crate) struct Print();

impl Print {
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
