use nu_ansi_term::{AnsiString, AnsiStrings, Color};

pub(crate) struct Output();

impl Output {
    #[allow(dead_code)]
    pub(crate) fn success(message: &str) {
        let message: &[AnsiString] = &[Color::Green.bold().paint(message)];

        Self::print_to_stdout(message);
    }

    #[allow(dead_code)]
    pub(crate) fn info(message: &str) {
        let message: &[AnsiString] = &[Color::Blue.paint(message)];

        Self::print_to_stdout(message);
    }

    #[allow(dead_code)]
    pub(crate) fn warn(message: &str) {
        let message: &[AnsiString] = &[Color::Yellow.paint(message)];

        Self::print_to_stdout(message);
    }

    #[allow(dead_code)]
    pub(crate) fn error(message: &str) {
        let message: &[AnsiString] = &[Color::Red.bold().paint(message)];

        Self::print_to_stdout(message);
    }

    fn print_to_stdout(message: &[AnsiString]) {
        eprintln!("{}", AnsiStrings(message));
    }
}