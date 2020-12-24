mod config;
use config::Config;

mod parser;
mod subparsers;


fn main() {
    let config: Config = parser::get_user_config();
    println!("{:#?}", config);
}
