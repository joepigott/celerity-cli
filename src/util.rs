use chrono::{Duration, NaiveDateTime};

pub struct ListInfo {
    pub completed: bool,
    pub before: Option<NaiveDateTime>,
    pub after: Option<NaiveDateTime>,
    pub shorter: Option<Duration>,
    pub longer: Option<Duration>,
    pub lower: Option<u8>,
    pub higher: Option<u8>,
}
