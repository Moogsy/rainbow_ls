use std::fs::{DirEntry, ReadDir};
use std::path::PathBuf;
use std::io::Error;

use crate::types::Config;
pub fn display_path(config: &Config, path_buf: &PathBuf, read_dir: &Vec<Result<DirEntry, Error>>) {

    let title: &str = &path_buf.to_string_lossy();
    println!("{}", title);
    
    let mut entries: Vec<&DirEntry> = Vec::new();
    let mut errors: Vec<&Error> = Vec::new();

    for entry in read_dir {
        match entry {
            Ok(dir_entry) => {
                entries.push(dir_entry);
            },
            Err(error) => errors.push(error),
        }
    }
}