//
// Copyright (c) 2016, Boris Popov <popov@whitekefir.ru>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use logger;

pub struct Env {
    device: String,
    dir:    String,
    script: String
}

impl Env {

    pub fn new() -> Env {
        return Env{device: "".to_string(),
                   dir:    "".to_string(),
                   script: "".to_string()}
    }

    pub fn load(&self, l: &logger::Logger) {
        self.load_device(l);
        self.load_dir(l);
        self.load_script(l);
    }

    fn load_device(&self, l: &logger::Logger) {
        //
        //TODO
        //
    }

    fn load_dir(&self, l: &logger::Logger) {
        //
        //TODO
        //
    }

    fn load_script(&self, l: &logger::Logger) {
        //
        //TODO
        //
    }
}
