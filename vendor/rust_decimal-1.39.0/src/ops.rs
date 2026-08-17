// This code (in fact, this library) is heavily inspired by the dotnet Decimal number library
// implementation. Consequently, a huge thank you for to all the contributors to that project
// whose work has also inspired the solutions found here.

pub(crate) mod array;

#[cfg(feature = "legacy-ops")]
mod legacy;
#[cfg(feature = "legacy-ops")]
pub(crate) use legacy::{add_impl, cmp_impl, div_impl, mul_impl, rem_impl, sub_impl};

#[cfg(not(feature = "legacy-ops"))]
mod add;
#[cfg(not(feature = "legacy-ops"))]
mod cmp;
#[cfg(not(feature = "legacy-ops"))]
pub(in crate::ops) mod common;
#[cfg(not(feature = "legacy-ops"))]
mod div;
#[cfg(not(feature = "legacy-ops"))]
mod mul;
#[cfg(not(feature = "legacy-ops"))]
mod rem;

#[cfg(not(feature = "legacy-ops"))]
pub(crate) use add::{add_impl, sub_impl};
#[cfg(not(feature = "legacy-ops"))]
pub(crate) use cmp::cmp_impl;
#[cfg(not(feature = "legacy-ops"))]
pub(crate) use div::div_impl;
#[cfg(not(feature = "legacy-ops"))]
pub(crate) use mul::mul_impl;
#[cfg(not(feature = "legacy-ops"))]
pub(crate) use rem::rem_impl;
