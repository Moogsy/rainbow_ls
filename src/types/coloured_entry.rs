use std::cmp::Ordering;
use std::ffi::{OsStr, OsString};
use std::fs::{self, DirEntry, FileType, Metadata};
use std::io::Error;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::time::SystemTime;

use unicode_segmentation::UnicodeSegmentation;

use colored::Colorize;

use crate::types::{Config, RgbColor};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Directory,
    File,
    Executable,
    Symlink,
    Unknown,
}

pub struct ColouredEntry {
    // Front stuff
    pub name: OsString,
    pub formatted_name: OsString,
    pub extension: Option<OsString>,
    pub colour: RgbColor,
    pub path: PathBuf,
    len: usize,

    // Acquired from Metadata
    pub kind: Kind,
    pub size_bytes: Option<usize>,
    pub created_at: Option<SystemTime>,
    pub modified_at: Option<SystemTime>,
    pub accessed_at: Option<SystemTime>,
}

impl ColouredEntry {
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

        RgbColor { red, green, blue }
    }

    fn make_kind(mode: usize, file_type: FileType) -> Kind {
        if file_type.is_file() {
            if mode & 0o1111 != 0 {
                // executable
                Kind::Executable
            } else {
                Kind::File
            }
        } else if file_type.is_dir() {
            Kind::Directory
        } else {
            Kind::Symlink
        }
    }

    fn make_formatted_name(
        config: &Config,
        file_name: &OsString,
        kind: &Kind,
        color: &RgbColor,
    ) -> (OsString, usize) {
        let (codes, maybe_prefix, maybe_suffix): (&Vec<u8>, Option<OsString>, Option<OsString>) =
            match kind {
                Kind::File => (
                    &config.files,
                    config.prefix.files.clone(),
                    config.suffix.files.clone(),
                ),
                Kind::Directory => (
                    &config.directories,
                    config.prefix.directories.clone(),
                    config.suffix.directories.clone(),
                ),
                Kind::Executable => (
                    &config.executables,
                    config.prefix.executables.clone(),
                    config.suffix.executables.clone(),
                ),
                Kind::Symlink => (
                    &config.symlinks,
                    config.prefix.symlinks.clone(),
                    config.suffix.symlinks.clone(),
                ),
                Kind::Unknown => (
                    &config.unknowns,
                    config.prefix.unknowns.clone(),
                    config.suffix.unknowns.clone(),
                ),
            };

        let mut len: usize = 0;
        let mut formatted_content: String = String::new();

        if let Some(prefix) = maybe_prefix {
            let lossy_prefix = prefix.to_string_lossy();
            len += lossy_prefix.grapheme_indices(true).count();
            formatted_content.push_str(&lossy_prefix);
        }

        let lossy_file_name = file_name.to_string_lossy();
        len += lossy_file_name.grapheme_indices(true).count();
        formatted_content.push_str(&lossy_file_name);

        if let Some(suffix) = maybe_suffix {
            let lossy_suffix = suffix.to_string_lossy();
            len += lossy_suffix.grapheme_indices(true).count();
            formatted_content.push_str(&lossy_suffix);
        }

        let mut styled_content =
            formatted_content.truecolor(color.red as u8, color.green as u8, color.blue as u8);

        for code in codes {
            styled_content = match code {
                0 => styled_content,
                1 => styled_content.bold(),
                2 => styled_content.dimmed(),
                3 => styled_content.italic(),
                4 => styled_content.underline(),
                5 => styled_content.blink(),
                6 => styled_content.blink(),
                7 => styled_content.reversed(),
                8 => styled_content.hidden(),
                9 => styled_content.strikethrough(),
                _ => styled_content,
            };
        }

        (OsString::from(styled_content.to_string()), len)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn new(file_name: OsString, dir_entry: &DirEntry, config: &Config) -> Self {
        let extension: Option<OsString> = dir_entry.path().extension().map(OsStr::to_os_string);

        let lossy_name: &str = &file_name.to_string_lossy();
        let mut colour: RgbColor = Self::make_colors(config, lossy_name, &extension);
        colour.pad_lowest(config.minimal_rgb_sum);

        let path_buf: PathBuf = dir_entry.path();

        // Stuff extracted from metadata

        let maybe_metadata: Result<Metadata, Error> = if config.follow_symlinks {
            fs::metadata(&path_buf)
        } else {
            dir_entry.metadata()
        };

        let mut kind: Kind = Kind::Unknown;
        let mut size_bytes: Option<usize> = None;

        let mut created_at: Option<SystemTime> = None;
        let mut modified_at: Option<SystemTime> = None;
        let mut accessed_at: Option<SystemTime> = None;

        if let Ok(metadata) = maybe_metadata {
            let file_type: FileType = metadata.file_type();

            size_bytes = Some(metadata.len() as usize);

            created_at = metadata.created().ok();
            modified_at = metadata.modified().ok();
            accessed_at = metadata.accessed().ok();

            if cfg!(unix) {
                let retrieved_mode: usize = metadata.permissions().mode() as usize;
                kind = Self::make_kind(retrieved_mode, file_type);
            } else {
                kind = Self::make_kind(0, file_type);
            }
        }

        let (formatted_name, len): (OsString, usize) =
            Self::make_formatted_name(config, &file_name, &kind, &colour);

        Self {
            name: file_name,
            formatted_name,
            extension,
            colour,
            path: dir_entry.path(),
            len,

            kind,
            size_bytes,
            created_at,
            modified_at,
            accessed_at,
        }
    }
}

impl Eq for ColouredEntry {}

impl PartialEq for ColouredEntry {
    fn eq(&self, other: &Self) -> bool {
        &self.path == &other.path
    }
}

impl Ord for ColouredEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        let other_cmp: (&Kind, &Option<OsString>, &OsString) =
            (&other.kind, &other.extension, &other.name);

        (&self.kind, &self.extension, &self.name).cmp(&other_cmp)
    }
}

impl PartialOrd for ColouredEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cmp: Ordering = self.cmp(other);
        Some(cmp)
    }
}
