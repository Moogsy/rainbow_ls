use std::env::{self, ArgsOs};
use std::ffi::OsString;
use std::path::PathBuf;

use crate::Config;
use crate::subparsers;

// There must be a way to add functions in hashmaps without any hacky stuff
// But for now, let's leave it like that, that gigantic match mess

fn dispatch_flag_arg(config: &mut Config, arg: &str) -> Result<(), ()> {
    match arg {
        "--help" => {
            subparsers::print_help();
        },
        "-1" | "-opl" | "--one-per-line" => {
            config.one_per_line = true;
        },
        "-ll" | "--long-listing" => {
            config.is_long_listing = true;
        },
        "-gdf" | "--group-directories-first" => {
            config.group_directories_first = true;
        },
        "-rev"  | "--reverse" => {
            config.reverse = true; 
        },
        "-sd" | "--show-dotfiles" => {
            config.show_dotfiles = true;
        },
        "-sb" | "--show-backups" => {
            config.show_backups = true;
        },
        "-rec" | "--recursive" => {
            config.recursive = true;
        },
        "-fs" | "--follow-symlinks" => {
            config.follow_symlinks = true;
        },
      _ => return Err(()),
    }

    Ok(())

}

fn dispatch_keyword_arg(mut config: Config, left: &str, right: OsString) -> Config {
    match left {
        "--titles" => {
            config.titles = subparsers::formatting_args(left, right);
        },
        "--files" => {
            config.files = subparsers::formatting_args(left, right);
        },
        "--directories" => {
            config.directories = subparsers::formatting_args(left, right);
        },
        "--executables" => {
            config.executables = subparsers::formatting_args(left, right);
        },
        "--symlinks" => {
            config.symlinks = subparsers::formatting_args(left, right);
        },
        "--unknown" => {
            config.unknowns = subparsers::formatting_args(left, right);
        },
        "--files-prefix" => {
            config.prefix.files = Some(right);
        },
        "--directories-prefix" => {
            config.prefix.directories = Some(right);
        },
        "--executables-prefix" => {
            config.prefix.executables = Some(right);
        },
        "--symlinks-prefix" => {
            config.prefix.symlinks = Some(right);
        },
        "--unknowns-prefix" => {
            config.prefix.unknowns = Some(right);
        },
        "--files-suffix" => {
            config.suffix.files = Some(right)
        },
        "--directories-suffix" => {
            config.suffix.directories = Some(right);
        },
        "--executables-suffix" => {
            config.suffix.directories = Some(right);
        },
        "--symlinks-suffix" => {
            config.suffix.symlinks = Some(right);
        },
        "--unkowns-suffix" => {
            config.suffix.unknowns = Some(right);
        },
        "--color-seed" => {
            config.color_seed = subparsers::color_seed(right) 
        },
        "--sum" => {
            config.minimal_rgb_sum = subparsers::minimal_rgb_sum(right);
        },
        "--time-formatting" => {
            config.time_formatting = right;
        },
        "--unit-size" => {
            config.unit_size = subparsers::unit_size(right);
        },
        "--sort-by" => {
            config.sort_by = subparsers::sort_by(right);
        },
        "--separator" => {
            config.separator = right;
        },
        "--padding" => {
            config.padding = subparsers::padding(right);
        },
        "--include-pattern" => {
            config.include_pattern = subparsers::regex_patterns(left, right);
        },
        "--exclude-pattern" => {
            config.exclude_pattern = subparsers::regex_patterns(left, right);
        },
        "--width" => {
            config.term_width = subparsers::width(right);
        },
        _ => {
            subparsers::unrecognized_kwarg(left);
        },
    }
    config
}

pub fn parse_user_args() -> (Config, Vec<PathBuf>) {
    let mut config: Config = Config::default();
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut args_iter: ArgsOs = env::args_os();

    while let Some(os_left) = args_iter.next() {

        let left: &str = &os_left.to_string_lossy(); 

        if !left.starts_with("-") {
            let pathbuf: PathBuf = PathBuf::from(os_left);
            paths.push(pathbuf);
            continue;
        }
        if dispatch_flag_arg(&mut config, left).is_ok() {
            continue; 
        }
        if !left.starts_with("--") {
            subparsers::unrecognized_flag(left);
        }
        if let Some(os_right) = args_iter.next() {
            config = dispatch_keyword_arg(config , left, os_right);
        } else {
            subparsers::unfilled_argument(left);
        }
    }
    paths = subparsers::default_to_curr_dir(paths);
    
    (config, paths)
}


