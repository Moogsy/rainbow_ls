use term_size;

use std::ffi::OsString;
use std::time::SystemTime;

#[derive(Debug)]
pub enum SortingReference {
    Name,
    Size,
    Extension,
    CreationDate,
    AccessDate,
    ModificationDate,
}

#[derive(Debug)]
pub enum UnitSize {
    Byte,
    Bit,
} 

impl UnitSize {
    fn convert(&self) {
        let base: usize;

        match self {
            UnitSize::Bit => base = 10,
            UnitSize::Byte => base = 2,
        }

    }

}

#[derive(Debug, Default)]
pub struct AddedStr {
    pub files: Option<OsString>,
    pub directories: Option<OsString>,
    pub symlinks: Option<OsString>,
    pub unknowns: Option<OsString>,
}

#[derive(Debug)]
pub struct Config {
    // Fomatting
    pub files: Vec<u8>,
    pub directories: Vec<u8>,
    pub symlinks: Vec<u8>,
    pub unknowns: Vec<u8>,

    pub prefix: AddedStr,
    pub suffix: AddedStr,

    pub minimal_rgb_sum: u16,
    pub one_per_line: bool,
    pub time_formatting: OsString,
    pub unit_size: UnitSize,

    // Sorting
    pub sort_by: SortingReference,
    pub group_directories_first: bool,
    pub reverse_output: bool,

    // Spacing
    pub separator: OsString,
    pub padding: OsString,

    // Ignored stuff
    pub show_dotfiles: bool,
    pub show_backups: bool,

    // Searching options
    pub recursive: bool,
    pub follow_symlinks: bool,

    pub include_pattern: bool,
    pub excluse_pattern: bool,

    // Auto generated //
    
    pub term_width: Option<usize>, 
    pub color_seed: usize,
}

impl Default for Config {
    fn default() -> Self {

        let color_seed: usize = {
            match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                Ok(n) => (n.as_millis() % usize::MAX as u128) as usize, // ew
                Err(_) => 2,
            }
        };

        Self {
            files: Vec::new(),
            directories: Vec::new(),
            symlinks: Vec::new(),
            unknowns: Vec::new(),

            prefix: AddedStr::default(),
            suffix: AddedStr {directories: Some(OsString::from("/")), ..Default::default()},

            minimal_rgb_sum: 512,
            one_per_line: false,

            sort_by: SortingReference::Name,
            group_directories_first: true,
            reverse_output: false,

            separator: OsString::from("  "),
            padding: OsString::from(" "),

            show_dotfiles: false,
            show_backups: false,

            recursive: false,

            term_width: term_size::dimensions().map(|(w, _)| w),
            color_seed
        }
    }
}