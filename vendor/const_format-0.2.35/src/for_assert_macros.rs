#![allow(non_fmt_panics)]

use crate::pargument::PArgument;

#[track_caller]
pub const fn assert_(cond: bool, message: &'static str) {
    if cond {
        panic!("{}", message)
    }
}

// The `T` type parameter is there just so that the PARGUMENTS associated constant
// is evaluated lazily.
pub trait ConcatArgsIf<T, const COND: bool> {
    const PARGUMENTS: &'static [PArgument];
}

impl<S, T> ConcatArgsIf<T, false> for S {
    const PARGUMENTS: &'static [PArgument] = &[];
}
