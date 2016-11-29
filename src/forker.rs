//
// Copyright (c) 2016, Boris Popov <popov@whitekefir.ru>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

extern crate libc;

use self::libc::fork;
use worker;

pub struct Forker {
}

impl Forker {
    pub fn new() -> Forker {
        Forker{}
    }

    pub fn spawn(&self) {
        let pid: i32;
        unsafe {
            pid = fork();
        }

        if pid < 0 {
            panic!("Can't fork!");
        }

        if pid != 0 {
            return;
        }

        self.main();
    }

    fn main(&self) {
        worker::main();
    }
}
