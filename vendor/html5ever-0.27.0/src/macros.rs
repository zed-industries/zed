// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

macro_rules! unwrap_or_else {
    ($opt:expr, $else_block:block) => {
        match $opt {
            None => $else_block,
            Some(x) => x,
        }
    };
}

macro_rules! unwrap_or_return {
    ($opt:expr, $retval:expr) => {
        unwrap_or_else!($opt, { return $retval })
    };
}

macro_rules! time {
    ($e:expr) => {{
        let now = ::std::time::Instant::now();
        let result = $e;
        let d = now.elapsed();
        let dt = d.as_secs() * 1_000_000_000 + u64::from(d.subsec_nanos());
        (result, dt)
    }};
}
