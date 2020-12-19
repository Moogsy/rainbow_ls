use std::borrow;
use std::slice;
use std::io;
use std::ffi;
use std::fs;
use std::iter;

use super::filetype;
use crate::parser;

// It was html all along :)

type ColumnDisplayIterator<'a> = 

iter::Enumerate<
    iter::Zip<
        iter::StepBy<
            iter::Skip<
                slice::Iter<'a, filetype::Entry>
            >
        >, 
        slice::Iter<'a, usize>
    >
>;


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
    let num_rows: usize = (entries.len() / num_columns) + 1;
    let mut column_length: usize = 0;
    
    for entry in entries.iter().skip(num_rows * column).take(num_rows) {
        column_length = entry.name.len().max(column_length);
    }

    column_length
}

fn get_column_lengths(config: &parser::Config, entries: &Vec<filetype::Entry>) -> Vec<usize> {
    let mut best_column_lengths: Vec<usize> = Vec::new();

    if let Some(width) = config.term_width {
        for num_columns in 1..entries.len() {
            let mut column_lengths: Vec<usize> = Vec::new();

            for column in 0..num_columns {
                column_lengths.push(get_column_length(&entries, num_columns, column));
            }

            let length_sum: usize = column_lengths.iter().sum();
            let total_sep_length: usize = config.separator.len() * (column_lengths.len() - 1);
            let total_length: usize = length_sum + total_sep_length;
 
            if total_length <= width && column_lengths.len() > best_column_lengths.len() {
                best_column_lengths = column_lengths;
            }
        }
    } else {
        eprintln!("Failed to get current term's size");
        best_column_lengths.push(get_column_length(&entries, 1, 0));
    }
    best_column_lengths
}

fn show_entries(entries: Vec<filetype::Entry>, 
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


        let no_separator_index: usize = column_display_iterator.len() - 1;

        for (inner_index, (entry, column_size)) in column_display_iterator {

            let formatted_name: ffi::OsString = entry.get_formatted_name(config);
            let diff: usize = column_size - entry.name.len();
            
            if let Some(name_str) = formatted_name.to_str() {
                print!("{}", name_str);

            } else {
                let lossy_name: borrow::Cow<str> = formatted_name.to_string_lossy();
                print!("{}", lossy_name);
            }

            for _ in 0..diff { 
                print!("{}", config.padding);
            }

            if inner_index != no_separator_index {
                print!("{}", config.separator);
            }
        }
        println!();
    }
}

/// Pretty prints out read_dirs 
pub fn read_dir(config: &parser::Config, read_dir: fs::ReadDir) {
    let (entries, errors): (Vec<filetype::Entry>, Vec<io::Error>) = read_dirs_to_entry(read_dir);
    let column_sizes: Vec<usize> = get_column_lengths(config, &entries);

    show_entries(entries, config, column_sizes);
    
    for error in errors {
        println!("{}", error);
    }
}