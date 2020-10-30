/// Rainbow-ls listing files with a lot of colours
/// Copyright (C) 2020 - Saphielle Akiyama
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU General Public License for more details.
///
/// You should have received a copy of the GNU General Public License
/// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::env;
use std::process;
use std::path;

const VALID_ESCAPE_DIGITS: [u8; 8] = [0, 1, 2, 3, 4, 5, 7, 8];
const HELP_TXT: &str = r#"
##########################
# Per entry type control #
##########################

--file [codes] 
--dir [codes] 
--symlink [codes] 
--unknown [codes]  

Where codes is one or more of:
0 - Normal Style
1 - Bold
2 - Dim
3 - Italic
4 - Underlined
5 - Blinking
7 - Reverse
8 - Invisible
 
Specify some formatting code to use for an entry type.

Examples:
    --file 1   // file is bold
    --file 12  // file is bold and dim
    --file 1 --dir 2 // file is bold, dir is dim

#################
# Color control #
#################

--sum [lowest_sum]

Specifies the minimal sum of the red, green and blue
components of the colors. Cannot be over 765 (255 * 3).

Example:
    --sum 512 // This will be bright
    --sum 100 // This will have a wide range, from very dark to very bright

"#;


#[derive(Debug)]
pub struct Config {
    pub ok_directories: Vec<path::PathBuf>,
    pub err_directories: Vec<path::PathBuf>,
    pub file: Vec<u8>,
    pub dir: Vec<u8>,
    pub symlink: Vec<u8>,
    pub unknown: Vec<u8>,
    pub min_colour_sum: u32,
}

impl Config {
    pub fn new() -> Self {
        Self {
            // Curr dir should be passed in argv
            ok_directories: Vec::new(), 
            err_directories: Vec::new(),
            file: vec![1],
            dir: vec![1, 7],
            symlink: vec![1, 3],
            unknown: vec![1, 4],
            min_colour_sum: 255,
        }
    }
}

/// Tries to parse the escape digits
/// Panics if it fails to do so or if an invalid digit was passed
fn parse_formatting_arg(curr_config: &mut Vec<u8>, right_arg: String) {
    curr_config.clear(); // clearing defaults
    for letter in right_arg.chars() {
        // .expect doesn't support format string, .unwrap_or_else avoids the overhead of formatting it
        // even in case of ok.
        let digit: u8 = letter
            .to_digit(10)
            .unwrap_or_else(|| panic!("Failed to convert {} to a number", letter))
            as u8;

        if VALID_ESCAPE_DIGITS.contains(&digit) {
            curr_config.push(digit);
        } else {
            panic!("Digit not in {:?}", VALID_ESCAPE_DIGITS)
        }
    }
}

/// Tries to parse the arg passed as the minimal sum of r,g,b.
/// Panics if the the arg is negative or higher than the maximal sum
fn parse_min_sum(curr_config: &mut u32, right_arg: String) {
    let min_sum: u32 = right_arg
        .trim()
        .parse()
        .expect("Sum must be a positive int");

    if min_sum > 765 {
        panic!("Expected sum to be an int between 0 and 765, got {}", min_sum);
    } else {
        *curr_config = min_sum;
    }
}

/// Gets all remaining args after a "--" to treat them as file paths
fn parse_double_dash(right_arg: String, arg_iter: &mut env::Args, untreated_args: &mut Vec<String>) {
    untreated_args.push(right_arg);
    while let Some(potential_fp) = arg_iter.next() {
        untreated_args.push(potential_fp);
    }
}

fn check_pathbuf_validity(pathbuf: path::PathBuf, config: &mut Config) {
    if pathbuf.read_dir().is_ok() {
        config.ok_directories.push(pathbuf);
    } else {
        config.err_directories.push(pathbuf);
    }
}

fn check_no_path_passed(config: &mut Config) {
    if config.ok_directories.is_empty() && config.err_directories.is_empty() {
        let curr_dir: path::PathBuf = env::current_dir()
            .expect("No path args were passed and couldn't read / get current directory");
        
        config.ok_directories.push(curr_dir);
    }
}

fn parse_untreated_args(config: &mut Config, untreated_args: &mut Vec<String>) {
    let mut found_exec_path: bool = false;
    for arg in untreated_args.iter() {
        let pathbuf: path::PathBuf = path::PathBuf::from(arg);

        // Assume that the first file path is the executable's
        if pathbuf.is_dir() {  
            check_pathbuf_validity(pathbuf, config);

        } else if found_exec_path {
            config.err_directories.push(pathbuf);

        // GNU coreutils' ls prints the name of the first passed file, but reports other ones
        } else { 
            println!("{:?}", pathbuf);
            found_exec_path = true;
        }
    }
    check_no_path_passed(config);
}

pub fn parse_args() -> Config {
    let mut config: Config = Config::new();

    let mut args: env::Args = env::args();

    // We'll take care of them later, potential file paths
    let mut untreated_args: Vec<String> = Vec::new();

    while let Some(left_arg) = args.next() {
        
        if !left_arg.starts_with("--") { // wiki said to not rely on arg order 
            untreated_args.push(left_arg);
            continue;
        }

        if left_arg == "--help" {
            println!("{}", HELP_TXT);  // hardcoded help command is big hahayes moment
            process::exit(0);
        }

        if let Some(right_arg) = args.next() {
            match left_arg.as_str() {
                "--file" => parse_formatting_arg(&mut config.file, right_arg),
                "--dir" => parse_formatting_arg(&mut config.dir, right_arg),
                "--symlink" => parse_formatting_arg(&mut config.symlink, right_arg),
                "--unknown" => parse_formatting_arg(&mut config.unknown, right_arg),
                "--sum" => parse_min_sum(&mut config.min_colour_sum, right_arg),
                "--" => { // args after a double dash should be treated as a path
                    parse_double_dash(right_arg, &mut args, &mut untreated_args);
                    break;
                },
                unknown_arg => panic!("Unrecognized argument: {}", unknown_arg),
            }

        } else {
            panic!("Argument {} not filled", left_arg);
        }
    }
    parse_untreated_args(&mut config, &mut untreated_args);
    println!("{:?}", config);

    config

}
