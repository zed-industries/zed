//! Deferred value initialization.
//!
//! The [`Fill`] trait is a way to bridge APIs that may not be directly
//! compatible with other constructor methods.
//!
//! The `Fill` trait is automatically implemented for closures, so can usually
//! be used in libraries that can't implement the trait themselves.
//!
//! ```
//! use value_bag::{ValueBag, fill::Slot};
//!
//! let value = ValueBag::from_fill(&|slot: Slot| {
//!     #[derive(Debug)]
//!     struct MyShortLivedValue;
//!
//!     slot.fill_debug(&MyShortLivedValue)
//! });
//!
//! assert_eq!("MyShortLivedValue", format!("{:?}", value));
//! ```
//!
//! The trait can also be implemented manually:
//!
//! ```
//! # use std::fmt::Debug;
//! use value_bag::{ValueBag, Error, fill::{Slot, Fill}};
//!
//! struct FillDebug;
//!
//! impl Fill for FillDebug {
//!     fn fill(&self, slot: Slot) -> Result<(), Error> {
//!         slot.fill_debug(&42i64 as &dyn Debug)
//!     }
//! }
//!
//! let value = ValueBag::from_fill(&FillDebug);
//!
//! assert_eq!(None, value.to_i64());
//! ```

use crate::std::fmt;

use super::internal::{Internal, InternalVisitor};
use super::{Error, ValueBag};

impl<'v> ValueBag<'v> {
    /// Get a value from a fillable slot.
    pub const fn from_fill<T>(value: &'v T) -> Self
    where
        T: Fill,
    {
        ValueBag {
            inner: Internal::Fill(value),
        }
    }
}

/// A type that requires extra work to convert into a [`ValueBag`](../struct.ValueBag.html).
///
/// This trait is an advanced initialization API.
/// It's intended for erased values coming from other logging frameworks that may need
/// to perform extra work to determine the concrete type to use.
pub trait Fill {
    /// Fill a value.
    fn fill(&self, slot: Slot) -> Result<(), Error>;
}

impl<F> Fill for F
where
    F: Fn(Slot) -> Result<(), Error>,
{
    fn fill(&self, slot: Slot) -> Result<(), Error> {
        (self)(slot)
    }
}

/// A value slot to fill using the [`Fill`](trait.Fill.html) trait.
pub struct Slot<'s, 'f> {
    visitor: &'s mut dyn InternalVisitor<'f>,
}

impl<'s, 'f> fmt::Debug for Slot<'s, 'f> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Slot").finish()
    }
}

impl<'s, 'f> Slot<'s, 'f> {
    pub(crate) fn new(visitor: &'s mut dyn InternalVisitor<'f>) -> Self {
        Slot { visitor }
    }

    pub(crate) fn fill<F>(self, f: F) -> Result<(), Error>
    where
        F: FnOnce(&mut dyn InternalVisitor<'f>) -> Result<(), Error>,
    {
        f(self.visitor)
    }

    /// Fill the slot with a value.
    ///
    /// The given value doesn't need to satisfy any particular lifetime constraints.
    pub fn fill_any<T>(self, value: T) -> Result<(), Error>
    where
        T: Into<ValueBag<'f>>,
    {
        self.fill(|visitor| value.into().inner.internal_visit(visitor))
    }
}

#[cfg(test)]
mod tests {
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::*;

    use super::*;
    use crate::std::string::ToString;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn fill_value_borrowed() {
        struct TestFill;

        impl Fill for TestFill {
            fn fill(&self, slot: Slot) -> Result<(), Error> {
                let dbg = &1 as &dyn fmt::Debug;

                slot.fill_debug(dbg)
            }
        }

        assert_eq!("1", ValueBag::from_fill(&TestFill).to_string());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn fill_cast() {
        struct TestFill;

        impl Fill for TestFill {
            fn fill(&self, slot: Slot) -> Result<(), Error> {
                slot.fill_any("a string")
            }
        }

        assert_eq!(
            "a string",
            ValueBag::from_fill(&TestFill)
                .to_borrowed_str()
                .expect("invalid value")
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn fill_fn_cast() {
        assert_eq!(
            42u64,
            ValueBag::from_fill(&|slot: Slot| slot.fill_any(42u64))
                .to_u64()
                .unwrap()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn fill_fn_borrowed() {
        #[derive(Debug)]
        struct MyValue;

        let value = MyValue;
        assert_eq!(
            format!("{:?}", value),
            format!(
                "{:?}",
                ValueBag::from_fill(&|slot: Slot| slot.fill_debug(&value))
            )
        );
    }
}
