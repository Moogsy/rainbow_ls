use std::ffi;
use std::cmp;
use std::fs;
use std::time;

use unicode_segmentation::UnicodeSegmentation;

use crate::parser;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Kind {
    Directory,
    File(bool), // is_dotfile
    Symlink,
    Unknown,
}

#[derive(Debug)]
pub struct Entry {
    // Name related stuff
    pub name: ffi::OsString,
    pub formatted_name: ffi::OsString,
    len: usize,
    pub extension: Option<ffi::OsString>,

    // File_type
    pub kind: Kind,

    // Metadata
    pub size: Option<usize>,
    pub created_at: Option<time::SystemTime>,
    pub edited_at: Option<time::SystemTime>,
    pub accessed_at: Option<time::SystemTime>,
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

impl Entry {
    fn determine_kind(lossy_name: &str, dir_entry: &fs::DirEntry) -> Kind {
        if let Ok(file_type) = dir_entry.file_type() {
            
            if file_type.is_file() {
                if lossy_name.starts_with(".") {
                    Kind::File(true)
                } else {
                    Kind::File(false)
                }
            } else if file_type.is_dir() {
                Kind::Directory
            } else {
                Kind::Symlink
            }
        } else {
            Kind::Unknown
        }
    }

    fn format_filename(formatted_name: &mut ffi::OsString, codes: &Vec<u8>) {
        for code in codes {
            let code_str: String = format!("\x1b[{}m", code);
            let to_push: ffi::OsString = ffi::OsString::from(code_str);
            formatted_name.push(to_push);
        }
    }


    fn determine_formatted_name(config: &parser::Config, name: &ffi::OsString, kind: &Kind, color: &mut RgbColor) -> ffi::OsString {
        let starting_seq: String = format!("\x1B[38;2;{};{};{}m", color.red, color.green, color.blue);
        let mut formatted_name: ffi::OsString = ffi::OsString::from(starting_seq);

        match kind {
            Kind::File(_) => Self::format_filename(&mut formatted_name, &config.files),
            Kind::Directory => Self::format_filename(&mut formatted_name, &config.directories),
            Kind::Symlink => Self::format_filename(&mut formatted_name, &config.symlinks),
            Kind::Unknown => Self::format_filename(&mut formatted_name, &config.unknowns),
        };

        formatted_name.push(name);
        formatted_name.push("\x1B[0;00m");

        formatted_name

    }
    fn determine_extension(dir_entry: &fs::DirEntry) -> Option<ffi::OsString> {
        if let Some(ext) = dir_entry.path().extension() {
            Some(ext.to_os_string())
        } else {
            None
        }
    }

    fn make_colors(lossy_name: &str, extension: &Option<ffi::OsString>) -> RgbColor {
        let mut prod: u32 = 11;

        if let Some(ext) = extension {
            for byte in ext.to_string_lossy().bytes() {
                prod = prod.wrapping_mul(byte as u32);
            }
        } else {
            for byte in lossy_name.bytes() {
                prod = prod.wrapping_mul(byte as u32);
            }
        }

        let (green, blue): (u32, u32) = (prod / 255, prod % 255);
        let (mut red, green): (u32, u32) = (green / 255, green % 255);
        red %= 255;
        
        RgbColor {
            red: red as u8, 
            green: green as u8, 
            blue: blue as u8,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn new(config: &parser::Config, name: ffi::OsString, dir_entry: fs::DirEntry) -> Self {
        let lossy_name: &str = &name.to_string_lossy();

        let extension: Option<ffi::OsString> = Self::determine_extension(&dir_entry);
        let kind: Kind = Self::determine_kind(lossy_name, &dir_entry);

        let mut color: RgbColor = Self::make_colors(lossy_name, &extension);
        color.pad_lowest(config.min_rgb_sum);


        let mut created_at: Option<time::SystemTime> = None;
        let mut edited_at: Option<time::SystemTime> = None;
        let mut accessed_at: Option<time::SystemTime> = None;
        let mut size: Option<usize> = None;

        if let Ok(metadata) = dir_entry.metadata() {
            created_at = metadata.created().ok();
            edited_at = metadata.modified().ok();
            accessed_at = metadata.accessed().ok();
            size = Some(metadata.len() as usize);
        }

        Self {
            // Name related stuff (odd order due to borrows)
            formatted_name: Self::determine_formatted_name(config, &name, &kind, &mut color),
            extension,
            len: lossy_name.graphemes(true).count(),
            name,

            // Metadata 
            kind,
            created_at,
            edited_at,
            accessed_at,
            size,
        }
    }
}

impl PartialEq for Entry {
   fn eq(&self, other: &Self) -> bool {
       (&self.kind, &self.extension, &self.name) == (&other.kind, &other.extension, &other.name)
   } 
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        (&self.kind, &self.extension, &self.name).cmp(&(&other.kind, &other.extension, &other.name))
    }
}

// Must be there but eh
impl Eq for Entry {
}

