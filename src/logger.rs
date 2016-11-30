//
// Copyright (c) 2016, Boris Popov <popov@whitekefir.ru>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

extern crate syslog;
use self::syslog::{Facility, Severity};
use self::syslog::unix;
use std::io;

pub struct Logger {
    writer: Box<syslog::Logger>
}

impl Logger {
    pub fn new(name: &str) -> Logger {
        let res = unix(Facility::LOG_USER);
        if res.is_ok() {
            let mut w = res.unwrap();
            w.set_process_name(name.to_string());
            return Logger{writer: w};
        }
        panic!("Impossible to connect to syslog!");
    }

    pub fn info(&self, s: &str) {
        let res = self.writer.send_3164(Severity::LOG_INFO, s);
        if res.is_err() {
            self.to_console(res);
        }
    }

    pub fn error(&self, s: &str) {
        let res = self.writer.send_3164(Severity::LOG_ERR, s);
        if res.is_err() {
            self.to_console(res);
        }
    }

    fn to_console(&self, r: Result<usize, io::Error>) {
        println!("Impossible to write to syslog: {:?}", r);
    }
}
