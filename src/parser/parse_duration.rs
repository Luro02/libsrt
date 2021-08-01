use core::time::Duration;

use super::ParserError;
use crate::utils::Spanned;

trait DurationExt {
    #[inline]
    #[must_use]
    fn from_hours(hours: u64) -> Duration { Duration::from_mins(hours * 60) }

    #[inline]
    #[must_use]
    fn from_mins(mins: u64) -> Duration { Duration::from_secs(mins * 60) }
}

impl DurationExt for Duration {}

fn parse_u64(string: Spanned<&str>) -> Result<u64, ParserError> {
    string
        .parse::<u64>()
        .map_err(|e| ParserError::parse_int_error(e, string.range()))
}

/// Parses a `String` of the following format:
/// `hours:minutes:seconds,milliseconds`
pub(crate) fn parse_duration(input: Spanned<&str>) -> Result<Duration, ParserError> {
    if let [Some(hours), Some(minutes), Some(seconds_millis)] = input.split_at_most::<_, 3>(':') {
        if let (seconds, Some(millis)) = seconds_millis.split_once(',') {
            let mut result = Duration::from_secs(0);

            result += Duration::from_hours(parse_u64(hours)?);
            result += Duration::from_mins(parse_u64(minutes)?);
            result += Duration::from_secs(parse_u64(seconds)?);
            result += Duration::from_millis(parse_u64(millis)?);

            return Ok(result);
        }
    }

    Err(ParserError::invalid_duration(0..input.len()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // TODO: test for errors
    #[test]
    fn test_parse_duration() {
        assert_eq!(
            parse_duration(Spanned::new("00:00:06,500")),
            Ok(Duration::from_secs(6) + Duration::from_millis(500))
        );
        assert_eq!(
            parse_duration(Spanned::new("00:00:11,000")),
            Ok(Duration::from_secs(11))
        );
        assert_eq!(
            parse_duration(Spanned::new("00:00:11,00")),
            Ok(Duration::from_secs(11))
        );
        assert_eq!(
            parse_duration(Spanned::new("00:00:11,001")),
            Ok(Duration::from_secs(11) + Duration::from_millis(1))
        );
        assert_eq!(
            parse_duration(Spanned::new("999:99:99,999")),
            Ok(Duration::from_hours(999)
                + Duration::from_mins(99)
                + Duration::from_secs(99)
                + Duration::from_millis(999))
        );
    }
}
