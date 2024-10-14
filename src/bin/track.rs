use error_stack::{Report, Result, ResultExt};

use time_tracker::{
    error::{AppError, Suggestion},
    init,
};

fn main() -> Result<(), AppError> {
    init::error_reporting();

    Ok(())
}
