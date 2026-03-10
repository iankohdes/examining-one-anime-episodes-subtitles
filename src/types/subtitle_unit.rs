use crate::types::srt_index::SrtIndex;
use crate::types::timing::Timing;

/// A `SubtitleUnit` represents one complete group of lines in an SRT file,
/// where groups of lines are separated by blank lines in well-formed SRT
/// files. Each group has varying numbers of lines, but the first two lines
/// must always be the index ([`SrtIndex`]) and timing ([`Timing`])
/// respectively. All successive lines are the subtitle content.
///
/// An SRT file, after parsing, consists of a collection of `SubtitleUnit`
/// elements. See below an example of a subtitle block.
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
    pub lines: Vec<String>
}