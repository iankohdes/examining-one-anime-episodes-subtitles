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

fn remove_empty_first_element(input: Vec<&str>) -> Vec<&str> {
    if input[0] == "" {
        input[1..].to_vec()
    } else {
        input
    }
}
