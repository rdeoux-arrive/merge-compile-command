#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]

use log::{error, info};
use serde_json::{from_str, to_string_pretty, Value};
use simple_logger::SimpleLogger;
use std::env::current_dir;
use std::fs::{read_to_string, write};
use walkdir::WalkDir;

const FILENAME: &str = "compile_commands.json";

fn main() {
    SimpleLogger::new().env().init().ok();

    let mut cc = Vec::new();
    match current_dir() {
        Ok(path) => {
            let mut out = path.clone();
            out.push(FILENAME);
            for entry in WalkDir::new(&path).into_iter().filter_entry(|entry| {
                entry.file_type().is_dir()
                    || (entry.file_type().is_file()
                        && entry.file_name() == FILENAME
                        && entry.path() != out)
            }) {
                match entry {
                    Ok(entry) if entry.file_type().is_file() => {
                        let path = entry.path();
                        info!("loading {}", path.display());
                        match read_to_string(path) {
                            Ok(source) => match from_str(&source) {
                                Ok(Value::Array(mut array)) => cc.append(&mut array),
                                Ok(..) => error!("{} does not contains an array", path.display()),
                                Err(err) => {
                                    error!(
                                        "failed to parse the content of {}: {err}",
                                        path.display()
                                    );
                                }
                            },
                            Err(err) => error!("failed to read {}: {err}", path.display()),
                        }
                    }
                    Ok(..) => {}
                    Err(err) => error!("failed to walk in {}: {err}", path.display()),
                }
            }
        }
        Err(err) => error!("failed to get the current directory: {err}"),
    }

    if !cc.is_empty() {
        info!("saving {FILENAME}");
        match to_string_pretty(&cc) {
            Ok(out) => {
                if let Err(err) = write(FILENAME, out) {
                    error!("failed to write into compile_command.json: {err}");
                }
            }
            Err(err) => error!("failed to serialize: {err}"),
        }
    }
}
