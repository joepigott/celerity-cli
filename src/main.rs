use clap::Parser;
use cli::{Cli, Command};
use color::term::Red;
use taskscheduler::{NaiveTask, UpdateTask};

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
        Command::Add {
            title,
            deadline,
            duration,
            priority,
        } => {
            let new_task = NaiveTask::new(title, deadline, duration, priority);
            request::add(config.server.host, config.server.port, new_task)
        }
        Command::Update { id, title, deadline, duration, priority } => {
            let update_task = UpdateTask::new(id)
                .with_title(title)
                .with_deadline(deadline)
                .with_duration(duration)
                .with_priority(priority);
            request::update(config.server.host, config.server.port, update_task)
        }
        Command::Enable => {
            request::enable(config.server.host, config.server.port, true)
        }
        Command::Disable => {
            request::enable(config.server.host, config.server.port, false)
        }
    }
}
