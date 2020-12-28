use std::borrow::Cow;
use crate::types::ColoredEntry;

pub fn show(colored_entries: Vec<ColoredEntry>) {
    for entry in colored_entries {
        let lossy_name: Cow<str> = entry.formatted_name.to_string_lossy();
        println!("{}", lossy_name);
    }
}

