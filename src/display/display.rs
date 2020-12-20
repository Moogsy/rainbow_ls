use std::slice;
use std::borrow;
use std::ffi;
use std::io;
use std::fs;
use std::iter;

use super::filetype;
use crate::parser;


// Memes
type EntryDisplayIterator<'a> = iter::Take<iter::Skip<slice::Iter<'a, filetype::Entry>>>;
type ColumnDisplayIterator<'a> = 
iter::Enumerate<iter::Zip<iter::StepBy<iter::Skip<slice::Iter<'a, filetype::Entry>>>, slice::Iter<'a, usize>>>;

fn handle_sorting_flags(config: &parser::Config, mut entries: Vec<filetype::Entry>) -> Vec<filetype::Entry> {
    entries.sort();

    if config.reverse_file_order {
        entries.reverse()
    } 

    entries
}



fn read_dirs_to_entry(config: &parser::Config, read_dir: fs::ReadDir) -> (Vec<filetype::Entry>, Vec<io::Error>, usize) {
    let mut errors: Vec<io::Error> = Vec::new();
    let mut entries: Vec<filetype::Entry> = Vec::new();
    let mut total_length: usize = 0;

    for read_dir_entry in read_dir.into_iter() {
        
        match read_dir_entry {
            Ok(dir_entry) => {

                let name: ffi::OsString = dir_entry.file_name();

                if config.show_dotfiles {
                    let entry: filetype::Entry = filetype::Entry::new(config, name, dir_entry);
                    total_length += entry.len() + config.separator.len();
                    entries.push(entry);

                } else if !name.to_string_lossy().starts_with(".") {
                    let entry: filetype::Entry = filetype::Entry::new(config, name, dir_entry);
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

fn get_column_length(entries: &Vec<filetype::Entry>, num_columns: usize, column: usize) -> usize {
    let num_rows: usize = (entries.len() / num_columns) + 1;
    let mut column_length: usize = 0;
    
    let entry_iterator: EntryDisplayIterator = entries.iter().skip(num_rows * column).take(num_rows);

    for entry in entry_iterator {
        column_length = entry.len().max(column_length);
    }

    column_length
}

fn get_column_lengths(config: &parser::Config, entries: &Vec<filetype::Entry>) -> Vec<usize> {
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

fn show_multiline(entries: Vec<filetype::Entry>, 
                  config: &parser::Config, 
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

        let lossy_name: borrow::Cow<str> = entry.formatted_name.to_string_lossy();
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
pub fn read_dir(config: &parser::Config, read_dir: fs::ReadDir) {
    let (entries, errors, total_length): (Vec<filetype::Entry>, Vec<io::Error>, usize) = read_dirs_to_entry(config, read_dir);
    
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