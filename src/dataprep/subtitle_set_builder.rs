#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::types::srt_index::{SrtIndex, SrtIndexError};
use crate::types::timestamp::{Timestamp, TimestampError};
use crate::types::timing::{Timing, TimingError};
use std::str::FromStr;

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
    pub index: SrtIndex,
    pub timing: Timing,
    pub lines: Vec<String>,
}

/// A full subtitle file, in its logical grouped form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubtitleSet {
    pub units: Vec<SubtitleUnit>,
}
