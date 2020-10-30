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
    pub file: Vec<u8>,
    pub dir: Vec<u8>,
    pub symlink: Vec<u8>,
    pub unknown: Vec<u8>,
    pub min_colour_sum: u32,
}

impl Config {
    pub fn new() -> Self {
        Self {
            file: vec![1],
            dir: vec![1, 7],
            symlink: vec![1, 3],
            unknown: vec![1, 4],
            min_colour_sum: 255,
        }
    }
}

fn parse_colour_arg(curr_config: &mut Vec<u8>, right_arg: String) {
    curr_config.clear(); // clearing defaults
    for letter in right_arg.chars() {
        if let Some(digit) = letter.to_digit(10) {
            let digit = digit as u8;
            if VALID_ESCAPE_DIGITS.contains(&digit) {
                curr_config.push(digit);
            } else {
                panic!("Digit not in {:?}", VALID_ESCAPE_DIGITS)
            }
        } else {
            panic!("Right argument must be a chain of digits")
        }
    }
}

fn parse_min_sum(curr_config: &mut u32, right_arg: String) {
    let right: u32 = right_arg.trim().parse().expect("Sum must be an int");
    if 0 < right || right > 765 {
        panic!("Expected sum to be an int between 0 and 765, got {}", right);
    } else {
        *curr_config = right;
    }
}

pub fn parse_args() -> Config {
    let mut config: Config = Config::new();

    let mut args: env::Args = env::args();

    while let Some(left_arg) = args.next() {
        if !left_arg.starts_with("--") {
            continue;
        } else if left_arg == "--help" {
            println!("{}", HELP_TXT);  // hardcoded help command is big hahayes moment
            process::exit(0);
        }

        if let Some(right_arg) = args.next() {
            match left_arg.as_str() {
                "--file" => parse_colour_arg(&mut config.file, right_arg),
                "--dir" => parse_colour_arg(&mut config.dir, right_arg),
                "--symlink" => parse_colour_arg(&mut config.symlink, right_arg),
                "--unknown" => parse_colour_arg(&mut config.unknown, right_arg),
                "--sum" => parse_min_sum(&mut config.min_colour_sum, right_arg),
                _ => panic!("Unrecognized argument"),
            }
        } else {
            panic!("Argument not filled")
        }
    }

    println!("{:?}", config);

    config

}
