// Copyright 2016 - 2018 Ulrik Sverdrup "bluss"
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// for debugging -- like println during debugging
macro_rules! dprint {
    ($($t:tt)*) => {
        debug!(println!($($t)*))
    }
}

/*
macro_rules! debug {
    ($e:expr) => {
        $e;
    }
}
*/

macro_rules! debug {
    ($e:expr) => {
    }
}

