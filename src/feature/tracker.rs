mod flatfile;
mod reporter;

use chrono::{DateTime, Utc};
use error_stack::Result;
use serde::{Deserialize, Serialize};

pub use flatfile::FlatFileTracker;
pub use reporter::{ReportTimespan, Reporter, ReporterError};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct StartTime(DateTime<Utc>);

impl StartTime {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub const fn timestamp_millis(&self) -> i64 {
        self.0.timestamp_millis()
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct EndTime(DateTime<Utc>);
impl EndTime {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub const fn timestamp_millis(&self) -> i64 {
        self.0.timestamp_millis()
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
    fn start(&mut self) -> Result<StartupStatus, TrackerError>;

    fn stop(&mut self) -> Result<(), TrackerError>;

    fn is_running(&self) -> bool;

    fn records(&self) -> Result<impl Iterator<Item = TimeRecord>, TrackerError>;
}

#[cfg(test)]
mod tlib {
    use super::*;

    #[derive(Debug, Default)]
    pub struct FakeTracker {
        tracking: Option<StartTime>,
        records: Vec<TimeRecord>,
    }

    impl Tracker for FakeTracker {
        fn start(&mut self) -> Result<StartupStatus, TrackerError> {
            if self.tracking.is_some() {
                Ok(StartupStatus::Running)
            } else {
                self.tracking = Some(StartTime::now());
                Ok(StartupStatus::Started)
            }
        }

        fn stop(&mut self) -> Result<(), TrackerError> {
            let start = self.tracking.take().unwrap();
            let end = EndTime::now();
            let record = TimeRecord { start, end };
            self.records.push(record);

            Ok(())
        }

        fn is_running(&self) -> bool {
            self.tracking.is_some()
        }

        fn records(&self) -> Result<impl Iterator<Item = TimeRecord>, TrackerError> {
            Ok(self.records.iter().cloned())
        }
    }
}
