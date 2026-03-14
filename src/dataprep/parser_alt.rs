use std::cmp::PartialEq;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::dataprep::parser::SubtitleParserError;
use crate::types::srt_index::{SrtIndex, SrtIndexError};
use crate::types::subtitle_unit::SubtitleUnit;
use crate::types::timing::TimingError;
use crate::types::parse_state::ParseState;

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
            if current_line.is_none() {
                return Ok(output);
            } else if current_line.unwrap().unwrap() == "" && current_state == ParseState::Empty {
                current_line = raw_iter.next();
            } else if current_line.is_some() && current_state == ParseState::Empty {
                let srt_index = current_line.unwrap().unwrap().parse::<SrtIndex>().unwrap();
            }
        }
    }
}