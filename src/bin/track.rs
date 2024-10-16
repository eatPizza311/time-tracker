use error_stack::Result;

use time_tracker::{error::AppError, init};

fn main() -> Result<(), AppError> {
    init::error_reporting();
    init::tracing();

    Ok(())
}
