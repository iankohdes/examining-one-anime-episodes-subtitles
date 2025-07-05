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

const MINI_KANA_JSON_PATH: &str = "data/raw/mini_kana_mappings.json";
const UNWANTED_CHARACTERS_PATH: &str = "data/raw/unwanted_characters.txt";

pub fn clean_subtitles(raw_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    //! Cleans an ingested subtitle string in the following order:
    //!
    //! - Remove parentheses and their contents
    //! - Remove unwanted characters ([`remove_unwanted_characters`]; see also
    //!   [`helper_dedupe_and_sort`] for a definition of unwanted characters)
    //! - Convert mini-kana characters to their regular-sized counterparts
    //!   ([`convert_mini_kana_to_regular`])
    //!
    //! After this step, the output is ready for **subtitle processing**.

    let unwanted_characters_raw = fs::read_to_string(UNWANTED_CHARACTERS_PATH)?;
    let unwanted_characters: HashSet<char> =
        unwanted_characters_raw.chars().collect();

    let mini_kana_mappings: HashMap<SmallKana, RegularKana> =
        ingest_json_file(MINI_KANA_JSON_PATH)?;

    let parentheses_and_their_contents_removed: String =
        remove_parentheses_and_contents(raw_input);

    let unwanted_chars_removed_and_small_kana_as_regular: String =
        parentheses_and_their_contents_removed
            .chars()
            .filter(|x: &char| !unwanted_characters.contains(x))
            .map(|x: char| convert_mini_kana_to_regular(&x, &mini_kana_mappings))
            .collect();

    Ok(unwanted_chars_removed_and_small_kana_as_regular)
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
    //!
    //! ---
    //!
    //! Usage example:
    //!
    //! ```
    //! let test_string = "えっと… あの どうすれば？サイコパ99スを読み取る銃だ―australiaセーフティーが 解除される―…";
    //! let deduped_and_sorted: String = helper_dedupe_and_sort(test_string);
    //!
    //! println!("{test_string}");
    //! println!("{deduped_and_sorted}");  //  9ailrstu―…あうえがさすだっとどのばみるれをィイコサスセテパフー取解読銃除？
    //! ```

    let deduped_and_sorted: String = xs
        .chars()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    println!("{deduped_and_sorted}");
}

fn remove_parentheses_and_contents(input: &str) -> String {
    //! Removes parentheses in a string, along with all characters enclosed
    //! within. Works on both regular and full-width parentheses:
    //!
    //! - Regular parentheses are `(` and `)`
    //! - Full-width parentheses are `（` and `）` (used in Japanese)
    //!
    //! **Very important.** This function does _not_ check if there are equal
    //! numbers of opening and closing parentheses. Should there be an unequal
    //! number of opening and closing parenthesis symbols, the function will
    //! still return an output, albeit one that is incorrect.
    //!
    //! **_Always check the input string._**

    let mut result = String::new();
    let mut depth: u32 = 0;

    for char in input.chars() {
        match char {
            '(' | '（' => depth += 1,
            ')' | '）' => depth = depth.saturating_sub(1),
            _ if depth == 0 => result.push(char),
            _ => {}  // Reminder: returns unit type (i.e. does nothing)
        }
    }

    result
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