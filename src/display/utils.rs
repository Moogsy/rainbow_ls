use std::borrow::Cow;
use std::env;
use std::fs::DirEntry;
use std::io::Error;
use std::path::PathBuf;

use crate::types::{Config, ColoredEntry, Kind, SortingReference};

pub fn  print_title(path_buf: &PathBuf) {
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

pub fn  divide_entries(read_dir: &Vec<Result<DirEntry, Error>>) -> (Vec<&DirEntry>, Vec<&Error>) {

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

pub fn  is_allowed_filename(config: &Config, lossy_file_name: Cow<str>) -> bool {

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

pub fn  sort_entries(config: &Config, mut entries: Vec<ColoredEntry>) -> Vec<ColoredEntry> {

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
            entries.sort_unstable_by_key(|entry| (sort_by(entry), entry.name.clone()))
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

