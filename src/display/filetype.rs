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

    fn prep_cmp(&self) -> (ffi::OsString, ffi::OsString, path::PathBuf) {
        let filename: ffi::OsString = self.dir_entry.file_name();

        let extension: ffi::OsString;

        if let Some(ext) = self.dir_entry.path().extension() {
            extension = ext.to_os_string();
        } else {
            extension = ffi::OsString::from("?");
        }

        let path: path::PathBuf = self.dir_entry.path();

        (extension, filename, path)
    }

    /// Color related stuff
    fn make_color_component<T: Hash>(item: &T, hasher: &mut hash_map::DefaultHasher) -> u8 {
        item.hash(hasher);
        (hasher.finish() % 255) as u8
    }

    fn get_color(&self) -> (u8, u8, u8) {
        let mut hasher = hash_map::DefaultHasher::new();
        
        let red: u8 = Self::make_color_component(&self.name, &mut hasher);
        let green: u8 = Self::make_color_component(&self.kind, &mut hasher);
        let blue: u8 = Self::make_color_component(&self.name.len(), &mut hasher);

        (red, green, blue)
    }

    fn format_filename(formatted_name: &mut ffi::OsString, codes: &Vec<u8>) {
        for code in codes {
            let code_str: String = code.to_string();
            let to_push: ffi::OsString = ffi::OsString::from(code_str);
            formatted_name.push(to_push);
        }
    }

    fn pad_filename(&self, formatted_name: &mut ffi::OsString, longest_name_length: usize) {
        let filename_len: usize = self.name.len();
        let diff: usize = longest_name_length.max(filename_len) - filename_len;
        let sep: ffi::OsString = ffi::OsString::from(" ");
        for _ in 0..diff {
            formatted_name.push(&sep);
        }
    }

    pub fn get_formatted_name(&self, longest_name_length: usize, config: &parser::Config) -> ffi::OsString {
        
        let (red, green, blue): (u8, u8, u8) = self.get_color();
        let starting_seq: String = format!("\x1B[38;2;{};{};{}m", red, green, blue);
        let mut formatted_name: ffi::OsString = ffi::OsString::from(starting_seq);

        formatted_name.push(&self.name);

        self.pad_filename(&mut formatted_name, longest_name_length);
        formatted_name.push("\x1B[0;00m");

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

