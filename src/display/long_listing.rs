use std::fs::{self, DirEntry};
use std::ffi::OsString;
use std::io::Error;
use std::os::unix::fs::{MetadataExt, PermissionsExt};

use libc;

use crate::types::{ColoredEntry, Config, LongListingEntry};


fn numeral_amount(mut num: usize) -> usize {
    let mut res: usize = 0;

    while num >= 1 {
        num  /= 10;
        res += 1;
    }

    res
}


pub fn show(coloured_entries: Vec<ColoredEntry>, config: &Config) -> Vec<Error> {
    let mut long_listing_entries: Vec<LongListingEntry> = Vec::new();
    let mut new_errors: Vec<Error> = Vec::new();

    let mut max_lens: [usize; 5] = [0, 0, 0, 0, 0];
    
    for coloured_entry in coloured_entries {
        match fs::metadata(&coloured_entry.path) {
            Ok(meta) => {
                let ll_entry: LongListingEntry = LongListingEntry::new(coloured_entry, meta, config);

                let nums_amount: usize = numeral_amount(ll_entry.hard_link_count as usize);
                max_lens[0] = max_lens[0].max(nums_amount);

                if let Some(owner) = &ll_entry.owner {
                    let owner_len: usize = owner.len();
                    max_lens[1] = max_lens[1].max(owner_len);
                }

                if let Some(group) = &ll_entry.group {
                    let grp_len: usize = group.len();
                    max_lens[2] = max_lens[2].max(grp_len);
                }

            },
            Err(err) => new_errors.push(err),
        }
    }

    new_errors
}