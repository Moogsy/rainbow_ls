use std::fs;
use std::io;
use super::filetype;

pub fn display_read_dir(read_dir: fs::ReadDir) {
    let mut errors: Vec<io::Error> = Vec::new();
    let mut entries: Vec<filetype::Entry> = Vec::new();

    for read_dir_entry in read_dir {
        match read_dir_entry {
            Ok(dir_entry) => {
                let entry: filetype::Entry = filetype::Entry::from(dir_entry);
                entries.push(entry);
            },
            Err(error) => errors.push(error),
        }
    }

}