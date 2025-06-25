#![allow(dead_code)]

fn main() {
    todo!()
}

struct FilePath(String);

const fn create_filepath(path: String) -> FilePath {
    FilePath(path)
}

const MINI_KANA_JSON_PATH: FilePath = create_filepath(String::from("data/raw/mini_kana_mappings.json"));