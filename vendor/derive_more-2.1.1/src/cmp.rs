//! [`core::cmp::AssertParamIsEq`] reimplementation.

use ::core;
use core::marker::PhantomData;
use core::prelude::v1::*;

/// Same as [`core::cmp::AssertParamIsEq`], but reimplemented on our side, because the original is
/// not a part of stable API and could be changed any time.
#[allow(missing_debug_implementations)]
pub struct AssertParamIsEq<T: Eq + ?Sized> {
    _field: PhantomData<T>,
}
