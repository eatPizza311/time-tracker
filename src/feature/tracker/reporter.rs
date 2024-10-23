use std::time::Duration;

use chrono::Utc;
use error_stack::{Result, ResultExt};

use crate::feature::tracker::Tracker;

#[derive(Debug, Clone, Copy)]
enum ReportTimespan {
    Last(Duration),
}

#[derive(Debug, thiserror::Error)]
#[error("reporter error")]
pub struct ReporterError;

pub trait Reporter: Tracker {
    fn total_duration(&self, timespan: ReportTimespan) -> Result<Duration, ReporterError> {
        match timespan {
            ReportTimespan::Last(duration) => {
                let earliest_time_to_consider = (Utc::now() - duration).timestamp_millis();
                let total_ms = self
                    .records()
                    .change_context(ReporterError)
                    .attach_printable("failed to get records")?
                    .filter_map(|rec| {
                        if rec.start.timestamp_millis() >= earliest_time_to_consider {
                            let time_delta =
                                rec.end.timestamp_millis() - rec.start.timestamp_millis();
                            Some(time_delta)
                        } else {
                            None
                        }
                    })
                    .sum::<i64>();
                Ok(Duration::from_millis(total_ms as u64))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::feature::tracker::tlib::FakeTracker;

    use super::*;

    impl Reporter for FakeTracker {}

    #[test]
    fn calculates_correct_duration_when_there_are_no_records() {
        // Given a default tracker
        let tracker = FakeTracker::default();

        // When calculating duration
        let duration = tracker
            .total_duration(ReportTimespan::Last(Duration::from_secs(1)))
            .unwrap();

        // Then there is no duration due to no records
        assert_eq!(duration, Duration::from_millis(0));
    }

    #[test]
    fn calculates_correct_duration_when_there_are_two_records() {
        // Given a tracker of two records
        let mut tracker = FakeTracker::default();

        for _ in 0..2 {
            tracker.start().unwrap();
            std::thread::sleep(Duration::from_millis(10));
            tracker.stop().unwrap()
        }

        // When the duration is calculated
        let duration = tracker
            .total_duration(ReportTimespan::Last(Duration::from_secs(1)))
            .unwrap();
        // Then duration is at least 20ms
        assert!(duration >= Duration::from_millis(20));
    }
}
