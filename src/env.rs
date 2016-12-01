//
// Copyright (c) 2016, Boris Popov <popov@whitekefir.ru>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

extern crate libc;

use logger;
use self::libc::getenv;

use std::ffi::CString;

pub struct Env {
    device: String,
    dir:    String,
    script: String
}

impl Env {
    pub fn new() -> Env {
        return Env{device: "".to_string(),
                   dir:    "".to_string(),
                   script: "".to_string()}
    }

    pub fn load(&mut self, l: &logger::Logger) {
        self.load_device(l);
        self.load_dir(l);
        self.load_script(l);
    }

    pub fn getDir(&self) -> &str {
        return &self.dir;
    }

    pub fn getScript(&self) -> &str {
        return &self.script;
    }

    fn load_device(&mut self, l: &logger::Logger) {
        self.device = self.load_("AMAE_DEVICE", l);
    }

    fn load_dir(&mut self, l: &logger::Logger) {
        self.dir = self.load_("AMAE_MOUNT_DIR", l);
    }

    fn load_script(&mut self, l: &logger::Logger) {
        self.script = self.load_("AMAE_SCRIPT", l);
    }

    fn load_(&self, name: &str, l: &logger::Logger) -> String {
        let cname = CString::new(name).unwrap();
        let cname_ptr = cname.as_ptr();
        unsafe {
            let out = getenv(cname_ptr);
            if out.is_null() { self.error_(name, l); }
            let cstring = CString::from_raw(out);
            let res = cstring.into_string();
            match res {
                Ok(v)  => return v,
                Err(e) => {
                    let mut ss = self.ef(name);
                    ss += " ";
                    ss += &(format!("{}", e));
                    self.error__(ss, l);
                    return "".to_string();
                }
            }
        }
    }

    fn error_(&self, name: &str, l: &logger::Logger) {
        self.error__(self.ef(name), l);
    }

    fn ef(&self, name: &str) -> String {
        let em = "Can't get env: ".to_string() +
            name + "!";
        return em;
    }

    fn error__(&self, s: String, l: &logger::Logger) {
        l.error(s.as_str());
        panic!(s);
    }
}
/*
#[test]
fn hello() {
    let x = Err("test");
    assert_eq!("test", x.unwrap());
}
 */
