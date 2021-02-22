use std::borrow::Cow;
use crate::types::ColouredEntry;

pub fn show(colored_entries: Vec<ColouredEntry>) {
    for entry in colored_entries {
        let lossy_name: Cow<str> = entry.formatted_name.to_string_lossy();
        println!("{}", lossy_name);
    }
}

