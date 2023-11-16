use std::io::{stderr, stdout, IsTerminal, StdoutLock, Write};

use nu_ansi_term::{AnsiString, AnsiStrings, Color};

pub(crate) struct Print();

impl Print {
    /// print to `stdout` in blue
    pub(crate) fn blue_stdout(message: &str, lock: &mut StdoutLock) {
        match stdout().is_terminal() {
            true => Self::stdout(&[Color::Blue.bold().paint(message)], lock),
            false => writeln!(lock, "{}", message).unwrap(),
        }
    }

    /// print to `stderr` in blue
    pub(crate) fn blue_stderr(message: &str) {
        match stdout().is_terminal() {
            true => Self::stderr(&[Color::Blue.bold().paint(message)]),
            false => eprintln!("{}", message),
        }
    }

    /// print to `stderr` in red
    pub(crate) fn error(message: &str) {
        let message: String = "Error: ".to_owned() + message;

        match stderr().is_terminal() {
            true => Self::stderr(&[Color::Red.bold().paint(message)]),
            false => eprintln!("{}", message),
        }
    }

    /// print `ANSIStrings` to `stderr`
    fn stderr(message: &[AnsiString]) {
        eprintln!("{}", AnsiStrings(message));
    }

    /// print `ANSIStrings` to `stdout`
    fn stdout(message: &[AnsiString], lock: &mut StdoutLock) {
        writeln!(lock, "{}", AnsiStrings(message)).unwrap();
    }
}
