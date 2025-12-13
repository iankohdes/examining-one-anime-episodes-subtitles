#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

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

<<<<<<< HEAD
#[derive(Debug, Clone, PartialEq, Eq)]
struct RawSubtitleUnit {
    raw_index: String,
    raw_timing: String,
    raw_lines: Vec<String>,
}

impl TryFrom<&str> for RawSubtitleUnit {
    type Error = &'static str;

    // ["", "266", "00:18:25,437 --> 00:18:27,439", "そのための", "パラライザーだろうが！―"]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.trim().is_empty() {
            return Err("RawSubtitleUnit cannot be empty");
        } else if s.contains("\n\n") {
            return Err("RawSubtitleUnit cannot contain empty newline")
        }

        let s_split: Vec<&str> = s.split("\n").map(|s| s.trim()).collect();
        let s_split_cleaned: Vec<&str> = remove_empty_first_element(s_split);

        let raw_index = s_split_cleaned[0].to_string();
        let raw_timing = s_split_cleaned[1].to_string();
        let raw_lines = s_split_cleaned[2..].iter().map(|&s| s.to_string()).collect();

        Ok(RawSubtitleUnit { raw_index, raw_timing, raw_lines })
    }
}
