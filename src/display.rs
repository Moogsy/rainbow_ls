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

use super::filetype;

/// Displays all entries on a single line
pub fn one_line(dir_entries: &Vec<filetype::Entry>) {
    let mapped: Vec<String> = dir_entries
        .iter()
        .map(|entry| entry.get_formatted_filename(0))
        .collect();

    println!("{}", mapped.join(filetype::SEP));
}

/// Displays all entries in some kind of rows and rolumns
pub fn multiline(dir_entries: &mut Vec<filetype::Entry>, longest_name_length: usize, term_width: usize) {
    let per_line: usize = term_width / (longest_name_length + filetype::SEP_LEN);

    let mut temp: Vec<String> = Vec::new();

    for entry in dir_entries {
        temp.push(entry.get_formatted_filename(longest_name_length));

        if temp.len() == per_line {
            print!("{}\n", temp.join(filetype::SEP));
            temp.clear();
        }
    }

    if !temp.is_empty() {
        print!("{}\n", temp.join(filetype::SEP));
    }
}

