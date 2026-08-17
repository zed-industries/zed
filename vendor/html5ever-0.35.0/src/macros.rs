// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

macro_rules! unwrap_or_return {
    ($opt:expr) => {{
        let Some(x) = $opt else {
            return;
        };
        x
    }};
    ($opt:expr, $retval:expr) => {{
        let Some(x) = $opt else {
            return $retval;
        };
        x
    }};
}
pub(crate) use unwrap_or_return;

macro_rules! time {
    ($e:expr) => {{
        let t0 = ::std::time::Instant::now();
        let result = $e;
        let dt = t0.elapsed().as_nanos() as u64;
        (result, dt)
    }};
}
pub(crate) use time;
