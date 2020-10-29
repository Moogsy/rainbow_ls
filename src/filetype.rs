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

use std::fs;
use std::cmp;
use std::ffi;
use std::path;

pub const SEP: &str = "  ";
pub const SEP_LEN: usize = SEP.len();

// should be lower than 512
const MIN_COLOR_SUM: u32 = 512;

// Can this be moved into some kind of python-like enums ?
// Escape codes that will be used,
// Refer to Entry::format_filename for their meaning
const FILE_FORMATTING: [u8; 1] = [1];
const DIR_FORMATTING: [u8; 2] = [1, 7];
const SYMLINK_FORMATTING: [u8; 2] = [1, 3];
const UNKNOWN_FORMATTING: [u8; 2] = [1, 4];

// Used to be in it's own mod, but not feeling like zipping it
#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum EntryType {
    Directory,
    File,
    Symlink,
    Unknown,
}

pub struct Entry {
    entry: fs::DirEntry,
    type_: EntryType,
    color: (u8, u8, u8),  // rgb
    pub file_name: ffi::OsString,
}

// Worth rewriting with generics?
fn divmod(x: u32, y: u32) -> (u32, u32) {
    (x / y, x % y)
}

impl Entry {
    /// Determines whether the entry is a file, dir, symlink or unknown
    fn determine_type_from_entry(entry: &fs::DirEntry) -> EntryType {
        if let Ok(metadata) = entry.metadata() {
            if metadata.is_file() {
                EntryType::File
            } else if metadata.is_dir() {
                EntryType::Directory
            } else {
                EntryType::Symlink
            }
        } else {
            EntryType::Unknown
        }
    }

    /// Sort a given vector of indexes in order to only have the indexes of the darkest colors
    fn sort_lowest_colors_indexes(lowest_colors_indexes: &mut Vec<usize>,
                                  max_color_index: usize,
                                  colors: &[u32; 3]) {

        lowest_colors_indexes.remove(max_color_index);
        if colors[lowest_colors_indexes[0]] > colors[lowest_colors_indexes[1]] {
            lowest_colors_indexes.reverse();
        }
    }

    /// A lower level function that mutates the array of colors to brighten it
    fn pad_given_lowest_colors(lowest_colors_indexes: &mut Vec<usize>,
                               colors: &mut [u32; 3],
                               color_sum: u32) -> (u8, u8, u8) {

        let mut color_sum: u32 = color_sum.clone();

        for color_index in lowest_colors_indexes.iter() {
            let pot_new_color: u32 = colors[*color_index] + (MIN_COLOR_SUM - color_sum);
            if pot_new_color < 255 {
                colors[*color_index] = pot_new_color;
                let [r, g, b]: [u32; 3] = *colors;
                return (r as u8, g as u8, b as u8)
            } else {
                colors[*color_index] = u8::MAX as u32;
                color_sum = colors.iter().sum();
            }
        }
        let [r, g, b]: [u32; 3] = *colors;
        (r as u8, g as u8, b as u8)
    }

    /// Takes a tuple of rgb + their sum that was already calculated earlier
    /// And then returns a "pastel" version of it
    fn pad_lowest_colors((red, green, blue): (u32, u32, u32), color_sum: u32) -> (u8, u8, u8) {
        let mut colors: [u32; 3] = [red, green, blue];
        let max_color_index: usize = colors
            .iter()
            .enumerate()
            .max_by_key(|&(_, item)| item)
            .unwrap().0; // we know that it isn't empty, unwrap safely

        let mut lowest_colors_indexes: Vec<usize> = vec![0, 1, 2];
        Self::sort_lowest_colors_indexes(&mut lowest_colors_indexes, max_color_index, &colors);
        Self::pad_given_lowest_colors(&mut lowest_colors_indexes, &mut colors, color_sum)
    }

    /// Helper function that turns a string into a rgb tuple
    fn determine_color_from_string(string: &mut String) -> (u8, u8, u8) {
        let mut prod: u32 = 2;
        unsafe {
            for n in string.as_mut_vec().iter() {
                prod = prod.wrapping_mul(*n as u32);
            }
        }
        let (green, blue): (u32, u32) = divmod(prod, 255);
        let (mut red, green): (u32, u32) = divmod(green, 255);
        red %= 255;
        let color_sum: u32 = red + green + blue;

        if color_sum > MIN_COLOR_SUM {
            (red as u8, green as u8, blue as u8)
        } else {
            Self::pad_lowest_colors((red, green, blue), color_sum)
       }
    }

    /// Gets an fs::Direntry's filename, return "?" if it errors out or idk
    fn entry_to_string(entry: &fs::DirEntry) -> String {
        let os_filename = entry.file_name();
        let filename: String = os_filename
            .to_str()
            .unwrap_or("?")
            .to_string();

        filename
    }

    /// A higher level func that uses the filename as a whole to get a color
    fn entry_to_color(entry: &fs::DirEntry) -> (u8, u8, u8) {
        let mut filename: String = Self::entry_to_string(entry);
        Self::determine_color_from_string(&mut filename)
    }

    /// Helper function that determines a color from a file extension
    /// Or uses the filename as a whole if it failed to parse the ext
    fn determine_color_from_ext(ext: &ffi::OsStr, entry: &fs::DirEntry) -> (u8, u8, u8) {
        if let Some(ext) = ext.to_str() {
            Self::determine_color_from_string(&mut String::from(ext))
        } else {
            Self::entry_to_color(entry)
        }
    }

    /// Determines a color using the file extension or the filename as a whole
    /// if it couldn't find an extension
    fn extension_to_color(entry: &fs::DirEntry) -> (u8, u8, u8) {
        if let Some(ext) = entry.path().extension() {
            Self::determine_color_from_ext(ext, entry)
        } else {
            Self::entry_to_color(entry)
        }
    }

    /// Determines a color depending on extension
    fn determine_color_from_entry(entry: &fs::DirEntry, type_: &EntryType) -> (u8, u8, u8) {
        if type_ == &EntryType::File {
            Self::extension_to_color(entry)
        } else {
            Self::entry_to_color(entry)
        }
    }

    /// New instance of file from fs::Direntry
    pub fn from_read_dir(entry: fs::DirEntry) -> Entry {
        let type_: EntryType = Entry::determine_type_from_entry(&entry);
        let file_name: ffi::OsString = entry.file_name();
        Self {
            file_name,
            color: Self::determine_color_from_entry(&entry, &type_),
            type_,
            entry,
        }
    }

    /// formats a text
    /// 0 - Normal Style
    /// 1 - Bold
    /// 2 - Dim
    /// 3 - Italic
    /// 4 - Underlined
    /// 5 - Blinking
    /// 7 - Reverse
    /// 8 - Invisible
    fn format_filename(escape_seq: &mut String, codes: &[u8]) {
        for code in codes { // the joys of having mutable string
            escape_seq.push_str(format!("\x1b[{}m", code).as_str());
        }
    }

    fn pad_filename(&self, file_name: &str, escape_seq: &mut String, longest_name_length: usize) {
        let filename_len: usize = file_name.len();
        let diff: usize = longest_name_length.max(filename_len) - filename_len;
        for _ in 0..diff {
            escape_seq.push_str(" ");
        }
    }

    /// Returns the filename with it's proper ansi seq
    /// Pass 0 for no padding
    pub fn get_formatted_filename(&self, longest_name_length: usize) -> String {
        let (r, g, b): &(u8, u8, u8) = &self.color;
        let mut escape_seq: String = format!("\x1B[38;2;{};{};{}m", r, g, b);
        let escape_ptr: &mut String = &mut escape_seq;

        let _ = match self.type_ {
            EntryType::File => Self::format_filename(escape_ptr, &FILE_FORMATTING),
            EntryType::Directory => Self::format_filename(escape_ptr, &DIR_FORMATTING),
            EntryType::Symlink => Self::format_filename(escape_ptr, &SYMLINK_FORMATTING),
            EntryType::Unknown => Self::format_filename(escape_ptr, &UNKNOWN_FORMATTING),
        };

        let file_name: &str = self.file_name
            .to_str()
            .unwrap_or("?");

        escape_seq.push_str(file_name);
        escape_seq.push_str("\x1B[0;00m");

        self.pad_filename(file_name, &mut escape_seq, longest_name_length);

        escape_seq
    }

    fn prep_cmp(&self) -> (&EntryType, String, path::PathBuf) {
        let path: path::PathBuf = self.entry.path();
        let default: String = String::from("?");

        if let Some(ext) = path.extension() {
            if let Some(ext_str) = ext.to_str() {
                (&self.type_, String::from(ext_str), path)
            } else {
                (&self.type_, default, path)
            }

        } else {
            (&self.type_, default, path)
        }
    }
}

impl Eq for Entry {} // empty but needed for some reson

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        (&self.type_, &self.entry.path()) == (&other.type_, &other.entry.path())
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.prep_cmp().cmp(&other.prep_cmp())
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
