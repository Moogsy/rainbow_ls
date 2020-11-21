use std::env;

mod args;
mod display;

fn main() {
    let config: args::Config = args::Config::from(env::args());

    for read_dir in config.ok_dirs {
    }
}