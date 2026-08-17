//! [AnyVec] operations.
//!
//! You don't need, nor can't construct anything from this module manually.
//!
//! [AnyVec]: crate::AnyVec

mod temp;
mod iter;
pub(crate) mod swap_remove;
pub(crate) mod remove;
pub(crate) mod drain;
pub(crate) mod splice;
pub(crate) mod pop;

pub use temp::TempValue;
pub use iter::Iter;

use crate::any_vec_ptr::AnyVecPtr;

/// Lazily `pop` on consumption/drop.
///
/// This is created by [`AnyVec::pop`].
///
/// [`AnyVec::pop`]: crate::AnyVec::pop
pub type Pop<'a, Traits, M> = TempValue<pop::Pop<'a, AnyVecPtr<Traits, M>>>;

/// Lazily `remove` element on consumption/drop.
///
/// This is created by [`AnyVec::remove`].
///
/// [`AnyVec::remove`]: crate::AnyVec::remove
pub type Remove<'a, Traits, M> = TempValue<remove::Remove<'a, AnyVecPtr<Traits, M>>>;

/// Lazily `swap_remove` element on consumption/drop.
///
/// This is created by [`AnyVec::swap_remove`].
///
/// [`AnyVec::swap_remove`]: crate::AnyVec::swap_remove
pub type SwapRemove<'a, Traits, M> = TempValue<swap_remove::SwapRemove<'a, AnyVecPtr<Traits, M>>>;

///  A draining [`ElementIterator`] for [`AnyVec`]. Return items as [`Element`]s.
///
/// This is created by [`AnyVec::drain`].
///
/// [`AnyVec`]: crate::AnyVec
/// [`AnyVec::drain`]: crate::AnyVec::drain
/// [`Element`]: crate::element::Element
/// [`ElementIterator`]: crate::iter::ElementIterator
pub type Drain<'a, Traits, M> = Iter<drain::Drain<'a, AnyVecPtr<Traits, M>>>;

///  A splicing [`ElementIterator`] for [`AnyVec`]. Return items as [`Element`]s.
///
/// This is created by [`AnyVec::splice`].
///
/// [`AnyVec`]: crate::AnyVec
/// [`AnyVec::splice`]: crate::AnyVec::splice
/// [`Element`]: crate::element::Element
/// [`ElementIterator`]: crate::iter::ElementIterator
pub type Splice<'a, Traits, M, I> = Iter<splice::Splice<'a, AnyVecPtr<Traits, M>, I>>;