use crate::dataprep::parser::SubtitleParser;
use crate::types::srt_index::SrtIndex;
use crate::types::subtitle_unit::SubtitleUnit;
use crate::types::timing::Timing;

/// See [`SrtIndex`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexOnly {
    pub index: SrtIndex,
}

impl IndexOnly {
    pub fn new(index: SrtIndex) -> Self {
        IndexOnly { index }
    }
}

/// See [`Timing`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexAndTiming {
    pub index: SrtIndex,
    pub timing: Timing,
}

impl IndexAndTiming {
    pub fn new(index: SrtIndex, timing: Timing) -> Self {
        IndexAndTiming { index, timing }
    }
}

/// This type describes the incremental build of a [`SubtitleUnit`], starting
/// with `Empty`, [`IndexOnly`], [`IndexAndTiming`] and finally a `SubtitleUnit`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseState {
    Empty,
    IndexOnly(IndexOnly),
    IndexAndTiming(IndexAndTiming),
    SubtitleUnit(SubtitleUnit),
}