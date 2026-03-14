use std::cmp::PartialEq;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::dataprep::parser::SubtitleParserError;
use crate::types::srt_index::{SrtIndex, SrtIndexError};
use crate::types::subtitle_unit::SubtitleUnit;
use crate::types::timing::{Timing, TimingError};
use crate::types::parse_state::{IndexAndTiming, IndexOnly, ParseState};

#[derive(Debug, Clone, PartialEq, Eq)]
struct SrtParser {
    output: Vec<SubtitleUnit>
}

#[derive(Debug, PartialEq, Eq)]
enum SrtParserError {
    Io(String),
    SrtIndexError(SrtIndexError),
    TimingError(TimingError),
    SubtitleParserError(String)
}

impl SrtParser {
    pub fn parse(reader: BufReader<File>) -> Result<Vec<SubtitleUnit>, SrtParserError> {
        let mut output: Vec<SubtitleUnit> = Vec::new();
        let mut current_state: ParseState = ParseState::Empty;

        let mut raw_iter = reader.lines();
        let mut current_line = raw_iter.next();

        loop {
            match &current_state {
                ParseState::Empty => {
                    if current_line.is_none() { return Ok(output)} else {
                        let srt_index = current_line.unwrap().unwrap().parse::<SrtIndex>().unwrap();
                        current_line = raw_iter.next();
                        current_state = ParseState::IndexOnly(IndexOnly::new(srt_index));
                    }
                }
                ParseState::IndexOnly(current_state) => {
                    let srt_index = current_state.index;
                    let timing = current_line.unwrap().unwrap().parse::<Timing>().unwrap();
                    current_line = raw_iter.next();
                    current_state = ParseState::IndexAndTiming(IndexAndTiming::new(srt_index, timing));
                }
            }
        }
    }
}