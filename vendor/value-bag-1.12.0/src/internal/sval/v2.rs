//! Integration between `Value` and `sval`.
//!
//! This module allows any `Value` to implement the `Value` trait,
//! and for any `Value` to be captured as a `Value`.

use crate::{
    fill::Slot,
    internal::{Internal, InternalVisitor},
    std::{any::Any, fmt},
    Error, ValueBag,
};

impl<'v> ValueBag<'v> {
    /// Get a value from a structured type.
    ///
    /// This method will attempt to capture the given value as a well-known primitive
    /// before resorting to using its `Value` implementation.
    pub fn capture_sval2<T>(value: &'v T) -> Self
    where
        T: value_bag_sval2::lib::Value + 'static,
    {
        Self::try_capture(value).unwrap_or(ValueBag {
            inner: Internal::Sval2(value),
        })
    }

    /// Get a value from a structured type without capturing support.
    pub const fn from_sval2<T>(value: &'v T) -> Self
    where
        T: value_bag_sval2::lib::Value,
    {
        ValueBag {
            inner: Internal::AnonSval2(value),
        }
    }

    /// Get a value from a structured type without capturing support.
    #[inline]
    pub const fn from_dyn_sval2(value: &'v dyn Value) -> Self {
        ValueBag {
            inner: Internal::AnonSval2(value),
        }
    }
}

pub(crate) trait DowncastValue {
    fn as_any(&self) -> &dyn Any;
    fn as_super(&self) -> &dyn Value;
}

impl<T: value_bag_sval2::lib::Value + 'static> DowncastValue for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_super(&self) -> &dyn Value {
        self
    }
}

impl<'s, 'f> Slot<'s, 'f> {
    /// Fill the slot with a structured value.
    ///
    /// The given value doesn't need to satisfy any particular lifetime constraints.
    pub fn fill_sval2<T>(self, value: T) -> Result<(), Error>
    where
        T: value_bag_sval2::lib::Value,
    {
        self.fill(|visitor| visitor.sval2(&value))
    }
}

impl<'v> value_bag_sval2::lib::Value for ValueBag<'v> {
    fn stream<'sval, S: value_bag_sval2::lib::Stream<'sval> + ?Sized>(
        &'sval self,
        s: &mut S,
    ) -> value_bag_sval2::lib::Result {
        use value_bag_sval2::lib_ref::ValueRef as _;

        self.stream_ref(s)
    }
}

impl<'sval> value_bag_sval2::lib_ref::ValueRef<'sval> for ValueBag<'sval> {
    fn stream_ref<S: value_bag_sval2::lib::Stream<'sval> + ?Sized>(
        &self,
        s: &mut S,
    ) -> value_bag_sval2::lib::Result {
        struct Sval2Visitor<'a, S: ?Sized>(&'a mut S);

        impl<'a, 'v, S: value_bag_sval2::lib::Stream<'v> + ?Sized> InternalVisitor<'v>
            for Sval2Visitor<'a, S>
        {
            fn fill(&mut self, v: &dyn crate::fill::Fill) -> Result<(), Error> {
                v.fill(crate::fill::Slot::new(self))
            }

            fn debug(&mut self, v: &dyn fmt::Debug) -> Result<(), Error> {
                value_bag_sval2::fmt::stream_debug(self.0, v).map_err(Error::from_sval2)
            }

            fn display(&mut self, v: &dyn fmt::Display) -> Result<(), Error> {
                value_bag_sval2::fmt::stream_display(self.0, v).map_err(Error::from_sval2)
            }

            fn u64(&mut self, v: u64) -> Result<(), Error> {
                self.0.u64(v).map_err(Error::from_sval2)
            }

            fn i64(&mut self, v: i64) -> Result<(), Error> {
                self.0.i64(v).map_err(Error::from_sval2)
            }

            fn u128(&mut self, v: &u128) -> Result<(), Error> {
                self.0.u128(*v).map_err(Error::from_sval2)
            }

            fn i128(&mut self, v: &i128) -> Result<(), Error> {
                self.0.i128(*v).map_err(Error::from_sval2)
            }

            fn f64(&mut self, v: f64) -> Result<(), Error> {
                self.0.f64(v).map_err(Error::from_sval2)
            }

            fn bool(&mut self, v: bool) -> Result<(), Error> {
                self.0.bool(v).map_err(Error::from_sval2)
            }

            fn char(&mut self, v: char) -> Result<(), Error> {
                let mut buf = [0; 4];
                let v = v.encode_utf8(&mut buf);

                self.0.value_computed(v).map_err(Error::from_sval2)
            }

            fn str(&mut self, v: &str) -> Result<(), Error> {
                self.0.value_computed(v).map_err(Error::from_sval2)
            }

            fn borrowed_str(&mut self, v: &'v str) -> Result<(), Error> {
                self.0.value(v).map_err(Error::from_sval2)
            }

            fn none(&mut self) -> Result<(), Error> {
                self.0.null().map_err(Error::from_sval2)
            }

            #[cfg(feature = "error")]
            fn error(&mut self, v: &(dyn std::error::Error + 'static)) -> Result<(), Error> {
                self.display(&v)
            }

            fn sval2(&mut self, v: &dyn Value) -> Result<(), Error> {
                self.0.value_computed(v).map_err(Error::from_sval2)
            }

            fn borrowed_sval2(&mut self, v: &'v dyn Value) -> Result<(), Error> {
                self.0.value(v).map_err(Error::from_sval2)
            }

            #[cfg(feature = "serde1")]
            fn serde1(
                &mut self,
                v: &dyn crate::internal::serde::v1::Serialize,
            ) -> Result<(), Error> {
                crate::internal::serde::v1::sval2(self.0, v)
            }

            #[cfg(feature = "seq")]
            fn seq(&mut self, v: &dyn crate::internal::seq::Seq) -> Result<(), Error> {
                self.0.seq_begin(None).map_err(Error::from_sval2)?;

                let mut s = seq::StreamVisitor {
                    stream: &mut *self.0,
                    err: None,
                };
                v.visit(&mut s);
                if let Some(e) = s.err {
                    return Err(Error::from_sval2(e));
                }

                self.0.seq_end().map_err(Error::from_sval2)
            }

            #[cfg(feature = "seq")]
            fn borrowed_seq(&mut self, v: &'v dyn crate::internal::seq::Seq) -> Result<(), Error> {
                self.0.seq_begin(None).map_err(Error::from_sval2)?;

                let mut s = seq::StreamVisitor {
                    stream: &mut *self.0,
                    err: None,
                };
                v.borrowed_visit(&mut s);
                if let Some(e) = s.err {
                    return Err(Error::from_sval2(e));
                }

                self.0.seq_end().map_err(Error::from_sval2)
            }

            fn poisoned(&mut self, msg: &'static str) -> Result<(), Error> {
                Err(Error::msg(msg))
            }
        }

        self.internal_visit(&mut Sval2Visitor(s))
            .map_err(Error::into_sval2)?;

        Ok(())
    }
}

pub use value_bag_sval2::dynamic::Value;

pub(in crate::internal) fn fmt(f: &mut fmt::Formatter, v: &dyn Value) -> Result<(), Error> {
    value_bag_sval2::fmt::stream_to_fmt(f, v)?;
    Ok(())
}

#[cfg(feature = "serde1")]
pub(in crate::internal) fn serde1<S>(s: S, v: &dyn Value) -> Result<S::Ok, S::Error>
where
    S: value_bag_serde1::lib::Serializer,
{
    value_bag_sval2::serde1::serialize(s, v)
}

pub(crate) fn internal_visit(v: &dyn Value, visitor: &mut dyn InternalVisitor<'_>) -> bool {
    let mut visitor = VisitorStream {
        visitor,
        text_buf: Default::default(),
    };

    value_bag_sval2::lib::stream_computed(&mut visitor, v).is_ok()
}

pub(crate) fn borrowed_internal_visit<'v>(
    v: &'v dyn Value,
    visitor: &mut dyn InternalVisitor<'v>,
) -> bool {
    let mut visitor = VisitorStream {
        visitor,
        text_buf: Default::default(),
    };

    value_bag_sval2::lib::stream(&mut visitor, v).is_ok()
}

struct VisitorStream<'a, 'v> {
    visitor: &'a mut dyn InternalVisitor<'v>,
    text_buf: value_bag_sval2::buffer::TextBuf<'v>,
}

impl<'a, 'v> value_bag_sval2::lib::Stream<'v> for VisitorStream<'a, 'v> {
    fn null(&mut self) -> value_bag_sval2::lib::Result {
        self.visitor.none().map_err(Error::into_sval2)
    }

    fn bool(&mut self, v: bool) -> value_bag_sval2::lib::Result {
        self.visitor.bool(v).map_err(Error::into_sval2)
    }

    fn i64(&mut self, v: i64) -> value_bag_sval2::lib::Result {
        self.visitor.i64(v).map_err(Error::into_sval2)
    }

    fn u64(&mut self, v: u64) -> value_bag_sval2::lib::Result {
        self.visitor.u64(v).map_err(Error::into_sval2)
    }

    fn i128(&mut self, v: i128) -> value_bag_sval2::lib::Result {
        self.visitor.i128(&v).map_err(Error::into_sval2)
    }

    fn u128(&mut self, v: u128) -> value_bag_sval2::lib::Result {
        self.visitor.u128(&v).map_err(Error::into_sval2)
    }

    fn f64(&mut self, v: f64) -> value_bag_sval2::lib::Result {
        self.visitor.f64(v).map_err(Error::into_sval2)
    }

    fn text_begin(&mut self, _: Option<usize>) -> value_bag_sval2::lib::Result {
        self.text_buf.clear();
        Ok(())
    }

    fn text_fragment_computed(&mut self, f: &str) -> value_bag_sval2::lib::Result {
        self.text_buf
            .push_fragment_computed(f)
            .map_err(|_| value_bag_sval2::lib::Error::new())
    }

    fn text_fragment(&mut self, f: &'v str) -> value_bag_sval2::lib::Result {
        self.text_buf
            .push_fragment(f)
            .map_err(|_| value_bag_sval2::lib::Error::new())
    }

    fn text_end(&mut self) -> value_bag_sval2::lib::Result {
        if let Some(v) = self.text_buf.as_borrowed_str() {
            self.visitor.borrowed_str(v).map_err(Error::into_sval2)
        } else {
            self.visitor
                .str(self.text_buf.as_str())
                .map_err(Error::into_sval2)
        }
    }

    fn seq_begin(&mut self, _: Option<usize>) -> value_bag_sval2::lib::Result {
        value_bag_sval2::lib::error()
    }

    fn seq_value_begin(&mut self) -> value_bag_sval2::lib::Result {
        value_bag_sval2::lib::error()
    }

    fn seq_value_end(&mut self) -> value_bag_sval2::lib::Result {
        value_bag_sval2::lib::error()
    }

    fn seq_end(&mut self) -> value_bag_sval2::lib::Result {
        value_bag_sval2::lib::error()
    }
}

impl Error {
    pub(in crate::internal) fn from_sval2(_: value_bag_sval2::lib::Error) -> Self {
        Error::msg("`sval` serialization failed")
    }

    pub(in crate::internal) fn into_sval2(self) -> value_bag_sval2::lib::Error {
        value_bag_sval2::lib::Error::new()
    }
}

#[cfg(feature = "seq")]
pub(crate) mod seq {
    use super::*;

    use crate::{
        internal::seq::{ExtendValue, Visitor},
        std::ops::ControlFlow,
    };

    pub(super) struct StreamVisitor<'a, S: ?Sized> {
        pub(super) stream: &'a mut S,
        pub(super) err: Option<value_bag_sval2::lib::Error>,
    }

    impl<'a, 'sval, S: value_bag_sval2::lib::Stream<'sval> + ?Sized> Visitor<'sval>
        for StreamVisitor<'a, S>
    {
        fn element(&mut self, v: ValueBag) -> ControlFlow<()> {
            if let Err(e) = self.stream.seq_value_begin() {
                self.err = Some(e);
                return ControlFlow::Break(());
            }

            if let Err(e) = value_bag_sval2::lib::stream_computed(&mut *self.stream, v) {
                self.err = Some(e);
                return ControlFlow::Break(());
            }

            if let Err(e) = self.stream.seq_value_end() {
                self.err = Some(e);
                return ControlFlow::Break(());
            }

            ControlFlow::Continue(())
        }

        fn borrowed_element(&mut self, v: ValueBag<'sval>) -> ControlFlow<()> {
            if let Err(e) = self.stream.seq_value_begin() {
                self.err = Some(e);
                return ControlFlow::Break(());
            }

            if let Err(e) = value_bag_sval2::lib_ref::stream_ref(&mut *self.stream, v) {
                self.err = Some(e);
                return ControlFlow::Break(());
            }

            if let Err(e) = self.stream.seq_value_end() {
                self.err = Some(e);
                return ControlFlow::Break(());
            }

            ControlFlow::Continue(())
        }
    }

    #[inline]
    pub(crate) fn extend<'a, 'b, S: Default + ExtendValue<'a>>(v: &'b dyn Value) -> Option<S> {
        let mut stream = Root {
            seq: None,
            text_buf: Default::default(),
            depth: 0,
        };

        value_bag_sval2::lib::stream_computed(&mut stream, v).ok()?;

        stream.seq
    }

    #[inline]
    pub(crate) fn extend_borrowed<'a, S: Default + ExtendValue<'a>>(v: &'a dyn Value) -> Option<S> {
        let mut stream = Root {
            seq: None,
            text_buf: Default::default(),
            depth: 0,
        };

        value_bag_sval2::lib::stream(&mut stream, v).ok()?;

        stream.seq
    }

    struct Root<'v, S> {
        seq: Option<S>,
        text_buf: value_bag_sval2::buffer::TextBuf<'v>,
        depth: usize,
    }

    fn extend_borrowed_internal<'sval>(
        seq: Option<&mut impl ExtendValue<'sval>>,
        depth: usize,
        v: impl Into<ValueBag<'sval>>,
    ) -> value_bag_sval2::lib::Result {
        if depth != 1 {
            return Ok(());
        }

        if let Some(seq) = seq {
            seq.extend_borrowed(v.into().inner);

            Ok(())
        } else {
            value_bag_sval2::lib::error()
        }
    }

    fn extend_internal<'a, 'sval>(
        seq: Option<&mut impl ExtendValue<'sval>>,
        depth: usize,
        v: impl Into<ValueBag<'a>>,
    ) -> value_bag_sval2::lib::Result {
        if depth != 1 {
            return Ok(());
        }

        if let Some(seq) = seq {
            seq.extend(v.into().inner);

            Ok(())
        } else {
            value_bag_sval2::lib::error()
        }
    }

    impl<'sval, S: Default + ExtendValue<'sval>> value_bag_sval2::lib::Stream<'sval>
        for Root<'sval, S>
    {
        fn null(&mut self) -> value_bag_sval2::lib::Result {
            extend_borrowed_internal(self.seq.as_mut(), self.depth, ())
        }

        fn bool(&mut self, v: bool) -> value_bag_sval2::lib::Result {
            extend_borrowed_internal(self.seq.as_mut(), self.depth, v)
        }

        fn i64(&mut self, v: i64) -> value_bag_sval2::lib::Result {
            extend_borrowed_internal(self.seq.as_mut(), self.depth, v)
        }

        fn u64(&mut self, v: u64) -> value_bag_sval2::lib::Result {
            extend_borrowed_internal(self.seq.as_mut(), self.depth, v)
        }

        fn i128(&mut self, v: i128) -> value_bag_sval2::lib::Result {
            #[cfg(feature = "inline-i128")]
            {
                extend_borrowed_internal(self.seq.as_mut(), self.depth, v)
            }
            #[cfg(not(feature = "inline-i128"))]
            {
                extend_internal(self.seq.as_mut(), self.depth, &v)
            }
        }

        fn u128(&mut self, v: u128) -> value_bag_sval2::lib::Result {
            #[cfg(feature = "inline-i128")]
            {
                extend_borrowed_internal(self.seq.as_mut(), self.depth, v)
            }
            #[cfg(not(feature = "inline-i128"))]
            {
                extend_internal(self.seq.as_mut(), self.depth, &v)
            }
        }

        fn f64(&mut self, v: f64) -> value_bag_sval2::lib::Result {
            extend_borrowed_internal(self.seq.as_mut(), self.depth, v)
        }

        fn text_begin(&mut self, _: Option<usize>) -> value_bag_sval2::lib::Result {
            self.text_buf.clear();
            Ok(())
        }

        fn text_fragment_computed(&mut self, f: &str) -> value_bag_sval2::lib::Result {
            self.text_buf
                .push_fragment_computed(f)
                .map_err(|_| value_bag_sval2::lib::Error::new())
        }

        fn text_fragment(&mut self, f: &'sval str) -> value_bag_sval2::lib::Result {
            self.text_buf
                .push_fragment(f)
                .map_err(|_| value_bag_sval2::lib::Error::new())
        }

        fn text_end(&mut self) -> value_bag_sval2::lib::Result {
            if let Some(v) = self.text_buf.as_borrowed_str() {
                extend_borrowed_internal(self.seq.as_mut(), self.depth, v)
            } else {
                let v = self.text_buf.as_str();
                extend_internal(self.seq.as_mut(), self.depth, v)
            }
        }

        fn seq_begin(&mut self, _: Option<usize>) -> value_bag_sval2::lib::Result {
            if self.seq.is_none() {
                self.seq = Some(S::default());
            }

            self.depth += 1;

            // Treat nested complex values as null
            // This ensures an upstream visitor sees them, but won't
            // be able to convert them into anything meaningful
            if self.depth > 1 {
                if let Some(ref mut seq) = self.seq {
                    seq.extend_borrowed(ValueBag::from(()).inner);
                }
            }

            Ok(())
        }

        fn seq_value_begin(&mut self) -> value_bag_sval2::lib::Result {
            Ok(())
        }

        fn seq_value_end(&mut self) -> value_bag_sval2::lib::Result {
            Ok(())
        }

        fn seq_end(&mut self) -> value_bag_sval2::lib::Result {
            self.depth -= 1;

            Ok(())
        }
    }
}

#[cfg(feature = "owned")]
pub(crate) mod owned {
    impl value_bag_sval2::lib::Value for crate::OwnedValueBag {
        fn stream<'sval, S: value_bag_sval2::lib::Stream<'sval> + ?Sized>(
            &'sval self,
            s: &mut S,
        ) -> value_bag_sval2::lib::Result {
            value_bag_sval2::lib_ref::ValueRef::stream_ref(&self.by_ref(), s)
        }
    }

    pub(crate) type OwnedValue = value_bag_sval2::buffer::Value<'static>;

    pub(crate) fn buffer(
        v: impl value_bag_sval2::lib::Value,
    ) -> Result<OwnedValue, value_bag_sval2::buffer::Error> {
        OwnedValue::collect_owned(v)
    }
}

impl<'v> From<&'v dyn Value> for ValueBag<'v> {
    #[inline]
    fn from(v: &'v dyn Value) -> Self {
        ValueBag::from_dyn_sval2(v)
    }
}

impl<'v> From<Option<&'v dyn Value>> for ValueBag<'v> {
    #[inline]
    fn from(v: Option<&'v dyn Value>) -> Self {
        ValueBag::from_option(v)
    }
}

impl<'v, 'u> From<&'v &'u dyn Value> for ValueBag<'v>
where
    'u: 'v,
{
    #[inline]
    fn from(v: &'v &'u dyn Value) -> Self {
        ValueBag::from_dyn_sval2(*v)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::*;

    use super::*;
    use crate::test::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn sval2_capture() {
        assert_eq!(
            ValueBag::capture_sval2(&42u64).to_test_token(),
            TestToken::U64(42)
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn sval2_fill() {
        assert_eq!(
            ValueBag::from_fill(&|slot: Slot| slot.fill_sval2(42u64)).to_test_token(),
            TestToken::Sval { version: 2 },
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn sval2_capture_cast() {
        assert_eq!(
            42u64,
            ValueBag::capture_sval2(&42u64)
                .to_u64()
                .expect("invalid value")
        );

        assert_eq!(
            "a string",
            ValueBag::capture_sval2(&"a string")
                .to_borrowed_str()
                .expect("invalid value")
        );

        #[cfg(feature = "std")]
        assert_eq!(
            "a string",
            ValueBag::capture_sval2(&"a string")
                .to_str()
                .expect("invalid value")
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn sval2_capture_cast_borrowed_str() {
        struct Number<'a>(&'a str);

        impl<'a> value_bag_sval2::lib::Value for Number<'a> {
            fn stream<'sval, S: value_bag_sval2::lib::Stream<'sval> + ?Sized>(
                &'sval self,
                stream: &mut S,
            ) -> value_bag_sval2::lib::Result {
                stream.tagged_begin(Some(&value_bag_sval2::lib::tags::NUMBER), None, None)?;
                stream.value(self.0)?;
                stream.tagged_end(Some(&value_bag_sval2::lib::tags::NUMBER), None, None)
            }
        }

        assert_eq!(
            "123.456e789",
            ValueBag::capture_sval2(&Number("123.456e789"))
                .to_borrowed_str()
                .expect("invalid value")
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn sval2_from_cast() {
        assert_eq!(
            42u64,
            ValueBag::from_sval2(&42u64)
                .to_u64()
                .expect("invalid value")
        );

        #[cfg(feature = "std")]
        assert_eq!(
            "a string",
            ValueBag::from_sval2(&"a string")
                .to_str()
                .expect("invalid value")
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn sval2_downcast() {
        #[derive(Debug, PartialEq, Eq)]
        struct Timestamp(usize);

        impl value_bag_sval2::lib::Value for Timestamp {
            fn stream<'sval, S: value_bag_sval2::lib::Stream<'sval> + ?Sized>(
                &'sval self,
                stream: &mut S,
            ) -> value_bag_sval2::lib::Result {
                stream.u64(self.0 as u64)
            }
        }

        let ts = Timestamp(42);

        assert_eq!(
            &ts,
            ValueBag::capture_sval2(&ts)
                .downcast_ref::<Timestamp>()
                .expect("invalid value")
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn sval2_structured() {
        let value = ValueBag::from(42u64);

        value_bag_sval2::test::assert_tokens(&value, &[value_bag_sval2::test::Token::U64(42)]);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn sval2_debug() {
        struct TestSval;

        impl value_bag_sval2::lib::Value for TestSval {
            fn stream<'sval, S: value_bag_sval2::lib::Stream<'sval> + ?Sized>(
                &'sval self,
                stream: &mut S,
            ) -> value_bag_sval2::lib::Result {
                stream.u64(42)
            }
        }

        assert_eq!(
            format!("{:04?}", 42u64),
            format!("{:04?}", ValueBag::capture_sval2(&TestSval)),
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn sval2_visit() {
        ValueBag::from_sval2(&42u64)
            .visit(TestVisit::default())
            .expect("failed to visit value");
        ValueBag::from_sval2(&-42i64)
            .visit(TestVisit::default())
            .expect("failed to visit value");
        ValueBag::from_sval2(&11f64)
            .visit(TestVisit::default())
            .expect("failed to visit value");
        ValueBag::from_sval2(&true)
            .visit(TestVisit::default())
            .expect("failed to visit value");
        ValueBag::from_sval2(&"some borrowed string")
            .visit(TestVisit::default())
            .expect("failed to visit value");
        ValueBag::from_sval2(&'n')
            .visit(TestVisit {
                str: "n",
                ..Default::default()
            })
            .expect("failed to visit value");
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg(feature = "serde1")]
    fn sval2_serde1() {
        use value_bag_serde1::test::{assert_ser_tokens, Token};

        struct TestSval;

        impl value_bag_sval2::lib::Value for TestSval {
            fn stream<'sval, S: value_bag_sval2::lib::Stream<'sval> + ?Sized>(
                &'sval self,
                stream: &mut S,
            ) -> value_bag_sval2::lib::Result {
                stream.u64(42)
            }
        }

        assert_ser_tokens(&ValueBag::capture_sval2(&TestSval), &[Token::U64(42)]);
    }

    #[cfg(feature = "seq")]
    mod seq_support {
        use super::*;

        use crate::std::vec::Vec;

        #[test]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
        fn sval2_stream_borrowed_str_seq() {
            let value = ValueBag::from_seq_slice(&["a", "b", "c"]);

            value_bag_sval2::test::assert_tokens(&value, {
                use value_bag_sval2::test::Token::*;

                &[
                    SeqBegin(None),
                    SeqValueBegin,
                    TextBegin(Some(1)),
                    TextFragment("a"),
                    TextEnd,
                    SeqValueEnd,
                    SeqValueBegin,
                    TextBegin(Some(1)),
                    TextFragment("b"),
                    TextEnd,
                    SeqValueEnd,
                    SeqValueBegin,
                    TextBegin(Some(1)),
                    TextFragment("c"),
                    TextEnd,
                    SeqValueEnd,
                    SeqEnd,
                ]
            });
        }

        #[test]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
        fn sval2_stream_str_seq() {
            let value = ValueBag::from_fill(&|slot: Slot| slot.fill_seq_slice(&["a", "b", "c"]));

            value_bag_sval2::test::assert_tokens(&value, {
                use value_bag_sval2::test::Token::*;

                &[
                    SeqBegin(None),
                    SeqValueBegin,
                    TextBegin(Some(1)),
                    TextFragmentComputed("a".into()),
                    TextEnd,
                    SeqValueEnd,
                    SeqValueBegin,
                    TextBegin(Some(1)),
                    TextFragmentComputed("b".into()),
                    TextEnd,
                    SeqValueEnd,
                    SeqValueBegin,
                    TextBegin(Some(1)),
                    TextFragmentComputed("c".into()),
                    TextEnd,
                    SeqValueEnd,
                    SeqEnd,
                ]
            });
        }

        #[test]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
        #[cfg(feature = "alloc")]
        fn sval2_borrowed_str_to_seq() {
            use crate::std::borrow::Cow;

            assert_eq!(
                vec![
                    Some(Cow::Borrowed("a string 1")),
                    Some(Cow::Borrowed("a string 2")),
                    Some(Cow::Borrowed("a string 3"))
                ],
                ValueBag::capture_sval2(&[&"a string 1", &"a string 2", &"a string 3",])
                    .to_str_seq::<Vec<Option<Cow<str>>>>()
                    .expect("invalid value")
            );
        }

        #[test]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
        fn sval2_to_seq() {
            assert_eq!(
                vec![Some(1.0), None, None, Some(2.0), Some(3.0), None],
                ValueBag::capture_sval2(&[
                    &1.0 as &dyn Value,
                    &true as &dyn Value,
                    &[1.0, 2.0, 3.0] as &dyn Value,
                    &2.0 as &dyn Value,
                    &3.0 as &dyn Value,
                    &"a string" as &dyn Value,
                ])
                .to_f64_seq::<Vec<Option<f64>>>()
                .expect("invalid value")
            );
        }

        #[test]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
        fn sval2_as_seq() {
            assert_eq!(
                vec![1.0, 2.0, 3.0],
                ValueBag::capture_sval2(&[1.0, 2.0, 3.0,]).as_f64_seq::<Vec<f64>>()
            );
        }
    }

    #[cfg(feature = "std")]
    mod std_support {
        use super::*;

        use crate::std::borrow::ToOwned;

        #[test]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
        fn sval2_cast() {
            assert_eq!(
                "a string",
                ValueBag::capture_sval2(&"a string".to_owned())
                    .by_ref()
                    .to_str()
                    .expect("invalid value")
            );
        }
    }

    #[cfg(feature = "owned")]
    mod owned_support {
        use super::*;

        #[test]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
        fn sval2_to_owned_poison() {
            struct Kaboom;

            impl value_bag_sval2::lib::Value for Kaboom {
                fn stream<'sval, S: value_bag_sval2::lib::Stream<'sval> + ?Sized>(
                    &'sval self,
                    _: &mut S,
                ) -> value_bag_sval2::lib::Result {
                    value_bag_sval2::lib::error()
                }
            }

            let value = ValueBag::capture_sval2(&Kaboom)
                .to_owned()
                .by_ref()
                .to_test_token();

            assert_eq!(
                TestToken::Poisoned("failed to buffer the value".into()),
                value
            );
        }
    }
}
