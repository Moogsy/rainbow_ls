use std::collections::{HashSet, VecDeque};
use std::io::Error;
use std::fs::{self, DirEntry};
use std::path::PathBuf;
use std::ffi::OsString;

mod display;
mod parser;
mod subparsers;
mod types;
use types::Config;
mod utils;

// Some prototyping, will fix "later"
fn call_recursive(config: Config, paths: Vec<PathBuf>) {

    let mut stack: VecDeque<PathBuf> = paths.into_iter().collect();
    let mut seen: HashSet<OsString> = HashSet::new();  // circular symlinks

    while let Some(path_buf) = stack.pop_front() {
        for read_dir in path_buf.read_dir() {

            let collected_read_dir: Vec<Result<DirEntry, Error>> = read_dir.into_iter().collect();
            display::display_path(&config, &path_buf, &collected_read_dir); 

            for dir_entry in collected_read_dir.into_iter().filter_map(Result::ok) {

                if let Ok(metadata) = dir_entry.metadata() {

                    let path: PathBuf = dir_entry.path();
                    let os_str_name: OsString = path.as_os_str().to_os_string();

                    if seen.contains(&os_str_name) {
                        continue; 
                    }
                    if metadata.file_type().is_dir() {
                        seen.insert(os_str_name);
                        stack.push_back(path);

                    } else if metadata.file_type().is_symlink() && config.follow_symlinks {

                        seen.insert(os_str_name);

                        if let Ok(link) = fs::read_link(&path) {
                            stack.push_back(link);
                        } else {
                            stack.push_back(path);
                        }
                    }
                }
            }
        }
    }
}


fn call_non_recursive(config: Config, paths: Vec<PathBuf>) {
    for path_buf in paths { 
        for read_dir in path_buf.read_dir() {
            let collected_read_dir: Vec<Result<DirEntry, Error>> = read_dir.into_iter().collect();
            display::display_path(&config, &path_buf, &collected_read_dir);
        }
    }
}



fn main() {
    let (config, paths): (Config, Vec<PathBuf>) = parser::parse_user_args();
    
    if config.recursive {
        call_recursive(config, paths);
    } else {
        call_non_recursive(config, paths);
    }
}


