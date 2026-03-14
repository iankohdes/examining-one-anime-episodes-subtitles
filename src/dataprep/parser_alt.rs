use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use crate::dataprep::parser::SubtitleParserError;
use crate::types::srt_index::SrtIndexError;
use crate::types::timing::TimingError;

struct SrtParser {
    parse_object: BufReader<File>
}

enum SrtParserError {
    Io(std::io::Error),
    SrtIndexError(SrtIndexError),
    TimingError(TimingError),
    SubtitleParserError(String)
}