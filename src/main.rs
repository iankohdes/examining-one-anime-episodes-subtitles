#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod dataprep;

use dataprep::cleaning::{clean_subtitles, helper_dedupe_and_sort};
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use crate::dataprep::ingestion::ingest_subtitle_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ingested_subtitles =
        ingest_subtitle_file("data/raw/psycho-pass-s01e01-jp.srt")?;
    let cleaned_subtitles = clean_subtitles(&ingested_subtitles)?;

    println!("{cleaned_subtitles}");

    Ok(())
}