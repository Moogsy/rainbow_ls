use std::ffi;
use std::path;
use std::cmp;
use std::collections::hash_map;
use std::fs;
use std::hash::{Hash, Hasher};

use crate::parser;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Kind {
    Directory,
    File,
    Symlink,
    Unknown,
}

#[derive(Debug)]
pub struct Entry {
    pub name: ffi::OsString,
    dir_entry: fs::DirEntry,
    kind: Kind,
}

struct RgbColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl RgbColor {
    fn get_sum(&self) -> u16 {
        self.red as u16 + self.green as u16 + self.blue as u16
    }

    fn pad_lowest(&mut self, min_rgb_sum: u16) {
        let mut colors_sum: u16 = self.get_sum();

        // Already good
        if colors_sum > min_rgb_sum {
            return;
        }

        let mut colors: [&mut u8; 3] = [&mut self.red, &mut self.green, &mut self.blue];
        colors.sort_unstable();

        let highest_addable_value: u8 = u8::MAX - *colors[2];

        let diff: u16 = min_rgb_sum - colors_sum;

        // Just increment all 3 colors simultaneously 
        if (highest_addable_value as u16 * 3) > diff {
            let to_add: u8 = (diff / 3) as u8;
            for color in colors.iter_mut() {
                **color = **color + to_add;
            }
            return;
        } 

        // Increment them by ascending color value
        for color in colors.iter_mut() {
            let potential_new_color: u16 = **color as u16 + (min_rgb_sum - colors_sum);

            if potential_new_color < u8::MAX as u16 {
                **color = potential_new_color as u8;
                return;

            } else {
                let old_color = **color;

                **color = u8::MAX;

                colors_sum += (u8::MAX - old_color) as u16;


            }
        }
    }
}

impl From<(u8, u8, u8)> for RgbColor {
    fn from(colors: (u8, u8, u8)) -> Self {
        let (red, green, blue) = colors;
        Self {red, green, blue}
    }
}

impl Entry {
    fn determine_kind(dir_entry: &fs::DirEntry) -> Kind {
        if let Ok(file_type) = dir_entry.file_type() {
            if file_type.is_file() {
                Kind::File
            } else if file_type.is_dir() {
                Kind::Directory
            } else {
                Kind::Symlink
            }
        } else {
            Kind::Unknown
        }
    }

    fn get_extension(&self) -> ffi::OsString {
        if let Some(ext) = self.dir_entry.path().extension() {
            ext.to_os_string()
        } else {
            ffi::OsString::from("?")
        }
    }

    fn prep_cmp(&self) -> (&Kind, ffi::OsString, ffi::OsString, path::PathBuf) {
        let filename: ffi::OsString = self.dir_entry.file_name();
        let extension: ffi::OsString = self.get_extension();
        let path: path::PathBuf = self.dir_entry.path();

        (&self.kind, extension, filename, path)
    }

    /// Color related stuff
    fn make_color_component<T: Hash>(item: &T, hasher: &mut hash_map::DefaultHasher) -> u8 {
        item.hash(hasher);
        (hasher.finish() % 255) as u8
    }
    fn get_color(&self) -> RgbColor {
        let mut hasher: hash_map::DefaultHasher = hash_map::DefaultHasher::new();
        
        let red: u8 = Self::make_color_component(&self.get_extension(), &mut hasher);
        let green: u8 = Self::make_color_component(&self.name, &mut hasher);
        let blue: u8 = Self::make_color_component(&self.kind, &mut hasher);

        RgbColor::from((red, green, blue))
    }

    fn format_filename(formatted_name: &mut ffi::OsString, codes: &Vec<u8>) {
        for code in codes {
            let code_str: String = format!("\x1b[{}m", code);
            let to_push: ffi::OsString = ffi::OsString::from(code_str);
            formatted_name.push(to_push);
        }
    }

    fn pad_filename(&self, formatted_name: &mut ffi::OsString, longest_name_len: usize) {
        let filename_len: usize = self.name.len();
        // Allows us to pass 0 for no padding
        let diff: usize = longest_name_len.max(filename_len) - filename_len;
        let sep: ffi::OsString = ffi::OsString::from(" ");
        for _ in 0..diff {
            formatted_name.push(&sep);
        }
    }

    pub fn get_formatted_name(&self, longest_name_length: usize, config: &parser::Config) -> ffi::OsString {

        let mut color: RgbColor = self.get_color();
        color.pad_lowest(config.min_rgb_sum);
        
        let starting_seq: String = format!("\x1B[38;2;{};{};{}m", color.red, color.green, color.blue);
        let mut formatted_name: ffi::OsString = ffi::OsString::from(starting_seq);

        match self.kind {
            Kind::File => Self::format_filename(&mut formatted_name, &config.files),
            Kind::Directory => Self::format_filename(&mut formatted_name, &config.directories),
            Kind::Symlink => Self::format_filename(&mut formatted_name, &config.symlinks),
            Kind::Unknown => Self::format_filename(&mut formatted_name, &config.unknowns),
        };

        formatted_name.push(&self.name);
        formatted_name.push("\x1B[0;00m");

        self.pad_filename(&mut formatted_name, longest_name_length);
        formatted_name
    }
}

impl From<fs::DirEntry> for Entry {
    fn from(dir_entry: fs::DirEntry) -> Self {
        Self {
            kind: Self::determine_kind(&dir_entry),
            name: dir_entry.file_name(),
            dir_entry: dir_entry,
        }
    }

}

impl PartialEq for Entry {
   fn eq(&self, other: &Self) -> bool {
       self.prep_cmp() == other.prep_cmp()
   } 
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.prep_cmp().cmp(&other.prep_cmp())
    }
}

// Must be there but eh
impl Eq for Entry {
}

