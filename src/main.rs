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
use std::path;

mod filetype;
mod display;
mod parse;


fn get_metrics(dir_entries: &Vec<filetype::Entry>) -> (usize, usize) {
    let mut total_length: usize = filetype::SEP_LEN * (dir_entries.len() - 1);
    let mut longest_name: usize = 0;
    for entry in dir_entries.iter() {
        let fn_len: usize = entry.file_name.len();
        total_length += fn_len;
        if longest_name < fn_len {
            longest_name = fn_len;
        }
    }
    (total_length, longest_name)
}


fn main() {
    let dir: path::PathBuf = if let Some(path_string) = env::args().nth(1) {
        path::PathBuf::from(path_string)
    } else {
        env::current_dir().expect("Failed to get current exec path")
    };

    let mut dir_entries: Vec<filetype::Entry> = dir
        .read_dir()
        .expect("Failed to read dir")
        .filter_map(Result::ok)
        .map(filetype::Entry::from_read_dir)
        .collect();

    dir_entries.sort();

    let term_width: usize = {
        if let Some(t_size) = term_size::dimensions() {
            t_size.0
        } else {
            panic!("Failed to get term's size")
        }
    };

    let (total_length, longest_name_length): (usize, usize) = get_metrics(&dir_entries);

    if total_length <= term_width {
        display::one_line(&dir_entries);
    } else {
        display::multiline(&mut dir_entries, longest_name_length, term_width);
    }
}
