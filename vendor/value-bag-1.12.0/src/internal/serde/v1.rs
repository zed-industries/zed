//! Integration between `Value` and `serde`.
//!
//! This module allows any `Value` to implement the `Serialize` trait,
//! and for any `Serialize` to be captured as a `Value`.

use crate::{
    fill::Slot,
    internal::{Internal, InternalVisitor},
    std::{any::Any, fmt},
    Error, ValueBag,
};

use value_bag_serde1::lib::ser::{Error as SerdeError, Impossible};

impl<'v> ValueBag<'v> {
    /// Get a value from a structured type.
    ///
    /// This method will attempt to capture the given value as a well-known primitive
    /// before resorting to using its `Value` implementation.
    pub fn capture_serde1<T>(value: &'v T) -> Self
    where
        T: value_bag_serde1::lib::Serialize + 'static,
    {
        Self::try_capture(value).unwrap_or(ValueBag {
            inner: Internal::Serde1(value),
        })
    }

    /// Get a value from a structured type without capturing support.
    pub const fn from_serde1<T>(value: &'v T) -> Self
    where
        T: value_bag_serde1::lib::Serialize,
    {
        ValueBag {
            inner: Internal::AnonSerde1(value),
        }
    }

    // NOTE: no `from_dyn_serde1` until `erased-serde` stabilizes
    pub(crate) const fn from_dyn_serde1(value: &'v dyn Serialize) -> Self {
        ValueBag {
            inner: Internal::AnonSerde1(value),
        }
    }
}

pub(crate) trait DowncastSerialize {
    fn as_any(&self) -> &dyn Any;
    fn as_super(&self) -> &dyn Serialize;
}

impl<T: value_bag_serde1::lib::Serialize + 'static> DowncastSerialize for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_super(&self) -> &dyn Serialize {
        self
    }
}

impl<'s, 'f> Slot<'s, 'f> {
    /// Fill the slot with a structured value.
    ///
    /// The given value doesn't need to satisfy any particular lifetime constraints.
    pub fn fill_serde1<T>(self, value: T) -> Result<(), Error>
    where
        T: value_bag_serde1::lib::Serialize,
    {
        self.fill(|visitor| visitor.serde1(&value))
    }
}

impl<'v> value_bag_serde1::lib::Serialize for ValueBag<'v> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: value_bag_serde1::lib::Serializer,
    {
        struct Serde1Visitor<S>
        where
            S: value_bag_serde1::lib::Serializer,
        {
            inner: Option<S>,
            result: Option<Result<S::Ok, S::Error>>,
        }

        impl<S> Serde1Visitor<S>
        where
            S: value_bag_serde1::lib::Serializer,
        {
            fn result(&self) -> Result<(), Error> {
                match self.result {
                    Some(Ok(_)) => Ok(()),
                    Some(Err(ref e)) => Err(Error::serde(e)),
                    None => Err(Error::msg("`serde` serialization didn't produce a result")),
                }
            }

            fn serializer(&mut self) -> Result<S, Error> {
                self.inner
                    .take()
                    .ok_or_else(|| Error::msg("`serde` serializer is in an invalid state"))
            }

            fn into_result(self) -> Result<S::Ok, S::Error> {
                self.result.unwrap_or_else(|| {
                    Err(S::Error::custom(
                        "`serde` serialization didn't produce a result",
                    ))
                })
            }
        }

        impl<'v, S> InternalVisitor<'v> for Serde1Visitor<S>
        where
            S: value_bag_serde1::lib::Serializer,
        {
            fn fill(&mut self, v: &dyn crate::fill::Fill) -> Result<(), Error> {
                v.fill(Slot::new(self))
            }

            fn debug(&mut self, v: &dyn fmt::Debug) -> Result<(), Error> {
                struct DebugToDisplay<T>(T);

                impl<T> fmt::Display for DebugToDisplay<T>
                where
                    T: fmt::Debug,
                {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        fmt::Debug::fmt(&self.0, f)
                    }
                }

                self.result = Some(self.serializer()?.collect_str(&DebugToDisplay(v)));
                self.result()
            }

            fn display(&mut self, v: &dyn fmt::Display) -> Result<(), Error> {
                self.result = Some(self.serializer()?.collect_str(v));
                self.result()
            }

            fn u64(&mut self, v: u64) -> Result<(), Error> {
                self.result = Some(self.serializer()?.serialize_u64(v));
                self.result()
            }

            fn i64(&mut self, v: i64) -> Result<(), Error> {
                self.result = Some(self.serializer()?.serialize_i64(v));
                self.result()
            }

            fn u128(&mut self, v: &u128) -> Result<(), Error> {
                self.result = Some(self.serializer()?.serialize_u128(*v));
                self.result()
            }

            fn i128(&mut self, v: &i128) -> Result<(), Error> {
                self.result = Some(self.serializer()?.serialize_i128(*v));
                self.result()
            }

            fn f64(&mut self, v: f64) -> Result<(), Error> {
                self.result = Some(self.serializer()?.serialize_f64(v));
                self.result()
            }

            fn bool(&mut self, v: bool) -> Result<(), Error> {
                self.result = Some(self.serializer()?.serialize_bool(v));
                self.result()
            }

            fn char(&mut self, v: char) -> Result<(), Error> {
                self.result = Some(self.serializer()?.serialize_char(v));
                self.result()
            }

            fn str(&mut self, v: &str) -> Result<(), Error> {
                self.result = Some(self.serializer()?.serialize_str(v));
                self.result()
            }

            fn none(&mut self) -> Result<(), Error> {
                self.result = Some(self.serializer()?.serialize_none());
                self.result()
            }

            #[cfg(feature = "error")]
            fn error(&mut self, v: &(dyn std::error::Error + 'static)) -> Result<(), Error> {
                self.result = Some(self.serializer()?.collect_str(v));
                self.result()
            }

            #[cfg(feature = "sval2")]
            fn sval2(&mut self, v: &dyn crate::internal::sval::v2::Value) -> Result<(), Error> {
                self.result = Some(crate::internal::sval::v2::serde1(self.serializer()?, v));
                self.result()
            }

            fn serde1(&mut self, v: &dyn Serialize) -> Result<(), Error> {
                self.result = Some(value_bag_serde1::erased::serialize(v, self.serializer()?));
                self.result()
            }

            #[cfg(feature = "seq")]
            fn seq(&mut self, v: &dyn crate::internal::seq::Seq) -> Result<(), Error> {
                self.result = Some(serialize_seq(self.serializer()?, v));
                self.result()
            }

            fn poisoned(&mut self, msg: &'static str) -> Result<(), Error> {
                self.result = Some(Err(S::Error::custom(msg)));
                self.result()
            }
        }

        let mut visitor = Serde1Visitor {
            inner: Some(s),
            result: None,
        };

        self.internal_visit(&mut visitor)
            .map_err(S::Error::custom)?;

        visitor.into_result()
    }
}

pub use value_bag_serde1::erased::Serialize;

pub(in crate::internal) fn fmt(f: &mut fmt::Formatter, v: &dyn Serialize) -> Result<(), Error> {
    fmt::Debug::fmt(&value_bag_serde1::fmt::to_debug(v), f)?;
    Ok(())
}

#[cfg(feature = "sval2")]
pub(in crate::internal) fn sval2<'sval, S: value_bag_sval2::lib::Stream<'sval> + ?Sized>(
    s: &mut S,
    v: &dyn Serialize,
) -> Result<(), Error> {
    value_bag_sval2::serde1::stream(s, v).map_err(Error::from_sval2)
}

#[cfg(feature = "seq")]
fn serialize_seq<S: value_bag_serde1::lib::Serializer>(
    s: S,
    seq: &dyn crate::internal::seq::Seq,
) -> Result<S::Ok, S::Error> {
    use crate::std::ops::ControlFlow;

    use value_bag_serde1::lib::ser::SerializeSeq;

    struct SerializeVisitor<S: SerializeSeq> {
        serializer: S,
        err: Option<S::Error>,
    }

    impl<'v, S: SerializeSeq> crate::internal::seq::Visitor<'v> for SerializeVisitor<S> {
        fn element(&mut self, v: ValueBag) -> ControlFlow<()> {
            match self.serializer.serialize_element(&v) {
                Ok(()) => ControlFlow::Continue(()),
                Err(e) => {
                    self.err = Some(e);
                    ControlFlow::Break(())
                }
            }
        }
    }

    let mut s = SerializeVisitor {
        serializer: s.serialize_seq(None)?,
        err: None,
    };
    seq.visit(&mut s);
    if let Some(e) = s.err {
        return Err(e);
    }

    s.serializer.end()
}

pub(crate) fn internal_visit(v: &dyn Serialize, visitor: &mut dyn InternalVisitor<'_>) -> bool {
    struct VisitorSerializer<'a, 'v>(&'a mut dyn InternalVisitor<'v>);

    impl<'a, 'v> value_bag_serde1::lib::Serializer for VisitorSerializer<'a, 'v> {
        type Ok = ();
        type Error = Unsupported;

        type SerializeSeq = Impossible<Self::Ok, Self::Error>;
        type SerializeTuple = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
        type SerializeMap = Impossible<Self::Ok, Self::Error>;
        type SerializeStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

        fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
            self.0.u64(v as u64).map_err(|_| Unsupported)
        }

        fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
            self.0.u64(v as u64).map_err(|_| Unsupported)
        }

        fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
            self.0.u64(v as u64).map_err(|_| Unsupported)
        }

        fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
            self.0.u64(v).map_err(|_| Unsupported)
        }

        fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
            self.0.u128(&v).map_err(|_| Unsupported)
        }

        fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
            self.0.i64(v as i64).map_err(|_| Unsupported)
        }

        fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
            self.0.i64(v as i64).map_err(|_| Unsupported)
        }

        fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
            self.0.i64(v as i64).map_err(|_| Unsupported)
        }

        fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
            self.0.i64(v).map_err(|_| Unsupported)
        }

        fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
            self.0.i128(&v).map_err(|_| Unsupported)
        }

        fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
            self.0.f64(v as f64).map_err(|_| Unsupported)
        }

        fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
            self.0.f64(v).map_err(|_| Unsupported)
        }

        fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
            self.0.char(v).map_err(|_| Unsupported)
        }

        fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
            self.0.bool(v).map_err(|_| Unsupported)
        }

        fn serialize_some<T>(self, v: &T) -> Result<Self::Ok, Self::Error>
        where
            T: value_bag_serde1::lib::Serialize + ?Sized,
        {
            v.serialize(self)
        }

        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            self.0.none().map_err(|_| Unsupported)
        }

        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            self.0.none().map_err(|_| Unsupported)
        }

        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
            Err(Unsupported)
        }

        fn serialize_str(self, s: &str) -> Result<Self::Ok, Self::Error> {
            self.0.str(s).map_err(|_| Unsupported)
        }

        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
            Err(Unsupported)
        }

        fn serialize_unit_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            Err(Unsupported)
        }

        fn serialize_newtype_struct<T>(
            self,
            _: &'static str,
            _: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: value_bag_serde1::lib::Serialize + ?Sized,
        {
            Err(Unsupported)
        }

        fn serialize_newtype_variant<T>(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: value_bag_serde1::lib::Serialize + ?Sized,
        {
            Err(Unsupported)
        }

        fn serialize_seq(
            self,
            _: core::option::Option<usize>,
        ) -> Result<Self::SerializeSeq, Self::Error> {
            Err(Unsupported)
        }

        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
            Err(Unsupported)
        }

        fn serialize_tuple_struct(
            self,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeTupleStruct, Self::Error> {
            Err(Unsupported)
        }

        fn serialize_tuple_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeTupleVariant, Self::Error> {
            Err(Unsupported)
        }

        fn serialize_map(
            self,
            _: core::option::Option<usize>,
        ) -> Result<Self::SerializeMap, Self::Error> {
            Err(Unsupported)
        }

        fn serialize_struct(
            self,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeStruct, Self::Error> {
            Err(Unsupported)
        }

        fn serialize_struct_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeStructVariant, Self::Error> {
            Err(Unsupported)
        }
    }

    value_bag_serde1::erased::serialize(v, VisitorSerializer(visitor)).is_ok()
}

impl Error {
    fn serde(e: impl fmt::Display) -> Self {
        Error::try_boxed("`serde` serialization failed", e)
    }
}

#[derive(Debug)]
struct Unsupported;

impl fmt::Display for Unsupported {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid cast")
    }
}

impl value_bag_serde1::lib::ser::Error for Unsupported {
    fn custom<T>(_: T) -> Self
    where
        T: fmt::Display,
    {
        Unsupported
    }
}

impl value_bag_serde1::lib::ser::StdError for Unsupported {}

#[cfg(feature = "seq")]
pub(crate) mod seq {
    use super::*;

    use crate::internal::seq::ExtendValue;

    #[inline]
    pub(crate) fn extend<'a, S: Default + ExtendValue<'a>>(v: &dyn Serialize) -> Option<S> {
        use crate::std::marker::PhantomData;

        struct Root<S>(PhantomData<S>);

        struct Seq<S>(S);

        impl<'a, S: Default + ExtendValue<'a>> value_bag_serde1::lib::Serializer for Root<S> {
            type Ok = S;

            type Error = Unsupported;

            type SerializeSeq = Seq<S>;

            type SerializeTuple = Seq<S>;

            type SerializeTupleStruct = value_bag_serde1::lib::ser::Impossible<S, Unsupported>;

            type SerializeTupleVariant = value_bag_serde1::lib::ser::Impossible<S, Unsupported>;

            type SerializeMap = value_bag_serde1::lib::ser::Impossible<S, Unsupported>;

            type SerializeStruct = value_bag_serde1::lib::ser::Impossible<S, Unsupported>;

            type SerializeStructVariant = value_bag_serde1::lib::ser::Impossible<S, Unsupported>;

            fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_some<T: value_bag_serde1::lib::Serialize + ?Sized>(
                self,
                value: &T,
            ) -> Result<Self::Ok, Self::Error> {
                value.serialize(self)
            }

            fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_unit_variant(
                self,
                _: &'static str,
                _: u32,
                _: &'static str,
            ) -> Result<Self::Ok, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_newtype_struct<T: value_bag_serde1::lib::Serialize + ?Sized>(
                self,
                _: &'static str,
                _: &T,
            ) -> Result<Self::Ok, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_newtype_variant<T: value_bag_serde1::lib::Serialize + ?Sized>(
                self,
                _: &'static str,
                _: u32,
                _: &'static str,
                _: &T,
            ) -> Result<Self::Ok, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
                Ok(Seq(S::default()))
            }

            fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
                self.serialize_seq(Some(len))
            }

            fn serialize_tuple_struct(
                self,
                _: &'static str,
                _: usize,
            ) -> Result<Self::SerializeTupleStruct, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_tuple_variant(
                self,
                _: &'static str,
                _: u32,
                _: &'static str,
                _: usize,
            ) -> Result<Self::SerializeTupleVariant, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_struct(
                self,
                _: &'static str,
                _: usize,
            ) -> Result<Self::SerializeStruct, Self::Error> {
                Err(Unsupported)
            }

            fn serialize_struct_variant(
                self,
                _: &'static str,
                _: u32,
                _: &'static str,
                _: usize,
            ) -> Result<Self::SerializeStructVariant, Self::Error> {
                Err(Unsupported)
            }
        }

        impl<'a, S: ExtendValue<'a>> value_bag_serde1::lib::ser::SerializeSeq for Seq<S> {
            type Ok = S;

            type Error = Unsupported;

            fn serialize_element<T: value_bag_serde1::lib::Serialize + ?Sized>(
                &mut self,
                value: &T,
            ) -> Result<(), Self::Error> {
                self.0.extend(Internal::AnonSerde1(&value));
                Ok(())
            }

            fn end(self) -> Result<Self::Ok, Self::Error> {
                Ok(self.0)
            }
        }

        impl<'a, S: ExtendValue<'a>> value_bag_serde1::lib::ser::SerializeTuple for Seq<S> {
            type Ok = S;

            type Error = Unsupported;

            fn serialize_element<T: value_bag_serde1::lib::Serialize + ?Sized>(
                &mut self,
                value: &T,
            ) -> Result<(), Self::Error> {
                value_bag_serde1::lib::ser::SerializeSeq::serialize_element(self, value)
            }

            fn end(self) -> Result<Self::Ok, Self::Error> {
                value_bag_serde1::lib::ser::SerializeSeq::end(self)
            }
        }

        value_bag_serde1::lib::Serialize::serialize(v, Root::<S>(Default::default())).ok()
    }
}

#[cfg(feature = "owned")]
pub(crate) mod owned {
    use crate::std::boxed::Box;

    impl value_bag_serde1::lib::Serialize for crate::OwnedValueBag {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: value_bag_serde1::lib::Serializer,
        {
            value_bag_serde1::lib::Serialize::serialize(&self.by_ref(), s)
        }
    }

    pub(crate) type OwnedSerialize = Box<value_bag_serde1::buf::Owned>;

    pub(crate) fn buffer(
        v: impl value_bag_serde1::lib::Serialize,
    ) -> Result<OwnedSerialize, value_bag_serde1::buf::Error> {
        value_bag_serde1::buf::Owned::buffer(v).map(Box::new)
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
    fn serde1_capture() {
        assert_eq!(
            ValueBag::capture_serde1(&42u64).to_test_token(),
            TestToken::U64(42)
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn serde1_fill() {
        assert_eq!(
            ValueBag::from_fill(&|slot: Slot| slot.fill_serde1(42u64)).to_test_token(),
            TestToken::Serde { version: 1 },
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn serde1_capture_cast() {
        assert_eq!(
            42u64,
            ValueBag::capture_serde1(&42u64)
                .to_u64()
                .expect("invalid value")
        );

        assert_eq!(
            "a string",
            ValueBag::capture_serde1(&"a string")
                .to_borrowed_str()
                .expect("invalid value")
        );

        #[cfg(feature = "std")]
        assert_eq!(
            "a string",
            ValueBag::capture_serde1(&"a string")
                .to_str()
                .expect("invalid value")
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn serde1_from_cast() {
        assert_eq!(
            42u64,
            ValueBag::from_serde1(&42u64)
                .to_u64()
                .expect("invalid value")
        );

        #[cfg(feature = "std")]
        assert_eq!(
            "a string",
            ValueBag::from_serde1(&"a string")
                .to_str()
                .expect("invalid value")
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn serde1_downcast() {
        #[derive(Debug, PartialEq, Eq)]
        struct Timestamp(usize);

        impl value_bag_serde1::lib::Serialize for Timestamp {
            fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
            where
                S: value_bag_serde1::lib::Serializer,
            {
                s.serialize_u64(self.0 as u64)
            }
        }

        let ts = Timestamp(42);

        assert_eq!(
            &ts,
            ValueBag::capture_serde1(&ts)
                .downcast_ref::<Timestamp>()
                .expect("invalid value")
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn serde1_structured() {
        use value_bag_serde1::test::{assert_ser_tokens, Token};

        assert_ser_tokens(&ValueBag::from(42u64), &[Token::U64(42)]);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn serde1_debug() {
        struct TestSerde;

        impl value_bag_serde1::lib::Serialize for TestSerde {
            fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
            where
                S: value_bag_serde1::lib::Serializer,
            {
                s.serialize_u64(42)
            }
        }

        assert_eq!(
            format!("{:04?}", 42u64),
            format!("{:04?}", ValueBag::capture_serde1(&TestSerde)),
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn serde1_visit() {
        ValueBag::from_serde1(&42u64)
            .visit(TestVisit::default())
            .expect("failed to visit value");
        ValueBag::from_serde1(&-42i64)
            .visit(TestVisit::default())
            .expect("failed to visit value");
        ValueBag::from_serde1(&11f64)
            .visit(TestVisit::default())
            .expect("failed to visit value");
        ValueBag::from_serde1(&true)
            .visit(TestVisit::default())
            .expect("failed to visit value");
        ValueBag::from_serde1(&"some string")
            .visit(TestVisit::default())
            .expect("failed to visit value");
        ValueBag::from_serde1(&'n')
            .visit(TestVisit::default())
            .expect("failed to visit value");
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg(feature = "sval2")]
    fn serde1_sval2() {
        use value_bag_sval2::test::Token;

        struct TestSerde;

        impl value_bag_serde1::lib::Serialize for TestSerde {
            fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
            where
                S: value_bag_serde1::lib::Serializer,
            {
                s.serialize_u64(42)
            }
        }

        let value = ValueBag::from_serde1(&TestSerde);

        value_bag_sval2::test::assert_tokens(&value, &[Token::U64(42)]);
    }

    #[cfg(feature = "seq")]
    mod seq_support {
        use super::*;

        use crate::std::vec::Vec;

        #[test]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
        fn serde1_stream_str_seq() {
            use value_bag_serde1::test::{assert_ser_tokens, Token};

            assert_ser_tokens(
                &ValueBag::from_seq_slice(&["a", "b", "c"]),
                &[
                    Token::Seq { len: None },
                    Token::Str("a"),
                    Token::Str("b"),
                    Token::Str("c"),
                    Token::SeqEnd,
                ],
            );
        }

        #[test]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
        fn serde1_to_seq() {
            assert_eq!(
                vec![Some(1.0), None, Some(2.0), Some(3.0), None],
                ValueBag::capture_serde1(&[
                    &1.0 as &dyn Serialize,
                    &true as &dyn Serialize,
                    &2.0 as &dyn Serialize,
                    &3.0 as &dyn Serialize,
                    &"a string" as &dyn Serialize,
                ])
                .to_f64_seq::<Vec<Option<f64>>>()
                .expect("invalid value")
            );
        }

        #[test]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
        fn serde1_as_seq() {
            assert_eq!(
                vec![1.0, 2.0, 3.0],
                ValueBag::capture_serde1(&[1.0, 2.0, 3.0,]).as_f64_seq::<Vec<f64>>()
            );
        }
    }

    #[cfg(feature = "std")]
    mod std_support {
        use super::*;

        use crate::std::borrow::ToOwned;

        #[test]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
        fn serde1_cast() {
            assert_eq!(
                "a string",
                ValueBag::capture_serde1(&"a string".to_owned())
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
        fn serde1_to_owned_poison() {
            struct Kaboom;

            impl value_bag_serde1::lib::Serialize for Kaboom {
                fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
                where
                    S: value_bag_serde1::lib::Serializer,
                {
                    Err(S::Error::custom("kaboom"))
                }
            }

            let value = ValueBag::capture_serde1(&Kaboom)
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
