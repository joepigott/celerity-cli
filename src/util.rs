use chrono::{Duration, NaiveDateTime};

/// Contains information to be used in filtering the task list.
pub struct ListInfo {
    pub completed: bool,
    pub date_format: String,
    pub before: Option<NaiveDateTime>,
    pub after: Option<NaiveDateTime>,
    pub shorter: Option<Duration>,
    pub longer: Option<Duration>,
    pub lower: Option<u8>,
    pub higher: Option<u8>,
}
