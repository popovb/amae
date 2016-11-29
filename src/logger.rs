//
// Copyright (c) 2016, Boris Popov <popov@whitekefir.ru>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

extern crate syslog;
use logger::syslog::{Facility, Severity};

pub struct Logger {

    name:    &'static str,
    version: &'static str
}

impl Logger {
    /*
    pub fn start(&self) {
        //self.name = &NAME;
        //self.version = VERSION.to_string();
        
        //self.name = "MountAndExec";
        //
        //TODO
        //
        //return;
    }
    */
    pub fn new(n: &'static str,
               v: &'static str) -> Logger {

        match syslog::unix(Facility::LOG_USER) {
            Err(e)     => println!("impossible to connect to syslog: {:?}", e),
            Ok(writer) => {
                let r = writer.send(Severity::LOG_ALERT, "hello world");
                if r.is_err() {
                    println!("error sending the log {}", r.err().expect("got error"));
                }
            }
        }


        
        //
        //TODO make writer
        //
        Logger{name: n, version: v}
    }
}
