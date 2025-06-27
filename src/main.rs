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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mini_kana_mappings = ingest_mini_kana_mappings()?;
    // let test_string = convert_mini_kana_to_regular(&"こうなっちまったら最後―", &mini_kana_mappings);
    // let test_string_2 = "えっと… あの どうすれば？サイコパ99スを読み取る銃だ―australiaセーフティーが 解除される―…";
    //
    // let deduped_and_sorted: String = test_string_2
    //     .chars()
    //     .collect::<BTreeSet<_>>()
    //     .into_iter()
    //     .collect();
    //
    // println!("こうなっちまったら最後―");
    // println!("{test_string_2}");
    // println!("{deduped_and_sorted}");

    let raw_content: String = fs::read_to_string("data/raw/psycho-pass-s01e01-jp.srt")?;
    let normalised_raw_content: String = raw_content.replace("\r\n", "\n");

    let raw_content_split: Vec<&str> = split_into_raw_subtitle_units(&normalised_raw_content);
    let subtitles: Vec<&str> = raw_content_split
        .iter()
        .flat_map(|x| get_subtitles_from_unit(x))
        .collect();
    let subtitles_concat = subtitles.join("");

    helper_dedupe_and_sort(&subtitles_concat);

    Ok(())
}

fn split_into_raw_subtitle_units(raw: &str) -> Vec<&str> {
    raw.split("\n\n").collect()
}

fn get_subtitles_from_unit(subtitle_unit: &str) -> Vec<&str> {
    subtitle_unit.split('\n').skip(2).collect()
}
