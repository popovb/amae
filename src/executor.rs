//
// Copyright (c) 2016, Boris Popov <popov@whitekefir.ru>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use logger;
use env;
use std::process::Command;

pub struct Executor {
}

impl Executor {
    pub fn new() -> Executor {
        Executor{}
    }

    pub fn start(&self, l: &logger::Logger, e: &env::Env) {
        let es = e.getScript().to_string() + " " + e.getDir();
        let res = Command::new(e.getScript())
            .arg(e.getDir()).status();
        match res {
            Ok(o) => {
                if o.success() {
                    let mut ss = "Executing ".to_string();
                    ss += &es;
                    ss += " OK!";
                    l.info(&ss);
                } else {
                    let os = o.code();
                    let mut ec = es;
                    match os {
                        None => {
                            ec += "Terminated by a signal!";
                        },
                        Some(t) => {
                            ec += " Exit code: ";
                            ec += &(format!("{}", t));
                        }
                    }
                    l.info(&ec);
                }
            },
            Err(e) => {
                let mut ss = "Error of executing ".to_string();
                ss += &es;
                ss += " ";
                ss += &(format!("{}", e));
                self.error__(ss, l);
            }
        }
    }

    fn error__(&self, s: String, l: &logger::Logger) {
        l.error(&s);
        panic!(s);
    }
}
