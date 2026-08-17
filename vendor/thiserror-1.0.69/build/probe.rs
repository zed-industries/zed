// This code exercises the surface area that we expect of the Error generic
// member access API. If the current toolchain is able to compile it, then
// thiserror is able to provide backtrace support.

#![feature(error_generic_member_access)]

use core::fmt::{self, Debug, Display};
use std::error::{Error, Request};

struct MyError(Thing);
struct Thing;

impl Debug for MyError {
    fn fmt(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl Display for MyError {
    fn fmt(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl Error for MyError {
    fn provide<'a>(&'a self, request: &mut Request<'a>) {
        request.provide_ref(&self.0);
    }
}

// Include in sccache cache key.
const _: Option<&str> = option_env!("RUSTC_BOOTSTRAP");
