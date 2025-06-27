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

/// Newtype representing an unwanted character as uncovered by the application
/// of [`helper_dedupe_and_sort`].
#[derive(Deserialize, Debug)]
struct UnwantedChar(char);

const MINI_KANA_JSON_PATH: &'static str = "data/raw/mini_kana_mappings.json";
const UNWANTED_CHARACTERS_PATH: &'static str = "data/raw/unwanted_characters.txt";

// pub fn clean_subtitles(raw_input: &str) -> String {
//     let mini_kana_mappings: HashMap<SmallKana, RegularKana> =
//         ingest_json_file(MINI_KANA_JSON_PATH)?;
// 
//     let unwanted_characters_raw = fs::read_to_string(UNWANTED_CHARACTERS_PATH)?;
//     let unwanted_characters: HashSet<UnwantedChar> =
//         unwanted_characters_raw.collect();
// 
//     todo!()
// }

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

fn clean_at_character_level(
    input: &str,
    kana_mapping: &HashMap<SmallKana, RegularKana>,
) -> String {
    //! Applies **character-level** cleaning functions to a string of subtitle
    //! text. These are the two character-level cleaning operations:
    //!
    //! - Remove unwanted characters (punctuation, spaces, non-Japanese characters,
    //!   [_chōonpu_](https://en.wikipedia.org/wiki/Ch%C5%8Donpu))
    //! - Convert small kana (or [sokuon](https://en.wikipedia.org/wiki/Sokuon))
    //!   to their regular-sized variants

    let converted_input: String = input
        .chars()
        .map(|x| convert_mini_kana_to_regular(&x, kana_mapping))
        .collect();

    converted_input
}

fn convert_mini_kana_to_regular(
    input: &char,
    kana_mapping: &HashMap<SmallKana, RegularKana>,
) -> char {
    //! **Character-level** cleaning function that transforms a kana character
    //! into its regular size if it’s found to be a small version, and returns
    //! the input unchanged otherwise.
    //!
    //! Small kana versions indicate digraphs, lengthened vowels or the presence
    //! of double consonants.

    let typed_input = SmallKana(*input);

    let unwrapped_output: char = match kana_mapping.get(&typed_input) {
        Some(regular_kana) => regular_kana.0,
        None => typed_input.0
    };

    unwrapped_output
}