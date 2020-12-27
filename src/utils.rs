use std::ffi::OsString;

use unicode_segmentation::UnicodeSegmentation;

pub fn os_string_graphene_len(os_string: &OsString) ->  usize {
    os_string.to_string_lossy().graphemes(true).count()
}