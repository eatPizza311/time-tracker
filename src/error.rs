//! Top-level error types

#[derive(Debug, thiserror::Error)]
#[error("an application is occured")]
pub struct AppError;

/// A suggestion displayed to the user
pub struct Suggestion(pub &'static str);
