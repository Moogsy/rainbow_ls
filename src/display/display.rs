use std::io;
use std::process;
use std::ffi;
use std::fs;

use term_size;

use super::filetype;
use crate::parser;

const SEP: &str = " ";


fn get_max_column_per_line(longest_name_len: usize) -> usize {
    if let Some((width, _)) = term_size::dimensions() {
        width / (longest_name_len + 1)
    } else {
        eprintln!("Failed to get current term's size");
        process::exit(1);
    }
}
fn get_longest_name_len(dir_entries: &Vec<filetype::Entry>) -> usize {
    let mut longest_name_len: usize = 0;
    for entry in dir_entries.iter() {
        let filename_len: usize = entry.name.len();
        if longest_name_len < filename_len {
            longest_name_len = filename_len;
        }
    }
    longest_name_len
}

fn read_dirs_to_entry(read_dir: fs::ReadDir) -> (Vec<filetype::Entry>, Vec<io::Error>) {
    let mut errors: Vec<io::Error> = Vec::new();
    let mut entries: Vec<filetype::Entry> = Vec::new();

    for read_dir_entry in read_dir.into_iter() {
        match read_dir_entry {
            Ok(dir_entry) => {
                let entry: filetype::Entry = filetype::Entry::from(dir_entry);
                entries.push(entry);
            },
            Err(error) => errors.push(error),
        }
    }
    entries.sort();

    (entries, errors)
}

fn show_one_line(entries: Vec<filetype::Entry>, config: &parser::Config) {
    for entry in entries {
        let formatted_name: ffi::OsString = entry.get_formatted_name(&config);

        if let Some(name_str) = formatted_name.to_str() {
            print!("{}{}", name_str, SEP);
        } else {
            print!("{}{}", formatted_name.to_string_lossy(), SEP);
        }
    }
}

#[allow(unused_variables)]
fn show_multiline(entries: Vec<filetype::Entry>, 
                  config: &parser::Config, 
                  longest_name_len: usize,
                  max_column_per_line: usize) {

    for index in 0..4 {
        println!("Index={}", index);

        for entry in entries.iter().skip(index).step_by(4) {

            let formatted_name: ffi::OsString = entry.get_formatted_name(config);
            let diff: usize = longest_name_len - entry.name.len();
            
            if let Some(name_str) = formatted_name.to_str() {
                print!("{}{}", name_str, SEP);

            } else {
                let lossy_name = formatted_name.to_string_lossy();
                print!("{}{}", lossy_name, SEP);
            }
            for _ in 0..diff {
                print!(" ");
            }
        }
        println!();
    }
}

/// Pretty prints out read_dirs 
pub fn read_dir(config: &parser::Config, read_dir: fs::ReadDir) {
    let (entries, errors): (Vec<filetype::Entry>, Vec<io::Error>) = read_dirs_to_entry(read_dir);
    let longest_name_len = get_longest_name_len(&entries);
    let max_column_per_line: usize = get_max_column_per_line(longest_name_len);

    println!("max={}, len={}", max_column_per_line, entries.len());

    if entries.len() > max_column_per_line {
        show_multiline(entries, config, longest_name_len, max_column_per_line);
    } else {
        show_one_line(entries, config);
    }
    
    for error in errors {
        println!("{}", error);
    }
}