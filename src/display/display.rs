use std::borrow::Cow;
use std::ffi::OsString;
use std::io::Error;
use std::fs::ReadDir;

use std::slice::Iter;
use std::iter::{Take, Skip, Enumerate, StepBy, Zip};

use super::filetype::Entry;
use crate::parser::Config;
use super::sort_by;


// Memes
type EntryDisplayIterator<'a> = Take<Skip<Iter<'a, Entry>>>;
type ColumnDisplayIterator<'a> = Enumerate<Zip<StepBy<Skip<Iter<'a, Entry>>>, Iter<'a, usize>>>;

fn handle_sorting_flags(config: &Config, mut entries: Vec<Entry>) -> Vec<Entry> {
    if config.group_directories_first {
        entries = sort_by::groupdirs(config, entries);
    } else {
        entries = sort_by::default(config, entries);
    };

    if config.reverse_file_order {
        entries.reverse()
    } 

    entries
}



fn read_dirs_to_entry(config: &Config, read_dir: ReadDir) -> (Vec<Entry>, Vec<Error>, usize) {
    let mut errors: Vec<Error> = Vec::new();
    let mut entries: Vec<Entry> = Vec::new();
    let mut total_length: usize = 0;

    for read_dir_entry in read_dir.into_iter() {
        
        match read_dir_entry {
            Ok(dir_entry) => {

                let name: OsString = dir_entry.file_name();

                if config.show_dotfiles {
                    let entry: Entry = Entry::new(config, name, dir_entry);
                    total_length += entry.len() + config.separator.len();
                    entries.push(entry);

                } else if !name.to_string_lossy().starts_with(".") {
                    let entry: Entry = Entry::new(config, name, dir_entry);
                    total_length += entry.len() + config.separator.len();
                    entries.push(entry);
                }
            },
            Err(error) => errors.push(error),
        }
    }

    entries = handle_sorting_flags(config, entries);

    (entries, errors, total_length)
}

fn get_column_length(entries: &Vec<Entry>, num_columns: usize, column: usize) -> usize {
    let num_rows: usize = (entries.len() / num_columns) + 1;
    let mut column_length: usize = 0;
    
    let entry_iterator: EntryDisplayIterator = entries.iter().skip(num_rows * column).take(num_rows);

    for entry in entry_iterator {
        column_length = entry.len().max(column_length);
    }

    column_length
}

fn get_column_lengths(config: &Config, entries: &Vec<Entry>) -> Vec<usize> {
    let mut best_column_lengths: Vec<usize> = Vec::new();

    // This can likely be optimized, 
    // but for now, let's get away with a working implementation
    
    for num_columns in 1..entries.len() {
        let mut column_lengths: Vec<usize> = Vec::new();

        for column in 0..num_columns {
            let column_length: usize = get_column_length(&entries, num_columns, column);
            column_lengths.push(column_length);
        }

        let length_sum: usize = column_lengths.iter().sum();
        let total_sep_length: usize = config.separator.len() * (column_lengths.len() - 1);
        let total_length: usize = length_sum + total_sep_length;

        if total_length <= config.term_width && column_lengths.len() > best_column_lengths.len() {
            best_column_lengths = column_lengths;
        }
    }
    best_column_lengths
}

fn show_multiline(entries: Vec<Entry>, 
                  config: &Config, 
                  column_sizes: Vec<usize>) {

    let num_columns: usize = column_sizes.len();
    let num_rows: usize = (entries.len() / num_columns) + 1;

    for index in 0..num_rows {

    let column_display_iterator: ColumnDisplayIterator = entries
        .iter()
        .skip(index)
        .step_by(num_rows)
        .zip(column_sizes.iter())
        .enumerate();

    let no_separator_index: usize = column_display_iterator.len().max(1) - 1;

    for (inner_index, (entry, column_size)) in column_display_iterator {

        let lossy_name: Cow<str> = entry.formatted_name.to_string_lossy();
        print!("{}", lossy_name);

        let diff: usize = column_size - entry.len();
        for _ in 0..diff { 
            print!("{}", config.padding);
        }

        // There must be a way to directly get the index of that one
        if inner_index != no_separator_index {
            print!("{}", config.separator);
        }
    }
    println!();
    }
}

/// Pretty prints out read_dirs 
pub fn read_dir(config: &Config, read_dir: ReadDir) {
    let (entries, errors, total_length): (Vec<Entry>, Vec<Error>, usize) = read_dirs_to_entry(config, read_dir);
    
    if total_length <= config.term_width {
        for entry in entries.iter().take(entries.len().max(1) - 1) {
            print!("{}{}", entry.formatted_name.to_string_lossy(), config.separator);
        }
        if let Some(entry) = entries.last() {
            print!("{}\n", entry.formatted_name.to_string_lossy())
        }
    } else { 
        let column_sizes: Vec<usize> = get_column_lengths(config, &entries);
        show_multiline(entries, config, column_sizes);
    }
        
    for error in errors {
        println!("{}", error);
    }
}