//
// Copyright (c) 2016, Boris Popov <popov@whitekefir.ru>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use logger::Logger;
use program::{NAME, VERSION};
use env::Env;
use mounter::Mounter;
use executor::Executor;

pub fn main() {
    let logger = Logger::new(NAME);
    let name_ver = NAME.to_string() + "-" + VERSION;
    let start = name_ver.clone() + " starting!";
    logger.info(&start);

    let env = Env::new();
    env.load(&logger);

    let mnt = Mounter::new();
    mnt.mount(&logger, &env);

    let ex = Executor::new();
    ex.start(&logger, &env);

    let end = name_ver.clone() + " exiting!";
    logger.info(&end);
}
