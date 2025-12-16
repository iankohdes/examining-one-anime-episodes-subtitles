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
    NoTimestampSeparator,
    MultipleTimestampSeparators,
    Timestamp(TimestampError),
}

impl From<TimestampError> for TimingError {
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
            return Err(TimingError::NoTimestampSeparator)
        } else if split_s_elems > 2 {
            return Err(TimingError::MultipleTimestampSeparators)
        }

        let split_s_clone: Vec<&str> = split_s.clone().collect();

        let start_raw: &str = split_s_clone[0].trim();
        let start_timestamp = start_raw.parse::<Timestamp>()?;

        let end_raw: &str = split_s_clone[1].trim();
        let end_timestamp = end_raw.parse::<Timestamp>()?;

        Ok(Timing{start: start_timestamp, end: end_timestamp})
    }
}