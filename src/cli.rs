use chrono::{Duration, NaiveDateTime};
use clap::{Parser, Subcommand};

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

    /// Enable the scheduler
    Enable,

    /// Disable the scheduler
    Disable,
}

fn date_parser(s: &str) -> Result<NaiveDateTime, String> {
    let format = crate::config::date_format()?;
    NaiveDateTime::parse_from_str(s, &format).map_err(|e| e.to_string())
}

fn duration_parser(s: &str) -> Result<Duration, String> {
    let unit = s.chars().last().ok_or("Please provide a duration unit")?;
    let value = &s[..s.len() - 1]
        .parse::<usize>()
        .map_err(|_| "Invalid duration value")?;

    match unit {
        's' => Ok(Duration::seconds(*value as i64)),
        'm' => Ok(Duration::minutes(*value as i64)),
        'h' => Ok(Duration::hours(*value as i64)),
        'd' => Ok(Duration::days(*value as i64)),
        _ => Err("Invalid duration unit")?,
    }
}
