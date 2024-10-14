//! Application setup

use clap_verbosity_flag::Verbosity;
use error_stack::{fmt::ColorMode, Report};
use owo_colors::OwoColorize;
use tracing_log::AsTrace;
use tracing_subscriber::EnvFilter;

use crate::error::Suggestion;

pub fn error_reporting() {
    Report::set_color_mode(ColorMode::Color);
    Report::install_debug_hook::<Suggestion>(|Suggestion(value), context| {
        let body = format!("suggestion: {value}");
        match context.color_mode() {
            ColorMode::Color => context.push_body(body.bright_blue().to_string()),
            ColorMode::Emphasis => context.push_body(body.italic().to_string()),
            ColorMode::None => context.push_body(body),
        }
    })
}
