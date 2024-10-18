use error_stack::{Result, ResultExt};

use time_tracker::{error::AppError, feature::cli, init};

fn main() -> Result<(), AppError> {
    init::error_reporting();
    init::tracing();

    cli::run()
        .change_context(AppError)
        .attach_printable("failed to run CLI")?;

    Ok(())
}
