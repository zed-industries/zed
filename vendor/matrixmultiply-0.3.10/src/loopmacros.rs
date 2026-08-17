// Copyright 2016 - 2018 Ulrik Sverdrup "bluss"
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Unroll only in non-debug builds

#[cfg(not(debug_assertions))]
macro_rules! repeat {
    (1 $e:expr) => { $e; };
    (2 $e:expr) => { $e;$e; };
    (3 $e:expr) => { $e;$e; $e; };
    (4 $e:expr) => { $e;$e; $e;$e; };
    (5 $e:expr) => { $e;$e; $e;$e; $e; };
    (6 $e:expr) => { $e;$e; $e;$e; $e;$e; };
    (7 $e:expr) => { $e;$e; $e;$e; $e;$e; $e; };
    (8 $e:expr) => { $e;$e; $e;$e; $e;$e; $e;$e; };
}

#[cfg(debug_assertions)]
macro_rules! loop4 {
    ($i:ident, $e:expr) => {
        for $i in 0..4 { $e }
    }
}

#[cfg(feature = "cgemm")]
macro_rules! loop2 {
    ($i:ident, $e:expr) => {{
        let $i = 0; $e;
        let $i = 1; $e;
    }}
}

#[cfg(not(debug_assertions))]
macro_rules! loop4 {
    ($i:ident, $e:expr) => {{
        let $i = 0; $e;
        let $i = 1; $e;
        let $i = 2; $e;
        let $i = 3; $e;
    }}
}

#[cfg(debug_assertions)]
macro_rules! loop8 {
    ($i:ident, $e:expr) => {
        for $i in 0..8 { $e }
    }
}

#[cfg(not(debug_assertions))]
macro_rules! loop8 {
    ($i:ident, $e:expr) => {{
        let $i = 0; $e;
        let $i = 1; $e;
        let $i = 2; $e;
        let $i = 3; $e;
        let $i = 4; $e;
        let $i = 5; $e;
        let $i = 6; $e;
        let $i = 7; $e;
    }}
}

#[cfg(debug_assertions)]
macro_rules! unroll_by {
    ($by:tt => $ntimes:expr, $e:expr) => {
        for _ in 0..$ntimes { $e }
    }
}

#[cfg(not(debug_assertions))]
macro_rules! unroll_by {
    ($by:tt => $ntimes:expr, $e:expr) => {{
        // using while loop to avoid problems
        // with requiring inlining of foor loop parts
        let k = $ntimes;
        let mut _index = 0;
        let _target = k / $by;
        while _index < _target {
            repeat!($by $e);
            _index += 1;
        }

        let mut _index = 0;
        let _target = k % $by;
        while _index < _target {
            $e;
            _index += 1;
        }
    }}
}

#[allow(unused)]
#[cfg(debug_assertions)]
macro_rules! unroll_by_with_last {
    ($by:tt => $ntimes:expr, $is_last:ident, $e:expr) => {{
        let k = $ntimes - 1;
        let $is_last = false;
        for _ in 0..k {
            $e;
        }
        let $is_last = true;
        #[allow(unused_assignments)]
        $e;
    }}
}

#[allow(unused)]
#[cfg(not(debug_assertions))]
macro_rules! unroll_by_with_last {
    ($by:tt => $ntimes:expr, $is_last:ident, $e:expr) => {{
        let k = $ntimes - 1;
        let $is_last = false;
        for _ in 0..k / $by {
            repeat!($by $e);
        }
        for _ in 0..k % $by {
            $e;
        }
        let $is_last = true;
        #[allow(unused_assignments)]
        $e;
    }}
}
