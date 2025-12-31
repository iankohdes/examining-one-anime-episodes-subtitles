#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod dataprep;
mod types;

use dataprep::cleaning::{clean_subtitles, helper_dedupe_and_sort};
use dataprep::parser::SubtitleParser;
use serde::Deserialize;
use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use std::{env, fs};

use crate::dataprep::ingestion::SafeFilePath;
use crate::dataprep::subtitle_set_builder::SubtitleUnit;
use crate::types::srt_index::SrtIndex;
use crate::types::timing::Timing;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We need this SafeFilePath -> File -> BufReader chain because:
    // 1. SafeFilePath validates the path (correct extension, no path traversal, file exists)
    // 2. File::open gives us the file handle
    // 3. BufReader wraps it to provide buffered reading (implements BufRead trait)
    //
    // The parser's parse() method takes anything that implements BufRead, which is why
    // we need the BufReader wrapper. This design keeps the parser flexible: it can work
    // with files, strings wrapped in Cursor, network streams, or any BufRead source.
    //
    // Note: SubtitleParser has stubbed convenience methods parse_file() and parse_str()
    // that will handle this setup internally once implemented. You can give it a shot!
    let filepath = "data/raw/psycho-pass-s01e01-jp.srt";
    let checked_path_result = SafeFilePath::try_from(filepath)?;
    let file = File::open(checked_path_result)?;
    let reader = BufReader::new(file);

    let mut parser = SubtitleParser::new();

    // The parser takes a BufRead and returns an iterator over Result<SubtitleUnit, Error>.
    // This is the lazy approach - parse units one at a time rather than loading everything
    // into memory. Once you implement the parse method, you can iterate like this:
    //
    // for unit_result in parser.parse(reader)? {
    //     match unit_result {
    //         Ok(subtitle_unit) => println!("{:?}", subtitle_unit),
    //         Err(e) => eprintln!("Error: {}", e),
    //     }
    // }
    //
    // Or collect eagerly: let units: Result<Vec<_>, _> = parser.parse(reader)?.collect();
    //
    // When we expose this to Python via PyO3, the iterator maps nicely to Python
    // generators. You'll use #[pyclass] and implement __iter__/__next__ so Python code
    // can do: for unit in subtitle_parser.parse(content): ...
    // And of course it also will work with a Polars DataFrame, because we can
    // convert the Vec<SubtitleUnit> into a DataFrame easily or even stream rows.

    Ok(())
}
