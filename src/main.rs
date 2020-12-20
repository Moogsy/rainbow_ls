use std::env::{self, Args};

mod parser;
use parser::{Config, PassedFiles};
mod display;

fn main() {
    let args: Args = env::args();

    let (config, passed_files): (Config, PassedFiles) = parser::get_user_input(args);

    for read_dir in passed_files.ok_dirs {
        display::read_dir(&config, read_dir);
    }
}