use self::ErrorImpl::ShortCircuit;
use crate::error::Error;
use crate::sealed;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::fmt::{self, Debug, Display};
use serde::ser::{
    SerializeMap as _, SerializeSeq as _, SerializeStruct as _, SerializeStructVariant as _,
    SerializeTuple as _, SerializeTupleStruct as _, SerializeTupleVariant as _,
};

// TRAITS //////////////////////////////////////////////////////////////////////

/// An object-safe equivalent of Serde's `Serialize` trait.
///
/// Any implementation of Serde's `Serialize` converts seamlessly to a
/// `&dyn erased_serde::Serialize` or `Box<dyn erased_serde::Serialize>` trait
/// object.
///
/// ```rust
/// use erased_serde::{Serialize, Serializer};
/// use std::collections::BTreeMap as Map;
/// use std::io;
///
/// fn main() {
///     // Construct some serializers.
///     let json = &mut serde_json::Serializer::new(io::stdout());
///     let cbor = &mut serde_cbor::Serializer::new(serde_cbor::ser::IoWrite::new(io::stdout()));
///
///     // The values in this map are boxed trait objects. Ordinarily this would not
///     // be possible with serde::Serializer because of object safety, but type
///     // erasure makes it possible with erased_serde::Serializer.
///     let mut formats: Map<&str, Box<dyn Serializer>> = Map::new();
///     formats.insert("json", Box::new(<dyn Serializer>::erase(json)));
///     formats.insert("cbor", Box::new(<dyn Serializer>::erase(cbor)));
///
///     // These are boxed trait objects as well. Same thing here - type erasure
///     // makes this possible.
///     let mut values: Map<&str, Box<dyn Serialize>> = Map::new();
///     values.insert("vec", Box::new(vec!["a", "b"]));
///     values.insert("int", Box::new(65536));
///
///     // Pick a Serializer out of the formats map.
///     let format = formats.get_mut("json").unwrap();
///
///     // Pick a Serialize out of the values map.
///     let value = values.get("vec").unwrap();
///
///     // This line prints `["a","b"]` to stdout.
///     value.erased_serialize(format).unwrap();
/// }
/// ```
///
/// This trait is sealed and can only be implemented via a `serde::Serialize`
/// impl.
#[cfg_attr(
    not(no_diagnostic_namespace),
    diagnostic::on_unimplemented(
        message = "the trait bound `{Self}: serde::Serialize` is not satisfied",
        label = "the trait `serde::Serialize` is not implemented for `{Self}`, so it does not implement `erased_serde::Serialize`",
    )
)]
pub trait Serialize: sealed::serialize::Sealed {
    fn erased_serialize(&self, serializer: &mut dyn Serializer) -> Result<(), Error>;

    #[doc(hidden)]
    fn do_erased_serialize(&self, serializer: &mut dyn Serializer) -> Result<(), ErrorImpl>;
}

/// An object-safe equivalent of Serde's `Serializer` trait.
///
/// Any implementation of Serde's `Serializer` can be converted to a
/// `&dyn erased_serde::Serializer` or `Box<dyn erased_serde::Serializer>` trait
/// object using `erased_serde::Serializer::erase`.
///
/// ```rust
/// use erased_serde::{Serialize, Serializer};
/// use std::collections::BTreeMap as Map;
/// use std::io;
///
/// fn main() {
///     // Construct some serializers.
///     let json = &mut serde_json::Serializer::new(io::stdout());
///     let cbor = &mut serde_cbor::Serializer::new(serde_cbor::ser::IoWrite::new(io::stdout()));
///
///     // The values in this map are boxed trait objects. Ordinarily this would not
///     // be possible with serde::Serializer because of object safety, but type
///     // erasure makes it possible with erased_serde::Serializer.
///     let mut formats: Map<&str, Box<dyn Serializer>> = Map::new();
///     formats.insert("json", Box::new(<dyn Serializer>::erase(json)));
///     formats.insert("cbor", Box::new(<dyn Serializer>::erase(cbor)));
///
///     // These are boxed trait objects as well. Same thing here - type erasure
///     // makes this possible.
///     let mut values: Map<&str, Box<dyn Serialize>> = Map::new();
///     values.insert("vec", Box::new(vec!["a", "b"]));
///     values.insert("int", Box::new(65536));
///
///     // Pick a Serializer out of the formats map.
///     let format = formats.get_mut("json").unwrap();
///
///     // Pick a Serialize out of the values map.
///     let value = values.get("vec").unwrap();
///
///     // This line prints `["a","b"]` to stdout.
///     value.erased_serialize(format).unwrap();
/// }
/// ```
///
/// This trait is sealed and can only be implemented via a `serde::Serializer`
/// impl.
pub trait Serializer: sealed::serializer::Sealed {
    fn erased_serialize_bool(&mut self, v: bool);
    fn erased_serialize_i8(&mut self, v: i8);
    fn erased_serialize_i16(&mut self, v: i16);
    fn erased_serialize_i32(&mut self, v: i32);
    fn erased_serialize_i64(&mut self, v: i64);
    fn erased_serialize_i128(&mut self, v: i128);
    fn erased_serialize_u8(&mut self, v: u8);
    fn erased_serialize_u16(&mut self, v: u16);
    fn erased_serialize_u32(&mut self, v: u32);
    fn erased_serialize_u64(&mut self, v: u64);
    fn erased_serialize_u128(&mut self, v: u128);
    fn erased_serialize_f32(&mut self, v: f32);
    fn erased_serialize_f64(&mut self, v: f64);
    fn erased_serialize_char(&mut self, v: char);
    fn erased_serialize_str(&mut self, v: &str);
    fn erased_serialize_bytes(&mut self, v: &[u8]);
    fn erased_serialize_none(&mut self);
    fn erased_serialize_some(&mut self, value: &dyn Serialize);
    fn erased_serialize_unit(&mut self);
    fn erased_serialize_unit_struct(&mut self, name: &'static str);
    fn erased_serialize_unit_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    );
    fn erased_serialize_newtype_struct(&mut self, name: &'static str, value: &dyn Serialize);
    fn erased_serialize_newtype_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &dyn Serialize,
    );
    fn erased_serialize_seq(
        &mut self,
        len: Option<usize>,
    ) -> Result<&mut dyn SerializeSeq, ErrorImpl>;
    fn erased_serialize_tuple(&mut self, len: usize) -> Result<&mut dyn SerializeTuple, ErrorImpl>;
    fn erased_serialize_tuple_struct(
        &mut self,
        name: &'static str,
        len: usize,
    ) -> Result<&mut dyn SerializeTupleStruct, ErrorImpl>;
    fn erased_serialize_tuple_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<&mut dyn SerializeTupleVariant, ErrorImpl>;
    fn erased_serialize_map(
        &mut self,
        len: Option<usize>,
    ) -> Result<&mut dyn SerializeMap, ErrorImpl>;
    fn erased_serialize_struct(
        &mut self,
        name: &'static str,
        len: usize,
    ) -> Result<&mut dyn SerializeStruct, ErrorImpl>;
    fn erased_serialize_struct_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<&mut dyn SerializeStructVariant, ErrorImpl>;
    fn erased_is_human_readable(&self) -> bool;
    #[doc(hidden)]
    fn erased_display_error(&self) -> &dyn Display;
}

impl dyn Serializer {
    return_impl_trait! {
        /// Convert any Serde `Serializer` to a trait object.
        ///
        /// ```rust
        /// use erased_serde::{Serialize, Serializer};
        /// use std::collections::BTreeMap as Map;
        /// use std::io;
        ///
        /// fn main() {
        ///     // Construct some serializers.
        ///     let json = &mut serde_json::Serializer::new(io::stdout());
        ///     let cbor = &mut serde_cbor::Serializer::new(serde_cbor::ser::IoWrite::new(io::stdout()));
        ///
        ///     // The values in this map are boxed trait objects. Ordinarily this would not
        ///     // be possible with serde::Serializer because of object safety, but type
        ///     // erasure makes it possible with erased_serde::Serializer.
        ///     let mut formats: Map<&str, Box<dyn Serializer>> = Map::new();
        ///     formats.insert("json", Box::new(<dyn Serializer>::erase(json)));
        ///     formats.insert("cbor", Box::new(<dyn Serializer>::erase(cbor)));
        ///
        ///     // These are boxed trait objects as well. Same thing here - type erasure
        ///     // makes this possible.
        ///     let mut values: Map<&str, Box<dyn Serialize>> = Map::new();
        ///     values.insert("vec", Box::new(vec!["a", "b"]));
        ///     values.insert("int", Box::new(65536));
        ///
        ///     // Pick a Serializer out of the formats map.
        ///     let format = formats.get_mut("json").unwrap();
        ///
        ///     // Pick a Serialize out of the values map.
        ///     let value = values.get("vec").unwrap();
        ///
        ///     // This line prints `["a","b"]` to stdout.
        ///     value.erased_serialize(format).unwrap();
        /// }
        /// ```
        pub fn erase<S>(serializer: S) -> impl Serializer [erase::Serializer<S>]
        where
            S: serde::Serializer,
        {
            erase::Serializer::new(serializer)
        }
    }
}

// IMPL ERASED SERDE FOR SERDE /////////////////////////////////////////////////

impl<T> Serialize for T
where
    T: ?Sized + serde::Serialize,
{
    fn erased_serialize(&self, serializer: &mut dyn Serializer) -> Result<(), Error> {
        match self.do_erased_serialize(serializer) {
            Ok(()) => Ok(()),
            Err(ShortCircuit) => Err(serde::ser::Error::custom(serializer.erased_display_error())),
            Err(ErrorImpl::Custom(msg)) => Err(serde::ser::Error::custom(msg)),
        }
    }

    fn do_erased_serialize(&self, serializer: &mut dyn Serializer) -> Result<(), ErrorImpl> {
        self.serialize(MakeSerializer(serializer))
    }
}

impl<T> sealed::serialize::Sealed for T where T: ?Sized + serde::Serialize {}

mod erase {
    use core::mem;

    pub enum Serializer<S>
    where
        S: serde::Serializer,
    {
        Ready(S),
        Seq(S::SerializeSeq),
        Tuple(S::SerializeTuple),
        TupleStruct(S::SerializeTupleStruct),
        TupleVariant(S::SerializeTupleVariant),
        Map(S::SerializeMap),
        Struct(S::SerializeStruct),
        StructVariant(S::SerializeStructVariant),
        Error(S::Error),
        Complete(S::Ok),
        Unusable,
    }

    impl<S> Serializer<S>
    where
        S: serde::Serializer,
    {
        pub(crate) fn new(serializer: S) -> Self {
            Serializer::Ready(serializer)
        }

        pub(crate) fn take(&mut self) -> Self {
            mem::replace(self, Serializer::Unusable)
        }

        pub(crate) fn take_serializer(&mut self) -> S {
            match self.take() {
                Serializer::Ready(serializer) => serializer,
                _ => unreachable!(),
            }
        }
    }
}

impl<T> Serializer for erase::Serializer<T>
where
    T: serde::Serializer,
{
    fn erased_serialize_bool(&mut self, v: bool) {
        *self = match self.take_serializer().serialize_bool(v) {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }

    fn erased_serialize_i8(&mut self, v: i8) {
        *self = match self.take_serializer().serialize_i8(v) {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }

    fn erased_serialize_i16(&mut self, v: i16) {
        *self = match self.take_serializer().serialize_i16(v) {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }

    fn erased_serialize_i32(&mut self, v: i32) {
        *self = match self.take_serializer().serialize_i32(v) {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }

    fn erased_serialize_i64(&mut self, v: i64) {
        *self = match self.take_serializer().serialize_i64(v) {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }

    fn erased_serialize_i128(&mut self, v: i128) {
        *self = match self.take_serializer().serialize_i128(v) {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }

    fn erased_serialize_u8(&mut self, v: u8) {
        *self = match self.take_serializer().serialize_u8(v) {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }

    fn erased_serialize_u16(&mut self, v: u16) {
        *self = match self.take_serializer().serialize_u16(v) {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }

    fn erased_serialize_u32(&mut self, v: u32) {
        *self = match self.take_serializer().serialize_u32(v) {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }

    fn erased_serialize_u64(&mut self, v: u64) {
        *self = match self.take_serializer().serialize_u64(v) {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }

    fn erased_serialize_u128(&mut self, v: u128) {
        *self = match self.take_serializer().serialize_u128(v) {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }

    fn erased_serialize_f32(&mut self, v: f32) {
        *self = match self.take_serializer().serialize_f32(v) {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }

    fn erased_serialize_f64(&mut self, v: f64) {
        *self = match self.take_serializer().serialize_f64(v) {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }

    fn erased_serialize_char(&mut self, v: char) {
        *self = match self.take_serializer().serialize_char(v) {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }

    fn erased_serialize_str(&mut self, v: &str) {
        *self = match self.take_serializer().serialize_str(v) {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }

    fn erased_serialize_bytes(&mut self, v: &[u8]) {
        *self = match self.take_serializer().serialize_bytes(v) {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }

    fn erased_serialize_none(&mut self) {
        *self = match self.take_serializer().serialize_none() {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }

    fn erased_serialize_some(&mut self, value: &dyn Serialize) {
        *self = match self.take_serializer().serialize_some(value) {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }

    fn erased_serialize_unit(&mut self) {
        *self = match self.take_serializer().serialize_unit() {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }

    fn erased_serialize_unit_struct(&mut self, name: &'static str) {
        *self = match self.take_serializer().serialize_unit_struct(name) {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }

    fn erased_serialize_unit_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) {
        *self = match self
            .take_serializer()
            .serialize_unit_variant(name, variant_index, variant)
        {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }

    fn erased_serialize_newtype_struct(&mut self, name: &'static str, value: &dyn Serialize) {
        *self = match self.take_serializer().serialize_newtype_struct(name, value) {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }

    fn erased_serialize_newtype_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &dyn Serialize,
    ) {
        *self = match self.take_serializer().serialize_newtype_variant(
            name,
            variant_index,
            variant,
            value,
        ) {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }

    fn erased_serialize_seq(
        &mut self,
        len: Option<usize>,
    ) -> Result<&mut dyn SerializeSeq, ErrorImpl> {
        match self.take_serializer().serialize_seq(len) {
            Ok(ok) => {
                *self = erase::Serializer::Seq(ok);
                Ok(self)
            }
            Err(err) => {
                *self = erase::Serializer::Error(err);
                Err(ShortCircuit)
            }
        }
    }

    fn erased_serialize_tuple(&mut self, len: usize) -> Result<&mut dyn SerializeTuple, ErrorImpl> {
        match self.take_serializer().serialize_tuple(len) {
            Ok(ok) => {
                *self = erase::Serializer::Tuple(ok);
                Ok(self)
            }
            Err(err) => {
                *self = erase::Serializer::Error(err);
                Err(ShortCircuit)
            }
        }
    }

    fn erased_serialize_tuple_struct(
        &mut self,
        name: &'static str,
        len: usize,
    ) -> Result<&mut dyn SerializeTupleStruct, ErrorImpl> {
        match self.take_serializer().serialize_tuple_struct(name, len) {
            Ok(ok) => {
                *self = erase::Serializer::TupleStruct(ok);
                Ok(self)
            }
            Err(err) => {
                *self = erase::Serializer::Error(err);
                Err(ShortCircuit)
            }
        }
    }

    fn erased_serialize_tuple_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<&mut dyn SerializeTupleVariant, ErrorImpl> {
        match self
            .take_serializer()
            .serialize_tuple_variant(name, variant_index, variant, len)
        {
            Ok(ok) => {
                *self = erase::Serializer::TupleVariant(ok);
                Ok(self)
            }
            Err(err) => {
                *self = erase::Serializer::Error(err);
                Err(ShortCircuit)
            }
        }
    }

    fn erased_serialize_map(
        &mut self,
        len: Option<usize>,
    ) -> Result<&mut dyn SerializeMap, ErrorImpl> {
        match self.take_serializer().serialize_map(len) {
            Ok(ok) => {
                *self = erase::Serializer::Map(ok);
                Ok(self)
            }
            Err(err) => {
                *self = erase::Serializer::Error(err);
                Err(ShortCircuit)
            }
        }
    }

    fn erased_serialize_struct(
        &mut self,
        name: &'static str,
        len: usize,
    ) -> Result<&mut dyn SerializeStruct, ErrorImpl> {
        match self.take_serializer().serialize_struct(name, len) {
            Ok(ok) => {
                *self = erase::Serializer::Struct(ok);
                Ok(self)
            }
            Err(err) => {
                *self = erase::Serializer::Error(err);
                Err(ShortCircuit)
            }
        }
    }

    fn erased_serialize_struct_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<&mut dyn SerializeStructVariant, ErrorImpl> {
        match self
            .take_serializer()
            .serialize_struct_variant(name, variant_index, variant, len)
        {
            Ok(ok) => {
                *self = erase::Serializer::StructVariant(ok);
                Ok(self)
            }
            Err(err) => {
                *self = erase::Serializer::Error(err);
                Err(ShortCircuit)
            }
        }
    }

    fn erased_is_human_readable(&self) -> bool {
        match self {
            erase::Serializer::Ready(serializer) => serializer.is_human_readable(),
            _ => unreachable!(),
        }
    }

    fn erased_display_error(&self) -> &dyn Display {
        match self {
            erase::Serializer::Error(err) => err,
            _ => unreachable!(),
        }
    }
}

impl<T> sealed::serializer::Sealed for erase::Serializer<T> where T: serde::Serializer {}

pub enum ErrorImpl {
    ShortCircuit,
    Custom(Box<String>),
}

impl Display for ErrorImpl {
    fn fmt(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl Debug for ErrorImpl {
    fn fmt(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl serde::ser::StdError for ErrorImpl {}

impl serde::ser::Error for ErrorImpl {
    fn custom<T: Display>(msg: T) -> Self {
        ErrorImpl::Custom(Box::new(msg.to_string()))
    }
}

// IMPL SERDE FOR ERASED SERDE /////////////////////////////////////////////////

/// Serialize the given type-erased serializable value.
///
/// This can be used to implement `serde::Serialize` for trait objects that have
/// `erased_serde::Serialize` as a supertrait.
///
/// ```
/// trait Event: erased_serde::Serialize {
///     /* ... */
/// }
///
/// impl<'a> serde::Serialize for dyn Event + 'a {
///     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
///     where
///         S: serde::Serializer,
///     {
///         erased_serde::serialize(self, serializer)
///     }
/// }
/// ```
///
/// Since this is reasonably common, the `serialize_trait_object!` macro
/// generates such a Serialize impl.
///
/// ```
/// use erased_serde::serialize_trait_object;
/// #
/// # trait Event: erased_serde::Serialize {}
///
/// serialize_trait_object!(Event);
/// ```
pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ?Sized + Serialize,
    S: serde::Serializer,
{
    let mut erased = erase::Serializer::new(serializer);
    match value.do_erased_serialize(&mut erased) {
        Ok(()) | Err(ShortCircuit) => {}
        Err(ErrorImpl::Custom(msg)) => return Err(serde::ser::Error::custom(msg)),
    }
    match erased {
        erase::Serializer::Complete(ok) => Ok(ok),
        erase::Serializer::Error(err) => Err(err),
        _ => unreachable!(),
    }
}

serialize_trait_object!(Serialize);

struct MakeSerializer<TraitObject>(TraitObject);

impl<'a> serde::Serializer for MakeSerializer<&'a mut (dyn Serializer + '_)> {
    type Ok = ();
    type Error = ErrorImpl;
    type SerializeSeq = MakeSerializer<&'a mut dyn SerializeSeq>;
    type SerializeTuple = MakeSerializer<&'a mut dyn SerializeTuple>;
    type SerializeTupleStruct = MakeSerializer<&'a mut dyn SerializeTupleStruct>;
    type SerializeTupleVariant = MakeSerializer<&'a mut dyn SerializeTupleVariant>;
    type SerializeMap = MakeSerializer<&'a mut dyn SerializeMap>;
    type SerializeStruct = MakeSerializer<&'a mut dyn SerializeStruct>;
    type SerializeStructVariant = MakeSerializer<&'a mut dyn SerializeStructVariant>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.0.erased_serialize_bool(v);
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.0.erased_serialize_i8(v);
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.0.erased_serialize_i16(v);
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.0.erased_serialize_i32(v);
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.0.erased_serialize_i64(v);
        Ok(())
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.0.erased_serialize_i128(v);
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.0.erased_serialize_u8(v);
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.0.erased_serialize_u16(v);
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.0.erased_serialize_u32(v);
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.0.erased_serialize_u64(v);
        Ok(())
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.0.erased_serialize_u128(v);
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.0.erased_serialize_f32(v);
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.0.erased_serialize_f64(v);
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.0.erased_serialize_char(v);
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.0.erased_serialize_str(v);
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.0.erased_serialize_bytes(v);
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.0.erased_serialize_none();
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.0.erased_serialize_some(&value);
        Ok(())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.0.erased_serialize_unit();
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.0.erased_serialize_unit_struct(name);
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.0
            .erased_serialize_unit_variant(name, variant_index, variant);
        Ok(())
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.0.erased_serialize_newtype_struct(name, &value);
        Ok(())
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.0
            .erased_serialize_newtype_variant(name, variant_index, variant, &value);
        Ok(())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.0.erased_serialize_seq(len).map(MakeSerializer)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.0.erased_serialize_tuple(len).map(MakeSerializer)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.0
            .erased_serialize_tuple_struct(name, len)
            .map(MakeSerializer)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.0
            .erased_serialize_tuple_variant(name, variant_index, variant, len)
            .map(MakeSerializer)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.0.erased_serialize_map(len).map(MakeSerializer)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.0
            .erased_serialize_struct(name, len)
            .map(MakeSerializer)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.0
            .erased_serialize_struct_variant(name, variant_index, variant, len)
            .map(MakeSerializer)
    }

    #[cfg(not(feature = "alloc"))]
    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Display,
    {
        unreachable!()
    }

    fn is_human_readable(&self) -> bool {
        self.0.erased_is_human_readable()
    }
}

pub trait SerializeSeq {
    fn erased_serialize_element(&mut self, value: &dyn Serialize) -> Result<(), ErrorImpl>;
    fn erased_end(&mut self);
}

impl<T> SerializeSeq for erase::Serializer<T>
where
    T: serde::Serializer,
{
    fn erased_serialize_element(&mut self, value: &dyn Serialize) -> Result<(), ErrorImpl> {
        let erase::Serializer::Seq(serializer) = self else {
            unreachable!();
        };
        serializer.serialize_element(value).map_err(|err| {
            *self = erase::Serializer::Error(err);
            ShortCircuit
        })
    }

    fn erased_end(&mut self) {
        let erase::Serializer::Seq(serializer) = self.take() else {
            unreachable!();
        };
        *self = match serializer.end() {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }
}

impl serde::ser::SerializeSeq for MakeSerializer<&mut dyn SerializeSeq> {
    type Ok = ();
    type Error = ErrorImpl;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.0.erased_serialize_element(&value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.0.erased_end();
        Ok(())
    }
}

pub trait SerializeTuple {
    fn erased_serialize_element(&mut self, value: &dyn Serialize) -> Result<(), ErrorImpl>;
    fn erased_end(&mut self);
}

impl<T> SerializeTuple for erase::Serializer<T>
where
    T: serde::Serializer,
{
    fn erased_serialize_element(&mut self, value: &dyn Serialize) -> Result<(), ErrorImpl> {
        let erase::Serializer::Tuple(serializer) = self else {
            unreachable!();
        };
        serializer.serialize_element(value).map_err(|err| {
            *self = erase::Serializer::Error(err);
            ShortCircuit
        })
    }

    fn erased_end(&mut self) {
        let erase::Serializer::Tuple(serializer) = self.take() else {
            unreachable!();
        };
        *self = match serializer.end() {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }
}

impl serde::ser::SerializeTuple for MakeSerializer<&mut dyn SerializeTuple> {
    type Ok = ();
    type Error = ErrorImpl;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.0.erased_serialize_element(&value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.0.erased_end();
        Ok(())
    }
}

pub trait SerializeTupleStruct {
    fn erased_serialize_field(&mut self, value: &dyn Serialize) -> Result<(), ErrorImpl>;
    fn erased_end(&mut self);
}

impl<T> SerializeTupleStruct for erase::Serializer<T>
where
    T: serde::Serializer,
{
    fn erased_serialize_field(&mut self, value: &dyn Serialize) -> Result<(), ErrorImpl> {
        let erase::Serializer::TupleStruct(serializer) = self else {
            unreachable!();
        };
        serializer.serialize_field(value).map_err(|err| {
            *self = erase::Serializer::Error(err);
            ShortCircuit
        })
    }

    fn erased_end(&mut self) {
        let erase::Serializer::TupleStruct(serializer) = self.take() else {
            unreachable!();
        };
        *self = match serializer.end() {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }
}

impl serde::ser::SerializeTupleStruct for MakeSerializer<&mut dyn SerializeTupleStruct> {
    type Ok = ();
    type Error = ErrorImpl;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.0.erased_serialize_field(&value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.0.erased_end();
        Ok(())
    }
}

pub trait SerializeTupleVariant {
    fn erased_serialize_field(&mut self, value: &dyn Serialize) -> Result<(), ErrorImpl>;
    fn erased_end(&mut self);
}

impl<T> SerializeTupleVariant for erase::Serializer<T>
where
    T: serde::Serializer,
{
    fn erased_serialize_field(&mut self, value: &dyn Serialize) -> Result<(), ErrorImpl> {
        let erase::Serializer::TupleVariant(serializer) = self else {
            unreachable!();
        };
        serializer.serialize_field(value).map_err(|err| {
            *self = erase::Serializer::Error(err);
            ShortCircuit
        })
    }

    fn erased_end(&mut self) {
        let erase::Serializer::TupleVariant(serializer) = self.take() else {
            unreachable!();
        };
        *self = match serializer.end() {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }
}

impl serde::ser::SerializeTupleVariant for MakeSerializer<&mut dyn SerializeTupleVariant> {
    type Ok = ();
    type Error = ErrorImpl;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.0.erased_serialize_field(&value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.0.erased_end();
        Ok(())
    }
}

pub trait SerializeMap {
    fn erased_serialize_key(&mut self, key: &dyn Serialize) -> Result<(), ErrorImpl>;
    fn erased_serialize_value(&mut self, value: &dyn Serialize) -> Result<(), ErrorImpl>;
    fn erased_serialize_entry(
        &mut self,
        key: &dyn Serialize,
        value: &dyn Serialize,
    ) -> Result<(), ErrorImpl>;
    fn erased_end(&mut self);
}

impl<T> SerializeMap for erase::Serializer<T>
where
    T: serde::Serializer,
{
    fn erased_serialize_key(&mut self, key: &dyn Serialize) -> Result<(), ErrorImpl> {
        let erase::Serializer::Map(serializer) = self else {
            unreachable!();
        };
        serializer.serialize_key(key).map_err(|err| {
            *self = erase::Serializer::Error(err);
            ShortCircuit
        })
    }

    fn erased_serialize_value(&mut self, value: &dyn Serialize) -> Result<(), ErrorImpl> {
        let erase::Serializer::Map(serializer) = self else {
            unreachable!();
        };
        serializer.serialize_value(value).map_err(|err| {
            *self = erase::Serializer::Error(err);
            ShortCircuit
        })
    }

    fn erased_serialize_entry(
        &mut self,
        key: &dyn Serialize,
        value: &dyn Serialize,
    ) -> Result<(), ErrorImpl> {
        let erase::Serializer::Map(serializer) = self else {
            unreachable!();
        };
        serializer.serialize_entry(key, value).map_err(|err| {
            *self = erase::Serializer::Error(err);
            ShortCircuit
        })
    }

    fn erased_end(&mut self) {
        let erase::Serializer::Map(serializer) = self.take() else {
            unreachable!();
        };
        *self = match serializer.end() {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }
}

impl serde::ser::SerializeMap for MakeSerializer<&mut dyn SerializeMap> {
    type Ok = ();
    type Error = ErrorImpl;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.0.erased_serialize_key(&key)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.0.erased_serialize_value(&value)
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: ?Sized + serde::Serialize,
        V: ?Sized + serde::Serialize,
    {
        self.0.erased_serialize_entry(&key, &value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.0.erased_end();
        Ok(())
    }
}

pub trait SerializeStruct {
    fn erased_serialize_field(
        &mut self,
        key: &'static str,
        value: &dyn Serialize,
    ) -> Result<(), ErrorImpl>;
    fn erased_skip_field(&mut self, key: &'static str) -> Result<(), ErrorImpl>;
    fn erased_end(&mut self);
}

impl<T> SerializeStruct for erase::Serializer<T>
where
    T: serde::Serializer,
{
    fn erased_serialize_field(
        &mut self,
        key: &'static str,
        value: &dyn Serialize,
    ) -> Result<(), ErrorImpl> {
        let erase::Serializer::Struct(serializer) = self else {
            unreachable!();
        };
        serializer.serialize_field(key, value).map_err(|err| {
            *self = erase::Serializer::Error(err);
            ShortCircuit
        })
    }

    fn erased_skip_field(&mut self, key: &'static str) -> Result<(), ErrorImpl> {
        let erase::Serializer::Struct(serializer) = self else {
            unreachable!();
        };
        serializer.skip_field(key).map_err(|err| {
            *self = erase::Serializer::Error(err);
            ShortCircuit
        })
    }

    fn erased_end(&mut self) {
        let erase::Serializer::Struct(serializer) = self.take() else {
            unreachable!();
        };
        *self = match serializer.end() {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }
}

impl serde::ser::SerializeStruct for MakeSerializer<&mut dyn SerializeStruct> {
    type Ok = ();
    type Error = ErrorImpl;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.0.erased_serialize_field(key, &value)
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.0.erased_skip_field(key)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.0.erased_end();
        Ok(())
    }
}

pub trait SerializeStructVariant {
    fn erased_serialize_field(
        &mut self,
        key: &'static str,
        value: &dyn Serialize,
    ) -> Result<(), ErrorImpl>;
    fn erased_skip_field(&mut self, key: &'static str) -> Result<(), ErrorImpl>;
    fn erased_end(&mut self);
}

impl<T> SerializeStructVariant for erase::Serializer<T>
where
    T: serde::Serializer,
{
    fn erased_serialize_field(
        &mut self,
        key: &'static str,
        value: &dyn Serialize,
    ) -> Result<(), ErrorImpl> {
        let erase::Serializer::StructVariant(serializer) = self else {
            unreachable!();
        };
        serializer.serialize_field(key, value).map_err(|err| {
            *self = erase::Serializer::Error(err);
            ShortCircuit
        })
    }

    fn erased_skip_field(&mut self, key: &'static str) -> Result<(), ErrorImpl> {
        let erase::Serializer::Struct(serializer) = self else {
            unreachable!();
        };
        serializer.skip_field(key).map_err(|err| {
            *self = erase::Serializer::Error(err);
            ShortCircuit
        })
    }

    fn erased_end(&mut self) {
        let erase::Serializer::StructVariant(serializer) = self.take() else {
            unreachable!();
        };
        *self = match serializer.end() {
            Ok(ok) => erase::Serializer::Complete(ok),
            Err(err) => erase::Serializer::Error(err),
        };
    }
}

impl serde::ser::SerializeStructVariant for MakeSerializer<&mut dyn SerializeStructVariant> {
    type Ok = ();
    type Error = ErrorImpl;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.0.erased_serialize_field(key, &value)
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.0.erased_skip_field(key)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.0.erased_end();
        Ok(())
    }
}

// IMPL ERASED SERDE FOR ERASED SERDE //////////////////////////////////////////

macro_rules! deref_erased_serializer {
    (<$T:ident> Serializer for $ty:ty $(where $($where:tt)*)?) => {
        impl<$T> Serializer for $ty $(where $($where)*)? {
            fn erased_serialize_bool(&mut self, v: bool) {
                (**self).erased_serialize_bool(v);
            }

            fn erased_serialize_i8(&mut self, v: i8) {
                (**self).erased_serialize_i8(v);
            }

            fn erased_serialize_i16(&mut self, v: i16) {
                (**self).erased_serialize_i16(v);
            }

            fn erased_serialize_i32(&mut self, v: i32) {
                (**self).erased_serialize_i32(v);
            }

            fn erased_serialize_i64(&mut self, v: i64) {
                (**self).erased_serialize_i64(v);
            }

            fn erased_serialize_i128(&mut self, v: i128) {
                (**self).erased_serialize_i128(v);
            }

            fn erased_serialize_u8(&mut self, v: u8) {
                (**self).erased_serialize_u8(v);
            }

            fn erased_serialize_u16(&mut self, v: u16) {
                (**self).erased_serialize_u16(v);
            }

            fn erased_serialize_u32(&mut self, v: u32) {
                (**self).erased_serialize_u32(v);
            }

            fn erased_serialize_u64(&mut self, v: u64) {
                (**self).erased_serialize_u64(v);
            }

            fn erased_serialize_u128(&mut self, v: u128) {
                (**self).erased_serialize_u128(v);
            }

            fn erased_serialize_f32(&mut self, v: f32) {
                (**self).erased_serialize_f32(v);
            }

            fn erased_serialize_f64(&mut self, v: f64) {
                (**self).erased_serialize_f64(v);
            }

            fn erased_serialize_char(&mut self, v: char) {
                (**self).erased_serialize_char(v);
            }

            fn erased_serialize_str(&mut self, v: &str) {
                (**self).erased_serialize_str(v);
            }

            fn erased_serialize_bytes(&mut self, v: &[u8]) {
                (**self).erased_serialize_bytes(v);
            }

            fn erased_serialize_none(&mut self) {
                (**self).erased_serialize_none();
            }

            fn erased_serialize_some(&mut self, value: &dyn Serialize) {
                (**self).erased_serialize_some(value);
            }

            fn erased_serialize_unit(&mut self) {
                (**self).erased_serialize_unit();
            }

            fn erased_serialize_unit_struct(&mut self, name: &'static str) {
                (**self).erased_serialize_unit_struct(name);
            }

            fn erased_serialize_unit_variant(&mut self, name: &'static str, variant_index: u32, variant: &'static str) {
                (**self).erased_serialize_unit_variant(name, variant_index, variant);
            }

            fn erased_serialize_newtype_struct(&mut self, name: &'static str, value: &dyn Serialize) {
                (**self).erased_serialize_newtype_struct(name, value);
            }

            fn erased_serialize_newtype_variant(&mut self, name: &'static str, variant_index: u32, variant: &'static str, value: &dyn Serialize) {
                (**self).erased_serialize_newtype_variant(name, variant_index, variant, value);
            }

            fn erased_serialize_seq(&mut self, len: Option<usize>) -> Result<&mut dyn SerializeSeq, ErrorImpl> {
                (**self).erased_serialize_seq(len)
            }

            fn erased_serialize_tuple(&mut self, len: usize) -> Result<&mut dyn SerializeTuple, ErrorImpl> {
                (**self).erased_serialize_tuple(len)
            }

            fn erased_serialize_tuple_struct(&mut self, name: &'static str, len: usize) -> Result<&mut dyn SerializeTupleStruct, ErrorImpl> {
                (**self).erased_serialize_tuple_struct(name, len)
            }

            fn erased_serialize_tuple_variant(&mut self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<&mut dyn SerializeTupleVariant, ErrorImpl> {
                (**self).erased_serialize_tuple_variant(name, variant_index, variant, len)
            }

            fn erased_serialize_map(&mut self, len: Option<usize>) -> Result<&mut dyn SerializeMap, ErrorImpl> {
                (**self).erased_serialize_map(len)
            }

            fn erased_serialize_struct(&mut self, name: &'static str, len: usize) -> Result<&mut dyn SerializeStruct, ErrorImpl> {
                (**self).erased_serialize_struct(name, len)
            }

            fn erased_serialize_struct_variant(&mut self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<&mut dyn SerializeStructVariant, ErrorImpl> {
                (**self).erased_serialize_struct_variant(name, variant_index, variant, len)
            }

            fn erased_is_human_readable(&self) -> bool {
                (**self).erased_is_human_readable()
            }

            fn erased_display_error(&self) -> &dyn Display {
                (**self).erased_display_error()
            }
        }

        impl<$T> sealed::serializer::Sealed for $ty $(where $($where)*)? {}
    };
}

deref_erased_serializer!(<T> Serializer for &mut T where T: ?Sized + Serializer);
deref_erased_serializer!(<T> Serializer for Box<T> where T: ?Sized + Serializer);

// TEST ////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{vec, vec::Vec};
    use serde_derive::Serialize;

    fn test_json<T>(t: T)
    where
        T: serde::Serialize,
    {
        let expected = serde_json::to_vec(&t).unwrap();

        // test borrowed trait object
        {
            let obj: &dyn Serialize = &t;

            let mut buf = Vec::new();

            {
                let mut ser = serde_json::Serializer::new(&mut buf);
                let ser: &mut dyn Serializer = &mut <dyn Serializer>::erase(&mut ser);

                obj.erased_serialize(ser).unwrap();
            }

            assert_eq!(buf, expected);
        }

        // test boxed trait object
        {
            let obj: Box<dyn Serialize> = Box::new(t);

            let mut buf = Vec::new();

            {
                let mut ser = serde_json::Serializer::new(&mut buf);
                let mut ser: Box<dyn Serializer> = Box::new(<dyn Serializer>::erase(&mut ser));

                obj.erased_serialize(&mut ser).unwrap();
            }

            assert_eq!(buf, expected);
        }
    }

    #[test]
    fn test_vec() {
        test_json(vec!["a", "b"]);
    }

    #[test]
    fn test_struct() {
        #[derive(Serialize)]
        struct S {
            f: usize,
        }

        test_json(S { f: 256 });
    }

    #[test]
    fn test_enum() {
        #[derive(Serialize)]
        enum E {
            Unit,
            Newtype(bool),
            Tuple(bool, bool),
            Struct { t: bool, f: bool },
        }

        test_json(E::Unit);
        test_json(E::Newtype(true));
        test_json(E::Tuple(true, false));
        test_json(E::Struct { t: true, f: false });
    }

    #[test]
    fn test_error_custom() {
        struct Kaboom;

        impl serde::Serialize for Kaboom {
            fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::Error as _;

                Err(S::Error::custom("kaboom"))
            }
        }

        let obj: &dyn Serialize = &Kaboom;

        let err = serde_json::to_vec(obj).unwrap_err();
        assert_eq!(err.to_string(), "kaboom");
    }

    #[test]
    fn assert_serialize() {
        fn assert<T: serde::Serialize>() {}

        assert::<&dyn Serialize>();
        assert::<&(dyn Serialize + Send)>();
        assert::<&(dyn Serialize + Sync)>();
        assert::<&(dyn Serialize + Send + Sync)>();
        assert::<&(dyn Serialize + Sync + Send)>();
        assert::<Vec<&dyn Serialize>>();
        assert::<Vec<&(dyn Serialize + Send)>>();

        assert::<Box<dyn Serialize>>();
        assert::<Box<dyn Serialize + Send>>();
        assert::<Box<dyn Serialize + Sync>>();
        assert::<Box<dyn Serialize + Send + Sync>>();
        assert::<Box<dyn Serialize + Sync + Send>>();
        assert::<Vec<Box<dyn Serialize>>>();
        assert::<Vec<Box<dyn Serialize + Send>>>();
    }

    #[test]
    fn test_dangle() {
        let mut json_serializer = serde_json::Serializer::new(Vec::new());
        let _erased_serializer = <dyn Serializer>::erase(&mut json_serializer);
        drop(json_serializer);
    }
}
