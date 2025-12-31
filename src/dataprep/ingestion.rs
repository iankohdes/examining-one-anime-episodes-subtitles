#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use anyhow::Result;
use serde::de::DeserializeOwned;
use serde_json::from_reader;
use std::convert::TryFrom;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

const PATH_CHAR_WHITELIST: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz01234567890-./";

#[derive(Debug)]
pub enum PathError {
    EmptyPath,
    FileNotFound,
    IllegalCharacters,
    IncorrectExtension,
}

impl Display for PathError {
    // The `fmt` function doesn’t actually get used when I run the code. I have
    // to implement it, however, otherwise I can’t use the `?` operator to
    // unwrap the output of `SafeFilePath::try_from`.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PathError::EmptyPath => write!(f, "File path is empty"),
            PathError::FileNotFound => write!(f, "File not found"),
            PathError::IllegalCharacters => write!(
                f,
                "Characters must be:\n ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz01234567890-./"
            ),
            PathError::IncorrectExtension => write!(f, "File extension is not .srt"),
        }
    }
}

impl Error for PathError {}

#[derive(Debug)]
pub struct SafeFilePath {
    get_path: PathBuf,
}

// The `TryFrom` trait has only one method to implement. See here for documentation:
// https://doc.rust-lang.org/std/convert/trait.TryFrom.html
impl TryFrom<&str> for SafeFilePath {
    type Error = PathError;

    /// Returns the **absolute** file path wrapped in a `Result` type.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let path = PathBuf::from(value);

        if value.trim().is_empty() {
            return Err(PathError::EmptyPath);
        } else if value.chars().all(|char| PATH_CHAR_WHITELIST.contains(char)) == false {
            return Err(PathError::IllegalCharacters);
        } else if path.extension() != Some("srt".as_ref()) {
            return Err(PathError::IncorrectExtension);
        };

        let result: Result<SafeFilePath, PathError> = match fs::canonicalize(&path) {
            Ok(absolute_path) => Ok(SafeFilePath {
                get_path: absolute_path,
            }),
            Err(_) => Err(PathError::FileNotFound),
        };

        result
    }
}

// This is needed to enable the usage of fs::read_to_string() on SafeFilePath.
impl AsRef<Path> for SafeFilePath {
    fn as_ref(&self) -> &Path {
        &self.get_path
    }
}

pub fn ingest_json_file<T>(file_path: &str) -> Result<T>
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
    let data: T = from_reader(reader)?;

    Ok(data)
}
