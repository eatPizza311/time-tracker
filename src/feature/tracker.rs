use chrono::{DateTime, Utc};
use error_stack::Result;
use serde::{Deserialize, Serialize};

mod flatfile;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct StartTime(DateTime<Utc>);

impl StartTime {
    pub fn now() -> Self {
        Self(Utc::now())
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct EndTime(DateTime<Utc>);
impl EndTime {
    pub fn now() -> Self {
        Self(Utc::now())
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct TimeRecord {
    start: StartTime,
    end: EndTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartupStatus {
    /// Time tracker started
    Started,
    /// Time tracker already running
    Running,
}

#[derive(Debug, thiserror::Error)]
#[error("filesystem tracker error")]
pub struct TrackerError;

pub trait Tracker {
    fn start(&self) -> Result<StartupStatus, TrackerError>;

    fn stop(&self) -> Result<(), TrackerError>;

    fn is_running(&self) -> bool;

    fn records(&self) -> Result<impl Iterator<Item = TimeRecord>, TrackerError>;
}
