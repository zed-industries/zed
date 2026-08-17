//! Traits for determining whether we can derive traits for a thing or not.
//!
//! These traits tend to come in pairs:
//!
//! 1. A "trivial" version, whose implementations aren't allowed to recursively
//!    look at other types or the results of fix point analyses.
//!
//! 2. A "normal" version, whose implementations simply query the results of a
//!    fix point analysis.
//!
//! The former is used by the analyses when creating the results queried by the
//! second.

use super::context::BindgenContext;

use std::cmp;
use std::ops;

/// A trait that encapsulates the logic for whether or not we can derive `Debug`
/// for a given thing.
pub(crate) trait CanDeriveDebug {
    /// Return `true` if `Debug` can be derived for this thing, `false`
    /// otherwise.
    fn can_derive_debug(&self, ctx: &BindgenContext) -> bool;
}

/// A trait that encapsulates the logic for whether or not we can derive `Copy`
/// for a given thing.
pub(crate) trait CanDeriveCopy {
    /// Return `true` if `Copy` can be derived for this thing, `false`
    /// otherwise.
    fn can_derive_copy(&self, ctx: &BindgenContext) -> bool;
}

/// A trait that encapsulates the logic for whether or not we can derive
/// `Default` for a given thing.
pub(crate) trait CanDeriveDefault {
    /// Return `true` if `Default` can be derived for this thing, `false`
    /// otherwise.
    fn can_derive_default(&self, ctx: &BindgenContext) -> bool;
}

/// A trait that encapsulates the logic for whether or not we can derive `Hash`
/// for a given thing.
pub(crate) trait CanDeriveHash {
    /// Return `true` if `Hash` can be derived for this thing, `false`
    /// otherwise.
    fn can_derive_hash(&self, ctx: &BindgenContext) -> bool;
}

/// A trait that encapsulates the logic for whether or not we can derive
/// `PartialEq` for a given thing.
pub(crate) trait CanDerivePartialEq {
    /// Return `true` if `PartialEq` can be derived for this thing, `false`
    /// otherwise.
    fn can_derive_partialeq(&self, ctx: &BindgenContext) -> bool;
}

/// A trait that encapsulates the logic for whether or not we can derive
/// `PartialOrd` for a given thing.
pub(crate) trait CanDerivePartialOrd {
    /// Return `true` if `PartialOrd` can be derived for this thing, `false`
    /// otherwise.
    fn can_derive_partialord(&self, ctx: &BindgenContext) -> bool;
}

/// A trait that encapsulates the logic for whether or not we can derive `Eq`
/// for a given thing.
pub(crate) trait CanDeriveEq {
    /// Return `true` if `Eq` can be derived for this thing, `false` otherwise.
    fn can_derive_eq(&self, ctx: &BindgenContext) -> bool;
}

/// A trait that encapsulates the logic for whether or not we can derive `Ord`
/// for a given thing.
pub(crate) trait CanDeriveOrd {
    /// Return `true` if `Ord` can be derived for this thing, `false` otherwise.
    fn can_derive_ord(&self, ctx: &BindgenContext) -> bool;
}

/// Whether it is possible or not to automatically derive trait for an item.
///
/// ```ignore
///         No
///          ^
///          |
///      Manually
///          ^
///          |
///         Yes
/// ```
///
/// Initially we assume that we can derive trait for all types and then
/// update our understanding as we learn more about each type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum CanDerive {
    /// Yes, we can derive automatically.
    #[default]
    Yes,

    /// The only thing that stops us from automatically deriving is that
    /// array with more than maximum number of elements is used.
    ///
    /// This means we probably can "manually" implement such trait.
    Manually,

    /// No, we cannot.
    No,
}

impl CanDerive {
    /// Take the least upper bound of `self` and `rhs`.
    pub(crate) fn join(self, rhs: Self) -> Self {
        cmp::max(self, rhs)
    }
}

impl ops::BitOr for CanDerive {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.join(rhs)
    }
}

impl ops::BitOrAssign for CanDerive {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.join(rhs);
    }
}
