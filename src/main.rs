use std::env;
use std::path;
use std::fs;
use std::cmp;
use std::ffi;

const SEP: &str = "  ";
const SEP_LEN: usize = SEP.len();

// Mess around with those if you want
const SYMLINK_COLOR: (u8, u8, u8) = (53, 204, 53);
const UNKNOWN_COLOR: (u8, u8, u8) = (255, 0, 0);
const MIN_COLOR_SUM: u32 = 500;


// Used to be in it's own mod, but not feeling like zipping it
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum EntryType {
    Directory,
    File,
    Symlink,
    Unknown,
}

#[derive(Debug)]
pub struct Entry {
    entry: fs::DirEntry,
    type_: EntryType,
    color: (u8, u8, u8),  // rgb
    pub file_name: ffi::OsString,
}

impl Entry {
    /// Determines whether the entry is a file, dir, symlink or unknown
    fn determine_type_from_entry(entry: &fs::DirEntry) -> EntryType {
        match entry.metadata() {
            Ok(metadata) => {
                if metadata.is_file() {
                    EntryType::File
                } else if metadata.is_dir() {
                    EntryType::Directory
                } else {
                    EntryType::Symlink
                }
            },
            Err(_) => EntryType::Unknown,
        }
    }
    
    fn sort_lowest_colors_indexes(lowest_colors_indexes: &mut Vec<usize>, 
                                  max_color_index: usize, 
                                  colors: &[u32; 3]) {

        lowest_colors_indexes.remove(max_color_index);
        if colors[lowest_colors_indexes[0]] > colors[lowest_colors_indexes[1]] {
            lowest_colors_indexes.reverse();
        }
    }
   
    /// Helper function that turns a string into a rgb tuple
    fn determine_color_from_string(string: &mut String) -> (u8, u8, u8) {
        unsafe {
            let mut sum: u32 = 2;
            for n in string.as_mut_vec().iter() {
                sum += *n as u32;
            }
            let (green, blue): (u32, u32) = (sum / 255, sum % 255); // I miss divmod
            let (mut red, green): (u32, u32) = (green / 255, green % 255);
            red %= 255;

            // big oh no moment
            let mut color_sum = red + green + blue;

            if color_sum < MIN_COLOR_SUM {
                let mut colors: [u32; 3] = [red, green, blue];

                let max_color_index: usize = colors
                    .iter()
                    .enumerate()
                    .max_by_key(|&(_, item)| item)
                    .unwrap().0; // we know that it isn't empty, unwrap safely

                let mut lowest_colors_indexes: Vec<usize> = vec![0, 1, 2];
                Entry::sort_lowest_colors_indexes(&mut lowest_colors_indexes, max_color_index, &colors);

                for color_index in lowest_colors_indexes.iter() {
                    let pot_new_color = colors[*color_index] + (MIN_COLOR_SUM - color_sum);
                    if pot_new_color < 255 {
                        colors[*color_index] = pot_new_color;
                        return (colors[0] as u8, colors[1] as u8, colors[2] as u8);

                    } else {
                        colors[*color_index] = u8::MAX as u32;
                        color_sum = colors.iter().sum();
                    }
                }
                return (colors[0] as u8, colors[1] as u8, colors[2] as u8);
                
            } else {
                (red as u8, green as u8, blue as u8)
            }
        }
    }
    
    /// Eh
    fn extension_to_color(entry: &fs::DirEntry) -> (u8, u8, u8) {
        match entry.path().extension() {
            None => {
                let mut fn_str: String = entry.file_name().to_string_lossy().to_string();
                Entry::determine_color_from_string(&mut fn_str)
            },
            Some(ext) => {
                match ext.to_str() {
                    None => UNKNOWN_COLOR,
                    Some(ext_str) => {
                        let mut string: String = String::from(ext_str);
                        Entry::determine_color_from_string(&mut string)
                    }
                }
            }
        }
    }

    /// Determines a color depending on extension
    /// Returns pink if no extension, red if it contains invalid utf8 chars 
    fn determine_color_from_entry(entry: &fs::DirEntry, type_: &EntryType) -> (u8, u8, u8) {
        match type_ {
            &EntryType::Directory => {
                let mut dn_str: String = entry.file_name().to_string_lossy().to_string();
                Entry::determine_color_from_string(&mut dn_str)

            },
            &EntryType::Symlink => SYMLINK_COLOR,
            &EntryType::Unknown => UNKNOWN_COLOR,
            &EntryType::File => {
                Entry::extension_to_color(entry)
           }
        }
    }

    /// New instance of file from fs::Direntry
    pub fn from_read_dir(entry: fs::DirEntry) -> Entry {
        let type_: EntryType = Entry::determine_type_from_entry(&entry);
        let file_name = entry.file_name();
        Entry {
            file_name,
            color: Entry::determine_color_from_entry(&entry, &type_),
            type_,
            entry,
        }
    }
    
    /// Pads the current filename for alignment
    pub fn pad_filename(&mut self, longest_name: usize) -> &mut Entry {
        let filename_len: usize = self.file_name.len();
        let diff: usize = longest_name.max(filename_len) - filename_len;

        for _ in 0..diff {
            self.file_name.push(" ");
        }
        self // don't question this, it works
    }

    /// formats a text, ig, can defo be optimize to reduce the amount of format! calls
    /// 0 - Normal Style
    /// 1 - Bold
    /// 2 - Dim
    /// 3 - Italic
    /// 4 - Underlined
    /// 5 - Blinking
    /// 7 - Reverse
    /// 8 - Invisible
    /// TODO: Handle the padding so the formatting comes first
    fn format_filename(txt: String, codes: Vec<u8>) -> String {
        let mut txt = txt.clone().to_string();
        for code in codes {
            txt = format!("\x1b[{}m{}\x1b[0m", code, txt);
        }
        txt
    }

    /// Returns the filename with it's proper ansi seq
    pub fn get_coloured_file_name(&self) -> String {
        let str_filename: String = self.file_name
            .to_str()
            .unwrap_or("?")
            .to_string();

        let (r, g, b): &(u8, u8, u8) = &self.color;
        let coloured: String = format!(
            "\x1B[38;2;{};{};{}m{}\x1B[0;00m", 
            r, g, b, str_filename);

        match self.type_ {
            EntryType::File => Entry::format_filename(coloured, vec![1]),
            EntryType::Directory => Entry::format_filename(coloured, vec![1, 7]),
            EntryType::Symlink => Entry::format_filename(coloured, vec![3, 4]),
            EntryType::Unknown => Entry::format_filename(coloured, vec![7, 4, 1]),
        }
    }
}
            


impl Eq for Entry {} // empty but needed for some reson

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        (&self.type_, &self.entry.path()) == (&other.type_, &other.entry.path())
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> cmp::Ordering {

        let left_path: &path::PathBuf = &self.entry.path();
        let left_ext: String = {
            match left_path.extension() {
                None => String::from(""),
                Some(ext) => ext.to_str().unwrap_or("").to_string(),
            }
        };

       let right_path: &path::PathBuf = &other.entry.path();
       let right_ext: String = {
           match right_path.extension() {
               None => String::from(""),
               Some(ext) => ext.to_str().unwrap_or("").to_string(),
           }
       };
       (&self.type_, left_ext, left_path).cmp(&(&other.type_, right_ext, right_path))
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}


fn get_metrics(dir_entries: &Vec<Entry>) -> (usize, usize) {
    let mut total_length: usize = SEP_LEN * (dir_entries.len() - 1);
    let mut longest_name: usize = 0;
    for entry in dir_entries.iter() {
        let fn_len = entry.file_name.len();
        total_length += fn_len;
        if longest_name < fn_len {
            longest_name = fn_len;
        }
    }
    (total_length, longest_name)
}
/// Those should be in their own module but idk how to circular import
fn display_one_line(dir_entries: &Vec<Entry>) {
    let mapped: Vec<String> = dir_entries
        .iter()
        .map(Entry::get_coloured_file_name)
        .collect();

    println!("{}", mapped.join(SEP));
}

fn display_multiline(dir_entries: &mut Vec<Entry>, longest_name: usize, term_width: usize) {
    let dir_entries: Vec<&mut Entry> = dir_entries
        .iter_mut()
        .map(|entry| entry.pad_filename(longest_name))
        .collect();
        
    let per_line: usize = term_width / (longest_name + SEP_LEN);

    let mut temp: Vec<String> = Vec::new();

    for entry in dir_entries {
        temp.push(entry.get_coloured_file_name());

        if temp.len() == per_line {
            print!("{}\n", temp.join(SEP));
            temp.clear();
        }
    }
    if !temp.is_empty() {
        print!("{}\n", temp.join(SEP));
    }
}

fn main() {
    let curr_exec_path: path::PathBuf = env::current_dir()
        .expect("Failed to get current exec path");

    // good meme
    let mut dir_entries: Vec<Entry> = curr_exec_path
        .read_dir()
        .expect("Failed to read dir")
        .filter_map(Result::ok)
        .map(Entry::from_read_dir)
        .collect();

    dir_entries.sort();
    
    let term_width: usize = match term_size::dimensions() {
        Some((w, _)) => w,
        None => panic!("Failed to get term size")
    };

    let (total_length, longest_name): (usize, usize) = get_metrics(&dir_entries);

    if total_length <= term_width {
        display_one_line(&dir_entries);
    } else {
        display_multiline(&mut dir_entries, longest_name, term_width);
        // Should be rotate it so entries are sorted in columns instead of lines
    }
}
