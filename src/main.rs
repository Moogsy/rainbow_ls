mod display;
mod parser;
mod subparsers;
mod types;
use types::Config;

// Some prototyping, looks oof
fn call_recursive(config: &Config) {
    for path_buf in &config.paths {
        for read_dir in path_buf.read_dir() {
            for res_entry in read_dir {
                if let Ok(dir_entry) = res_entry {
                    if let Ok(file_type) = dir_entry.file_type() {
                        if file_type.is_dir() {
                            todo!()
                        }
                    }
                }
            }
        }
    }
}


fn call_non_recursive(config: &Config) {
    for path_buf in &config.paths { 
        for read_dir in path_buf.read_dir() {
            display::display_path(config, &path_buf, read_dir);
        }
    }
}



fn main() {
    let config: Config = parser::get_user_config();
    
    if config.recursive {
        call_recursive(&config);
    } else {
        call_non_recursive(&config);
    }
}


