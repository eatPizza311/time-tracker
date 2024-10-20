use chrono::{DateTime, Utc};
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
