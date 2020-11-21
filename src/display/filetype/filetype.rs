use std::ffi;
use std::collections::hash_map;
use std::fs;
use std::hash::{Hash, Hasher};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Kind {
    Directory,
    File,
    Symlink,
    Unknown,
}

#[derive(Debug, Hash)]
pub struct Entry {
    name: ffi::OsString,
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

    pub fn get_formatted_name(&self, padding: usize) -> ffi::OsString {
        let (red, green, blue): (u8, u8, u8) = self.get_color();

        let name: ffi::OsString = self.name.clone();

        let starting_seq: String = format!("\x1B[38;2;{};{};{}m", red, green, blue);

        let mut formatted_name: ffi::OsString = ffi::OsString::from(starting_seq);





    }

}

impl From<fs::DirEntry> for Entry {
    fn from(dir_entry: fs::DirEntry) -> Self {
        Self {
            kind: Self::determine_kind(&dir_entry),
            name: dir_entry.file_name(),
        }
    }

}

