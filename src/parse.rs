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

fn parse_args() {
    let mut config: Config = Config::new();

    let mut args = env::args();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--file" => {
                if let Some(param) = args.next() {
                    for letter in param.chars() {
                        config.file.push(letter as u8);
                    }
                }
            },
            "--dir" => (),
            "--symlink" => (),
            "--unknown" => (),
            _ => ()
        }
    }
}
