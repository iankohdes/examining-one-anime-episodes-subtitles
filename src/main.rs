#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mini_kana_mappings = ingest_mini_kana_mappings()?;
    let test_string = convert_mini_kana_to_regular(&"こうなっちまったら最後―", &mini_kana_mappings);

    println!("こうなっちまったら最後―");
    println!("{test_string}");

    Ok(())
}

/// Path to a JSON file containing the regular-to-small kana mappings for 10
/// different sounds: _a_, _i_, _u_, _e_, _o_, _tsu_, _ya_, _yu_, _yo_, and _wa_.
/// These sounds are represented by both **hiragana** and **katakana**
/// syllabaries.
const MINI_KANA_JSON_PATH: &'static str = "data/raw/mini_kana_mappings.json";

#[derive(Deserialize, Eq, PartialEq, Hash, Debug)]
/// Newtype representing the **keys** in `mini_kana_mappings.json`.
struct SmallKana(char);
/// Newtype representing the _values_ in `mini_kana_mappings.json`.
#[derive(Deserialize, Eq, PartialEq, Hash, Debug)]
struct RegularKana(char);

fn ingest_mini_kana_mappings() -> Result<HashMap<SmallKana, RegularKana>, Box<dyn std::error::Error>> {
    //! Takes no arguments and returns a `Result` of `HashMap<SmallKana,
    //! RegularKana>` in the successful case and `Box<dyn std::error::Error>`
    //! otherwise.
    //!
    //! The reason that `ingest_mini_kana_mappings` does not take an argument is
    //! that it has only one possible input value: the `MINI_KANA_JSON_PATH`.
    //!
    //! Usage example:
    //!
    //! ```
    //! fn main() -> Result<(), Box<dyn std::error::Error>> {
    //!     let mini_kana_mappings = ingest_mini_kana_mappings()?;
    //!     println!("{mini_kana_mappings:?}");
    //!     Ok(())
    //! }
    //! ```

    let io_file = File::open(MINI_KANA_JSON_PATH)?;
    let buffer: BufReader<File> = BufReader::new(io_file);
    let mini_kana_mappings: HashMap<SmallKana, RegularKana> = serde_json::from_reader(buffer)?;
    Ok(mini_kana_mappings)
}

fn convert_mini_kana_to_regular(
    input: &str,
    kana_mapping: &HashMap<SmallKana, RegularKana>
) -> String {
    //! **String-level** cleaning function that maps `mini_kana_character_to_regular`
    //! to every `char` in the input.
    //! 
    //! Use this together with the `ingest_mini_kana_mappings` function to create
    //! the hash map that will be used as the second argument. The hash map is
    //! also passed to `mini_kana_character_to_regular` as a parameter.
    
    let converted_input: String = input
        .chars()
        .map(|x| mini_kana_character_to_regular(&x, kana_mapping))
        .collect();

    converted_input
}

fn mini_kana_character_to_regular(
    input: &char,
    kana_mapping: &HashMap<SmallKana, RegularKana>
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
