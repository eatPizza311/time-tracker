//! A filesystem tracker

// flat file tracker
// - "lockfile": tracker is running
// - "database file": JSON doc

use std::{fs::OpenOptions, path::PathBuf};

use error_stack::{Result, ResultExt};

#[derive(Debug, thiserror::Error)]
#[error("filesystem tracker error")]
pub struct FlatFileTrackerError;

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

    fn start(&self) -> Result<(), FlatFileTrackerError> {
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.lockfile)
            .change_context(FlatFileTrackerError)
            .attach_printable("unable to create new lockfile when start tracking")?;

        Ok(())
    }

    fn stop(&self) -> Result<(), FlatFileTrackerError> {
        std::fs::remove_file(&self.lockfile)
            .change_context(FlatFileTrackerError)
            .attach_printable("unable to delete lockfile")?;
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.lockfile.exists()
    }
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

        // When the tracker is stoped
        tracker.stop();

        // Then the tracker is not running
        assert!(!tracker.is_running());
    }
}
