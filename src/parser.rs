use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

use clap::{ArgAction, Parser as ClapParser};

use crate::subparsers;
use crate::Config;

#[derive(ClapParser, Debug)]
#[command(
    name = "rainbow_ls",
    version,
    disable_help_flag = false,
    disable_version_flag = false
)]
struct Cli {
    #[arg(short = '1', long = "one-per-line", action = ArgAction::SetTrue)]
    one_per_line: bool,
    #[arg(long = "long-listing", alias = "ll", action = ArgAction::SetTrue)]
    long_listing: bool,
    #[arg(long = "group-directories-first", alias = "gdf", action = ArgAction::SetTrue)]
    group_directories_first: bool,
    #[arg(long = "reverse", alias = "rev", action = ArgAction::SetTrue)]
    reverse: bool,
    #[arg(long = "show-dotfiles", alias = "sd", action = ArgAction::SetTrue)]
    show_dotfiles: bool,
    #[arg(long = "show-backups", alias = "sb", action = ArgAction::SetTrue)]
    show_backups: bool,
    #[arg(long = "recursive", alias = "rec", action = ArgAction::SetTrue)]
    recursive: bool,
    #[arg(long = "follow-symlinks", alias = "fs", action = ArgAction::SetTrue)]
    follow_symlinks: bool,

    #[arg(long = "titles", value_name = "DIGITS")]
    titles: Option<OsString>,
    #[arg(long = "files", value_name = "DIGITS")]
    files: Option<OsString>,
    #[arg(long = "directories", value_name = "DIGITS")]
    directories: Option<OsString>,
    #[arg(long = "executables", value_name = "DIGITS")]
    executables: Option<OsString>,
    #[arg(long = "symlinks", value_name = "DIGITS")]
    symlinks: Option<OsString>,
    #[arg(long = "unknown", value_name = "DIGITS")]
    unknowns: Option<OsString>,

    #[arg(long = "files-prefix", value_name = "STRING")]
    files_prefix: Option<OsString>,
    #[arg(long = "directories-prefix", value_name = "STRING")]
    directories_prefix: Option<OsString>,
    #[arg(long = "executables-prefix", value_name = "STRING")]
    executables_prefix: Option<OsString>,
    #[arg(long = "symlinks-prefix", value_name = "STRING")]
    symlinks_prefix: Option<OsString>,
    #[arg(long = "unknowns-prefix", value_name = "STRING")]
    unknowns_prefix: Option<OsString>,

    #[arg(long = "files-suffix", value_name = "STRING")]
    files_suffix: Option<OsString>,
    #[arg(long = "directories-suffix", value_name = "STRING")]
    directories_suffix: Option<OsString>,
    #[arg(long = "executables-suffix", value_name = "STRING")]
    executables_suffix: Option<OsString>,
    #[arg(long = "symlinks-suffix", value_name = "STRING")]
    symlinks_suffix: Option<OsString>,
    #[arg(
        long = "unkowns-suffix",
        value_name = "STRING",
        visible_alias = "unknowns-suffix"
    )]
    unknowns_suffix: Option<OsString>,

    #[arg(long = "color-seed", value_name = "SEED")]
    color_seed: Option<OsString>,
    #[arg(long = "sum", value_name = "SUM")]
    minimal_sum: Option<OsString>,
    #[arg(long = "time-formatting", value_name = "FORMAT")]
    time_formatting: Option<OsString>,
    #[arg(long = "unit-size", value_name = "UNIT")]
    unit_size: Option<OsString>,
    #[arg(long = "sort-by", value_name = "FIELD")]
    sort_by: Option<OsString>,
    #[arg(long = "separator", value_name = "SEP")]
    separator: Option<OsString>,
    #[arg(long = "padding", value_name = "CHAR")]
    padding: Option<OsString>,
    #[arg(long = "include-pattern", value_name = "REGEX")]
    include_pattern: Option<OsString>,
    #[arg(long = "exclude-pattern", value_name = "REGEX")]
    exclude_pattern: Option<OsString>,
    #[arg(long = "width", value_name = "WIDTH")]
    width: Option<OsString>,

    #[arg(value_name = "PATH", num_args = 0..)]
    paths: Vec<PathBuf>,
}

fn normalize_args() -> Vec<OsString> {
    let mut normalized: Vec<OsString> = Vec::new();

    for (index, argument) in env::args_os().enumerate() {
        if index == 0 {
            normalized.push(argument);
            continue;
        }

        let normalized_argument = match argument.to_str() {
            Some("-opl") => OsString::from("--one-per-line"),
            Some("-ll") => OsString::from("--long-listing"),
            Some("-gdf") => OsString::from("--group-directories-first"),
            Some("-rev") => OsString::from("--reverse"),
            Some("-sd") => OsString::from("--show-dotfiles"),
            Some("-sb") => OsString::from("--show-backups"),
            Some("-rec") => OsString::from("--recursive"),
            Some("-fs") => OsString::from("--follow-symlinks"),
            _ => argument,
        };

        normalized.push(normalized_argument);
    }

    normalized
}

pub fn parse_user_args() -> (Config, Vec<PathBuf>) {
    let cli = Cli::parse_from(normalize_args());

    let mut config: Config = Config::default();

    config.one_per_line = cli.one_per_line;
    config.is_long_listing = cli.long_listing;
    config.group_directories_first = cli.group_directories_first;
    config.reverse = cli.reverse;
    config.show_dotfiles = cli.show_dotfiles;
    config.show_backups = cli.show_backups;
    config.recursive = cli.recursive;
    config.follow_symlinks = cli.follow_symlinks;

    if let Some(titles) = cli.titles {
        config.titles = subparsers::formatting_args("--titles", titles);
    }
    if let Some(files) = cli.files {
        config.files = subparsers::formatting_args("--files", files);
    }
    if let Some(directories) = cli.directories {
        config.directories = subparsers::formatting_args("--directories", directories);
    }
    if let Some(executables) = cli.executables {
        config.executables = subparsers::formatting_args("--executables", executables);
    }
    if let Some(symlinks) = cli.symlinks {
        config.symlinks = subparsers::formatting_args("--symlinks", symlinks);
    }
    if let Some(unknowns) = cli.unknowns {
        config.unknowns = subparsers::formatting_args("--unknown", unknowns);
    }

    if let Some(files_prefix) = cli.files_prefix {
        config.prefix.files = Some(files_prefix);
    }
    if let Some(directories_prefix) = cli.directories_prefix {
        config.prefix.directories = Some(directories_prefix);
    }
    if let Some(executables_prefix) = cli.executables_prefix {
        config.prefix.executables = Some(executables_prefix);
    }
    if let Some(symlinks_prefix) = cli.symlinks_prefix {
        config.prefix.symlinks = Some(symlinks_prefix);
    }
    if let Some(unknowns_prefix) = cli.unknowns_prefix {
        config.prefix.unknowns = Some(unknowns_prefix);
    }

    if let Some(files_suffix) = cli.files_suffix {
        config.suffix.files = Some(files_suffix);
    }
    if let Some(directories_suffix) = cli.directories_suffix {
        config.suffix.directories = Some(directories_suffix);
    }
    if let Some(executables_suffix) = cli.executables_suffix {
        config.suffix.directories = Some(executables_suffix);
    }
    if let Some(symlinks_suffix) = cli.symlinks_suffix {
        config.suffix.symlinks = Some(symlinks_suffix);
    }
    if let Some(unknowns_suffix) = cli.unknowns_suffix {
        config.suffix.unknowns = Some(unknowns_suffix);
    }

    if let Some(color_seed) = cli.color_seed {
        config.color_seed = subparsers::color_seed(color_seed);
    }
    if let Some(minimal_sum) = cli.minimal_sum {
        config.minimal_rgb_sum = subparsers::minimal_rgb_sum(minimal_sum);
    }
    if let Some(time_formatting) = cli.time_formatting {
        config.time_formatting = time_formatting;
    }
    if let Some(unit_size) = cli.unit_size {
        config.unit_size = subparsers::unit_size(unit_size);
    }
    if let Some(sort_by) = cli.sort_by {
        config.sort_by = subparsers::sort_by(sort_by);
    }
    if let Some(separator) = cli.separator {
        config.separator = separator;
    }
    if let Some(padding) = cli.padding {
        config.padding = subparsers::padding(padding);
    }
    if let Some(include_pattern) = cli.include_pattern {
        config.include_pattern = subparsers::regex_patterns("--include-pattern", include_pattern);
    }
    if let Some(exclude_pattern) = cli.exclude_pattern {
        config.exclude_pattern = subparsers::regex_patterns("--exclude-pattern", exclude_pattern);
    }
    if let Some(width) = cli.width {
        config.term_width = subparsers::width(width);
    }

    let paths = subparsers::default_to_curr_dir(cli.paths);

    (config, paths)
}
