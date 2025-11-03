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
    /// Display each entry on its own line.
    ///
    /// When enabled the listing switches to a single-column layout, mirroring the behaviour of
    /// `ls -1`. This is handy when piping the output or when filenames are too wide for the
    /// column view.
    #[arg(short = '1', long = "one-per-line", action = ArgAction::SetTrue)]
    one_per_line: bool,
    /// Request the long-listing layout.
    ///
    /// Parsed for compatibility with the classic `ls -l` view. The current renderer keeps the
    /// compact layouts, but the option is preserved for future expansion.
    #[arg(long = "long-listing", alias = "ll", action = ArgAction::SetTrue)]
    long_listing: bool,
    /// List directories before any other entry type.
    ///
    /// Directories are grouped ahead of regular files while the selected sort field is still used
    /// inside each group. This matches the behaviour of `ls --group-directories-first`.
    #[arg(long = "group-directories-first", alias = "gdf", action = ArgAction::SetTrue)]
    group_directories_first: bool,
    /// Reverse the computed sort order.
    ///
    /// Apply this after any other ordering rule so the last entry becomes the first one displayed.
    #[arg(long = "reverse", alias = "rev", action = ArgAction::SetTrue)]
    reverse: bool,
    /// Include dot-prefixed entries in the output.
    ///
    /// Hidden files and directories (those starting with `.`) are ignored by default; this flag
    /// forces them to be listed.
    #[arg(long = "show-dotfiles", alias = "sd", action = ArgAction::SetTrue)]
    show_dotfiles: bool,
    /// Include editor backup files in the listing.
    ///
    /// Files ending with `~` are skipped unless this flag is provided.
    #[arg(long = "show-backups", alias = "sb", action = ArgAction::SetTrue)]
    show_backups: bool,
    /// Recurse into each directory that is encountered.
    ///
    /// Every directory is visited depth-first and printed with its own header before the entries
    /// are shown.
    #[arg(long = "recursive", alias = "rec", action = ArgAction::SetTrue)]
    recursive: bool,
    /// Follow symbolic links when recursing.
    ///
    /// Symlinks are resolved and their targets enqueued when traversal is recursive. This is
    /// ignored when recursion itself is disabled.
    #[arg(long = "follow-symlinks", alias = "fs", action = ArgAction::SetTrue)]
    follow_symlinks: bool,

    /// ANSI style codes to apply to directory headers.
    ///
    /// Provide digits (`0-9`) that map to Select Graphic Rendition attributes, such as `1` for
    /// bold or `4` for underline. Each digit is sent as its own escape code. The value is stored
    /// for upcoming improvements to recursive directory headings.
    #[arg(long = "titles", value_name = "DIGITS")]
    titles: Option<OsString>,
    /// ANSI style codes to apply to regular files.
    ///
    /// Supply digits (`0-9`) representing ANSI SGR attributes (for example `1` for bold). Each
    /// digit becomes an individual escape sequence.
    #[arg(long = "files", value_name = "DIGITS")]
    files: Option<OsString>,
    /// ANSI style codes to apply to directories.
    ///
    /// Accepts digits (`0-9`) that are interpreted as separate SGR attributes.
    #[arg(long = "directories", value_name = "DIGITS")]
    directories: Option<OsString>,
    /// ANSI style codes to apply to executables.
    ///
    /// Accepts digits (`0-9`) that are interpreted as separate SGR attributes.
    #[arg(long = "executables", value_name = "DIGITS")]
    executables: Option<OsString>,
    /// ANSI style codes to apply to symbolic links.
    ///
    /// Accepts digits (`0-9`) that are interpreted as separate SGR attributes.
    #[arg(long = "symlinks", value_name = "DIGITS")]
    symlinks: Option<OsString>,
    /// ANSI style codes to apply to entries that do not fit any other category.
    ///
    /// Accepts digits (`0-9`) that are interpreted as separate SGR attributes.
    #[arg(long = "unknown", value_name = "DIGITS")]
    unknowns: Option<OsString>,

    /// Prefix inserted before regular file names.
    ///
    /// The string is emitted immediately before the coloured name of each matching entry.
    #[arg(long = "files-prefix", value_name = "STRING")]
    files_prefix: Option<OsString>,
    /// Prefix inserted before directory names.
    ///
    /// Applied before the coloured directory name, letting you add glyphs or contextual labels.
    #[arg(long = "directories-prefix", value_name = "STRING")]
    directories_prefix: Option<OsString>,
    /// Prefix inserted before executable file names.
    ///
    /// Useful for decorating binaries with an indicator or emoji.
    #[arg(long = "executables-prefix", value_name = "STRING")]
    executables_prefix: Option<OsString>,
    /// Prefix inserted before symbolic link names.
    ///
    /// Often used to add arrows that highlight the indirection.
    #[arg(long = "symlinks-prefix", value_name = "STRING")]
    symlinks_prefix: Option<OsString>,
    /// Prefix inserted before names of entries that fall back to the unknown style.
    ///
    /// Lets you annotate file types that do not match any other category.
    #[arg(long = "unknowns-prefix", value_name = "STRING")]
    unknowns_prefix: Option<OsString>,

    /// Suffix appended after regular file names.
    ///
    /// Printed right after the coloured name; pair with prefixes to wrap entries.
    #[arg(long = "files-suffix", value_name = "STRING")]
    files_suffix: Option<OsString>,
    /// Suffix appended after directory names.
    ///
    /// Classic `ls` users can set this to `/` to mimic its output.
    #[arg(long = "directories-suffix", value_name = "STRING")]
    directories_suffix: Option<OsString>,
    /// Suffix appended after executable file names.
    ///
    /// Use this to flag binaries with punctuation or icons.
    #[arg(long = "executables-suffix", value_name = "STRING")]
    executables_suffix: Option<OsString>,
    /// Suffix appended after symbolic link names.
    ///
    /// Handy for reproducing the familiar `@` suffix.
    #[arg(long = "symlinks-suffix", value_name = "STRING")]
    symlinks_suffix: Option<OsString>,
    /// Suffix appended after entries that use the unknown style.
    ///
    /// Offers a fallback for everything that doesn't match the other categories.
    #[arg(
        long = "unkowns-suffix",
        value_name = "STRING",
        visible_alias = "unknowns-suffix"
    )]
    unknowns_suffix: Option<OsString>,

    /// Seed value used to derive deterministic RGB colours from file names.
    ///
    /// Different seeds shuffle the palette that is generated from extensions while keeping the
    /// same colour per file type during a run.
    #[arg(long = "color-seed", value_name = "SEED")]
    color_seed: Option<OsString>,
    /// Minimum brightness allowed for generated colours.
    ///
    /// Colours are brightened until the sum of their RGB components reaches this number (capped at
    /// 765). Increasing the value makes entries easier to read on dark backgrounds.
    #[arg(long = "sum", value_name = "SUM")]
    minimal_sum: Option<OsString>,
    /// Custom timestamp format used for long listings.
    ///
    /// Accepts `strftime`-style patterns and is retained for the future long-listing renderer.
    #[arg(long = "time-formatting", value_name = "FORMAT")]
    time_formatting: Option<OsString>,
    /// Measurement unit used when displaying file sizes.
    ///
    /// Choose between `bytes` and `bits` for size columns once the long-listing view is available.
    #[arg(long = "unit-size", value_name = "UNIT")]
    unit_size: Option<OsString>,
    /// Field used to sort entries.
    ///
    /// Supported values include `name`, `size`, `extension`, `color`, `creation_date`,
    /// `access_date`, and `modification_date`.
    #[arg(long = "sort-by", value_name = "FIELD")]
    sort_by: Option<OsString>,
    /// String inserted between columns.
    ///
    /// This value is printed between entries when using the multi-column layouts.
    #[arg(long = "separator", value_name = "SEP")]
    separator: Option<OsString>,
    /// Padding character used to align columns.
    ///
    /// Must be a single Unicode grapheme cluster; it is repeated until each column reaches its
    /// calculated width.
    #[arg(long = "padding", value_name = "CHAR")]
    padding: Option<OsString>,
    /// Regular expression used to force inclusion of specific entries.
    ///
    /// Only files whose names match the pattern are displayed when this option is provided.
    #[arg(long = "include-pattern", value_name = "REGEX")]
    include_pattern: Option<OsString>,
    /// Regular expression used to hide entries.
    ///
    /// Any file whose name matches the pattern is skipped.
    #[arg(long = "exclude-pattern", value_name = "REGEX")]
    exclude_pattern: Option<OsString>,
    /// Manually override the detected terminal width.
    ///
    /// Supplying a width in columns affects the heuristics that pick one-line versus multi-column
    /// layouts.
    #[arg(long = "width", value_name = "WIDTH")]
    width: Option<OsString>,

    /// Paths to list.
    ///
    /// If omitted the current working directory is used. Multiple paths can be supplied.
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
