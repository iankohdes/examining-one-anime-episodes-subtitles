use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use crate::dataprep::ingestion::ingest_json_file;

/// Newtype representing the **keys** in the file accessed by [`MINI_KANA_JSON_PATH`].
#[derive(Deserialize, Eq, PartialEq, Hash, Debug)]
struct SmallKana(char);

/// Newtype representing the _values_ in the file accessed by [`MINI_KANA_JSON_PATH`].
#[derive(Deserialize, Eq, PartialEq, Hash, Debug)]
struct RegularKana(char);

const MINI_KANA_JSON_PATH: &'static str = "data/raw/mini_kana_mappings.json";
const UNWANTED_CHARACTERS_PATH: &'static str = "data/raw/unwanted_characters.txt";

pub fn clean_subtitles(raw_input: &str) -> Result<String, dyn std::error::Error> {
    let unwanted_characters_raw = fs::read_to_string(UNWANTED_CHARACTERS_PATH)?;
    let unwanted_characters: HashSet<char> =
        unwanted_characters_raw.chars().collect();

    let mini_kana_mappings: HashMap<SmallKana, RegularKana> =
        ingest_json_file(MINI_KANA_JSON_PATH)?;

    let output_value = clean_ingested_subtitles(raw_input, &unwanted_characters, &mini_kana_mappings);
    output_value;

    Ok(output_value)
}

pub fn helper_dedupe_and_sort(xs: &str) {
    //! Deduplicates a string and sorts its characters, then **prints the result**.
    //!
    //! Useful for identifying unwanted characters to add to a blacklist. Uses
    //! the `BTreeSet` data structure.
    //!
    //! Unwanted characters are defined as follows:
    //!
    //! - Punctuation and spaces
    //! - Latin or non-Japanese characters
    //! - [_Chōonpu_](https://en.wikipedia.org/wiki/Ch%C5%8Donpu) (a symbol that
    //!   indicates the prolonged sound of the katakana character immediately
    //!   preceding it)
    //!
    //! _Chōonpu_ usually come after katakana characters and rarely after
    //! hiragana ones. I remove them because they convey no additional
    //! information about existing characters in a subtitle unit.
    //!
    //! The point of sorting is to (hopefully!) cluster unwanted characters
    //! away from kanji, hiragana and katakana.

    let deduped_and_sorted: String = xs
        .chars()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    println!("{deduped_and_sorted}");
}

fn clean_ingested_subtitles(
    input: &str,
    char_blacklist: &HashSet<char>,
    kana_mapping: &HashMap<SmallKana, RegularKana>,
) -> String {
    let unwanted_characters_removed = remove_unwanted_characters(input, char_blacklist);

    let small_kana_converted_to_regular: String = input
        .chars()
        .map(|x| convert_mini_kana_to_regular(&x, kana_mapping))
        .collect();

    small_kana_converted_to_regular
}

fn remove_unwanted_characters(
    input: &str,
    char_blacklist: &HashSet<char>,
) -> String {
    //! Filters out characters in the file accessed by [`UNWANTED_CHARACTERS_PATH`].
    //!
    //! Unwanted characters include:
    //!
    //! - Punctuation and spaces
    //! - Numbers and non-Japanese characters
    //! - _Chōonpu_
    //!
    //! Apply this function **before** [`convert_mini_kana_to_regular`].

    input.chars().filter(|x| !char_blacklist.contains(x)).collect()
}

fn convert_mini_kana_to_regular(
    input: &char,
    kana_mapping: &HashMap<SmallKana, RegularKana>,
) -> char {
    //! Transforms a kana character into its regular size if it’s found to be a
    //! small version, and returns the input unchanged otherwise.
    //!
    //! Small kana versions indicate digraphs, lengthened vowels or the presence
    //! of double consonants.
    //!
    //! Apply this function **after** [`remove_unwanted_characters`].

    let typed_input = SmallKana(*input);

    let unwrapped_output: char = match kana_mapping.get(&typed_input) {
        Some(regular_kana) => regular_kana.0,
        None => typed_input.0
    };

    unwrapped_output
}