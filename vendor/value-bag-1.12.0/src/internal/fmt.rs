//! Integration between `Value` and `std::fmt`.
//!
//! This module allows any `Value` to implement the `Debug` and `Display` traits,
//! and for any `Debug` or `Display` to be captured as a `Value`.

use crate::{
    fill::Slot,
    std::{any::Any, fmt},
    Error, ValueBag,
};

use super::{Internal, InternalVisitor};

impl<'v> ValueBag<'v> {
    /// Get a value from a debuggable type.
    ///
    /// This method will attempt to capture the given value as a well-known primitive
    /// before resorting to using its `Debug` implementation.
    pub fn capture_debug<T>(value: &'v T) -> Self
    where
        T: Debug + 'static,
    {
        Self::try_capture(value).unwrap_or(ValueBag {
            inner: Internal::Debug(value),
        })
    }

    /// Get a value from a displayable type.
    ///
    /// This method will attempt to capture the given value as a well-known primitive
    /// before resorting to using its `Display` implementation.
    pub fn capture_display<T>(value: &'v T) -> Self
    where
        T: Display + 'static,
    {
        Self::try_capture(value).unwrap_or(ValueBag {
            inner: Internal::Display(value),
        })
    }

    /// Get a value from a debuggable type without capturing support.
    pub const fn from_debug<T>(value: &'v T) -> Self
    where
        T: Debug,
    {
        ValueBag {
            inner: Internal::AnonDebug(value),
        }
    }

    /// Get a value from a displayable type without capturing support.
    pub const fn from_display<T>(value: &'v T) -> Self
    where
        T: Display,
    {
        ValueBag {
            inner: Internal::AnonDisplay(value),
        }
    }

    /// Get a value from a debuggable type without capturing support.
    #[inline]
    pub const fn from_dyn_debug(value: &'v dyn Debug) -> Self {
        ValueBag {
            inner: Internal::AnonDebug(value),
        }
    }

    /// Get a value from a displayable type without capturing support.
    #[inline]
    pub const fn from_dyn_display(value: &'v dyn Display) -> Self {
        ValueBag {
            inner: Internal::AnonDisplay(value),
        }
    }
}

pub(crate) trait DowncastDisplay {
    fn as_any(&self) -> &dyn Any;
    fn as_super(&self) -> &dyn fmt::Display;
}

impl<T: fmt::Display + 'static> DowncastDisplay for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_super(&self) -> &dyn fmt::Display {
        self
    }
}

impl<'a> fmt::Display for dyn DowncastDisplay + Send + Sync + 'a {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_super().fmt(f)
    }
}

pub(crate) trait DowncastDebug {
    fn as_any(&self) -> &dyn Any;
    fn as_super(&self) -> &dyn fmt::Debug;
}

impl<T: fmt::Debug + 'static> DowncastDebug for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_super(&self) -> &dyn fmt::Debug {
        self
    }
}

impl<'a> fmt::Debug for dyn DowncastDebug + Send + Sync + 'a {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_super().fmt(f)
    }
}

impl<'s, 'f> Slot<'s, 'f> {
    /// Fill the slot with a debuggable value.
    ///
    /// The given value doesn't need to satisfy any particular lifetime constraints.
    pub fn fill_debug<T>(self, value: T) -> Result<(), Error>
    where
        T: Debug,
    {
        self.fill(|visitor| visitor.debug(&value))
    }

    /// Fill the slot with a displayable value.
    ///
    /// The given value doesn't need to satisfy any particular lifetime constraints.
    pub fn fill_display<T>(self, value: T) -> Result<(), Error>
    where
        T: Display,
    {
        self.fill(|visitor| visitor.display(&value))
    }
}

pub use self::fmt::{Debug, Display};

impl<'v> Debug for ValueBag<'v> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct DebugVisitor<'a, 'b: 'a>(&'a mut fmt::Formatter<'b>);

        impl<'a, 'b: 'a, 'v> InternalVisitor<'v> for DebugVisitor<'a, 'b> {
            fn fill(&mut self, v: &dyn crate::fill::Fill) -> Result<(), Error> {
                v.fill(crate::fill::Slot::new(self))
            }

            fn debug(&mut self, v: &dyn Debug) -> Result<(), Error> {
                Debug::fmt(v, self.0)?;

                Ok(())
            }

            fn display(&mut self, v: &dyn Display) -> Result<(), Error> {
                Display::fmt(v, self.0)?;

                Ok(())
            }

            fn u64(&mut self, v: u64) -> Result<(), Error> {
                Debug::fmt(&v, self.0)?;

                Ok(())
            }

            fn i64(&mut self, v: i64) -> Result<(), Error> {
                Debug::fmt(&v, self.0)?;

                Ok(())
            }

            fn u128(&mut self, v: &u128) -> Result<(), Error> {
                Debug::fmt(&v, self.0)?;

                Ok(())
            }

            fn i128(&mut self, v: &i128) -> Result<(), Error> {
                Debug::fmt(&v, self.0)?;

                Ok(())
            }

            fn f64(&mut self, v: f64) -> Result<(), Error> {
                Debug::fmt(&v, self.0)?;

                Ok(())
            }

            fn bool(&mut self, v: bool) -> Result<(), Error> {
                Debug::fmt(&v, self.0)?;

                Ok(())
            }

            fn char(&mut self, v: char) -> Result<(), Error> {
                Debug::fmt(&v, self.0)?;

                Ok(())
            }

            fn str(&mut self, v: &str) -> Result<(), Error> {
                Debug::fmt(&v, self.0)?;

                Ok(())
            }

            fn none(&mut self) -> Result<(), Error> {
                self.debug(&format_args!("None"))
            }

            #[cfg(feature = "error")]
            fn error(&mut self, v: &(dyn std::error::Error + 'static)) -> Result<(), Error> {
                Debug::fmt(v, self.0)?;

                Ok(())
            }

            #[cfg(feature = "sval2")]
            fn sval2(&mut self, v: &dyn crate::internal::sval::v2::Value) -> Result<(), Error> {
                crate::internal::sval::v2::fmt(self.0, v)
            }

            #[cfg(feature = "serde1")]
            fn serde1(
                &mut self,
                v: &dyn crate::internal::serde::v1::Serialize,
            ) -> Result<(), Error> {
                crate::internal::serde::v1::fmt(self.0, v)
            }

            #[cfg(feature = "seq")]
            fn seq(&mut self, seq: &dyn crate::internal::seq::Seq) -> Result<(), Error> {
                let mut visitor = seq::FmtSeq(self.0.debug_list());
                seq.visit(&mut visitor);
                visitor.0.finish()?;

                Ok(())
            }

            fn poisoned(&mut self, msg: &'static str) -> Result<(), Error> {
                write!(self.0, "<{msg}>")?;

                Ok(())
            }
        }

        self.internal_visit(&mut DebugVisitor(f))
            .map_err(|_| fmt::Error)?;

        Ok(())
    }
}

impl<'v> Display for ValueBag<'v> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct DisplayVisitor<'a, 'b: 'a>(&'a mut fmt::Formatter<'b>);

        impl<'a, 'b: 'a, 'v> InternalVisitor<'v> for DisplayVisitor<'a, 'b> {
            fn fill(&mut self, v: &dyn crate::fill::Fill) -> Result<(), Error> {
                v.fill(crate::fill::Slot::new(self))
            }

            fn debug(&mut self, v: &dyn Debug) -> Result<(), Error> {
                Debug::fmt(v, self.0)?;

                Ok(())
            }

            fn display(&mut self, v: &dyn Display) -> Result<(), Error> {
                Display::fmt(v, self.0)?;

                Ok(())
            }

            fn u64(&mut self, v: u64) -> Result<(), Error> {
                Display::fmt(&v, self.0)?;

                Ok(())
            }

            fn i64(&mut self, v: i64) -> Result<(), Error> {
                Display::fmt(&v, self.0)?;

                Ok(())
            }

            fn u128(&mut self, v: &u128) -> Result<(), Error> {
                Display::fmt(&v, self.0)?;

                Ok(())
            }

            fn i128(&mut self, v: &i128) -> Result<(), Error> {
                Display::fmt(&v, self.0)?;

                Ok(())
            }

            fn f64(&mut self, v: f64) -> Result<(), Error> {
                Display::fmt(&v, self.0)?;

                Ok(())
            }

            fn bool(&mut self, v: bool) -> Result<(), Error> {
                Display::fmt(&v, self.0)?;

                Ok(())
            }

            fn char(&mut self, v: char) -> Result<(), Error> {
                Display::fmt(&v, self.0)?;

                Ok(())
            }

            fn str(&mut self, v: &str) -> Result<(), Error> {
                Display::fmt(&v, self.0)?;

                Ok(())
            }

            fn none(&mut self) -> Result<(), Error> {
                self.debug(&format_args!("None"))
            }

            #[cfg(feature = "error")]
            fn error(&mut self, v: &(dyn std::error::Error + 'static)) -> Result<(), Error> {
                Display::fmt(v, self.0)?;

                Ok(())
            }

            #[cfg(feature = "sval2")]
            fn sval2(&mut self, v: &dyn crate::internal::sval::v2::Value) -> Result<(), Error> {
                crate::internal::sval::v2::fmt(self.0, v)
            }

            #[cfg(feature = "serde1")]
            fn serde1(
                &mut self,
                v: &dyn crate::internal::serde::v1::Serialize,
            ) -> Result<(), Error> {
                crate::internal::serde::v1::fmt(self.0, v)
            }

            #[cfg(feature = "seq")]
            fn seq(&mut self, seq: &dyn crate::internal::seq::Seq) -> Result<(), Error> {
                let mut visitor = seq::FmtSeq(self.0.debug_list());
                seq.visit(&mut visitor);
                visitor.0.finish()?;

                Ok(())
            }

            fn poisoned(&mut self, msg: &'static str) -> Result<(), Error> {
                write!(self.0, "<{msg}>")?;

                Ok(())
            }
        }

        self.internal_visit(&mut DisplayVisitor(f))
            .map_err(|_| fmt::Error)?;

        Ok(())
    }
}

#[cfg(feature = "seq")]
mod seq {
    use super::*;
    use core::ops::ControlFlow;

    pub(super) struct FmtSeq<'a, 'b>(pub(super) fmt::DebugList<'b, 'a>);

    impl<'a, 'b, 'c> crate::internal::seq::Visitor<'c> for FmtSeq<'a, 'b> {
        fn element(&mut self, inner: ValueBag) -> ControlFlow<()> {
            self.0.entry(&inner);
            ControlFlow::Continue(())
        }
    }
}

#[cfg(feature = "owned")]
pub(crate) mod owned {
    use crate::std::{boxed::Box, fmt, string::ToString};

    impl fmt::Debug for crate::OwnedValueBag {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(&self.by_ref(), f)
        }
    }

    impl fmt::Display for crate::OwnedValueBag {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Display::fmt(&self.by_ref(), f)
        }
    }

    #[derive(Clone)]
    pub(crate) struct OwnedFmt(Box<str>);

    pub(crate) fn buffer_debug(v: impl fmt::Debug) -> OwnedFmt {
        OwnedFmt(format!("{:?}", v).into())
    }

    pub(crate) fn buffer_display(v: impl fmt::Display) -> OwnedFmt {
        OwnedFmt(v.to_string().into())
    }

    impl fmt::Debug for OwnedFmt {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Display::fmt(self, f)
        }
    }

    impl fmt::Display for OwnedFmt {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Display::fmt(&self.0, f)
        }
    }
}

impl<'v> From<&'v dyn Debug> for ValueBag<'v> {
    #[inline]
    fn from(v: &'v dyn Debug) -> Self {
        ValueBag::from_dyn_debug(v)
    }
}

impl<'v> From<Option<&'v dyn Debug>> for ValueBag<'v> {
    #[inline]
    fn from(v: Option<&'v dyn Debug>) -> Self {
        ValueBag::from_option(v)
    }
}

impl<'v, 'u> From<&'v &'u dyn Debug> for ValueBag<'v>
where
    'u: 'v,
{
    #[inline]
    fn from(v: &'v &'u dyn Debug) -> Self {
        ValueBag::from_dyn_debug(*v)
    }
}

impl<'v> From<&'v dyn Display> for ValueBag<'v> {
    #[inline]
    fn from(v: &'v dyn Display) -> Self {
        ValueBag::from_dyn_display(v)
    }
}

impl<'v> From<Option<&'v dyn Display>> for ValueBag<'v> {
    #[inline]
    fn from(v: Option<&'v dyn Display>) -> Self {
        ValueBag::from_option(v)
    }
}

impl<'v, 'u> From<&'v &'u dyn Display> for ValueBag<'v>
where
    'u: 'v,
{
    #[inline]
    fn from(v: &'v &'u dyn Display) -> Self {
        ValueBag::from_dyn_display(*v)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::*;

    use super::*;
    use crate::{
        std::string::ToString,
        test::{IntoValueBag, TestToken},
    };

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn fmt_capture() {
        assert_eq!(
            ValueBag::capture_debug(&1u16).to_test_token(),
            TestToken::U64(1)
        );
        assert_eq!(
            ValueBag::capture_display(&1u16).to_test_token(),
            TestToken::U64(1)
        );

        assert_eq!(
            ValueBag::capture_debug(&Some(1u16)).to_test_token(),
            TestToken::U64(1)
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn fmt_fill() {
        assert_eq!(
            ValueBag::from_fill(&|slot: Slot| slot.fill_debug(1u16)).to_test_token(),
            TestToken::Str("1".into())
        );
        assert_eq!(
            ValueBag::from_fill(&|slot: Slot| slot.fill_display(1u16)).to_test_token(),
            TestToken::Str("1".into())
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn fmt_capture_args() {
        assert_eq!(
            ValueBag::from_debug(&format_args!("a {}", "value")).to_string(),
            "a value"
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn fmt_cast() {
        assert_eq!(
            42u64,
            ValueBag::capture_debug(&42u64)
                .to_u64()
                .expect("invalid value")
        );

        assert_eq!(
            "a string",
            ValueBag::capture_display(&"a string")
                .to_borrowed_str()
                .expect("invalid value")
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn fmt_downcast() {
        #[derive(Debug, PartialEq, Eq)]
        struct Timestamp(usize);

        impl Display for Timestamp {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "time is {}", self.0)
            }
        }

        let ts = Timestamp(42);

        assert_eq!(
            &ts,
            ValueBag::capture_debug(&ts)
                .downcast_ref::<Timestamp>()
                .expect("invalid value")
        );

        assert_eq!(
            &ts,
            ValueBag::capture_display(&ts)
                .downcast_ref::<Timestamp>()
                .expect("invalid value")
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn fmt_debug() {
        assert_eq!(
            format!("{:?}", "a string"),
            format!("{:?}", "a string".into_value_bag().by_ref()),
        );

        assert_eq!(
            format!("{:04?}", 42u64),
            format!("{:04?}", 42u64.into_value_bag().by_ref()),
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn fmt_display() {
        assert_eq!(
            format!("{}", "a string"),
            format!("{}", "a string".into_value_bag().by_ref()),
        );

        assert_eq!(
            format!("{:04}", 42u64),
            format!("{:04}", 42u64.into_value_bag().by_ref()),
        );
    }

    #[cfg(feature = "seq")]
    mod seq_support {
        use super::*;

        #[test]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
        fn fmt_debug_seq() {
            assert_eq!(
                "[01, 02, 03]",
                format!("{:>02?}", ValueBag::from_seq_slice(&[1, 2, 3]))
            );
        }

        #[test]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
        fn fmt_display_seq() {
            assert_eq!(
                "[1, 2, 3]",
                format!("{}", ValueBag::from_seq_slice(&[1, 2, 3]))
            );
        }
    }
}
