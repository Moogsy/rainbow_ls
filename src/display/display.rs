use std::fs;
use std::io;
use std::process;
use std::ffi;

use term_size;

use super::filetype;
use crate::parser;

const SEP: &str = " ";
const SEP_LEN: usize = SEP.len();


fn get_max_per_line(longest_name_len: usize) -> usize {
    if let Some((width, _)) = term_size::dimensions() {
        width / (longest_name_len + SEP_LEN)
    } else {
        eprintln!("Failed to get current term's size");
        process::exit(1);
    }
}
fn get_metrics(dir_entries: &Vec<filetype::Entry>) -> (usize, usize) {
    let mut total_len: usize = SEP_LEN * (dir_entries.len() - 1);
    let mut longest_name_len: usize = 0;
    for entry in dir_entries.iter() {
        let fn_len: usize = entry.name.len();
        total_len += fn_len;
        if longest_name_len < fn_len {
            longest_name_len = fn_len;
        }
    }
    (total_len, longest_name_len)
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

fn show_one_line(entries: Vec<filetype::Entry>, errors: Vec<io::Error>, config: &parser::Config) {
    let mapped: Vec<ffi::OsString> = entries.iter()
        .map(|entry| entry.get_formatted_name(0, config))
        .collect();

    
    for entry in entries {
        let formatted_name: ffi::OsString = entry.get_formatted_name(0, &config);
        print!("{:?}", formatted_name);


    }


}


/// Pretty prints out read_dirs 
pub fn show_read_dirs(config: parser::Config, passed_files: parser::PassedFiles) {

    for read_dir in passed_files.ok_dirs {
        let (entries, errors): (Vec<filetype::Entry>, Vec<io::Error>) = read_dirs_to_entry(read_dir);
        let (total_len, longest_name_length): (usize, usize) = get_metrics(&entries);



        

    }
}