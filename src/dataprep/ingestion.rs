#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fs;
use std::fs::File;
use std::io::BufReader;
use serde::de::DeserializeOwned;
use anyhow::Result;
use crate::dataprep::cleaning::clean_subtitles;

pub fn ingest_subtitle_file(filepath: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
    //! Ingests a single subtitle file (i.e. a text file with an `.srt` extension).
    //!
    //! Note that this function **assumes** that the subtitle file is in the
    //! correct format, and **does not** check for file correctness (in either its
    //! content or extension). Future iterations of this function may add this
    //! feature.

    let raw_content: String = fs::read_to_string(filepath)?;
    let normalised_raw_content: String = raw_content.replace("\r\n", "\n");

    let subtitle_units: Vec<&str> = normalised_raw_content.split("\n\n").collect();
    let subtitles: String = subtitle_units
        .iter()
        .flat_map(|x| get_subtitles_from_unit(x))
        .collect();

    Ok(subtitles)
}

pub fn ingest_json_file<T>(file_path: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    //! Takes a file path (with `.json` extension) and returns a deserialised
    //! value of type `T`. `T` must implement `DeserializeOwned` to be
    //! deserialisable from JSON.
    //!
    //! For all other file extensions, use `fs::read_to_string` (from `std::fs`).

    let io_file = File::open(file_path)?;
    let reader: BufReader<File> = BufReader::new(io_file);
    let data: T = serde_json::from_reader(reader)?;

    Ok(data)
}

fn get_subtitles_from_unit(subtitle_unit: &str) -> Vec<&str> {
    //! Apply this function to one subtitle unit. (A subtitle unit comprises
    //! an index, a pair of timestamps and one or more lines of subtitle text.)
    //!
    //! We skip two places because, in a subtitle unit that has been converted
    //! into an iterator, the subtitle text starts at the third element
    //! (i.e. after the index and timestamps).

    subtitle_unit.split('\n').skip(2).collect()
}