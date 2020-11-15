use std::env;
use std::path;
use std::process;

use super::help;
use super::subparsers;


#[derive(Debug)]
pub struct Config {
    ok_dirs: Vec<path::PathBuf>,
    err_dirs: Vec<path::PathBuf>,

    files: Vec<u8>,
    directories: Vec<u8>,
    symlinks: Vec<u8>,

    args_iter: env::Args,
    min_rgb_sum: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ok_dirs: Vec::new(),
            err_dirs: Vec::new(),
            files: vec![1],
            directories: vec![2],
            symlinks: vec![3],
            args_iter: env::args(),
            min_rgb_sum: 255,
        }
    }
}

impl Config {

    pub fn new() -> Self {
        let mut d_self: Config = Self::default();
        let mut untreated_args: Vec<String> = Vec::new();

        while let Some(left_arg) = d_self.args_iter.next() {
            if !left_arg.starts_with("--") {
                untreated_args.push(left_arg);
                continue;
            }
            if left_arg == "--help" {
                println!("{}", help::TXT);
                process::exit(0);
            }
            if let Some(right_arg) = d_self.args_iter.next() {
                match left_arg.as_str() {
                    "--file" | "--files" => {
                        subparsers::formatting_args(&mut d_self.files, right_arg);
                    },
                    "--dir" | "--directory" | "--dirs" | "--directories" => {
                        subparsers::formatting_args(&mut d_self.directories, right_arg);
                    },
                    "--symlink" | "--symlinks" => {
                        subparsers::formatting_args(&mut d_self.symlinks, right_arg);
                    },
                    "--sum" => {
                        subparsers::minimal_sum(&mut d_self.min_rgb_sum, right_arg)
                    },
                    "--" => {
                        subparsers::consume_rest(&mut untreated_args, &mut d_self.args_iter);
                        untreated_args.push(right_arg);
                    }
                    unknown => panic!("Unrecognized argument: {}", unknown),
                }
            } else {
                panic!("Unfilled argument: {}", left_arg);
            }
        } 

        subparsers::untreated_args_to_pathbuf(&mut d_self.ok_dirs, &mut d_self.err_dirs, &mut untreated_args);

        d_self
    }
}
