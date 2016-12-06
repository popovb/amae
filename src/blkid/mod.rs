//
// Copyright (c) 2016, Boris Popov <popov@whitekefir.ru>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

extern crate libc;

use std::cell::Cell;
use self::libc::size_t;
use self::libc::c_int;
use self::libc::c_char;
use std::ptr;
use std::ffi::CString;

//////////////////////////////////////////////////////////////////
pub struct Prober<'a> {
    file:       &'a str,
    raw_prober: Cell<*const i32>
}

impl<'a> Prober<'a> {
    pub fn new(filename: &str) -> Prober {
        Prober{file: filename,
               raw_prober: Cell::new(ptr::null())}
    }

    pub fn probe(&self) -> Result<(), String> {
        let name = CString::new(self.file).unwrap();
        let name_raw = name.into_raw();
        unsafe {
            self.raw_prober.set(blkid_new_probe_from_filename(name_raw));
        }

        if self.raw_prober.get().is_null() {
            return Err("Error in blkid_new_probe_from_filename!".to_string());
        }

        let rr: c_int;
        unsafe {
            rr = blkid_do_probe(self.raw_prober.get());
        }

        if rr < 0 {
            let message = "Error in blkid_do_probe!".to_string();
            return Err(message);
        }
        Ok(())
    }

    pub fn getType(&self) -> Result<String, String> {
        return self.get("TYPE");
    }

    pub fn getLabel(&self) -> Result<String, String> {
        return self.get("LABEL");
    }

    pub fn get(&self, key: &str) -> Result<String, String> {
        let c_key = CString::new(key).unwrap();
        let c_key_raw = c_key.into_raw();
        let mut buff: c_char = 0;
        let mut p_buff: *mut c_char = &mut buff;
        let p_p_buff: *mut *mut c_char = &mut p_buff;
        let i: size_t = 0;
        let p_i: *const size_t  = &i;

        let res: c_int;
        unsafe {
            res = blkid_probe_lookup_value(self.raw_prober.get(),
                                           c_key_raw,
                                           p_p_buff,
                                           p_i);
        }

        if res < 0 {
            let message = "Error in blkid_probe_lookup_value!".to_string();
            return Err(message);
        }

        unsafe {
            let ss = CString::from_raw(p_buff);
            let sss = ss.into_string();
            match sss {
                Ok(r)  => Ok(r),
                _      => Err("Error in CString::into_string!".to_string())
            }
        }
    }
}

impl<'a> Drop for Prober<'a> {
    fn drop(&mut self) {
        let ptr = self.raw_prober.get();
        if ptr.is_null() {
            return;
        }

        unsafe {
            blkid_free_probe(ptr);
        }
    }
}
//////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////
#[link(name = "blkid")]
extern {
    fn blkid_new_probe_from_filename(file: *const c_char) -> *const i32;
    fn blkid_do_probe(prober: *const i32) -> c_int;
    fn blkid_probe_lookup_value(prober: *const i32,
                                key: *const c_char,
                                data: *mut *mut c_char,
                                size: *const size_t) -> c_int;
    fn blkid_free_probe(prober: *const i32);
}
//////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////
#[test]
fn test_blkid_01() {
    let p = Prober::new("/dev/sda1");
    let p_r = p.probe();
    match p_r {
        Err(r) => {
            println!("{}", r);
            return;
        },
        _       => println!("********* OK ***********")
    }

    let t = p.getType();
    match t {
        Ok(r)  => println!("{}", r),
        Err(r) => println!("{}", r)
    }
    
    let l = p.getLabel();
    match l {
        Ok(r)  => println!("{}", r),
        Err(r) => println!("{}", r)
    }

    let k = p.get("ARC");
    match k {
        Ok(r)  => println!("{}", r),
        Err(r) => println!("{}", r)
    }
}
//////////////////////////////////////////////////////////////////
