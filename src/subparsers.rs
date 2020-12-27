use std::process;
use std::env;

use std::ffi::OsString;
use std::path::PathBuf;

use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

use crate::types::{SortingReference, SizeMeasurementUnit};

/// TODO: Centralise everything so the err message is shown in red

// Flags

pub fn print_help() {
    println!("Help");
    process::exit(0);
}

// Kwargs

fn handle_digit(mut ret: Vec<u8>, digit: u32, left: &str, chr: char) -> Vec<u8> {
    if (0..9).contains(&digit) {
        ret.push(digit as u8);
    } else {
        eprintln!(r#"[{}] Expected a digit between 0 and 9 inclusive, got: {}."#, left, chr);
        process::exit(1);
    }
    ret
}

pub fn formatting_args(left: &str, right: OsString) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();

    for chr in right.to_string_lossy().chars() {
        if let Some(digit) = chr.to_digit(10) {
            ret = handle_digit(ret, digit, left, chr);
        } else {
            eprintln!(r#"[{}] Failed to convert "{}" to a digit."#, left, chr);
            process::exit(1);
        }
    }

    ret
}

fn check_sum_bounds(sum: u16) -> u16 {
    if sum < 765 {
        sum
    } else {
        eprintln!(r#"The provided minimal is too sum, max: 765, got: "{}"."#, sum);
        process::exit(1);
    }
}

pub fn minimal_rgb_sum(right: OsString) -> u16 {
    let lossy_right: &str = &right.to_string_lossy();

    if let Ok(sum) = lossy_right.parse::<u16>() {
        check_sum_bounds(sum)
    } else {
        eprintln!(r#"Failed to parse the minimal sum as a positive number, got: "{}"."#, lossy_right);
        process::exit(1);
    }
}

pub fn unit_size(right: OsString) -> SizeMeasurementUnit {
    let lossy_right: &str = &right.to_string_lossy();

    match lossy_right.to_lowercase().as_str() {
        "bytes" => SizeMeasurementUnit::Bytes,
        "bits" => SizeMeasurementUnit::Bits,
        _ => {
            eprintln!(r#"Failed to convert "{}" to a valid size unit (bytes/bits)."#, lossy_right);
            process::exit(1);
        },
    }
}

pub fn padding(right: OsString) -> OsString {

    let lossy_right: &str = &right.to_string_lossy();

    if lossy_right.grapheme_indices(true).count() == 1 {
        right
    } else {
        eprintln!(r#"Failed to parse "{}" as a valid padding char."#, lossy_right);
        process::exit(1);
    }
}

pub fn sort_by(right: OsString) -> SortingReference {
    let lossy_right: &str = &right.to_string_lossy();

    match lossy_right.to_lowercase().as_str() {
        "name" => SortingReference::Name,
        "size" => SortingReference::Size,
        "extension" => SortingReference::Extension,
        "creation_date" | "creationdate" => SortingReference::CreationDate,
        "access_date" | "accesdate" => SortingReference::AccessDate,
        "modification_date" | "ModificationDate" => SortingReference::ModificationDate,
        "color" | "colour" => SortingReference::Colour,
        _ => {
            eprintln!(
                r#"Unrecognized sort type: "{}" not contained in: \
                [name, size, extension, color / colour, creation_date / CreationDate, \
                access_date / AccessDate, modification_date / ModificationDate].
                "#, lossy_right);
                
            process::exit(1);
        }
    }
}

pub fn regex_patterns(left: &str, right: OsString) -> Option<Regex> {
    let lossy_right: &str = &right.to_string_lossy();

    let regex = Regex::new(lossy_right);

    match regex {
        Ok(re) => Some(re),
        Err(error) => {
            eprintln!(r#"[{}] Failed to compile "{}" into a valid regex."#, left, lossy_right);
            eprintln!(r#"Error: "{}""#, error);
            process::exit(1);
        }
    }
}

pub fn width(right: OsString) -> Option<usize> {
    let lossy_right = right.to_string_lossy();
    if let Ok(w) = lossy_right.parse::<usize>() {
        Some(w)
    } else {
        eprintln!(r#"Failed to convert "{}" to a valid width."#, lossy_right);
        process::exit(1);
    }

}

pub fn default_to_curr_dir(mut untreated_args: Vec<PathBuf>) -> Vec<PathBuf> {
    let curr_exe: &PathBuf = &env::current_exe().unwrap_or_else(|_| PathBuf::new());

    for path_buf in untreated_args.iter() {

        if path_buf == curr_exe {
            continue;
        }

        if path_buf.is_dir() {
            untreated_args.sort_unstable();
            return untreated_args;
        }
    }

    if let Ok(curr_dir) = env::current_dir() {
        vec![curr_dir]

    } else {
        eprintln!("Couldn't find any directories and couldn't default to the current one either");
        process::exit(1);
    }
}

// Some more error messages
pub fn unrecognized_flag(left_arg: &str) {
    eprintln!("Unrecognized flag: {}", left_arg);
    process::exit(1);
}

pub fn unrecognized_kwarg(left_arg: &str) {
    eprintln!(r#"Unrecognized keyword argument: "{}"."#, left_arg);
    process::exit(1);
}

pub fn unfilled_argument(left_arg: &str) {
    eprintln!("Unfilled argument for: {}", left_arg);
    process::exit(1);
}
