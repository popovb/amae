//
// Copyright (c) 2016, Boris Popov <popov@whitekefir.ru>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

extern crate libc;
use self::libc::sleep;

pub fn main() {
    //
    //TODO work
    //
    unsafe {
        sleep(15);
    }
}
//mod program;
//use program::{NAME, VERSION};

//mod logger;
//use logger::{Logger};

//use forker::hello;
//use forker::hello;
//use libc;
//use libc::fork;
//use libc::exit;


    //let logger = logger::Logger{name: NAME, version: VERSION};
    //logger.start();
    //let logger = Logger::new(NAME, VERSION);
    //let logger = Logger::new(NAME, VERSION);


    
    //println!("{}", NAME);
    //let pi = program::Info{name: NAME, version: VERSION};
    //pi.init();
    //println!("{}", pi.name());
    //println!("{}", pi.version());
    /*
    println!("Hello, world!");
    hello();

    let x: i32;
    unsafe {
        x = libc::fork();
    }

    if x == 0 {
        unsafe {
            libc::sleep(10);
            libc::exit(2);
        }
    }
    //
    unsafe {
        sleep(5);
    }
     
    //pub unsafe extern fn fork() -> pid_t
    println!("Exit!");
    */
