use std::cmp;
use std::io;
use std::ffi;
use std::fs;

use term_size;

use super::filetype;
use crate::parser;

const SEP: &str = " ";
const SEP_LEN: usize = SEP.len();

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

fn get_column_length(entries: &Vec<filetype::Entry>, num_columns: usize, column: usize) -> usize {
    let num_rows: usize = entries.len() / num_columns + 1;
    let mut column_length: usize = 0;
    
    for entry in entries.iter().skip(num_rows * column).take(num_rows) {
        column_length = cmp::max(column_length, entry.name.len());
    }

    return column_length;
}

fn get_column_lengths(entries: &Vec<filetype::Entry>) -> Vec<usize> {
    let mut best = Vec::new();

    if let Some((width, _)) = term_size::dimensions() {
        for index in 1..entries.len() {
            let mut lengths = Vec::new();

            for column in 0..index {
                lengths.push(get_column_length(&entries, index, column));
            }

            let length_sum: usize = lengths.iter().sum();
            let total_sep_length: usize = SEP_LEN * (lengths.len() - 1);
 
            if length_sum + total_sep_length <= width && lengths.len() > best.len() {
                best = lengths;
            }

        }
    } else {
        eprintln!("Failed to get current term's size");
        best.push(get_column_length(&entries, 1, 0));
    }
    best
}

#[allow(unused_variables)]
fn show_entries(entries: Vec<filetype::Entry>, 
                  config: &parser::Config, 
                  column_sizes: Vec<usize>) {

    let num_columns = column_sizes.len();
    let num_rows = entries.len()/num_columns + 1;

    //println!("cols={}, rows={}", num_columns, num_rows);

    for index in 0..num_rows {

        for (entry, column_size) in entries.iter().skip(index).step_by(num_rows).zip(column_sizes.iter()) {

            let formatted_name: ffi::OsString = entry.get_formatted_name(config);
            
            let diff = column_size - entry.name.len();
            
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
    let column_lengths = get_column_lengths(&entries);

    //println!("entries={}, cols={:?}", entries.len(), column_lengths);
    show_entries(entries, config, column_lengths);
    
    for error in errors {
        println!("{}", error);
    }
}