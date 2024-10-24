use std::time::Duration;

#[derive(Debug, Default)]
pub struct HourMinSecFormatter;

pub trait DurationFormatter {
    fn format(&self, duration: Duration) -> String;
}

impl DurationFormatter for HourMinSecFormatter {
    fn format(&self, duration: Duration) -> String {
        let duration_in_sec = duration.as_secs();
        let seconds = duration_in_sec % 60;
        let minutes = (duration_in_sec % 3600) / 60;
        let hours = duration_in_sec / 3600;

        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn format_seconds() {
        let duration = Duration::from_secs(5);

        let formatter = HourMinSecFormatter::default();

        let text = formatter.format(duration);

        assert_eq!(&text, "00:00:05");
    }

    #[test]
    fn format_three_hours_eleven_minute() {
        let duration = Duration::from_secs(11460);

        let formatter = HourMinSecFormatter::default();

        let text = formatter.format(duration);

        assert_eq!(&text, "03:11:00");
    }
}
