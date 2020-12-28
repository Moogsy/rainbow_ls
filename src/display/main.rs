use std::borrow::Cow;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::io::Error;
use std::process;
use std::ffi::OsString;

use crate::types::{ColoredEntry, Config};

use super::{long_listing, multiline, one_line, one_per_line, utils};

pub fn display_path(config: &Config, path_buf: &PathBuf, read_dir: &Vec<Result<DirEntry, Error>>) {

    utils::print_title(path_buf);

    let (entries, errors): (Vec<&DirEntry>, Vec<&Error>) = utils::divide_entries(read_dir);

    let mut colored_entries: Vec<ColoredEntry> = Vec::new();
    let mut total_len: usize = 0;

    for dir_entry in entries {

        let file_name: OsString = dir_entry.file_name();
        let lossy_file_name: Cow<str> = file_name.to_string_lossy(); 

        if !utils::is_allowed_filename(config, lossy_file_name) {
            continue;
        }

        let colored_entry: ColoredEntry = ColoredEntry::new(file_name, dir_entry, config);
        total_len += colored_entry.len();
        colored_entries.push(colored_entry);
    }
    
    colored_entries = utils::sort_entries(config, colored_entries);

    if config.one_per_line {
        one_per_line::show(colored_entries);
    } else if config.is_long_listing  {
        long_listing::show(colored_entries, config);
    } else if let Some(term_width) = config.term_width {
        if total_len < term_width {
            one_line::show(colored_entries, config);
        } else {
            multiline::show(colored_entries, config);
        }
    } else {
        eprintln!("Failed to get terminal size and none was provided either.");
        process::exit(1);
    }

    for error in errors {
        println!("{}", error);
    }
}
