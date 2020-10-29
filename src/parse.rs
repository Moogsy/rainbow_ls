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

#[derive(Debug)]
struct Config {
    pub file: Vec<u8>,
    pub dir: Vec<u8>,
    pub symlink: Vec<u8>,
    pub unknown: Vec<u8>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            file: Vec::new(),
            dir: Vec::new(),
            symlink: Vec::new(),
            unknown: Vec::new(),
        }
    }
}

pub fn parse_args() {
    let mut config: Config = Config::new();

    let mut args = env::args();

    while let Some(left_arg) = args.next() {
        if !left_arg.starts_with("--") {
            continue;
        }
        if let Some(right_arg) = args.next() {
            let curr_config: &mut Vec<u8> = {
                match left_arg.as_str() {
                    "--file" => &mut config.file,
                    "--dir" => &mut config.dir,
                    "--symlink" => &mut config.symlink,
                    "--unknown" => &mut config.unknown,
                    _ => panic!("Unrecognized argument"),
                }
            };
            for letter in right_arg.chars() {
                if let Some(digit) = letter.to_digit(10) {
                    curr_config.push(digit as u8);
                } else {
                    panic!("Right argument must be a chain of digits")
                }
            }
        } else {
            panic!("Argument left empty");
        }
    }
    println!("{:?}", config);
}
