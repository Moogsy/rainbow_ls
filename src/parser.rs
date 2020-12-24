use std::env::{self, ArgsOs};
use std::ffi::OsString;

use crate::Config;
use crate::subparsers;

fn dispatch_flag_arg(config: &mut Config, arg: &str) -> Result<(), ()> {
    match arg {
        "--help" => {
            subparsers::print_help();
        },
        "-sd" | "--show-dotfiles" => {
            config.show_dotfiles = true;
        },
        "-sb" | "--show-backups" => {
            config.show_backups = true;
        },
        "-r"  | "--reverse" => {
            config.reverse_output = true; 
        },
        "-gdf" | "--group-directories-first" => {
            config.group_directories_first = true;
        },
        _ => return Err(()),
    }

    Ok(())

}

fn dispatch_keyword_arg(mut config: Config, left: &str, right: OsString) -> Config {
    match left {
        "--files" => {
            config.files = subparsers::formatting_args(left, right);
        },
        "--directories" => {
            config.directories = subparsers::formatting_args(left, right);
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
        "--symlinks-suffix" => {
            config.suffix.symlinks = Some(right);
        },
        "--unkowns-suffix" => {
            config.suffix.unknowns = Some(right);
        },
        "--sum" => {
            config.minimal_rgb_sum = subparsers::minimal_rgb_sum(right);
        },
        "--separator" => {
            config.separator = right;
        },
        "--padding" => {
            config.padding = subparsers::padding(right);
        },
        "--sort-by" => {
            config.sort_by = subparsers::sort_by(right);
        },
        "--width" => {
            config.term_width = Some(subparsers::width(right));
        },
        _ => {
            eprintln!("oh no");
        }
    }
    config
}

pub fn get_user_config() -> Config {

    let mut config: Config = Config::default();
    let mut untreated_args: Vec<OsString> = Vec::new();

    let mut args_iter: ArgsOs = env::args_os();


    // I have no idea about what i'm doing but let's pretend I do 
    while let Some(os_left) = args_iter.next() {

        let left: &str = &os_left.to_string_lossy(); 

        if !left.starts_with("-") {
            untreated_args.push(os_left);
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
    config
}


