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

pub struct Mounter {
    label: Option<String>
}

impl Mounter {

    pub fn new() -> Mounter {
        Mounter{label: None}
    }

    pub fn mount(&self, l: &logger::Logger, e: &env::Env) {
        //self.load_label();
        if !self.verify_label(e.getLabel()) {
            //
            //TODO to log
            //
            exit(0);
        }

        //self.load_fs_type();
        //self.process();
    }

    fn verify_label(&self, templ: &str) -> bool {
        let lbl :String;
        match self.label {
            None    => { return false; },
            Some(ref l) => { lbl = l.to_string(); }
        }

        let res = Regex::new(templ);
        match res {
            Ok(re) => re.is_match(&lbl),
            _      => false
        }
    }
}

#[test]
fn test_verify_label_01() {
    let i = Mounter{label: Some("BNC-124".to_string())};
    assert_eq!(false, i.verify_label(".*ANC.*"));
}

#[test]
fn test_verify_label_02() {
    let i = Mounter{label: Some("XANC-124".to_string())};
    assert_eq!(true, i.verify_label(".*ANC.*"));
}

#[test]
fn test_verify_label_04() {
    let i = Mounter{label: Some("ANC".to_string())};
    assert_eq!(true, i.verify_label(".*ANC.*"));
}

#[test]
fn test_verify_label_05() {
    let i = Mounter{label: Some("01234_ ANC".to_string())};
    assert_eq!(true, i.verify_label(".*ANC.*"));
}

#[test]
fn test_verify_label_06() {
    let i = Mounter{label: Some("01234_ ANC 782883 ".to_string())};
    assert_eq!(true, i.verify_label(".*ANC.*"));
}

#[test]
fn test_verify_label_07() {
    let i = Mounter{label: Some(" ANC 782883 ".to_string())};
    assert_eq!(true, i.verify_label(".*ANC.*"));
}

#[test]
fn test_verify_label_08() {
    let i = Mounter{label: Some("ANC---!782883 ".to_string())};
    assert_eq!(true, i.verify_label(".*ANC.*"));
}

#[test]
fn test_verify_label_09() {
    let i = Mounter{label: Some("01234_ NC 782883 ".to_string())};
    assert_eq!(false, i.verify_label(".*ANC.*"));
}
