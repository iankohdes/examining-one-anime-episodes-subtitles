use std::str::FromStr;
use crate::types::timestamp;
use crate::types::timestamp::{Timestamp, TimestampError};

const TIMING_SEPARATOR: &str = "-->";

/// An instance of `Timing` is expected to have one, and only one, separator
/// and exactly two [`Timestamp`] instances: one to indicate when a subtitle
/// appears on the screen and another to indicate when it disappears.
///
/// A `Timing` separator looks as follows: `-->`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Timing {
    pub start: Timestamp,
    pub end: Timestamp,
}

#[derive(Debug)]
pub enum TimingError {
    EmptyTiming,
    MalformedTiming(String),
    Timestamp(TimestampError),
}

impl TimingError {
    pub fn malformed(msg: &str, original_input: &str) -> Self {
        TimingError::MalformedTiming(
            format!("{} (input: {})", msg, original_input)
        )
    }
}

impl From<TimestampError> for TimingError {
    // This allows me to convert a `TimestampError` to a `TimingError` using the `parse` function. Although
    // there is no explicit use of the `from` function, the `?` operator implicitly applies `from` when
    // dealing with the error branch (parsing returns a `Result` type).
    fn from(error: TimestampError) -> Self {
        TimingError::Timestamp(error)
    }
}

impl FromStr for Timing {
    type Err = TimingError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(TimingError::EmptyTiming)
        }

        let split_s = s.split(TIMING_SEPARATOR);  // This is an iterator
        let split_s_elems = split_s.clone().count();

        if split_s_elems == 1 {
            println!("Regarding timing string: {}", s);
            return Err(TimingError::malformed("Missing timestamp separator (-->)", s))
        } else if split_s_elems > 2 {
            println!("Regarding timing string: {}", s);
            return Err(TimingError::malformed("Multiple timestamp separators", s))
        }

        let split_s_clone: Vec<&str> = split_s.clone().collect();

        let start_raw: &str = split_s_clone[0].trim();
        let start_timestamp = start_raw.parse::<Timestamp>()?;

        let end_raw: &str = split_s_clone[1].trim();
        let end_timestamp = end_raw.parse::<Timestamp>()?;

        if start_timestamp > end_timestamp {
            return Err(TimingError::malformed("Start timestamp is later than end timestamp", s))
        }

        Ok(Timing{start: start_timestamp, end: end_timestamp})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_string() {
        let input = "00:18:25,437 --> 00:18:27,439";
        let parsed_input = input.parse::<Timing>();

        assert!(parsed_input.is_ok())
    }
}