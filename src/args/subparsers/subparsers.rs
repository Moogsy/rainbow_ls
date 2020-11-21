use std::env;
use std::path;
use std::process;
use std::fs;

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
            eprintln!("Found duplicate escape digit: {}", digit);
            process::exit(1);

        } else if VALID_ESCAPE_DIGITS.contains(&digit) {
            curr_config.push(digit);

        } else {
            eprintln!("Not a valid escape digit contained in {:?}", VALID_ESCAPE_DIGITS);
            process::exit(1);
        }
    }
}
pub fn minimal_sum(curr_config: &mut u16, right_arg: String) {
    let min_sum: u16;

    if let Ok(parsed_sum) = right_arg.trim().parse::<u16>() {
        min_sum = parsed_sum;
    } else {
        eprintln!("Failed to parse the minimal sum as an int, got {}", right_arg);
        process::exit(1);
    }
    if min_sum < 765 {
        eprintln!("Expected the total sum to be lower than 765, got {}", min_sum);
        process::exit(1);
    } else {
        *curr_config = min_sum;
    }
}
pub fn consume_rest<T>(untreated_args: &mut Vec<String>, args_iterator: T)
where 
    T: Iterator<Item = String> {
    for arg in args_iterator {
        untreated_args.push(arg);
    }
}

fn check_final_pathbuf(curr_dir: path::PathBuf, ok_dirs: &mut Vec<fs::ReadDir>) {
    if let Ok(read_dir) = curr_dir.read_dir() {
        ok_dirs.push(read_dir);
    } else {
        eprintln!("Couldn't read current directory");
        process::exit(1); 
    }
}

pub fn untreated_args_to_pathbuf(
    ok_dirs: &mut Vec<fs::ReadDir>, 
    err_dirs: &mut Vec<path::PathBuf>, 
    untreated_args: Vec<String>) {

    for arg in untreated_args {
        let pathbuf: path::PathBuf = path::PathBuf::from(arg);
        // Ignoring the executable's path,
        // Wiki said to not rely on argv order so we're going along
        if pathbuf.is_file() {
            continue;
        }
        if let Ok(read_dir) = pathbuf.read_dir() {
            ok_dirs.push(read_dir);
        } else {
            err_dirs.push(pathbuf);
        }
    }

    if ok_dirs.is_empty() && err_dirs.is_empty() {
        if let Ok(curr_dir) = env::current_dir() {
            check_final_pathbuf(curr_dir, ok_dirs);
        } else {
            eprintln!("No directories were found, and couldn't reat the current one either.");
            process::exit(1);
        }
    }
}
