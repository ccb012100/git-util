use nu_ansi_term::{AnsiString, AnsiStrings, Color};

pub(crate) struct Print();

impl Print {
    /// print to `stdout` in purple
    pub(crate) fn purple(message: &str) {
        let message: &[AnsiString] = &[Color::Purple.paint(message)];

        Self::stdout(message);
    }

    /// call `println!` macro without any additional formatting formatting
    pub(crate) fn print(message: &str) {
        println!("{}", message);
    }

    /// print `ANSIStrings` to `stdout`
    fn stdout(message: &[AnsiString]) {
        println!("{}", AnsiStrings(message));
    }
}
