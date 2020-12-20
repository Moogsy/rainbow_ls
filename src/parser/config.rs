use std::path::PathBuf;
use std::fs::ReadDir;
use std::process;
use term_size;

use super::subparsers::{self, SortBy};
use super::help;

#[derive(Debug)]
pub struct Config {
    // Kwargs
    pub files: Vec<u8>,
    pub directories: Vec<u8>,
    pub symlinks: Vec<u8>,
    pub unknowns: Vec<u8>,

    pub files_suffix: String,
    pub dotfiles_suffix: String,
    pub directories_suffix: String,
    pub symlinks_suffix: String,
    pub unknowns_suffix: String,

    pub min_rgb_sum: u16,
    pub sort_by: SortBy,

    pub separator: String,
    pub padding: char,

    // Flags
    pub show_dotfiles: bool,
    pub reverse_file_order: bool,
    pub group_directories_first: bool,
    
    // Auto-generated
    pub term_width: usize,
    pub color_seed: u32,
}


impl Default for Config {
    fn default() -> Self {
        if let Some((width, _)) = term_size::dimensions() {
            Self {
                // Kwargs
                files: vec![1],
                directories: vec![1, 7],
                symlinks: vec![1, 3],
                unknowns: vec![1, 4],

                files_suffix: String::from(""),
                directories_suffix: String::from(""),
                symlinks_suffix: String::from(""),
                unknowns_suffix: String::from(""),
                dotfiles_suffix: String::from(""),

                min_rgb_sum: 512,
                sort_by: SortBy::Name,

                separator: String::from("  "),
                padding: ' ',
                
                // Flags
                show_dotfiles: false,
                reverse_file_order: false,
                group_directories_first: true,

                // Auto generated
                term_width: width,
                color_seed: (&vec![7, 14] as *const Vec<i32> as i32).abs().max(3) as u32,
            }
        } else {
            eprintln!("Failed to get term's width");
            process::exit(1);
        }
    }
}

#[derive(Debug, Default)]
pub struct PassedFiles {
    pub ok_dirs: Vec<ReadDir>,
    pub err_dirs: Vec<PathBuf>,
}

fn dispatch_bool_arg(d_config: &mut Config, arg: &String) -> Result<(), ()> {
    match arg.as_str() {
        "--help" => {
            println!("{}", help::TXT);
            process::exit(0);
        },
        "--show-dotfiles" | "--all" | "-a" => {
            d_config.show_dotfiles = true;
        },
        "-r" | "--reverse" => {
            d_config.reverse_file_order = true;
        },
        "-dgdr" | "--dont-group-directories-first" => {
            d_config.group_directories_first = false;
        },
        _ => {
            return Err(());
        },
    }
    Ok(())
}

fn dispatch_keyword_arg<T>(d_config: &mut Config, 
                           left_arg: String, 
                           right_arg: String, 
                           untreated_args: &mut Vec<String>, 
                           args_iter: &mut T)
where T: Iterator<Item = String> {

    match left_arg.as_str() {
        "--files" => {
            subparsers::formatting_args(&mut d_config.files, right_arg)
        },
        "--directories" => {
            subparsers::formatting_args(&mut d_config.directories, right_arg)
        },
        "--symlinks" => {
            subparsers::formatting_args(&mut d_config.symlinks, right_arg)
        },
        "--unknowns" => {
            subparsers::formatting_args(&mut d_config.unknowns, right_arg)
        },

        "--files-suffix" => {
            d_config.files_suffix = right_arg;
        },
        "--dotfiles-suffix" => {
            d_config.dotfiles_suffix = right_arg;
        },
        "--directories-suffix" => {
            d_config.directories_suffix = right_arg;
        },
        "--symlinks-suffix" => {
            d_config.symlinks_suffix = right_arg;
        },
        "--unknowns-suffix" => {
            d_config.unknowns_suffix = right_arg;
        },

        "--sum" => {
            subparsers::minimal_sum(&mut d_config.min_rgb_sum, right_arg)
        },

        "--separator" => {
            d_config.separator = right_arg;
        },
        "--padding" => {
            subparsers::padding(&mut d_config.padding, right_arg);
        },

        "--sort-by" => {
            subparsers::sort_by(&mut d_config.sort_by, right_arg);
        },
        
        "--" => {
            untreated_args.push(right_arg);
            subparsers::consume_rest(untreated_args,  args_iter)
        },
        unknown => {
            eprintln!("Unrecognized argument: {}", unknown);
            process::exit(1);
        },
    }
} 

fn generate_config<T>(string_iter: T) -> (Config, Vec<String>)
where T: Iterator<Item = String> {
    let mut args_iter: T = string_iter.into_iter();
    let mut d_config: Config = Config::default();
    let mut untreated_args: Vec<String> = Vec::new();

    while let Some(left_arg) = args_iter.next() {

        // Likely a file / dir path
        if !left_arg.starts_with("-") {
            untreated_args.push(left_arg);
            continue;
        }

        // Skip if the arg is self sufficient
        if dispatch_bool_arg(&mut d_config, &left_arg).is_ok() {
            continue;
        }
        
        if let Some(right_arg) = args_iter.next() {
            dispatch_keyword_arg(&mut d_config, left_arg, right_arg, &mut untreated_args, &mut args_iter);
        } else {
            eprintln!("Unfilled argument: {}", left_arg);
            process::exit(1);
        }
    } 
    (d_config, untreated_args)
}

pub fn get_user_input<T>(string_iter: T) -> (Config, PassedFiles)
where T: Iterator<Item = String> {
    let (config, untreated_args): (Config, Vec<String>) = generate_config(string_iter);

    let (ok_dirs, err_dirs): (Vec<ReadDir>, Vec<PathBuf>) = subparsers::dispatch_untreated_args(untreated_args);

    let passed_files: PassedFiles = PassedFiles {ok_dirs, err_dirs};

    (config, passed_files)
}