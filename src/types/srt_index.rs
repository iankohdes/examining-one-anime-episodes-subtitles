use std::fmt::{write, Display};
use std::str::FromStr;

const PERMITTED_INDEX_CHARS: &str = "0123456789";
const U32_MAX_4294967295: usize = u32::MAX as usize;

/// Within an `.srt` file, when reading from top to bottom, instances of
/// `SrtIndex` must be monotonic increasing and the value of each increase
/// in step should be `1`, otherwise the subtitle file is not well-formed.
///
/// From the definition of [`PERMITTED_INDEX_CHARS`], it is implicitly
/// expected that a subtitle file’s indices are non-negative.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SrtIndex(u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SrtIndexError {
    EmptyIndex,
    IndexExceedsMaxU32Size(String),
    IndexContainsDisallowedChars(String),
}

impl Display for SrtIndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SrtIndexError::EmptyIndex => {
                write!(f, "SRT index (integer) is empty")
            }
            SrtIndexError::IndexExceedsMaxU32Size(string) => {
                write!(f, "Index {string} exceeds max u32 size")
            }
            SrtIndexError::IndexContainsDisallowedChars(string) => {
                write!(f, "Index {string} contains non-numeric characters")
            }
        }
    }
}

// Implement Error trait and Display trait -- all error types should both traits.

impl FromStr for SrtIndex {
    type Err = SrtIndexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(SrtIndexError::EmptyIndex);
        } else if s.chars().all(|char| PERMITTED_INDEX_CHARS.contains(char)) == false {
            return Err(SrtIndexError::IndexContainsDisallowedChars(s.to_string()));
        }

        let parsed_index = s.parse::<usize>().unwrap();
        if parsed_index > U32_MAX_4294967295 {
            return Err(SrtIndexError::IndexExceedsMaxU32Size(s.to_string()));
        }

        Ok(SrtIndex(parsed_index as u32))
    }
}
