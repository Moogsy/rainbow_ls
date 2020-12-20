use std::ffi::OsString;
use std::cmp::Ordering;
use std::fs::DirEntry;
use std::borrow::Cow;
use std::time::SystemTime;

use unicode_segmentation::UnicodeSegmentation;

use crate::parser::Config;

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
    pub name: OsString,
    pub formatted_name: OsString,
    len: usize,
    pub extension: Option<OsString>,

    // File_type
    pub kind: Kind,

    // Metadata
    pub size: Option<usize>,
    pub created_at: Option<SystemTime>,
    pub edited_at: Option<SystemTime>,
    pub accessed_at: Option<SystemTime>,
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
    fn determine_kind(config: &Config, name: &mut OsString, dir_entry: &DirEntry) -> Kind {
        if let Ok(file_type) = dir_entry.file_type() {
            if file_type.is_file() {
                if name.to_string_lossy().starts_with(".") {
                    name.push(&config.dotfiles_suffix);
                    Kind::File(true)
                } else {
                    name.push(&config.files_suffix);
                    Kind::File(false)
                }
            } else if file_type.is_dir() {
                name.push(&config.directories_suffix);
                Kind::Directory
            } else {
                name.push(&config.symlinks_suffix);
                Kind::Symlink
            }
        } else {
            name.push(&config.unknowns_suffix);
            Kind::Unknown
        }
    }

    fn format_filename(formatted_name: &mut OsString, codes: &Vec<u8>) {
        for code in codes {
            let code_str: String = format!("\x1b[{}m", code);
            let to_push: OsString = OsString::from(code_str);
            formatted_name.push(to_push);
        }
    }

    fn determine_formatted_name(config: &Config, name: &OsString, kind: &Kind, color: &mut RgbColor) -> OsString {
        let starting_seq: String = format!("\x1B[38;2;{};{};{}m", color.red, color.green, color.blue);
        let mut formatted_name: OsString = OsString::from(starting_seq);

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
    fn determine_extension(dir_entry: &DirEntry) -> Option<OsString> {
        if let Some(ext) = dir_entry.path().extension() {
            Some(ext.to_os_string())
        } else {
            None
        }
    }

    fn make_colors(config: &Config, lossy_name: &str, extension: &Option<OsString>) -> RgbColor {
        let mut prod: u32 = config.color_seed;

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

    fn info_from_metadata(dir_entry: &DirEntry) -> ([Option<SystemTime>; 3], Option<usize>){
        let mut created_at: Option<SystemTime> = None;
        let mut edited_at: Option<SystemTime> = None;
        let mut accessed_at: Option<SystemTime> = None;
        let mut size: Option<usize> = None;

        if let Ok(metadata) = dir_entry.metadata() {
            created_at = metadata.created().ok();
            edited_at = metadata.modified().ok();
            accessed_at = metadata.accessed().ok();
            size = Some(metadata.len() as usize);
        }
        ([created_at, edited_at, accessed_at], size)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn new(config: &Config, mut name: OsString, dir_entry: DirEntry) -> Self {

        let extension: Option<OsString> = Self::determine_extension(&dir_entry);
        
        let kind: Kind = Self::determine_kind(config, &mut name, &dir_entry);


        let lossy_name: Cow<str> = name.to_string_lossy();

        let mut color: RgbColor = Self::make_colors(config, &lossy_name, &extension);
        color.pad_lowest(config.min_rgb_sum);

        let ([created_at, edited_at, accessed_at], size) = Self::info_from_metadata(&dir_entry);

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
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.kind, &self.extension, &self.name).cmp(&(&other.kind, &other.extension, &other.name))
    }
}

// Must be there but eh
impl Eq for Entry {
}

