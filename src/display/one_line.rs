use std::borrow::Cow;
use crate::types::{ColouredEntry, Config};

pub fn show(colored_entries: Vec<ColouredEntry>, config: &Config) {

    let lossy_sep: Cow<str> = config.separator.to_string_lossy();

    for entry in colored_entries {
        let lossy_name: Cow<str>  = entry.formatted_name.to_string_lossy();
        print!("{}{}", lossy_name, lossy_sep);
    }
    println!();
}