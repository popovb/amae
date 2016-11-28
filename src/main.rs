//
// Copyright (c) 2016, Boris Popov <popov@whitekefir.ru>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

#![allow(non_snake_case)]

extern crate libc;

mod forker;
mod logger;

use forker::hello;
//use forker::hello;
//use libc;
//use libc::fork;
//use libc::exit;

fn main() {
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
    /*
    unsafe {
        sleep(5);
    }
     */
    //pub unsafe extern fn fork() -> pid_t
    println!("Exit!");
}
