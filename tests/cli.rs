use assert_cmd::Command;
use assert_fs::{fixture::ChildPath, prelude::PathChild, TempDir};
use testresult::TestResult;

fn temp_paths() -> (TempDir, ChildPath, ChildPath) {
    let temp = TempDir::new().unwrap();
    let db = temp.child("db.json");
    let lockfile = temp.child("lockfile");
    (temp, db, lockfile)
}

fn start_tracking(db: &ChildPath, lockfile: &ChildPath) -> Result<(), testresult::TestError> {
    Command::cargo_bin("track")?
        .arg("--db-dir")
        .arg(db.to_path_buf())
        .arg("--lockfile")
        .arg(lockfile.to_path_buf())
        .arg("start")
        .assert()
        .success();
    Ok(())
}

#[test]
fn status_code_is_error_if_no_command_specified() -> TestResult {
    Command::cargo_bin("track")?.assert().failure();
    Ok(())
}

#[test]
fn start_command_starts_tracking_time() -> TestResult {
    let (_temp, db, lockfile) = temp_paths();

    start_tracking(&db, &lockfile)?;

    assert!(lockfile.to_path_buf().exists());
    Ok(())
}

#[test]
fn stop_command_stop_tracking_time() -> TestResult {
    let (_temp, db, lockfile) = temp_paths();
    start_tracking(&db, &lockfile)?;

    Command::cargo_bin("track")?
        .arg("--db-dir")
        .arg(db.to_path_buf())
        .arg("--lockfile")
        .arg(lockfile.to_path_buf())
        .arg("stop")
        .assert()
        .success();

    assert!(!lockfile.to_path_buf().exists());
    Ok(())
}

#[test]
fn report_command_generates_report() -> TestResult {
    Command::cargo_bin("track")?.arg("start").assert().success();
    Command::cargo_bin("track")?.arg("stop").assert().success();

    Command::cargo_bin("track")?
        .arg("report")
        .assert()
        .stdout("00:00:00\n")
        .success();

    Ok(())
}
