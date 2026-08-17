//! The internal `Value` serialization API.
//!
//! This implementation isn't intended to be public. It may need to change
//! for optimizations or to support new external serialization frameworks.

use crate::{fill::Fill, Error, ValueBag};

pub(crate) mod cast;
#[cfg(feature = "error")]
pub(crate) mod error;
pub(crate) mod fmt;
#[cfg(feature = "seq")]
pub(crate) mod seq;
#[cfg(feature = "serde1")]
pub(crate) mod serde;
#[cfg(feature = "sval2")]
pub(crate) mod sval;

#[cfg(feature = "owned")]
pub(crate) mod owned;

#[cfg(feature = "owned")]
use crate::std::sync::Arc;

// NOTE: It takes less space to have separate variants for the presence
// of a `TypeId` instead of using `Option<T>`, because `TypeId` doesn't
// have a niche value
/// A container for a structured value for a specific kind of visitor.
#[derive(Clone)]
pub(crate) enum Internal<'v> {
    // Primitive values
    Signed(i64),
    Unsigned(u64),
    #[cfg(not(feature = "inline-i128"))]
    BigSigned(&'v i128),
    #[cfg(feature = "inline-i128")]
    BigSigned(i128),
    #[cfg(not(feature = "inline-i128"))]
    BigUnsigned(&'v u128),
    #[cfg(feature = "inline-i128")]
    BigUnsigned(u128),
    Float(f64),
    Bool(bool),
    Char(char),
    Str(&'v str),
    None,

    // Captured values
    Fill(&'v dyn Fill),
    Debug(&'v dyn fmt::DowncastDebug),
    Display(&'v dyn fmt::DowncastDisplay),
    #[cfg(feature = "error")]
    Error(&'v dyn error::DowncastError),
    #[cfg(feature = "sval2")]
    Sval2(&'v dyn sval::v2::DowncastValue),
    #[cfg(feature = "serde1")]
    Serde1(&'v dyn serde::v1::DowncastSerialize),

    // Anonymous values
    AnonDebug(&'v dyn fmt::Debug),
    AnonDisplay(&'v dyn fmt::Display),
    #[cfg(feature = "error")]
    AnonError(&'v (dyn error::Error + 'static)),
    #[cfg(feature = "sval2")]
    AnonSval2(&'v dyn sval::v2::Value),
    #[cfg(feature = "serde1")]
    AnonSerde1(&'v dyn serde::v1::Serialize),
    #[cfg(feature = "seq")]
    AnonSeq(&'v dyn seq::Seq),

    // Shared values
    #[cfg(feature = "owned")]
    SharedDebug(Arc<dyn fmt::DowncastDebug + Send + Sync>),
    #[cfg(feature = "owned")]
    SharedDisplay(Arc<dyn fmt::DowncastDisplay + Send + Sync>),
    #[cfg(all(feature = "error", feature = "owned"))]
    SharedError(Arc<dyn error::DowncastError + Send + Sync>),
    #[cfg(all(feature = "serde1", feature = "owned"))]
    SharedSerde1(Arc<dyn serde::v1::DowncastSerialize + Send + Sync>),
    #[cfg(all(feature = "sval2", feature = "owned"))]
    SharedSval2(Arc<dyn sval::v2::DowncastValue + Send + Sync>),
    #[cfg(all(feature = "seq", feature = "owned"))]
    SharedSeq(Arc<dyn seq::DowncastSeq + Send + Sync>),

    // NOTE: These variants exist because we can't clone an `Arc` in `const` fns
    // (plus we may not want to anyways)
    #[cfg(feature = "owned")]
    SharedRefDebug(&'v Arc<dyn fmt::DowncastDebug + Send + Sync>),
    #[cfg(feature = "owned")]
    SharedRefDisplay(&'v Arc<dyn fmt::DowncastDisplay + Send + Sync>),
    #[cfg(all(feature = "error", feature = "owned"))]
    SharedRefError(&'v Arc<dyn error::DowncastError + Send + Sync>),
    #[cfg(all(feature = "serde1", feature = "owned"))]
    SharedRefSerde1(&'v Arc<dyn serde::v1::DowncastSerialize + Send + Sync>),
    #[cfg(all(feature = "sval2", feature = "owned"))]
    SharedRefSval2(&'v Arc<dyn sval::v2::DowncastValue + Send + Sync>),
    #[cfg(all(feature = "seq", feature = "owned"))]
    SharedRefSeq(&'v Arc<dyn seq::DowncastSeq + Send + Sync>),

    // Poisoned value
    #[cfg_attr(not(feature = "owned"), allow(dead_code))]
    Poisoned(&'static str),
}

/// The internal serialization contract.
pub(crate) trait InternalVisitor<'v> {
    fn fill(&mut self, v: &dyn Fill) -> Result<(), Error>;

    fn debug(&mut self, v: &dyn fmt::Debug) -> Result<(), Error>;
    fn borrowed_debug(&mut self, v: &'v dyn fmt::Debug) -> Result<(), Error> {
        self.debug(v)
    }
    #[cfg(feature = "owned")]
    fn shared_debug(&mut self, v: &Arc<dyn fmt::DowncastDebug + Send + Sync>) -> Result<(), Error> {
        self.debug(v)
    }
    fn display(&mut self, v: &dyn fmt::Display) -> Result<(), Error>;
    fn borrowed_display(&mut self, v: &'v dyn fmt::Display) -> Result<(), Error> {
        self.display(v)
    }
    #[cfg(feature = "owned")]
    fn shared_display(
        &mut self,
        v: &Arc<dyn fmt::DowncastDisplay + Send + Sync>,
    ) -> Result<(), Error> {
        self.display(v)
    }

    fn u64(&mut self, v: u64) -> Result<(), Error>;
    fn i64(&mut self, v: i64) -> Result<(), Error>;
    fn u128(&mut self, v: &u128) -> Result<(), Error>;
    #[cfg(not(feature = "inline-i128"))]
    fn borrowed_u128(&mut self, v: &'v u128) -> Result<(), Error> {
        self.u128(v)
    }
    fn i128(&mut self, v: &i128) -> Result<(), Error>;
    #[cfg(not(feature = "inline-i128"))]
    fn borrowed_i128(&mut self, v: &'v i128) -> Result<(), Error> {
        self.i128(v)
    }
    fn f64(&mut self, v: f64) -> Result<(), Error>;
    fn bool(&mut self, v: bool) -> Result<(), Error>;
    fn char(&mut self, v: char) -> Result<(), Error>;

    fn str(&mut self, v: &str) -> Result<(), Error>;
    fn borrowed_str(&mut self, v: &'v str) -> Result<(), Error> {
        self.str(v)
    }

    fn none(&mut self) -> Result<(), Error>;

    #[cfg(feature = "error")]
    fn error(&mut self, v: &(dyn error::Error + 'static)) -> Result<(), Error>;
    #[cfg(feature = "error")]
    fn borrowed_error(&mut self, v: &'v (dyn error::Error + 'static)) -> Result<(), Error> {
        self.error(v)
    }
    #[cfg(all(feature = "error", feature = "owned"))]
    fn shared_error(
        &mut self,
        v: &Arc<dyn error::DowncastError + Send + Sync>,
    ) -> Result<(), Error> {
        self.error(v.as_super())
    }

    #[cfg(feature = "sval2")]
    fn sval2(&mut self, v: &dyn sval::v2::Value) -> Result<(), Error>;
    #[cfg(feature = "sval2")]
    fn borrowed_sval2(&mut self, v: &'v dyn sval::v2::Value) -> Result<(), Error> {
        self.sval2(v)
    }
    #[cfg(all(feature = "sval2", feature = "owned"))]
    fn shared_sval2(
        &mut self,
        v: &Arc<dyn sval::v2::DowncastValue + Send + Sync>,
    ) -> Result<(), Error> {
        self.sval2(v.as_super())
    }

    #[cfg(feature = "serde1")]
    fn serde1(&mut self, v: &dyn serde::v1::Serialize) -> Result<(), Error>;
    #[cfg(feature = "serde1")]
    fn borrowed_serde1(&mut self, v: &'v dyn serde::v1::Serialize) -> Result<(), Error> {
        self.serde1(v)
    }
    #[cfg(all(feature = "serde1", feature = "owned"))]
    fn shared_serde1(
        &mut self,
        v: &Arc<dyn serde::v1::DowncastSerialize + Send + Sync>,
    ) -> Result<(), Error> {
        self.serde1(v.as_super())
    }

    #[cfg(feature = "seq")]
    fn seq(&mut self, v: &dyn seq::Seq) -> Result<(), Error>;

    #[cfg(feature = "seq")]
    fn borrowed_seq(&mut self, v: &'v dyn seq::Seq) -> Result<(), Error> {
        self.seq(v)
    }

    #[cfg(all(feature = "seq", feature = "owned"))]
    fn shared_seq(&mut self, v: &Arc<dyn seq::DowncastSeq + Send + Sync>) -> Result<(), Error> {
        self.seq(v.as_super())
    }

    fn poisoned(&mut self, msg: &'static str) -> Result<(), Error>;
}

impl<'a, 'v, V: InternalVisitor<'v> + ?Sized> InternalVisitor<'v> for &'a mut V {
    fn fill(&mut self, v: &dyn Fill) -> Result<(), Error> {
        (**self).fill(v)
    }

    fn debug(&mut self, v: &dyn fmt::Debug) -> Result<(), Error> {
        (**self).debug(v)
    }

    fn borrowed_debug(&mut self, v: &'v dyn fmt::Debug) -> Result<(), Error> {
        (**self).borrowed_debug(v)
    }

    #[cfg(feature = "owned")]
    fn shared_debug(&mut self, v: &Arc<dyn fmt::DowncastDebug + Send + Sync>) -> Result<(), Error> {
        (**self).shared_debug(v)
    }

    fn display(&mut self, v: &dyn fmt::Display) -> Result<(), Error> {
        (**self).display(v)
    }

    fn borrowed_display(&mut self, v: &'v dyn fmt::Display) -> Result<(), Error> {
        (**self).borrowed_display(v)
    }

    #[cfg(feature = "owned")]
    fn shared_display(
        &mut self,
        v: &Arc<dyn fmt::DowncastDisplay + Send + Sync>,
    ) -> Result<(), Error> {
        (**self).shared_display(v)
    }

    fn u64(&mut self, v: u64) -> Result<(), Error> {
        (**self).u64(v)
    }

    fn i64(&mut self, v: i64) -> Result<(), Error> {
        (**self).i64(v)
    }

    fn u128(&mut self, v: &u128) -> Result<(), Error> {
        (**self).u128(v)
    }

    #[cfg(not(feature = "inline-i128"))]
    fn borrowed_u128(&mut self, v: &'v u128) -> Result<(), Error> {
        (**self).borrowed_u128(v)
    }

    fn i128(&mut self, v: &i128) -> Result<(), Error> {
        (**self).i128(v)
    }

    #[cfg(not(feature = "inline-i128"))]
    fn borrowed_i128(&mut self, v: &'v i128) -> Result<(), Error> {
        (**self).borrowed_i128(v)
    }

    fn f64(&mut self, v: f64) -> Result<(), Error> {
        (**self).f64(v)
    }

    fn bool(&mut self, v: bool) -> Result<(), Error> {
        (**self).bool(v)
    }

    fn char(&mut self, v: char) -> Result<(), Error> {
        (**self).char(v)
    }

    fn str(&mut self, v: &str) -> Result<(), Error> {
        (**self).str(v)
    }

    fn borrowed_str(&mut self, v: &'v str) -> Result<(), Error> {
        (**self).borrowed_str(v)
    }

    fn none(&mut self) -> Result<(), Error> {
        (**self).none()
    }

    #[cfg(feature = "error")]
    fn error(&mut self, v: &(dyn error::Error + 'static)) -> Result<(), Error> {
        (**self).error(v)
    }

    #[cfg(feature = "error")]
    fn borrowed_error(&mut self, v: &'v (dyn error::Error + 'static)) -> Result<(), Error> {
        (**self).borrowed_error(v)
    }

    #[cfg(all(feature = "error", feature = "owned"))]
    fn shared_error(
        &mut self,
        v: &Arc<dyn error::DowncastError + Send + Sync>,
    ) -> Result<(), Error> {
        (**self).shared_error(v)
    }

    #[cfg(feature = "sval2")]
    fn sval2(&mut self, v: &dyn sval::v2::Value) -> Result<(), Error> {
        (**self).sval2(v)
    }

    #[cfg(feature = "sval2")]
    fn borrowed_sval2(&mut self, v: &'v dyn sval::v2::Value) -> Result<(), Error> {
        (**self).borrowed_sval2(v)
    }

    #[cfg(all(feature = "sval2", feature = "owned"))]
    fn shared_sval2(
        &mut self,
        v: &Arc<dyn sval::v2::DowncastValue + Send + Sync>,
    ) -> Result<(), Error> {
        (**self).shared_sval2(v)
    }

    #[cfg(feature = "serde1")]
    fn serde1(&mut self, v: &dyn serde::v1::Serialize) -> Result<(), Error> {
        (**self).serde1(v)
    }

    #[cfg(feature = "serde1")]
    fn borrowed_serde1(&mut self, v: &'v dyn serde::v1::Serialize) -> Result<(), Error> {
        (**self).borrowed_serde1(v)
    }

    #[cfg(all(feature = "serde1", feature = "owned"))]
    fn shared_serde1(
        &mut self,
        v: &Arc<dyn serde::v1::DowncastSerialize + Send + Sync>,
    ) -> Result<(), Error> {
        (**self).shared_serde1(v)
    }

    #[cfg(feature = "seq")]
    fn seq(&mut self, seq: &dyn seq::Seq) -> Result<(), Error> {
        (**self).seq(seq)
    }

    #[cfg(feature = "seq")]
    fn borrowed_seq(&mut self, seq: &'v dyn seq::Seq) -> Result<(), Error> {
        (**self).borrowed_seq(seq)
    }

    #[cfg(all(feature = "seq", feature = "owned"))]
    fn shared_seq(&mut self, seq: &Arc<dyn seq::DowncastSeq + Send + Sync>) -> Result<(), Error> {
        (**self).shared_seq(seq)
    }

    fn poisoned(&mut self, msg: &'static str) -> Result<(), Error> {
        (**self).poisoned(msg)
    }
}

impl<'v> ValueBag<'v> {
    /// Visit the value using an internal visitor.
    #[inline]
    pub(crate) fn internal_visit(&self, visitor: impl InternalVisitor<'v>) -> Result<(), Error> {
        self.inner.internal_visit(visitor)
    }
}

impl<'v> Internal<'v> {
    #[inline]
    pub(crate) const fn by_ref(&self) -> Internal<'_> {
        match self {
            Internal::Signed(value) => Internal::Signed(*value),
            Internal::Unsigned(value) => Internal::Unsigned(*value),
            Internal::BigSigned(value) => Internal::BigSigned(*value),
            Internal::BigUnsigned(value) => Internal::BigUnsigned(*value),
            Internal::Float(value) => Internal::Float(*value),
            Internal::Bool(value) => Internal::Bool(*value),
            Internal::Char(value) => Internal::Char(*value),
            Internal::Str(value) => Internal::Str(value),
            Internal::None => Internal::None,

            Internal::Fill(value) => Internal::Fill(*value),

            Internal::AnonDebug(value) => Internal::AnonDebug(*value),
            Internal::Debug(value) => Internal::Debug(*value),

            Internal::AnonDisplay(value) => Internal::AnonDisplay(*value),
            Internal::Display(value) => Internal::Display(*value),

            #[cfg(feature = "error")]
            Internal::AnonError(value) => Internal::AnonError(*value),
            #[cfg(feature = "error")]
            Internal::Error(value) => Internal::Error(*value),

            #[cfg(feature = "sval2")]
            Internal::AnonSval2(value) => Internal::AnonSval2(*value),
            #[cfg(feature = "sval2")]
            Internal::Sval2(value) => Internal::Sval2(*value),

            #[cfg(feature = "serde1")]
            Internal::AnonSerde1(value) => Internal::AnonSerde1(*value),
            #[cfg(feature = "serde1")]
            Internal::Serde1(value) => Internal::Serde1(*value),

            #[cfg(feature = "seq")]
            Internal::AnonSeq(value) => Internal::AnonSeq(*value),

            #[cfg(feature = "owned")]
            Internal::SharedDebug(ref value) => Internal::SharedRefDebug(value),
            #[cfg(feature = "owned")]
            Internal::SharedDisplay(ref value) => Internal::SharedRefDisplay(value),
            #[cfg(all(feature = "error", feature = "owned"))]
            Internal::SharedError(ref value) => Internal::SharedRefError(value),
            #[cfg(all(feature = "serde1", feature = "owned"))]
            Internal::SharedSerde1(ref value) => Internal::SharedRefSerde1(value),
            #[cfg(all(feature = "sval2", feature = "owned"))]
            Internal::SharedSval2(ref value) => Internal::SharedRefSval2(value),
            #[cfg(all(feature = "seq", feature = "owned"))]
            Internal::SharedSeq(ref value) => Internal::SharedRefSeq(value),

            #[cfg(feature = "owned")]
            Internal::SharedRefDebug(value) => Internal::SharedRefDebug(*value),
            #[cfg(feature = "owned")]
            Internal::SharedRefDisplay(value) => Internal::SharedRefDisplay(*value),
            #[cfg(all(feature = "error", feature = "owned"))]
            Internal::SharedRefError(value) => Internal::SharedRefError(*value),
            #[cfg(all(feature = "serde1", feature = "owned"))]
            Internal::SharedRefSerde1(value) => Internal::SharedRefSerde1(*value),
            #[cfg(all(feature = "sval2", feature = "owned"))]
            Internal::SharedRefSval2(value) => Internal::SharedRefSval2(*value),
            #[cfg(all(feature = "seq", feature = "owned"))]
            Internal::SharedRefSeq(value) => Internal::SharedRefSeq(*value),

            Internal::Poisoned(msg) => Internal::Poisoned(msg),
        }
    }

    #[inline]
    pub(crate) fn internal_visit(
        &self,
        mut visitor: impl InternalVisitor<'v>,
    ) -> Result<(), Error> {
        match self {
            Internal::Signed(value) => visitor.i64(*value),
            Internal::Unsigned(value) => visitor.u64(*value),
            #[cfg(feature = "inline-i128")]
            Internal::BigSigned(value) => visitor.i128(value),
            #[cfg(feature = "inline-i128")]
            Internal::BigUnsigned(value) => visitor.u128(value),
            #[cfg(not(feature = "inline-i128"))]
            Internal::BigSigned(value) => visitor.borrowed_i128(value),
            #[cfg(not(feature = "inline-i128"))]
            Internal::BigUnsigned(value) => visitor.borrowed_u128(value),
            Internal::Float(value) => visitor.f64(*value),
            Internal::Bool(value) => visitor.bool(*value),
            Internal::Char(value) => visitor.char(*value),
            Internal::Str(value) => visitor.borrowed_str(value),
            Internal::None => visitor.none(),

            Internal::Fill(value) => visitor.fill(*value),

            Internal::AnonDebug(value) => visitor.borrowed_debug(*value),
            Internal::Debug(value) => visitor.borrowed_debug(value.as_super()),

            Internal::AnonDisplay(value) => visitor.borrowed_display(*value),
            Internal::Display(value) => visitor.borrowed_display(value.as_super()),

            #[cfg(feature = "error")]
            Internal::AnonError(value) => visitor.borrowed_error(*value),
            #[cfg(feature = "error")]
            Internal::Error(value) => visitor.borrowed_error(value.as_super()),

            #[cfg(feature = "sval2")]
            Internal::AnonSval2(value) => visitor.borrowed_sval2(*value),
            #[cfg(feature = "sval2")]
            Internal::Sval2(value) => visitor.borrowed_sval2(value.as_super()),

            #[cfg(feature = "serde1")]
            Internal::AnonSerde1(value) => visitor.borrowed_serde1(*value),
            #[cfg(feature = "serde1")]
            Internal::Serde1(value) => visitor.borrowed_serde1(value.as_super()),

            #[cfg(feature = "seq")]
            Internal::AnonSeq(value) => visitor.borrowed_seq(*value),

            #[cfg(feature = "owned")]
            Internal::SharedDebug(ref value) => visitor.shared_debug(value),
            #[cfg(feature = "owned")]
            Internal::SharedDisplay(ref value) => visitor.shared_display(value),
            #[cfg(all(feature = "error", feature = "owned"))]
            Internal::SharedError(ref value) => visitor.shared_error(value),
            #[cfg(all(feature = "serde1", feature = "owned"))]
            Internal::SharedSerde1(ref value) => visitor.shared_serde1(value),
            #[cfg(all(feature = "sval2", feature = "owned"))]
            Internal::SharedSval2(ref value) => visitor.shared_sval2(value),
            #[cfg(all(feature = "seq", feature = "owned"))]
            Internal::SharedSeq(value) => visitor.shared_seq(value),

            #[cfg(feature = "owned")]
            Internal::SharedRefDebug(value) => visitor.shared_debug(value),
            #[cfg(feature = "owned")]
            Internal::SharedRefDisplay(value) => visitor.shared_display(value),
            #[cfg(all(feature = "error", feature = "owned"))]
            Internal::SharedRefError(value) => visitor.shared_error(value),
            #[cfg(all(feature = "serde1", feature = "owned"))]
            Internal::SharedRefSerde1(value) => visitor.shared_serde1(value),
            #[cfg(all(feature = "sval2", feature = "owned"))]
            Internal::SharedRefSval2(value) => visitor.shared_sval2(value),
            #[cfg(all(feature = "seq", feature = "owned"))]
            Internal::SharedRefSeq(value) => visitor.shared_seq(value),

            Internal::Poisoned(msg) => visitor.poisoned(msg),
        }
    }
}
