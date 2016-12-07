//
// Copyright (c) 2016, Boris Popov <popov@whitekefir.ru>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

extern crate regex;

use logger;
use env;
use std::process::exit;
use self::regex::Regex;
use blkid;
use mount;
use mount::MountInfo;

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
        if !self.verify_label(e.getLabel()) {
            let i = format!("Label does not match {}.", e.getLabel());
            l.info(&i);
            exit(0);
        }
        self.process(l, e);
    }

    fn process(&mut self, l: &logger::Logger, e: &env::Env) {
        let source = e.getDevice().to_string();
        self.target = e.getDir().to_string();
        let mi = MountInfo{source: &source,
                           target: &self.target,
                           filesystem: &self.part_type};
        let res = mount::mount(&mi);
        match res {
            Err(e) => {
                l.info(&e);
                exit(0);
            },
            _ => {
                return;
            }
        }
    }

    fn verify_label(&self, templ: &str) -> bool {
        if self.part_label.len() == 0 {
            return false;
        }

        let res = Regex::new(templ);
        match res {
            Ok(re) => re.is_match(&self.part_label),
            _      => false
        }
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
                let mess = "LABEL=".to_string() + &self.part_label;
                l.info(&mess);
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
                let mess = "TYPE=".to_string() + &self.part_label;
                l.info(&mess);
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
            mount::umount(&self.target);
        }
    }
}
//////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////
#[test]
fn test_verify_label_01() {
    let i = Mounter{part_label: "BNC-124".to_string(),
                    target: "/mnt/temp".to_string(),
                    part_type: "".to_string()};
    assert_eq!(false, i.verify_label(".*ANC.*"));
}

#[test]
fn test_verify_label_02() {
    let i = Mounter{part_label: "XANC-124".to_string(),
                    target: "/mnt/temp".to_string(),
                    part_type: "".to_string()};
    assert_eq!(true, i.verify_label(".*ANC.*"));
}

#[test]
fn test_verify_label_04() {
    let i = Mounter{part_label: "ANC".to_string(),
                    target: "/mnt/temp".to_string(),
                    part_type: "".to_string()};
    assert_eq!(true, i.verify_label(".*ANC.*"));
}

#[test]
fn test_verify_label_05() {
    let i = Mounter{part_label: "01234_ ANC".to_string(),
                    target: "/mnt/temp".to_string(),
                    part_type: "".to_string()};
    assert_eq!(true, i.verify_label(".*ANC.*"));
}

#[test]
fn test_verify_label_06() {
    let i = Mounter{part_label: "01234_ ANC 782883 ".to_string(),
                    target: "/mnt/temp".to_string(),
                    part_type: "".to_string()};
    assert_eq!(true, i.verify_label(".*ANC.*"));
}

#[test]
fn test_verify_label_07() {
    let i = Mounter{part_label: " ANC 782883 ".to_string(),
                    target: "/mnt/temp".to_string(),
                    part_type: "".to_string()};
    assert_eq!(true, i.verify_label(".*ANC.*"));
}

#[test]
fn test_verify_label_08() {
    let i = Mounter{part_label: "ANC---!782883 ".to_string(),
                    target: "/mnt/temp".to_string(),
                    part_type: "".to_string()};
    assert_eq!(true, i.verify_label(".*ANC.*"));
}

#[test]
fn test_verify_label_09() {
    let i = Mounter{part_label: "01234_ NC 782883 ".to_string(),
                    target: "/mnt/temp".to_string(),
                    part_type: "".to_string()};
    assert_eq!(false, i.verify_label(".*ANC.*"));
}

#[test]
fn test_verify_label_10() {
    let i = Mounter{part_label: "".to_string(),
                    target: "/mnt/temp".to_string(),
                    part_type: "".to_string()};
    assert_eq!(false, i.verify_label(".*ANC.*"));
}
//////////////////////////////////////////////////////////////////
