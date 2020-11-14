use std::env;
use std::path;

const VALID_ESCAPE_DIGITS: [u8; 8] = [0, 1, 2, 3, 4, 5, 7, 8];

/// Tries to parse the escape digits
/// Panics if it fails to do so or if an invalid digit was passed
pub fn formatting_args(curr_config: &mut Vec<u8>, right_arg: String) {
    curr_config.clear(); 
    for letter in right_arg.chars() {
        let digit: u8 = letter
            .to_digit(10)
            .unwrap_or_else(|| panic!("Failed to convert {} to a number", letter))
            as u8;
        
        if curr_config.contains(&digit) {
            panic!("Found duplicate escape digit: {}", digit);

        } else if VALID_ESCAPE_DIGITS.contains(&digit) {
            curr_config.push(digit);
        } else {
            panic!("Not a valid escape digit contained in {:?}", VALID_ESCAPE_DIGITS)
        }
    }
}
pub fn minimal_sum(curr_config: &mut u16, right_arg: String) {
    let min_sum: u16 = right_arg.trim().parse().expect("Sum must be a positive number");

    if min_sum < 765 {
        panic!("Expected sum to be lower than 765, got {}", min_sum);
    } else {
        *curr_config = min_sum;
    }
}
pub fn consume_rest(untreated_args: &mut Vec<String>, args_iterator: &mut env::Args) {
    for arg in args_iterator {
        untreated_args.push(arg);
    }
}

fn is_readable_dir(path: String) -> Result<path::PathBuf, path::PathBuf> {
    let pathbuf: path::PathBuf = path::PathBuf::from(path);

    if pathbuf.is_dir() && pathbuf.read_dir().is_ok() {
        Ok(pathbuf)
    } else {
        Err(pathbuf)
    }
}

pub fn untreated_args_to_pathbuf(
    ok_dirs: &mut Vec<path::PathBuf>, 
    err_dirs: &mut Vec<path::PathBuf>, 
    untreated_args: &mut Vec<String>) {

    for arg in untreated_args {
        match is_readable_dir(arg.clone()) {
            Ok(pathbuf) => ok_dirs.push(pathbuf),
            Err(pathbuf) => err_dirs.push(pathbuf),
        }
    }
    if ok_dirs.is_empty() && err_dirs.is_empty() {
        let curr_dir: path::PathBuf = env::current_dir()
            .expect("No paths were passed and couldn't find/read the current directory");

        ok_dirs.push(curr_dir);
    }
}
