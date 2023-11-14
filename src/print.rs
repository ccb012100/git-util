use std::io::{stdout, IsTerminal, stderr};

use nu_ansi_term::{AnsiString, AnsiStrings, Color};

pub(crate) struct Print();

impl Print {
    /// print to `stdout` in blue
    pub(crate) fn blue(message: &str) {
        match stdout().is_terminal() {
            true => Self::stdout(&[Color::Blue.bold().paint(message)]),
            false => println!("{}", message),
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
    fn stdout(message: &[AnsiString]) {
        println!("{}", AnsiStrings(message));
    }
}
