//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for `std::ffi`.

use crate::std_facade::{Box, String, Vec};
use std::ffi::*;
use std::ops::RangeInclusive;

use crate::arbitrary::*;
use crate::collection::*;
use crate::strategy::statics::static_map;
use crate::strategy::*;

use super::string::not_utf8_bytes;

arbitrary!(CString,
    SFnPtrMap<VecStrategy<RangeInclusive<u8>>, Self>, SizeRange;
    args => static_map(vec(1..=::std::u8::MAX, args + 1), |mut vec| {
        vec.pop().unwrap();
        // Could use: Self::from_vec_unchecked(vec) safely.
        Self::new(vec).unwrap()
    })
);

arbitrary!(OsString, MapInto<StrategyFor<String>, Self>,
    <String as Arbitrary>::Parameters;
    a => any_with::<String>(a).prop_map_into()
);

macro_rules! dst_wrapped {
    ($($w: ident),*) => {
        $(arbitrary!($w<CStr>, MapInto<StrategyFor<CString>, Self>, SizeRange;
            a => any_with::<CString>(a).prop_map_into()
        );)*
        $(arbitrary!($w<OsStr>, MapInto<StrategyFor<OsString>, Self>,
            <String as Arbitrary>::Parameters;
            a => any_with::<OsString>(a).prop_map_into()
        );)*
    };
}

dst_wrapped!(Box);

#[cfg(feature = "unstable")]
use std::rc::Rc;
#[cfg(feature = "unstable")]
use std::sync::Arc;
#[cfg(feature = "unstable")]
dst_wrapped!(Rc, Arc);

arbitrary!(FromBytesWithNulError, SMapped<Option<u16>, Self>; {
    static_map(any::<Option<u16>>(), |opt_pos| {
        // We make some assumptions about the internal structure of
        // FromBytesWithNulError. However, these assumptions do not
        // involve any non-public API.
        if let Some(pos) = opt_pos {
            let pos = pos as usize;
            // Allocate pos + 2 so that we never reallocate:
            let mut v = Vec::<u8>::with_capacity(pos + 2);
            v.extend(::std::iter::repeat(1).take(pos));
            v.push(0);
            v.push(1);
            CStr::from_bytes_with_nul(v.as_slice()).unwrap_err()
        } else {
            CStr::from_bytes_with_nul(b"").unwrap_err()
        }
    })
});

arbitrary!(IntoStringError, SFnPtrMap<BoxedStrategy<Vec<u8>>, Self>;
    static_map(not_utf8_bytes(false).boxed(), |bytes|
        CString::new(bytes).unwrap().into_string().unwrap_err()
    )
);

#[cfg(test)]
mod test {
    no_panic_test!(
        c_string => CString,
        os_string => OsString,
        box_c_str => Box<CStr>,
        box_os_str => Box<OsStr>,
        into_string_error => IntoStringError,
        from_bytes_with_nul => FromBytesWithNulError
    );
    #[cfg(feature = "unstable")]
    no_panic_test!(
        rc_c_str => Rc<CStr>,
        rc_os_str => Rc<OsStr>,
        arc_c_str => Arc<CStr>,
        arc_os_str => Arc<OsStr>
    );
}
