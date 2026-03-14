use crate::types::srt_index::SrtIndex;
use crate::types::timing::Timing;

/// See [`SrtIndex`].
#[derive(Debug, Clone, PartialEq, Eq)]
struct IndexOnly {
    index: SrtIndex,
}

/// See [`Timing`].
#[derive(Debug, Clone, PartialEq, Eq)]
struct IndexAndTiming {
    index: SrtIndex,
    timing: Timing,
}

/// This type describes the incremental build of a [`SubtitleUnit`], starting
/// with [`IndexOnly`], [`IndexAndTiming`] and finally a `SubtitleUnit`.
#[derive(Debug, Clone, PartialEq, Eq)]
enum ParseState {
    IndexOnly,
    IndexAndTiming,
    SubtitleUnit,
}