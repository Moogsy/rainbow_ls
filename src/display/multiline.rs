use std::borrow::Cow;
use crate::types::{ColouredEntry, Config};

fn get_column_length(entries: &Vec<ColouredEntry>, num_columns: usize, column: usize) -> usize {
    let num_rows: usize = (entries.len() / num_columns) + 1;
    let mut column_length: usize = 0;
    
    let entry_iterator = entries.iter().skip(num_rows * column).take(num_rows);

    for entry in entry_iterator {
        column_length = entry.len().max(column_length);
    }

    column_length
}

fn get_column_lengths(config: &Config, entries: &Vec<ColouredEntry>) -> Vec<usize> {
    let mut best_column_lengths: Vec<usize> = Vec::new();

    // Wonder if there is some kind of formula that could help us 
    
    for num_columns in 1..entries.len() {
        let mut column_lengths: Vec<usize> = Vec::new();

        for column in 0..num_columns {
            let column_length: usize = get_column_length(&entries, num_columns, column);
            column_lengths.push(column_length);
        }

        let length_sum: usize = column_lengths.iter().sum();
        let total_sep_length: usize = config.separator.len() * (column_lengths.len() - 1);
        let total_length: usize = length_sum + total_sep_length;

        if total_length <= config.term_width.unwrap() && column_lengths.len() > best_column_lengths.len() {
            best_column_lengths = column_lengths;
        }
    }
    best_column_lengths
}

pub fn show(entries: Vec<ColouredEntry>, config: &Config) {

    let column_sizes = get_column_lengths(config, &entries);

    let num_columns: usize = column_sizes.len();
    let num_rows: usize = (entries.len() / num_columns) + 1;

    for index in 0..num_rows {

    let column_display_iterator= entries
        .iter()
        .skip(index)
        .step_by(num_rows)
        .zip(column_sizes.iter())
        .enumerate();

    let no_separator_index: usize = column_display_iterator.len().max(1) - 1;

    let lossy_sep: Cow<str>  = config.separator.to_string_lossy();
    let lossy_padding: Cow<str> = config.padding.to_string_lossy();

    for (inner_index, (entry, column_size)) in column_display_iterator {

        let lossy_name: Cow<str> = entry.formatted_name.to_string_lossy();
        print!("{}", lossy_name);

        let diff: usize = column_size - entry.len();
        for _ in 0..diff { 
            print!("{}", lossy_padding);
        }

        // There must be a way to directly get the index of that one
        if inner_index != no_separator_index {
            print!("{}", lossy_sep);
        }
    }
    println!();
    }
}

