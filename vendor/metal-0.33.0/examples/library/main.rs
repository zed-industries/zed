// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use metal::*;

const PROGRAM: &str = "";

fn main() {
    let device = Device::system_default().expect("no device found");

    let options = CompileOptions::new();
    let _library = device.new_library_with_source(PROGRAM, &options);
}
