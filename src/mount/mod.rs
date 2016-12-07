//
// Copyright (c) 2016, Boris Popov <popov@whitekefir.ru>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

extern crate libc;

use self::libc::c_char;
use self::libc::c_void;
use self::libc::c_ulong;
use self::libc::c_int;
use std::ptr;
use std::ffi::CString;
use strerror::getStrError;

//////////////////////////////////////////////////////////////////
pub struct MountInfo<'a> {
    source:     &'a str,
    target:     &'a str,
    filesystem: &'a str
}

pub fn mount(mi: &MountInfo) -> Result<(), String> {
    let c_source = CString::new(mi.source).unwrap();
    let source_raw = c_source.into_raw();
    let c_target = CString::new(mi.target).unwrap();
    let target_raw = c_target.into_raw();
    let c_filesystem = CString::new(mi.filesystem).unwrap();
    let filesystem_raw = c_filesystem.into_raw();
    let flags: c_ulong = 0;
    let data: *const c_void = ptr::null();
    let mut result: c_int = 0;
    unsafe {
        result = libc::mount(source_raw,
                             target_raw,
                             filesystem_raw,
                             flags,
                             data);
    }
    return get_result(result, "mount");
}

pub fn umount(mi: &MountInfo) -> Result<(), String> {
    let c_target = CString::new(mi.target).unwrap();
    let target_raw = c_target.into_raw();
    let mut result: c_int = 0;
    unsafe {
        result = libc::umount(target_raw);
    }
    return get_result(result, "umount");
}

fn get_result(result: c_int, s: &str) -> Result<(), String> {
    match result {
        0  => Ok(()),
        -1 => Err(getStrError()),
        _  => {
            let mut mess = "Unexpected error in mount::".to_string();
            mess += s;
            mess += "!";
            Err(mess)
        }
    }
}
//////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////
#[test]
fn test_mount_01() {
    let mi = MountInfo{source: "/dev/sdb1",
                       target: "/mnt/temp",
                       filesystem: "ext4"};
    println!("\n *** mount *** \n");
    let res = mount(&mi);
    match res {
        Err(e) => println!("\nMOUNT ERROR: {}", e),
        _      => println!("\nMOUNT OK")
    }

    let res2 = umount(&mi);
    match res2 {
        Err(e) => println!("\nUMOUNT ERROR: {}", e),
        _      => println!("\nUMOUNT OK")
    }
}
//////////////////////////////////////////////////////////////////
