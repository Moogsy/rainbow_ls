use std::path;
use std::fs;
use std::process;

use super::help;
use super::subparsers;

#[derive(Debug)]
pub struct Config {
    pub ok_dirs: Vec<fs::ReadDir>,
    pub err_dirs: Vec<path::PathBuf>,

    pub files: Vec<u8>,
    pub directories: Vec<u8>,
    pub symlinks: Vec<u8>,
    pub unknowns: Vec<u8>,

    pub min_rgb_sum: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ok_dirs: Vec::new(),
            err_dirs: Vec::new(),

            files: vec![1],
            directories: vec![2],
            symlinks: vec![3],
            unknowns: vec![4],
            
            min_rgb_sum: 255,
        }
    }
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
        "--sum" => {
            subparsers::minimal_sum(&mut d_config.min_rgb_sum, right_arg)
        },
        "--" => {
            untreated_args.push(right_arg);
            subparsers::consume_rest(untreated_args,  args_iter)
        }
        unknown => {
            eprintln!("Unrecognized argument: {}", unknown);
            process::exit(1);
        },
    }
} 

impl<T> From<T> for Config where T: IntoIterator<Item = String> {
    fn from(string_iter: T) -> Self {
        let mut args_iter = string_iter.into_iter();
        let mut d_config: Config = Config::default();
        let mut untreated_args: Vec<String> = Vec::new();

        while let Some(left_arg) = args_iter.next() {
            if !left_arg.starts_with("--") {
                untreated_args.push(left_arg);
                continue;
            }
            if left_arg == "--help" {
                println!("{}", help::TXT);
                process::exit(0);
            }
            
            if let Some(right_arg) = args_iter.next() {
                dispatch_keyword_arg(&mut d_config, left_arg, right_arg, &mut untreated_args, &mut args_iter);
            } else {
                eprintln!("Unfilled argument: {}", left_arg);
                process::exit(1);
            }
        } 
        subparsers::untreated_args_to_pathbuf(&mut d_config.ok_dirs, &mut d_config.err_dirs, untreated_args);
        d_config
    }
}