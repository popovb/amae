//
// Copyright (c) 2016, Boris Popov <popov@whitekefir.ru>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

#![allow(non_snake_case)]

mod forker;
mod worker;

fn main() {

    let forker = forker::Forker::new();
    forker.spawn();
}
