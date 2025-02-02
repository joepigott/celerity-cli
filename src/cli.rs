use chrono::{Duration, NaiveDateTime};
use clap::{Parser, Subcommand};
use taskscheduler::priority::{self, Priority};

#[derive(Parser)]
#[command(author, version, about, long_about = None, disable_help_flag = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// List tasks in the queue
    List {
        /// List completed tasks instead of in-progress tasks
        #[arg(short, long)]
        completed: bool,

        /// List tasks before the provided date/time
        #[arg(short, long, value_parser = date_parser)]
        before: Option<NaiveDateTime>,

        /// List tasks after the provided date/time
        #[arg(short, long, value_parser = date_parser)]
        after: Option<NaiveDateTime>,

        /// List tasks shorter than the provided duration
        #[arg(short, long, value_parser = duration_parser)]
        shorter: Option<Duration>,

        /// List tasks shorter than the provided duration
        #[arg(short, long, value_parser = duration_parser)]
        longer: Option<Duration>,

        /// List tasks with higher than the provided priority
        #[arg(short, long)]
        higher: Option<u8>,

        /// List tasks with lower than the provided priority
        #[arg(short = 'L', long)]
        lower: Option<u8>,
    },

    /// Add a task to the queue
    Add {
        /// The task's title
        #[arg(short, long, required = true)]
        title: String,

        /// The task's deadline
        #[arg(short, long, required = true, value_parser = date_parser)]
        deadline: NaiveDateTime,

        /// The task's estimated duration
        #[arg(short = 'D', long, required = true, value_parser = duration_parser)]
        duration: Duration,

        /// The task's priority. Lower values are higher priority.
        #[arg(short, long, required = true)]
        priority: u8,
    },

    /// Update a task's information
    Update {
        /// The ID of the task to be updated
        id: usize,

        /// A new title
        #[arg(short, long)]
        title: Option<String>,

        /// A new deadline
        #[arg(short, long, value_parser = date_parser)]
        deadline: Option<NaiveDateTime>,

        /// A new duration
        #[arg(short = 'D', long, value_parser = duration_parser)]
        duration: Option<Duration>,

        /// A new priority
        #[arg(short, long)]
        priority: Option<u8>,
    },

    /// Delete a task from the queue
    Delete {
        /// The ID of the task to be deleted
        id: usize,

        /// Delete a task from the completed list
        #[arg(short, long)]
        completed: bool,
    },

    /// Mark a task as complete
    Complete {
        /// The ID of the task to be marked as complete
        id: usize,
    },

    /// Enable the scheduler
    Enable,

    /// Disable the scheduler
    Disable,

    /// Fetch the active task
    Active,

    /// Fetch the scheduler status
    Status,

    /// Control the scheduler priority
    Priority {
        #[command(subcommand)]
        command: PriorityCommand,
    },
}

#[derive(Subcommand)]
pub enum PriorityCommand {
    /// Show the current priority
    Show,

    /// Set the scheduler priority
    Set {
        /// The priority algorithm to apply to the scheduler
        #[arg(value_parser = priority_parser)]
        priority: Box<dyn Priority>,
    },
}

/// Parse a `NaiveDateTime` from the command line. Uses the `date_format` field
/// from the config file.
fn date_parser(s: &str) -> Result<NaiveDateTime, String> {
    let format = crate::config::date_format()?;
    NaiveDateTime::parse_from_str(s, &format).map_err(|e| e.to_string())
}

/// Parse a `Duration` from the command line.
fn duration_parser(s: &str) -> Result<Duration, String> {
    let unit = s
        .chars()
        .last()
        .ok_or("Please provide a valid duration unit (s, m, h, d)")?;
    let value = &s[..s.len() - 1]
        .parse::<usize>()
        .map_err(|_| "Invalid duration value")?;

    match unit {
        's' => Ok(Duration::seconds(*value as i64)),
        'm' => Ok(Duration::minutes(*value as i64)),
        'h' => Ok(Duration::hours(*value as i64)),
        'd' => Ok(Duration::days(*value as i64)),
        _ => Err("Please provide a valid duration unit (s, m, h, d)")?,
    }
}

/// Parse a `dyn Priority` from the command line. Current limitations prevent
/// the initialization of unknown trait objects on the server, so options must
/// be predefined on the server for now.
fn priority_parser(s: &str) -> Result<Box<dyn Priority>, String> {
    Ok(match s.to_lowercase().as_str() {
        "fifo" => Box::new(priority::FIFO {}),
        "deadline" => Box::new(priority::Deadline {}),
        "shortest" => Box::new(priority::Shortest {}),
        "longest" => Box::new(priority::Longest {}),
        "highest" => Box::new(priority::HighestPriority {}),
        "lowest" => Box::new(priority::LowestPriority {}),
        _ => Err("Unknown priority")?,
    })
}
