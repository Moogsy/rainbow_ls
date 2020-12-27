use std::ffi::OsString;
use std::path::PathBuf;
use std::time::SystemTime;
use std::env;

use term_size;
use regex::Regex;

#[derive(Debug)]
pub enum SortingReference {
    Default,
    Name,
    Size,
    Extension,
    CreationDate,
    AccessDate,
    ModificationDate,
    Colour,
}

#[derive(Debug)]
pub enum SizeMeasurementUnit {
    Bytes,
    Bits,
} 

#[derive(Debug, Default)]
pub struct AddedStr {
    pub files: Option<OsString>,
    pub directories: Option<OsString>,
    pub executables: Option<OsString>,
    pub symlinks: Option<OsString>,
    pub unknowns: Option<OsString>,
}

#[derive(Debug)]
pub struct Config {
    // User input //

    // Formatting
    pub titles: Vec<u8>,
    pub files: Vec<u8>,
    pub directories: Vec<u8>,
    pub executables: Vec<u8>,
    pub symlinks: Vec<u8>,
    pub unknowns: Vec<u8>,

    pub prefix: AddedStr,
    pub suffix: AddedStr,

    pub color_seed: usize,
    pub minimal_rgb_sum: usize,
    pub one_per_line: bool,
    pub time_formatting: OsString,
    pub unit_size: SizeMeasurementUnit,

    // Sorting
    pub sort_by: SortingReference,
    pub uppercase_first: bool,
    pub group_directories_first: bool,
    pub reverse: bool,

    // Spacing
    pub separator: OsString,
    pub padding: OsString,

    // Ignored stuff
    pub show_dotfiles: bool,
    pub show_backups: bool,

    // Searching options
    pub recursive: bool,
    pub follow_symlinks: bool,

    pub include_pattern: Option<Regex>,
    pub exclude_pattern: Option<Regex>,

    // Auto generated //
    pub current_dir: Option<PathBuf>,
    pub term_width: Option<usize>, 
    pub paths: Vec<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {

        let color_seed: usize = {
            match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                Ok(n) => {
                    let remainder: u128 = n.as_millis() % usize::MAX as u128;
                    remainder.max(2) as usize
                },
                Err(_) => 2,
            }
        };

        Self {
            titles: Vec::new(),
            files: Vec::new(),
            directories: Vec::new(),
            executables: vec![1],
            symlinks: vec![4],
            unknowns: vec![3],

            prefix: AddedStr::default(),
            suffix: AddedStr {directories: Some(OsString::from("/")), ..Default::default()},

            color_seed,
            minimal_rgb_sum: 512,
            one_per_line: false,
            time_formatting: OsString::from("%b %m %H:%M"),
            unit_size: SizeMeasurementUnit::Bytes,

            sort_by: SortingReference::Default,
            uppercase_first: false,
            group_directories_first: false,
            reverse: false,

            separator: OsString::from("  "),
            padding: OsString::from(" "),

            show_dotfiles: false,
            show_backups: false,

            recursive: false,
            follow_symlinks: false,

            include_pattern: None,
            exclude_pattern: None,

            current_dir: env::current_dir().ok(),
            term_width: term_size::dimensions().map(|(w, _)| w),
            paths: Vec::new(),
        }
    }
}