#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let io_test_input = File::open(String::from("data/raw/mini_kana_mappings.json"))?;
    let test_buffer: BufReader<File> = BufReader::new(io_test_input);
    let test_hash_map: HashMap<SmallKana, RegularKana> = serde_json::from_reader(test_buffer)?;

    let test_string = convert_mini_kana_to_regular(&"こうなっちまったら最後―", &test_hash_map);
    println!("こうなっちまったら最後―");
    println!("{test_string}");

    Ok(())
}

const MINI_KANA_JSON_PATH: &'static str = "data/raw/mini_kana_mappings.json";

#[derive(Deserialize, Eq, PartialEq, Hash, Debug)]
struct SmallKana(char);
#[derive(Deserialize, Eq, PartialEq, Hash, Debug)]
struct RegularKana(char);

fn ingest_mini_kana_mappings() -> Result<HashMap<SmallKana, RegularKana>, Box<dyn std::error::Error>> {
    let io_file = File::open(MINI_KANA_JSON_PATH)?;
    let buffer: BufReader<File> = BufReader::new(io_file);
    let mini_kana_mappings: HashMap<SmallKana, RegularKana> = serde_json::from_reader(buffer)?;
    Ok(mini_kana_mappings)
}

fn convert_mini_kana_to_regular(
    input: &str,
    kana_mapping: &HashMap<SmallKana, RegularKana>
) -> String {
    /*
    [DONE] First, there should be a function that operates on the character level. It takes
    one character, checks to see if it’s in the kana_mapping hash map, then returns
    some output.

    [DONE] Next, there should be a function that maps mini_kana_character_to_regular to every
    character in the input string, converting small kana to regular.
    
    Finally, move the code over to cleaning.rs.
     */
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
    let typed_input = SmallKana(*input);

    let unwrapped_output: char = match kana_mapping.get(&typed_input) {
        Some(regular_kana) => regular_kana.0,
        None => typed_input.0
    };

    unwrapped_output
}
