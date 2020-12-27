use std::ffi::OsString;

use unicode_segmentation::UnicodeSegmentation;

pub fn os_string_graphene_len(os_string: &OsString) ->  usize {
    os_string.to_string_lossy().graphemes(true).count()
}

pub fn starts_with_lowercase(os_string: &OsString) -> bool {
    if let Some(chr) = os_string.to_string_lossy().chars().next() {
        chr.is_ascii_lowercase()
    } else {
        false
    }
}