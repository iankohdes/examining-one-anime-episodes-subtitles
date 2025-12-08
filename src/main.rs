#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod dataprep;

use dataprep::cleaning::{clean_subtitles, helper_dedupe_and_sort};
use dataprep::ingestion::ingest_subtitle_file;
use serde::Deserialize;
use std::collections::{BTreeSet, HashMap};
use std::{env, fs};
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ingested_subtitles = ingest_subtitle_file("data/raw/psycho-pass-s01e01-jp.srt")?;
    let cleaned_subtitles = clean_subtitles(&ingested_subtitles)?;

    println!("{cleaned_subtitles}");

    Ok(())
}
