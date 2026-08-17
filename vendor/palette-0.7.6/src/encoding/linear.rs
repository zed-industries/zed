//! Linear encoding

use core::marker::PhantomData;

use crate::{
    luma::LumaStandard,
    rgb::{RgbSpace, RgbStandard},
};

use super::{FromLinear, IntoLinear};

/// A generic standard with linear components.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Linear<S>(PhantomData<S>);

impl<Sp> RgbStandard for Linear<Sp>
where
    Sp: RgbSpace,
{
    type Space = Sp;
    type TransferFn = LinearFn;
}

impl<Wp> LumaStandard for Linear<Wp> {
    type WhitePoint = Wp;
    type TransferFn = LinearFn;
}

/// Linear color component encoding.
///
/// Converting anything from linear to linear space is a no-op and constant
/// time. This is a useful property in generic code, where the transfer
/// functions may be unknown.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct LinearFn;

impl<T> IntoLinear<T, T> for LinearFn {
    #[inline(always)]
    fn into_linear(x: T) -> T {
        x
    }
}

impl<T> FromLinear<T, T> for LinearFn {
    #[inline(always)]
    fn from_linear(x: T) -> T {
        x
    }
}
