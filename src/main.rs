use clap::Parser;
use cli::Cli;
use git::GitCommandResult;
use log::debug;
use print::Print;

mod cli;
mod commands;
mod git;
mod print;

fn main() -> ! {
    let cli = Cli::parse();

    cli.initialize_logger();

    #[cfg(windows)]
    {
        log::info!("On Windows; enabling ansi support...");
        nu_ansi_term::enable_ansi_support().unwrap();
    }

    debug!("parsed Cli: {:#?}", &cli);

    match cli.run_subcommand() {
        Ok(git_command) => match git_command {
            GitCommandResult::Success => std::process::exit(0),
            GitCommandResult::Error => std::process::exit(1),
        },
        Err(e) => {
            Print::error(&format!("{}", e));
            std::process::exit(1)
        }
    }
}
