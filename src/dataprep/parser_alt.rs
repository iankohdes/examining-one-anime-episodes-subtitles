use std::cmp::PartialEq;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::dataprep::parser::{SubtitleParser, SubtitleParserError};
use crate::types::srt_index::{SrtIndex, SrtIndexError};
use crate::types::subtitle_unit::SubtitleUnit;
use crate::types::timing::{Timing, TimingError};
use crate::types::parse_state::{IndexAndTiming, IndexOnly, ParseState};

/// `Parser` is a state machine. Here is its state-transition table:
///
/// | Current state | Input: *digit* | Input: *timestamps* | Input: *text* | Input: *blank line* | Input: *no more lines* |
/// | ------------- | -------------- | ------------------- | ------------- | ------------------- | ---------------------- |
/// | *Empty*       | *Index*        | error               | error         | error               | error                  |
/// | *Index*       | error          | *Timing*            | error         | error               | error                  |
/// | *Timing*      | error          | error               | *Subtitle*    | error               | error                  |
/// | *Subtitle*    | error          | error               | *Subtitle*    | *Empty*             | *End*                  |
///
/// To see the state machine’s visual representation, paste the following syntax in the [Mermaid live editor](https://mermaid.live/edit):
///
/// ```mermaid
/// graph LR
/// 	a((Start)) --> b((Empty))
/// 	b -->|Digit| c((Index))
/// 	c -->|Timestamps| d((Timing))
/// 	d -->|Text| e((Subtitle))
/// 	e -->|Text| e
/// 	e -->|Blank line| b((Empty))
/// 	e -->|No more lines| f((End))
/// ```
///
/// I may install the appropriate crate for the Mermaid diagram at a later time, so that it is directly
/// viewable in the Cargo-generated documentation.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Parser {
    Empty,
    IndexOnly(SrtIndex),
    IndexAndTiming { index: SrtIndex, timing: Timing },
    Complete(SubtitleUnit),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParserError {
    EmptyFile,
    ReadError(String),
    IllegalStateAndInput(String),
    IndexParseError(SrtIndexError),
    TimingParseError(TimingError),
}

impl Parser {
    pub fn parse(reader: BufReader<File>) -> Result<Vec<SubtitleUnit>, ParserError> {
        let parsed_input: Vec<SubtitleUnit> = Vec::new();
        let lines = reader.lines();
        let mut state = Self::Empty;

        for line in lines {
            let unwrapped_line = match line {
                Ok(raw_content) => raw_content,
                Err(e) => return Err(ParserError::ReadError(String::from("Could not read content; please check file"))),
            };

            if unwrapped_line.is_empty() { return Err(ParserError::EmptyFile); }

            todo!("Implement parsing logic using the transition method defined below")
        }

        Ok(parsed_input)
    }

    // Actually, I only need one function to do all the work of the below three! I can call it `.next_state()`.
    fn transition_to_index_only(self, raw_content: &String) -> Result<Self, ParserError> {
        // To ask: why am I using `self` instead of `&self`?
        todo!("Implement transition to index; note that `IndexOnly` can hold a value, so I don’t have to return `SrtIndex` explicitly")
    }

    fn transition_to_index_and_timing(self, raw_content: &String) -> Result<Self, ParserError> {
        todo!("Implement transition to `IndexAndTiming`")
    }

    fn transition_to_complete(self, raw_content: &String) -> Result<Self, ParserError> {
        todo!("Implement transition to `Complete`")
    }
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct SrtParser {
//     output: Vec<SubtitleUnit>
// }
//
// #[derive(Debug, PartialEq, Eq)]
// enum SrtParserError {
//     Io(String),
//     SrtIndexError(SrtIndexError),
//     TimingError(TimingError),
//     SubtitleParserError(String)
// }
//
// impl SrtParser {
//     pub fn parse(reader: BufReader<File>) -> Result<Vec<SubtitleUnit>, SrtParserError> {
//         let mut output: Vec<SubtitleUnit> = Vec::new();
//         let mut current_state: ParseState = ParseState::Empty;
//
//         let mut raw_iter = reader.lines();
//         let mut current_line = raw_iter.next();
//
//         loop {
//             match &current_state {
//                 ParseState::Empty => {
//                     if current_line.is_none() {
//                         return Ok(output)
//                     } else if current_line.unwrap().unwrap().is_empty() {
//                         current_line = raw_iter.next();
//                     } else {
//                         let srt_index = current_line.unwrap().unwrap().parse::<SrtIndex>().unwrap();
//                         current_line = raw_iter.next();
//                         current_state = ParseState::IndexOnly(IndexOnly::new(srt_index));
//                     }
//                 }
//                 ParseState::IndexOnly(current_state) => {
//                     let srt_index = current_state.index;
//                     let timing = current_line.unwrap().unwrap().parse::<Timing>().unwrap();
//                     current_line = raw_iter.next();
//                     current_state = ParseState::IndexAndTiming(IndexAndTiming::new(srt_index, timing));
//                 }
//             }
//         }
//     }
// }