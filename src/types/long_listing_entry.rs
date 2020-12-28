use std::ffi::{CStr, OsString};
use std::fs::Metadata;
use std::mem;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::time::UNIX_EPOCH;

use chrono::NaiveDateTime;
use libc::{self, group, passwd, size_t};

use crate::types::{ColoredEntry, Config};

const MODE_TABLE: [&str; 8] = ["---", "--x", "-w-", "-wx", "r--", "r-x", "rw-", "rwx"];

pub struct LongListingEntry {
    pub inner: ColoredEntry,
    pub formatted_perms: String,
    pub hard_link_count: u64,
    pub formatted_size: Option<String>,
    pub owner: Option<String>,
    pub group: Option<String>,
    pub last_modified: Option<String>,
}

impl LongListingEntry {
    fn parse_modes(modes: u32) -> String {
        let file_type: &str = {
            match modes & 0o170000 {
                0040000	=> "d",      // Directory.
                0020000	=> "c",      // Character device.
                0060000	=> "b",      // Block device.
                0100000	=> "-",      // Regular file.
                0010000	=> "p",      // FIFO.
                0120000	=> "l",      // Symbolic link.
                0140000	=> "s",      // Socket.
                _ => "?",            // Shoudln't happen
            }
        };

        let mut ret: String = String::from(file_type);

        // TODO: Cleanup that
        let perms: String = format!("{:o}", modes);
        
        for digit_str in perms[perms.len() - 3..].chars() {
            let perm_digit: usize = digit_str.to_digit(10).unwrap_or(0) as usize;
            if perm_digit < 8 {
                ret.push_str(MODE_TABLE[perm_digit])
            } else {
                ret.push_str(MODE_TABLE[0])
            }
        }

        ret
    }

    // Thank you, stranger
    fn get_user_name(uid: u32) -> Option<String> {
        let mut result: *mut passwd = std::ptr::null_mut();
        
        unsafe {
            let amt = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
                n if n < 0 => 512 as usize,
                n => n as usize,
            };
            let mut buf: Vec<i8> = Vec::with_capacity(amt);
            let mut pwd: passwd = mem::zeroed();
    
            let res: i32 = libc::getpwuid_r(uid, &mut pwd, buf.as_mut_ptr(), buf.capacity() as size_t, &mut result);
            
            match res {
                0 if !result.is_null() => {
                    let ptr: *const i8 = pwd.pw_name as *const _;
                    if let Ok(username) = CStr::from_ptr(ptr).to_str() {
                        Some(username.to_string())
                    } else {
                        None
                    }
               },
               _ => None
            }
        }
    }
    fn get_group_name(gid: u32) -> Option<String> {
        let mut result: *mut libc::group = std::ptr::null_mut();
        
        unsafe {
            let amt = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
                n if n < 0 => 512 as usize,
                n => n as usize,
            };
            let mut buf: Vec<i8> = Vec::with_capacity(amt);
            let mut grp: group = mem::zeroed();
    
            let res: i32 = libc::getgrgid_r(gid, &mut grp, buf.as_mut_ptr(), buf.capacity() as libc::size_t, &mut result);
            
            match res {
                0 if !result.is_null() => {
                    let ptr: *const i8 = grp.gr_name as *const _;
                    if let Ok(grpname) = CStr::from_ptr(ptr).to_str() {
                        Some(grpname.to_string())
                    } else {
                        None
                    }
               },
               _ => None
            }
        }
    }


    pub fn new(coloured_entry: ColoredEntry, meta: Metadata, config: &Config) -> Self {
        let modes: u32  = meta.permissions().mode();
        let uid: u32 = meta.uid();
        let gid: u32 = meta.gid(); 
        
        let mut formatted_dt: Option<String> = None;

        if let Some(last_modified) = &coloured_entry.modified_at {

            if let Ok(duration) = last_modified.duration_since(UNIX_EPOCH) {
                let elapsed: i64 = duration.as_secs() as i64;
                let naive_dt: NaiveDateTime = NaiveDateTime::from_timestamp(elapsed, 0);

                let lossy_formatting: &str = &config.time_formatting.to_string_lossy();
                
                let dt_string: String = naive_dt.format(lossy_formatting).to_string();
                formatted_dt = Some(dt_string);
            }
        }

        todo!("Convert the byte size depending on the specifier");

        Self {
            inner: coloured_entry,
            formatted_perms:  Self::parse_modes(modes),
            hard_link_count: meta.nlink(),
            owner: Self::get_user_name(uid),
            group: Self::get_group_name(gid),
            last_modified: formatted_dt,
        }
    }
}