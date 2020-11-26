use std::env;

mod parser;
mod display;

fn main() {
    let args: env::Args = env::args();

    let (config, passed_files): (parser::Config, parser::PassedFiles) = parser::get_user_input(args);

    for read_dir in passed_files.ok_dirs {
        display::read_dir(&config, read_dir);
    }


    


}