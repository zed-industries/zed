//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for std::borrow.

use crate::std_facade::fmt;
use crate::std_facade::{Cow, ToOwned};
use core::borrow::Borrow;

use crate::arbitrary::{any_with, Arbitrary, SMapped};
use crate::strategy::statics::static_map;

arbitrary!(
    [A: Arbitrary + Borrow<B>, B: ToOwned<Owned = A> + fmt::Debug + ?Sized]
    Cow<'static, B>, SMapped<A, Self>, A::Parameters;
    args => static_map(any_with::<A>(args), Cow::Owned)
);

lift1!([Borrow<B> + 'static, B: ToOwned<Owned = A> + fmt::Debug + ?Sized]
    Cow<'static, B>; base => static_map(base, Cow::Owned)
);
