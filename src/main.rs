use color::term::BrightRed;
use clap::Parser;
use cli::{CLI, Command};

mod config;
mod cli;
mod request;

fn main() {
    match dispatch() {
        Ok(result) => println!("{result}"),
        Err(e) => eprintln!("{}: {}", BrightRed("Error:"), e),
    }
}

fn dispatch() -> Result<String, String> {
    let config = config::config()?;
    let cli = CLI::parse();

    match cli.command {
        Command::List => {
            request::list(config.server.host, config.server.port)
        }
    }
}
