use clap::Parser;
use cli::{Cli, Command};
use color::term::Red;

mod cli;
mod config;
mod request;
mod util;

fn main() {
    match dispatch() {
        Ok(result) => println!("{result}"),
        Err(e) => eprintln!("{}: {}", Red("Error"), e),
    }
}

fn dispatch() -> Result<String, String> {
    let config = config::config()?;
    let cli = Cli::parse();

    match cli.command {
        Command::List {
            completed,
            before,
            after,
            shorter,
            longer,
            higher,
            lower,
        } => {
            let info = util::ListInfo {
                completed,
                before,
                after,
                shorter,
                longer,
                higher,
                lower,
            };
            request::list(config.server.host, config.server.port, info)
        }
    }
}
