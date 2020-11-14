use std::env;

mod parser;





fn main() {
    let config = parser::config::Config::new();
    println!("{:?}", config);
    
}