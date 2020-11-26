use std::env;

mod parser;
mod display;

fn main() {
    let args: env::Args = env::args();

    let (config, passed_files): (parser::Config, parser::PassedFiles) = parser::get_user_input(args);

    display::show_read_dirs(config, passed_files);

    


}