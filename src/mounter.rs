//
// Copyright (c) 2016, Boris Popov <popov@whitekefir.ru>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use logger;
use env;
use std::process::exit;
use blkid;
use mount;

//////////////////////////////////////////////////////////////////
pub struct Mounter {
    part_label: String,
    part_type:  String,
    target:     String
}

impl Mounter {
    pub fn new() -> Mounter {
        Mounter{
            part_label: "".to_string(),
            part_type:  "".to_string(),
            target:     "".to_string()
        }
    }

    pub fn mount(&mut self, l: &logger::Logger, e: &env::Env) {
        self.load_part_info(e.getDevice(), l);
        let result = self.verify_label(e.getLabel(), l);
        if ! result {
            let i = format!("Label does not match {}.", e.getLabel());
            l.info(&i);
            exit(0);
        }
        self.process(l, e);
    }

    fn process(&mut self, l: &logger::Logger, e: &env::Env) {
        let source = e.getDevice().to_string();
        self.target = e.getDir().to_string();
        let res = mount::mount(source.as_str(),
                               self.target.as_str(),
                               self.part_type.as_str());
        match res {
            Err(e) => {
                l.info(e.as_str());
                exit(0);
            },
            _ => {
                let i = format!("Mount {} to {} OK!",
                                source.as_str(),
                                self.target.as_str());
                l.info(i.as_str());
                return;
            }
        }
    }

    fn verify_label(&self, templ: &str, l: &logger::Logger) -> bool {
        if self.part_label.len() == 0 {
            return false;
        }
        return self.part_label.contains(templ);
    }

    fn load_part_info(&mut self, dev: &str, l: &logger::Logger) {
        let prober = blkid::Prober::new(dev);
        let res = prober.probe();
        if let Err(e) = res {
            l.error(&e);
            exit(1);
        }
        self.loadLabel(&prober, l);
        self.loadType(&prober, l);
    }

    fn loadLabel(&mut self, prober: &blkid::Prober,
                 l: &logger::Logger) {
        let lr = prober.getLabel();
        match lr {
            Ok(o) => {
                self.part_label = o;
            },
            Err(e) => {
                l.error(&e);
            }
        }
    }

    fn loadType(&mut self, prober: &blkid::Prober,
                l: &logger::Logger) {
        let lr = prober.getType();
        match lr {
            Ok(o) => {
                self.part_type = o;
            },
            Err(e) => {
                l.error(&e);
            }
        }
    }
}

impl Drop for Mounter {
    fn drop(&mut self) {
        if self.target.len() != 0 {
            //
            //TODO
            //
            //mount::umount(&self.target);
        }
    }
}
//////////////////////////////////////////////////////////////////
