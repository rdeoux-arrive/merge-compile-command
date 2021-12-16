#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]

use json::{parse, stringify, Array, JsonValue};
use log::{error, info};
use simple_logger::SimpleLogger;
use std::env::current_dir;
use std::fs::{read_to_string, write};
use walkdir::WalkDir;

const FILENAME: &str = "compile_commands.json";

fn main() {
    SimpleLogger::new().env().init().ok();

    let mut cc = Array::new();
    match current_dir() {
        Ok(path) => {
            for entry in WalkDir::new(&path).into_iter().filter_entry(|entry| {
                entry.file_type().is_dir()
                    || (entry.file_type().is_file() && entry.file_name() == FILENAME)
            }) {
                match entry {
                    Ok(entry) if entry.file_type().is_file() => {
                        let path = entry.path();
                        info!("loading {:?}", path);
                        match read_to_string(&path) {
                            Ok(source) => match parse(&source) {
                                Ok(JsonValue::Array(mut array)) => cc.append(&mut array),
                                Ok(..) => error!("{:?} does not contains an array", path),
                                Err(err) => {
                                    error!("failed to parse the content of {:?}: {}", path, err);
                                }
                            },
                            Err(err) => error!("failed to read {:?}: {}", path, err),
                        }
                    }
                    Ok(..) => {}
                    Err(err) => error!("failed to walk in {:?}: {}", path, err),
                }
            }
        }
        Err(err) => error!("failed to get the current directory: {}", err),
    }

    if !cc.is_empty() {
        info!("saving {:?}", FILENAME);
        if let Err(err) = write(FILENAME, stringify(cc)) {
            error!("failed to write into compile_command.json: {}", err);
        }
    }
}
