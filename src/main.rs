use std::collections::{HashSet, VecDeque};
use std::ffi::OsString;
use std::fs::{self, DirEntry};
use std::io::Error;
use std::path::{Path, PathBuf};

mod display;
mod parser;
mod subparsers;
mod types;
use types::Config;

fn collect_entries(path: &Path) -> Vec<Result<DirEntry, Error>> {
    match fs::read_dir(path) {
        Ok(read_dir) => read_dir.collect(),
        Err(error) => vec![Err(error)],
    }
}

fn visit_directory(config: &Config, path: &PathBuf) -> Vec<Result<DirEntry, Error>> {
    let entries = collect_entries(path);
    display::display_path(config, path, &entries);
    entries
}

fn enqueue_children(
    config: &Config,
    entries: &[Result<DirEntry, Error>],
    stack: &mut VecDeque<PathBuf>,
    seen: &mut HashSet<OsString>,
) {
    for dir_entry in entries.iter().filter_map(|entry| entry.as_ref().ok()) {
        let path = dir_entry.path();
        let file_type = match dir_entry.file_type() {
            Ok(file_type) => file_type,
            Err(_) => continue,
        };

        if file_type.is_dir() {
            let os_name = path.as_os_str().to_os_string();
            if seen.insert(os_name) {
                stack.push_back(path);
            }
        } else if file_type.is_symlink() && config.follow_symlinks {
            let os_name = path.as_os_str().to_os_string();
            if seen.insert(os_name) {
                let target = fs::read_link(&path).unwrap_or_else(|_| path.clone());
                stack.push_back(target);
            }
        }
    }
}

fn call_recursive(config: &Config, paths: Vec<PathBuf>) {
    let mut stack: VecDeque<PathBuf> = paths.into_iter().collect();
    let mut seen: HashSet<OsString> = HashSet::new();

    while let Some(path_buf) = stack.pop_front() {
        let entries = visit_directory(config, &path_buf);
        enqueue_children(config, &entries, &mut stack, &mut seen);
    }
}

fn call_non_recursive(config: &Config, paths: Vec<PathBuf>) {
    for path_buf in paths {
        visit_directory(config, &path_buf);
    }
}

fn main() {
    let (config, paths): (Config, Vec<PathBuf>) = parser::parse_user_args();

    if config.recursive {
        call_recursive(&config, paths);
    } else {
        call_non_recursive(&config, paths);
    }
}
