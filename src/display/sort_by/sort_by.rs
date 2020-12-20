use crate::parser::{Config, SortBy};
use super::super::filetype::{Entry, Kind};


pub fn default(config: &Config, mut entries: Vec<Entry>) -> Vec<Entry> { 
    match config.sort_by {
        SortBy::Name => entries.sort_unstable(),
        SortBy::Size => entries.sort_unstable_by_key(|entry| entry.size),
        SortBy::CreationDate => entries.sort_unstable_by_key(|entry| entry.created_at),
        SortBy::AccessDate => entries.sort_unstable_by_key(|entry| entry.accessed_at),
        SortBy::ModificationDate => entries.sort_unstable_by_key(|entry| entry.edited_at),
        SortBy::Extension => {
            entries.sort(); // Expensive way to avoid conflicts
            entries.sort_by_key(|entry| entry.extension.clone())
        }
    }
    entries
}
pub fn groupdirs(config: &Config, mut entries: Vec<Entry>) -> Vec<Entry> { 
    match config.sort_by {
        SortBy::Name => entries.sort_unstable(),
        SortBy::Size => entries.sort_unstable_by_key(|entry| (entry.kind != Kind::Directory, entry.size)),
        SortBy::CreationDate => entries.sort_unstable_by_key(|entry| (entry.kind != Kind::Directory, entry.created_at)),
        SortBy::AccessDate => entries.sort_unstable_by_key(|entry| (entry.kind != Kind::Directory, entry.accessed_at)),
        SortBy::ModificationDate => entries.sort_unstable_by_key(|entry| (entry.kind != Kind::Directory, entry.edited_at)),
        SortBy::Extension => {
            entries.sort(); // Expensive way to avoid conflicts
            entries.sort_by_key(|entry| entry.extension.clone());
        }
    }
    entries
}
