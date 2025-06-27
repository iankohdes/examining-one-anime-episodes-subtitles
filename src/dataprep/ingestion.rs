#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::BufReader;
use serde::de::DeserializeOwned;
use anyhow::Result;

pub fn ingest_json_file<T>(file_path: &'static str) -> Result<T>
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