use std::cmp::PartialEq;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use crate::dataprep::parser::{ParseResult, SubtitleParser, SubtitleParserError};
use crate::types::srt_index::{SrtIndex, SrtIndexError};
use crate::types::subtitle_unit::SubtitleUnit;
use crate::types::timing::{Timing, TimingError};

/// `Parser` is a finite state machine. Here is its state-transition table:
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
pub enum Parser {
    Empty,
    IndexOnly(SrtIndex),
    IndexAndTiming { index: SrtIndex, timing: Timing },
    Complete(SubtitleUnit),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserError {
    ReadError(String),
    IllegalStateAndInput(String),
    IndexParseError(SrtIndexError),
    TimingParseError(TimingError),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::ReadError(filename) => {
                write!(f, "Error while reading file: {filename}")
            }
            ParserError::IllegalStateAndInput(msg) => {
                write!(f, "Some illegal input received: {msg}")
            }
            ParserError::IndexParseError(msg) => {
                write!(f, "Incorrect format of subtitle index: {msg}")
            }
            ParserError::TimingParseError(msg) => {
                write!(f, "Incorrect format of timestamp(s): {msg}")
            }
        }
    }
}

impl Error for ParserError {}

impl From<SrtIndexError> for ParserError {
    fn from(err: SrtIndexError) -> Self {
        ParserError::IndexParseError(err)
    }
}

impl From<TimingError> for ParserError {
    fn from(err: TimingError) -> Self {
        ParserError::TimingParseError(err)
    }
}

impl Parser {
    pub fn parse(lines: Lines<BufReader<File>>) -> Result<Vec<SubtitleUnit>, ParserError> {
        let mut parsed_input = Vec::new();
        let mut state = Self::Empty;

        for line in lines {
            let unwrapped_line = match line {
                Ok(raw_content) => raw_content,
                Err(e) => return Err(ParserError::ReadError(String::from("Could not read content; please check file"))),
            };

            state = state.next_state(&mut parsed_input, &unwrapped_line)?;
        }

        // Relevant for the case where a subtitle file doesn’t end with a blank line
        match state {
            Self::Complete(subtitle_unit) => parsed_input.push(subtitle_unit),
            _ => {}
        }

        Ok(parsed_input)
    }

    fn next_state(self, accumulator: &mut Vec<SubtitleUnit>, raw_content: &String) -> Result<Self, ParserError> {
        match self {
            Parser::Empty => {
                if raw_content.is_empty() {
                    Ok(Self::Empty)
                } else {
                    let index = raw_content.parse::<SrtIndex>()?;
                    Ok(Self::IndexOnly(index))
                }
            }
            Parser::IndexOnly(index) => {
                let timing = raw_content.parse::<Timing>()?;
                Ok(Self::IndexAndTiming { index, timing })
            }
            Parser::IndexAndTiming { index, timing } => {
                // We do not test if `raw_content` can be parsed into an `SrtIndex`, because it’s very
                // possible that a subtitle contains just a number. If we are, however, successfully
                // able to parse `raw_content` into a `Timing` instance at this point, then this is
                // unexpected data and should be raised to the user. (It is reasonable to assume that
                // viewers don’t expect their subtitles to show timestamps.)
                if raw_content.parse::<Timing>().is_err() {
                    let mut subtitle_vec: Vec<String> = Vec::new();
                    subtitle_vec.push(raw_content.to_string());
                    Ok(Self::Complete(SubtitleUnit::new(index, timing, subtitle_vec)))
                } else {
                    Err(ParserError::IllegalStateAndInput(format!("Possible repetition of timestamps (unexpected input; if this is desired behaviour, add some text to string to make error go away): {raw_content}")))
                }
            }
            Parser::Complete(mut subtitle_unit) => {
                if raw_content.is_empty() {
                    accumulator.push(subtitle_unit);
                    Ok(Self::Empty)  // Reset condition
                } else if raw_content.parse::<Timing>().is_err() {
                    subtitle_unit.lines.push(raw_content.to_string());
                    Ok(Self::Complete(SubtitleUnit::new(subtitle_unit.index, subtitle_unit.timing, subtitle_unit.lines)))
                } else {
                    Err(ParserError::IllegalStateAndInput(format!("Possible repetition of timestamps (unexpected input; if this is desired behaviour, add some text to string to make error go away): {raw_content}")))
                }
            }
        }
    }
}