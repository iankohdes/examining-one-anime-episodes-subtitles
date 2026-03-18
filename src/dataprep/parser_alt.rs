use crate::dataprep::parser::SubtitleParserError;
use crate::types::parse_state::{IndexAndTiming, IndexOnly, ParseState};
use crate::types::srt_index::{SrtIndex, SrtIndexError};
use crate::types::subtitle_unit::SubtitleUnit;
use crate::types::timing::{Timing, TimingError};
use std::cmp::PartialEq;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, PartialEq, Eq)]
struct SrtParser {
    output: Vec<SubtitleUnit>,
}

#[derive(Debug, PartialEq, Eq)]
enum SrtParserError {
    Io(String),
    SrtIndexError(SrtIndexError),
    TimingError(TimingError),
    SubtitleParserError(String),
}

impl SrtParser {
    pub fn parse(reader: BufReader<File>) -> Result<Vec<SubtitleUnit>, SrtParserError> {
        let output: Vec<SubtitleUnit> = Vec::new();
        let mut state: ParseState = ParseState::Empty;

        for line in reader.lines() {
            match &state {
                ParseState::Empty => {
                    let current_line = line.unwrap();

                    if current_line.is_empty() {
                        // Empty line ("")
                        // TODO: Shall we return an error here?
                        continue;
                    }

                    // Non-empty line
                    let srt_index = current_line.parse::<SrtIndex>().unwrap();
                    state = ParseState::IndexOnly(IndexOnly::new(srt_index));
                }
                ParseState::IndexOnly(current_state_idx) => {
                    let srt_index = current_state_idx.index;

                    let current_line = line.unwrap();
                    let timing = current_line.parse::<Timing>().unwrap();
                    state = ParseState::IndexAndTiming(IndexAndTiming::new(srt_index, timing));
                }
                ParseState::IndexAndTiming(index_and_timing) => todo!("Parse IndexAndTiming"),
                ParseState::SubtitleUnit(subtitle_unit) => todo!("Parse SubtitleUnit"),
            }
        }

        Ok(output)
    }
}
