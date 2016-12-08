//
// Copyright (c) 2016, Boris Popov <popov@whitekefir.ru>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use logger;
use std::env;

//////////////////////////////////////////////////////////////////
pub struct Env {
    device: String,
    dir:    String,
    script: String,
    label:  String,
    umount: bool
}

impl Env {
    pub fn new() -> Env {
        return Env{device: "".to_string(),
                   dir:    "".to_string(),
                   label:  "".to_string(),
                   script: "".to_string(),
                   umount: false}
    }

    pub fn load(&mut self, l: &logger::Logger) {
        self.load_device(l);
        self.load_dir(l);
        self.load_script(l);
        self.load_label(l);
        self.load_umount(l);
    }

    pub fn getDir(&self) -> &str {
        return &self.dir;
    }

    pub fn getScript(&self) -> &str {
        return &self.script;
    }

    pub fn getLabel(&self) -> &str {
        return &self.label;
    }

    pub fn getDevice(&self) -> &str {
        return &self.device;
    }

    pub fn getUmount(&self) -> bool {
        return self.umount;
    }

    fn load_device(&mut self, l: &logger::Logger) {
        let k = "AMAE_DEVICE";
        self.device = self.load_(k);
        self.print(&k, &self.device, l);
    }

    fn load_dir(&mut self, l: &logger::Logger) {
        let k = "AMAE_MOUNT_DIR";
        self.dir = self.load_(k);
        self.print(&k, &self.dir, l);
    }

    fn load_script(&mut self, l: &logger::Logger) {
        let k = "AMAE_SCRIPT";
        self.script = self.load_(k);
        self.print(&k, &self.script, l);
    }

    fn load_label(&mut self, l: &logger::Logger) {
        let k = "AMAE_LABEL";
        self.label = self.load_(k);
        self.print(&k, &self.label, l);
    }

    fn load_umount(&mut self, l: &logger::Logger) {
        let k = "AMAE_UMOUNT";
        let x = self.load_(k);
        self.print(&k, &x, l);
        if x == "YES".to_string() {
            self.umount = true;
        }
    }

    fn print(&self, key: &str, val: &str,
             l: &logger::Logger) {
        let s = format!("env: {} -> {}", key, val);
        l.info(s.as_str());
    }

    fn load_(&self, name: &str) -> String {
        if name.len() == 0 {
            return "".to_string();
        }
        match env::var_os(name) {
            Some(val) => {
                if val.is_empty() {
                    return "".to_string();
                }
                return val.into_string().unwrap();
            },
            None      => {
                return "".to_string();
            }
        }
    }
}
//////////////////////////////////////////////////////////////////
