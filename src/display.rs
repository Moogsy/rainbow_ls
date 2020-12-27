use std::borrow::Cow;
use std::env;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::io::Error;
use std::process;
use std::ffi::OsString;

use crate::types::{ColoredEntry, Config, Kind, SortingReference};
use crate::utils;

fn print_title(path_buf: &PathBuf) {
    if let Ok(curr_dir) = env::current_dir() {

        if &curr_dir == path_buf { // nothing to do there
            return;
        }

        if let Ok(stripped) = path_buf.strip_prefix(curr_dir) {
            println!("{}", stripped.display())
        } else {
            println!("{}", path_buf.display())
        }
    } else {
        println!("{}", path_buf.display())
    }
}

fn divide_entries(read_dir: &Vec<Result<DirEntry, Error>>) -> (Vec<&DirEntry>, Vec<&Error>) {

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
    (entries, errors)
}

fn is_allowed_filename(config: &Config, lossy_file_name: Cow<str>) -> bool {

    if !config.show_dotfiles && lossy_file_name.starts_with('.') {
        return false;
    }
    if !config.show_backups && lossy_file_name.ends_with('~') {
        return false;
    }
    if let Some(include_pattern) = &config.include_pattern {
        return include_pattern.is_match(&lossy_file_name);
    }
    if let Some(exclude_pattern) = &config.exclude_pattern {
        return !exclude_pattern.is_match(&lossy_file_name);
    }

    true
}

fn sort_entries(config: &Config, mut entries: Vec<ColoredEntry>) -> Vec<ColoredEntry> {

    let sort_by  = |entry: &ColoredEntry| !(entry.kind == Kind::Directory && config.group_directories_first);

    match config.sort_by {
        SortingReference::AccessDate => {
            entries.sort_unstable_by_key(|entry| (sort_by(entry), entry.accessed_at))
        },
        SortingReference::Colour => {
            entries.sort_unstable_by_key(|entry| (sort_by(entry), entry.colour.as_tuple()))
        },
        SortingReference::CreationDate => {
            entries.sort_unstable_by_key(|entry| (sort_by(entry), entry.created_at))
        },
        SortingReference::Extension => {
            entries.sort_unstable_by_key(|entry| (sort_by(entry), entry.extension.clone()))
        },
        SortingReference::ModificationDate => {
            entries.sort_unstable_by_key(|entry| (sort_by(entry), entry.modified_at))
        },
        SortingReference::Name => { // newline to respect line length limit, feels less readable
            entries.sort_unstable_by_key(|entry| 
                (sort_by(entry), utils::starts_with_lowercase(&entry.name), entry.name.clone()))
        },
        SortingReference::Size => {
            entries.sort_unstable_by_key(|entry| (sort_by(entry), entry.size_bytes))
        },
        SortingReference::Default => {
            entries.sort_unstable();
        },
    }
    
    if config.reverse {
        entries.reverse()
    }

    entries
}

fn one_line_display(config: &Config, colored_entries: Vec<ColoredEntry>) {

    let lossy_sep: Cow<str> = config.separator.to_string_lossy();

    for entry in colored_entries {
        let lossy_name: Cow<str>  = entry.formatted_name.to_string_lossy();
        print!("{}{}", lossy_name, lossy_sep);
    }
    println!();
}

fn one_per_line_display(colored_entries: Vec<ColoredEntry>) {
    for entry in colored_entries {
        let lossy_name: Cow<str> = entry.formatted_name.to_string_lossy();
        println!("{}", lossy_name);
    }
}

#[allow(unused)]
fn multiline_display(config: &Config, colored_entries: Vec<ColoredEntry>, total_len: usize, term_width: usize) {
}

pub fn display_path(config: &Config, path_buf: &PathBuf, read_dir: &Vec<Result<DirEntry, Error>>) {


    let (entries, errors): (Vec<&DirEntry>, Vec<&Error>) = divide_entries(read_dir);

    let mut colored_entries: Vec<ColoredEntry> = Vec::new();
    let mut total_len: usize = 0;

    for dir_entry in entries {

        let file_name: OsString = dir_entry.file_name();
        let lossy_name: Cow<str> = file_name.to_string_lossy(); 

        if !is_allowed_filename(config, lossy_name) {
            continue;
        }

        let colored_entry: ColoredEntry = ColoredEntry::new(file_name, dir_entry, config);
        total_len += colored_entry.len();
        colored_entries.push(colored_entry);
    }
    
    colored_entries = sort_entries(config, colored_entries);

    print_title(path_buf);

    // Likely needs to be cleaned up
    if config.one_per_line {
        one_per_line_display(colored_entries);

    } else if let Some(term_width) = config.term_width {

        if total_len < term_width {
            one_line_display(config, colored_entries);
        } else {
            multiline_display(config, colored_entries, total_len, term_width);
        }

    } else {
        eprintln!("Failed to get terminal width, and none was specified either");
        process::exit(1);
    }

    for error in errors {
        println!("{}", error);
    }
}