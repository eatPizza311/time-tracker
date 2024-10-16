use error_stack::{Report, Result, ResultExt};

use time_tracker::{
    error::{AppError, Suggestion},
    init,
};
use tracing::{info, warn};

fn main() -> Result<(), AppError> {
    init::error_reporting();
    init::tracing();

    Ok(())
}
