//! A filesystem tracker

// flat file tracker
// - "lockfile": tracker is running
// - "database file": JSON doc

use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use error_stack::{Result, ResultExt};
use serde::{Deserialize, Serialize};

use crate::feature::tracker::{
    EndTime, StartTime, StartupStatus, TimeRecord, Tracker, TrackerError,
};

#[derive(Debug, thiserror::Error)]
#[error("filesystem tracker error")]
pub struct FlatFileTrackerError;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct LockfileData {
    start_time: StartTime,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
struct FlatFileDatabase {
    records: Vec<TimeRecord>,
}

impl FlatFileDatabase {
    fn push(&mut self, record: TimeRecord) {
        self.records.push(record);
    }
}

pub struct FlatFileTracker {
    db: PathBuf,
    lockfile: PathBuf,
}

impl FlatFileTracker {
    fn new<D, L>(db: D, lockfile: L) -> Self
    where
        D: Into<PathBuf>,
        L: Into<PathBuf>,
    {
        let db = db.into();
        let lockfile = lockfile.into();
        Self { db, lockfile }
    }

    fn start_impl(&self) -> Result<StartupStatus, FlatFileTrackerError> {
        let lockfile_data = {
            let start_time = StartTime::now();
            let data = LockfileData { start_time };
            serde_json::to_string(&data)
                .change_context(FlatFileTrackerError)
                .attach_printable("failed to serialize lockfile data")?
        };

        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.lockfile);

        if let Ok(mut file) = file {
            file.write_all(lockfile_data.as_bytes())
                .change_context(FlatFileTrackerError)
                .attach_printable("failed to write lockfile data")?;
            Ok(StartupStatus::Started)
        } else {
            Ok(StartupStatus::Running)
        }
    }

    fn stop_impl(&self) -> Result<(), FlatFileTrackerError> {
        let start = read_lockfile(&self.lockfile)?.start_time;
        let end = EndTime::now();
        let record = TimeRecord { start, end };
        let mut db = load_database(&self.db)?;
        db.push(record);
        save_database(&self.db, &db)?;

        std::fs::remove_file(&self.lockfile)
            .change_context(FlatFileTrackerError)
            .attach_printable("unable to delete lockfile")?;
        Ok(())
    }
}

impl Tracker for FlatFileTracker {
    fn start(&self) -> Result<StartupStatus, TrackerError> {
        self.start_impl().change_context(TrackerError)
    }

    fn stop(&self) -> Result<(), TrackerError> {
        self.stop_impl().change_context(TrackerError)
    }

    fn is_running(&self) -> bool {
        self.lockfile.exists()
    }

    fn records(&self) -> Result<impl Iterator<Item = TimeRecord>, TrackerError> {
        let db = load_database(&self.db).change_context(TrackerError)?;
        Ok(db.records.into_iter())
    }
}

fn save_database<P>(path: P, db: &FlatFileDatabase) -> Result<(), FlatFileTrackerError>
where
    P: AsRef<Path>,
{
    let db = serde_json::to_string(&db)
        .change_context(FlatFileTrackerError)
        .attach_printable("failed to serialize database")?;
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(path.as_ref())
        .change_context(FlatFileTrackerError)
        .attach_printable("failed to open database")?
        .write_all(db.as_bytes())
        .change_context(FlatFileTrackerError)
        .attach_printable("failed to write database")?;
    Ok(())
}

fn load_database<P>(db: P) -> Result<FlatFileDatabase, FlatFileTrackerError>
where
    P: AsRef<Path>,
{
    let mut db_buf = String::default();

    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(db.as_ref())
        .change_context(FlatFileTrackerError)
        .attach_printable("failed to load database")?
        .read_to_string(&mut db_buf)
        .change_context(FlatFileTrackerError)
        .attach_printable("failed to read database")?;

    if db_buf.is_empty() {
        Ok(FlatFileDatabase::default())
    } else {
        Ok(serde_json::from_str(&db_buf)
            .change_context(FlatFileTrackerError)
            .attach_printable("failed to deserialize database")?)
    }
}

fn read_lockfile<P>(lockfile: P) -> Result<LockfileData, FlatFileTrackerError>
where
    P: AsRef<Path>,
{
    let file = OpenOptions::new()
        .read(true)
        .open(lockfile.as_ref())
        .change_context(FlatFileTrackerError)
        .attach_printable("failed to open lockfile")?;

    serde_json::from_reader(file)
        .change_context(FlatFileTrackerError)
        .attach_printable("failed to deserialize lockfile")
}

#[cfg(test)]
mod tests {
    use assert_fs::{fixture::ChildPath, prelude::PathChild, TempDir};

    use super::*;

    fn temp_paths() -> (TempDir, ChildPath, ChildPath) {
        let temp = TempDir::new().unwrap();
        let db = temp.child("db.json");
        let lockfile = temp.child("lockfile");
        (temp, db, lockfile)
    }

    fn new_flat_file_tracker(db: &ChildPath, lockfile: &ChildPath) -> FlatFileTracker {
        FlatFileTracker::new(db.to_path_buf(), lockfile.to_path_buf())
    }

    #[test]
    fn is_running_returns_true_after_start() {
        let (_temp, db, lockfile) = temp_paths();

        // Given a default tracker
        let tracker = new_flat_file_tracker(&db, &lockfile);

        // When the tracker is started
        tracker.start().unwrap();

        // Then the tracker is running
        assert!(tracker.is_running());
    }

    #[test]
    fn is_running_returns_false_after_stop() {
        let (_temp, db, lockfile) = temp_paths();

        // Given a running new tracker
        let tracker = new_flat_file_tracker(&db, &lockfile);
        tracker.start().unwrap();

        // When the tracker is stopped
        tracker.stop().unwrap();

        // Then the tracker is not running
        assert!(!tracker.is_running());
    }

    #[test]
    fn time_record_created_when_tracker_stop() {
        // Given a running tracker
        let (_temp, db, lockfile) = temp_paths();
        let tracker = new_flat_file_tracker(&db, &lockfile);
        tracker.start().unwrap();

        // When the tracker is stopped
        tracker.stop().unwrap();

        // Then a record is created
        // Iter<Record>
        assert!(tracker.records().unwrap().next().is_some());
    }

    #[test]
    fn initial_start_return_started_state() {
        // Given a new tracker
        let (_temp, db, lockfile) = temp_paths();
        let tracker = new_flat_file_tracker(&db, &lockfile);

        // When the tracker is started
        let started = tracker.start().unwrap();

        // Then the `started` state is returned
        assert_eq!(started, StartupStatus::Started);
    }

    #[test]
    fn multiple_starts_return_already_running_state() {
        // Given a running tracker
        let (_temp, db, lockfile) = temp_paths();
        let tracker = new_flat_file_tracker(&db, &lockfile);
        tracker.start().unwrap();

        // When the tracker is started again
        let started = tracker.start().unwrap();

        // Then the `already_running` state is returned
        assert_eq!(started, StartupStatus::Running);
    }
}
