#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::str::FromStr;

const PERMITTED_TIMESTAMP_CHARS: &str = "01234567890:,";

/// One subtitle block in an SRT file.
///
/// ```text
/// 12
/// 00:01:02,510 --> 00:01:04,120
/// Hello world!
/// Potential second line
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubtitleUnit {
    pub index: u32,
    pub timing: Timing,
    pub lines: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Timing {
    pub start: Timestamp,
    pub end: Timestamp,
}

///Rich timestamp representation.
///Represents `hh:mm:ss,mmm`.
// Implement the TryFrom trait
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub milliseconds: u16,
}

/// A full subtitle file, in its logical grouped form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubtitleSet {
    pub units: Vec<SubtitleUnit>,
}

#[derive(Debug)]
pub enum TimestampError {
    EmptyMillisecondsValue,
    EmptySecondsValue,
    EmptyMinutesValue,
    EmptyHoursValue,
    MillisecondsValueExceeds999,
    SecondsValueExceeds59,
    MinutesValueExceeds59,
    MillisecondsValueExceeds16BitAllocation,
    SecondsValueExceeds8BitAllocation,
    MinutesValueExceeds8BitAllocation,
    HoursValueExceeds8BitAllocation,
    EmptyString,
    DisallowedCharacters,
    NewlineCharDetected,
    MalformedTimestamp,
}

impl FromStr for Timestamp {
    type Err = TimestampError;

    /// Allowed characters in a timestamp are: `01234567890:,`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn count_char_occurrences(xs: &str, x: char) -> usize {
            xs.chars().filter(|&char| char == x).count()
        }

        if s.is_empty() {
            return Err(TimestampError::EmptyString)
        } else if count_char_occurrences(s, ',') != 1 {
            return Err(TimestampError::MalformedTimestamp)
        } else if s.contains("\n") {
            return Err(TimestampError::NewlineCharDetected)
        } else if s.chars().all(|char| PERMITTED_TIMESTAMP_CHARS.contains(char)) == false {
            return Err(TimestampError::DisallowedCharacters)
        }

        let split_on_comma: Vec<&str> = s.split(",").collect();

        let raw_hh_mm_ss = split_on_comma[0];
        let split_hh_mm_ss: Vec<&str> = raw_hh_mm_ss.split(":").collect();
        if split_hh_mm_ss.len() != 3 {
            return Err(TimestampError::MalformedTimestamp)
        }

        let raw_hh = split_hh_mm_ss[0];
        let raw_mm = split_hh_mm_ss[1];
        let raw_ss = split_hh_mm_ss[2];
        if raw_hh.is_empty() {
            return Err(TimestampError::EmptyHoursValue)
        } else if raw_mm.is_empty() {
            return Err(TimestampError::EmptyMinutesValue)
        } else if raw_ss.is_empty() {
            return Err(TimestampError::EmptySecondsValue)
        }

        let raw_ms = split_on_comma[1];
        if raw_ms.is_empty() {
            return Err(TimestampError::EmptyMillisecondsValue)
        } else if count_char_occurrences(raw_ms, ':') > 0 {
            return Err(TimestampError::MalformedTimestamp)
        }

        let u8_max = u8::MAX as usize;

        let hours = raw_hh.parse::<usize>().unwrap();
        if hours > u8_max {
            return Err(TimestampError::HoursValueExceeds8BitAllocation)
        }
        let minutes = raw_mm.parse::<usize>().unwrap();
        if minutes > u8_max {
            return Err(TimestampError::MinutesValueExceeds8BitAllocation)
        } else if minutes > 59 {
            return Err(TimestampError::MinutesValueExceeds59)
        }
        let seconds = raw_ss.parse::<usize>().unwrap();
        if seconds > u8_max {
            return Err(TimestampError::SecondsValueExceeds8BitAllocation)
        } else if seconds > 59 {
            return Err(TimestampError::SecondsValueExceeds59)
        }
        let milliseconds = raw_ms.parse::<usize>().unwrap();
        if milliseconds > u16::MAX as usize {
            return Err(TimestampError::MillisecondsValueExceeds16BitAllocation)
        } else if milliseconds > 999 {
            return Err(TimestampError::MillisecondsValueExceeds999)
        }

        Ok(
            Timestamp{
                hours: hours as u8,
                minutes: minutes as u8,
                seconds: seconds as u8,
                milliseconds: milliseconds as u16
            }
        )
    }
}
