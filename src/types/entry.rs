use std::ffi::{OsStr, OsString};
use std::io::Error;
use std::time::SystemTime;
use std::fs::{self, DirEntry, FileType, Metadata};

use unicode_segmentation::UnicodeSegmentation;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use crate::types::Config;
use crate::utils;



#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Directory,
    File,
    Executable,
    Symlink,
    Unknown,
}

#[derive(Debug)]
pub struct RgbColor {
    red: usize,
    green: usize,
    blue: usize,
}

impl RgbColor {
    fn get_components_sum(&self) -> usize {
        self.red + self.green + self.blue
    }

    pub fn pad_lowest(&mut self, minimal_rgb_sum: usize) {

        let mut color_sum: usize = self.get_components_sum();

        if color_sum > minimal_rgb_sum { 
            return;
        }

        let mut colors: [&mut usize; 3] = [&mut self.red, &mut self.blue, &mut self.green];
        colors.sort_unstable();

        let highest_addable_value: usize = 256 - *colors[2];

        if color_sum + highest_addable_value > minimal_rgb_sum {
            *colors[2] += highest_addable_value;
            return;
        }

        for color in colors.iter_mut() {
            let new_color: usize = (minimal_rgb_sum - color_sum).min(255);
            color_sum += new_color;
            **color = new_color;
        }
    }
}

pub struct Entry {

    // Front stuff
    pub formatted_name: OsString,
    pub extension: Option<OsString>,
    pub color: RgbColor,
    len: usize,

    // Acquired from Metadata
    pub kind: Kind,
    pub mode: Option<usize>,
    pub size_bytes: Option<usize>, 
    pub created_at: Option<SystemTime>,
    pub edited_at: Option<SystemTime>,
    pub accessed_at: Option<SystemTime>,
}

impl Entry {
    fn make_colors(config: &Config, lossy_name: &str, extension: &Option<OsString>) -> RgbColor {
        let mut prod: usize = config.color_seed;

        if let Some(ext) = extension {
            for byte in ext.to_string_lossy().bytes() {
                prod = prod.wrapping_mul(byte as usize);
            }
        } else {
            for byte in lossy_name.bytes() {
                prod = prod.wrapping_mul(byte as usize);
            }
        }

        let (green, blue): (usize, usize) = (prod / 255, prod % 255);
        let (mut red, green): (usize, usize) = (green / 255, green % 255);
        red %= 255;

        RgbColor {red, green, blue}
    }

    #[cfg(unix)]
    fn make_kind(mode: usize, file_type: FileType) -> Kind {
        if file_type.is_file() {
            if mode & 0o1111 != 0 { // executable
                Kind::Executable
            } else {
                Kind::File
            }
        } else if file_type.is_dir() {
            Kind::Directory
        } else {
            Kind::Unknown
        }
    }

    #[cfg(not(unix))]
    fn make_kind(_: usize, file_type: FileType) -> Kind {
        if file_type.is_file() {
            Kind::File
        } else if file_type.is_dir() {
            Kind::Directory
        } else {
            Kind::Symlink
        }
    }

    fn make_formatted_name(config: &Config, file_name: &OsString, kind: &Kind, color: &RgbColor) -> (OsString, usize) {

        let initial_seq: String = format!("\x1B[38;2;{};{};{}m", color.red, color.green, color.blue);

        let mut working_seq: OsString = OsString::from(initial_seq);

        let (codes, maybe_prefix, maybe_suffix): (&Vec<u8>, Option<OsString>, Option<OsString>) = {
            match kind {
                Kind::File => (&config.files, config.prefix.files.clone(), config.suffix.files.clone(),),
                Kind::Directory => (&config.directories, config.prefix.directories.clone(), config.suffix.directories.clone(),),
                Kind::Executable => (&config.executables, config.prefix.executables.clone(), config.suffix.executables.clone(),),
                Kind::Symlink => (&config.symlinks, config.prefix.symlinks.clone(), config.suffix.symlinks.clone(),),
                Kind::Unknown => (&config.unknowns, config.prefix.unknowns.clone(), config.suffix.unknowns.clone(),),
            }
        };

        for code in codes {
            let code_str: String = format!("\x1b[{}m", code);
            working_seq.push(code_str)
        }

        let mut len: usize = 0;

        if let Some(prefix) = maybe_prefix {
            len += utils::os_string_graphene_len(&prefix);
            working_seq.push(prefix)
        }

        len += utils::os_string_graphene_len(&file_name);

        working_seq.push(file_name);

        if let Some(suffix) = maybe_suffix {
            len += utils::os_string_graphene_len(&suffix);
            working_seq.push(suffix)
        }

        working_seq.push("\x1B[0;00m");

        (working_seq, len)
    }

    fn new(dir_entry: DirEntry, config: &Config) -> Self {
        let file_name: OsString = dir_entry.file_name();
        let extension: Option<OsString> = dir_entry.path().extension().map(OsStr::to_os_string);

        let lossy_name: &str = &file_name.to_string_lossy(); 
        let color: RgbColor = Self::make_colors(config, lossy_name, &extension);

        // Stuff extracted from metadata

        let maybe_metadata: Result<Metadata, Error> = {
            if config.follow_symlinks {
                fs::metadata(dir_entry.path())
            } else {
                dir_entry.metadata()
            }
        };
        
        let mut kind: Kind = Kind::Unknown; 
        let mut size_bytes: Option<usize> = None;
        let mut mode: Option<usize> = None;

        let mut created_at: Option<SystemTime> = None;
        let mut edited_at: Option<SystemTime> = None;
        let mut accessed_at: Option<SystemTime> = None;

        if let Ok(metadata) = maybe_metadata {

            let file_type: FileType = metadata.file_type();

            size_bytes = Some(metadata.len() as usize);

            created_at = metadata.created().ok();
            edited_at = metadata.modified().ok();
            accessed_at = metadata.accessed().ok();

            if cfg!(target_os="linux") {
                let retrieved_mode: usize = metadata.permissions().mode() as usize;
                kind = Self::make_kind(retrieved_mode, file_type);
                mode = Some(retrieved_mode);
            } else {
                kind = Self::make_kind(0, file_type);
            }
        }

        let (formatted_name, len): (OsString, usize) = Self::make_formatted_name(config, &file_name, &kind, &color);

        Self {
            formatted_name,
            extension,
            color,
            len,

            kind,
            mode,
            size_bytes,
            created_at,
            edited_at,
            accessed_at,
        }
    }
}
