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
    match result {
        0  => Ok(()),
        -1 => Err(getStrError()),
        _  => Err("Unexpected error in mount::mount!".to_string()),
    }
}
/*
pub unsafe extern fn mount(src: *const c_char,
                           target: *const c_char,
                           fstype: *const c_char,
                           flags: c_ulong,
                           data: *const c_void)
                           -> c_int
*/
pub fn umount(mi: &MountInfo) -> Result<(), String> {
    //
    //TODO
    //
    Ok(())
}
//////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////
#[test]
fn test_mount_01() {
    let mi = MountInfo{source: "/dev/sdb1",
                       target: "/mnt/temp",
                       filesystem: "ext4"};
    let res = mount(&mi);
    //assert_eq!(false, i.verify_label(".*ANC.*"));
}

#[test]
#[ignore]
fn test_mount_02() {
    let mi = MountInfo{source: "/dev/sdb1",
                       target: "/mnt/temp",
                       filesystem: "ext4"};
    let res = umount(&mi);
    //assert_eq!(false, i.verify_label(".*ANC.*"));
}
//////////////////////////////////////////////////////////////////
