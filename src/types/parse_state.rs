use crate::types::srt_index::SrtIndex;
use crate::types::timing::Timing;

/// See [`SrtIndex`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexOnly {
    index: SrtIndex,
}

/// See [`Timing`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexAndTiming {
    index: SrtIndex,
    timing: Timing,
}

/// This type describes the incremental build of a [`SubtitleUnit`], starting
/// with `Empty`, [`IndexOnly`], [`IndexAndTiming`] and finally a `SubtitleUnit`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseState {
    Empty,
    IndexOnly,
    IndexAndTiming,
    SubtitleUnit,
}