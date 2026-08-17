/*!
A variant of [`sval::Stream`] for cases where a recursive API is needed.

# Limitations

Streaming via `sval_nested` will discard any [`sval::Stream::tag_hint`]s. `sval` allows tag hints to
appear anywhere in a stream, but this library enforces a stricter lifecycle, making those hints unreliable.
*/

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod error;
mod flat;
mod flat_enum;

pub use sval_ref::{Ref, ValueRef};

/**
A generic streaming result.
*/
pub type Result<T = (), E = Error> = sval::Result<T, E>;

pub use self::error::*;

use core::marker::PhantomData;

use self::flat::FlatStream;

/**
Stream a value through a stream.
*/
pub fn stream<'sval, S: Stream<'sval>>(stream: S, value: impl ValueRef<'sval>) -> Result<S::Ok> {
    stream.value(value)
}

/**
Stream a value through a stream.
*/
pub fn stream_ref<'sval, S: Stream<'sval>>(
    stream: S,
    value: &'sval (impl sval::Value + ?Sized),
) -> Result<S::Ok> {
    stream.value_ref(value)
}

/**
Stream a value through a stream with an arbitrarily short lifetime.
*/
pub fn stream_computed<'sval, S: Stream<'sval>>(
    stream: S,
    value: impl sval::Value,
) -> Result<S::Ok> {
    stream.value_computed(&value)
}

/**
A recursive variant of [`sval::Stream`].
*/
pub trait Stream<'sval> {
    /**
    The type of value produced by this stream on completion.
    */
    type Ok;

    /**
    Stream a sequence.
    */
    type Seq: StreamSeq<'sval, Ok = Self::Ok>;

    /**
    Stream a map.
    */
    type Map: StreamMap<'sval, Ok = Self::Ok>;

    /**
    Stream a tuple.
    */
    type Tuple: StreamTuple<'sval, Ok = Self::Ok>;

    /**
    Stream a record.
    */
    type Record: StreamRecord<'sval, Ok = Self::Ok>;

    /**
    Stream an enum.
    */
    type Enum: StreamEnum<'sval, Ok = Self::Ok>;

    /**
    Recurse into a value.
    */
    fn value<V: sval_ref::ValueRef<'sval>>(self, value: V) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::value(self, value)
    }

    /**
    Recurse into a value by reference.
    */
    fn value_ref<V: sval::Value + ?Sized>(self, value: &'sval V) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::value_ref(self, value)
    }

    /**
    Recurse into a value for an arbitrarily short lifetime.
    */
    fn value_computed<V: sval::Value>(self, value: V) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::value_computed(self, value)
    }

    /**
    Stream null, the absence of any other meaningful value.
    */
    fn null(self) -> Result<Self::Ok>;

    /**
    Stream a boolean.
    */
    fn bool(self, value: bool) -> Result<Self::Ok>;

    /**
    Stream a signed 8bit integer.
    */
    fn i8(self, value: i8) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::i8(self, value)
    }

    /**
    Stream a signed 16bit integer.
    */
    fn i16(self, value: i16) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::i16(self, value)
    }

    /**
    Stream a signed 32bit integer.
    */
    fn i32(self, value: i32) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::i32(self, value)
    }

    /**
    Stream a signed 64bit integer.
    */
    fn i64(self, value: i64) -> Result<Self::Ok>;

    /**
    Stream a signed 128bit integer.
    */
    fn i128(self, value: i128) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::i128(self, value)
    }

    /**
    Stream an unsigned 8bit integer.
    */
    fn u8(self, value: u8) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::u8(self, value)
    }

    /**
    Stream an unsigned 16bit integer.
    */
    fn u16(self, value: u16) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::u16(self, value)
    }

    /**
    Stream an unsigned 32bit integer.
    */
    fn u32(self, value: u32) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::u32(self, value)
    }

    /**
    Stream an unsigned 64bit integer.
    */
    fn u64(self, value: u64) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::u64(self, value)
    }

    /**
    Stream an unsigned 128bit integer.
    */
    fn u128(self, value: u128) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::u128(self, value)
    }

    /**
    Stream a 32bit floating point number.
    */
    fn f32(self, value: f32) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::f32(self, value)
    }

    /**
    Stream a 64bit floating point number.
    */
    fn f64(self, value: f64) -> Result<Self::Ok>;

    /**
    Stream UTF8 text.
    */
    fn text(self, text: &'sval str) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::text(self, text)
    }

    /**
    Stream UTF8 text, borrowed for an arbitrarily short lifetime.
    */
    fn text_computed(self, text: &str) -> Result<Self::Ok>;

    /**
    Streeam a bitstring.
    */
    fn binary(self, binary: &'sval [u8]) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::binary(self, binary)
    }

    /**
    Stream a bitstring, borrowed for an arbitrarily short lifetime.
    */
    fn binary_computed(self, binary: &[u8]) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::binary_computed(self, binary)
    }

    /**
    Stream a tag.
    */
    fn tag(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::tag(self, tag, label, index)
    }

    /**
    Stream a tagged value.
    */
    fn tagged<V: sval_ref::ValueRef<'sval>>(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        value: V,
    ) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::tagged(self, tag, label, index, value)
    }

    /**
    Stream a reference to a tagged value.
    */
    fn tagged_ref<V: sval::Value + ?Sized>(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        value: &'sval V,
    ) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::tagged_ref(self, tag, label, index, value)
    }

    /**
    Stream a tagged value, borrowed for an arbitrarily short lifetime.
    */
    fn tagged_computed<V: sval::Value>(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        value: V,
    ) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::tagged_computed(self, tag, label, index, value)
    }

    /**
    Stream a sequence.
    */
    fn seq_begin(self, num_entries: Option<usize>) -> Result<Self::Seq>;

    /**
    Stream a map.
    */
    fn map_begin(self, num_entries: Option<usize>) -> Result<Self::Map>;

    /**
    Stream a tuple.
    */
    fn tuple_begin(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        num_entries: Option<usize>,
    ) -> Result<Self::Tuple>;

    /**
    Stream a record.
    */
    fn record_begin(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        num_entries: Option<usize>,
    ) -> Result<Self::Record>;

    /**
    Stream an enum.
    */
    fn enum_begin(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> Result<Self::Enum>;
}

/**
A stream for a sequence.
*/
pub trait StreamSeq<'sval> {
    /**
    The type of value produced by this stream on completion.
    */
    type Ok;

    /**
    Stream a sequence element.
    */
    fn value<V: sval_ref::ValueRef<'sval>>(&mut self, value: V) -> Result {
        default_stream::seq_value(self, value)
    }

    /**
    Stream a reference to a sequence element.
    */
    fn value_ref<V: sval::Value + ?Sized>(&mut self, value: &'sval V) -> Result {
        default_stream::seq_value_ref(self, value)
    }

    /**
    Stream a sequence element, borrowed for an arbitrarily short lifetime.
    */
    fn value_computed<V: sval::Value>(&mut self, value: V) -> Result;

    /**
    Complete the sequence.
    */
    fn end(self) -> Result<Self::Ok>;
}

/**
A stream for a map.
*/
pub trait StreamMap<'sval> {
    /**
    The type of value produced by this stream on completion.
    */
    type Ok;

    /**
    Stream a map key.
    */
    fn key<V: sval_ref::ValueRef<'sval>>(&mut self, key: V) -> Result {
        default_stream::map_key(self, key)
    }

    /**
    Stream a reference to a map key.
    */
    fn key_ref<V: sval::Value + ?Sized>(&mut self, key: &'sval V) -> Result {
        default_stream::map_key_ref(self, key)
    }

    /**
    Stream a map key, borrowed for an arbitrarily short lifetime.
    */
    fn key_computed<V: sval::Value>(&mut self, key: V) -> Result;

    /**
    Stream a map value.
    */
    fn value<V: sval_ref::ValueRef<'sval>>(&mut self, value: V) -> Result {
        default_stream::map_value(self, value)
    }

    /**
    Stream a reference to a map value.
    */
    fn value_ref<V: sval::Value + ?Sized>(&mut self, value: &'sval V) -> Result {
        default_stream::map_value_ref(self, value)
    }

    /**
    Stream a map value, borrowed for an arbitrarily short lifetime.
    */
    fn value_computed<V: sval::Value>(&mut self, value: V) -> Result;

    /**
    Complete the map.
    */
    fn end(self) -> Result<Self::Ok>;
}

/**
A stream for a tuple.
*/
pub trait StreamTuple<'sval> {
    /**
    The type of value produced by this stream on completion.
    */
    type Ok;

    /**
    Stream a tuple field.
    */
    fn value<V: sval_ref::ValueRef<'sval>>(
        &mut self,
        tag: Option<sval::Tag>,
        index: sval::Index,
        value: V,
    ) -> Result {
        default_stream::tuple_value(self, tag, index, value)
    }

    /**
    Stream a reference to a tuple field.
    */
    fn value_ref<V: sval::Value + ?Sized>(
        &mut self,
        tag: Option<sval::Tag>,
        index: sval::Index,
        value: &'sval V,
    ) -> Result {
        default_stream::tuple_value_ref(self, tag, index, value)
    }

    /**
    Stream a tuple field, borrowed for an arbitrarily short lifetime.
    */
    fn value_computed<V: sval::Value>(
        &mut self,
        tag: Option<sval::Tag>,
        index: sval::Index,
        value: V,
    ) -> Result;

    /**
    Complete the tuple.
    */
    fn end(self) -> Result<Self::Ok>;
}

/**
A stream for a record.
*/
pub trait StreamRecord<'sval> {
    /**
    The type of value produced by this stream on completion.
    */
    type Ok;

    /**
    Stream a record field.
    */
    fn value<V: sval_ref::ValueRef<'sval>>(
        &mut self,
        tag: Option<sval::Tag>,
        label: sval::Label,
        value: V,
    ) -> Result {
        default_stream::record_value(self, tag, label, value)
    }

    /**
    Stream a reference to a record field.
    */
    fn value_ref<V: sval::Value + ?Sized>(
        &mut self,
        tag: Option<sval::Tag>,
        label: sval::Label,
        value: &'sval V,
    ) -> Result {
        default_stream::record_value_ref(self, tag, label, value)
    }

    /**
    Stream a record field, borrowed for an arbitrarily short lifetime.
    */
    fn value_computed<V: sval::Value>(
        &mut self,
        tag: Option<sval::Tag>,
        label: sval::Label,
        value: V,
    ) -> Result;

    /**
    Complete the record.
    */
    fn end(self) -> Result<Self::Ok>;
}

/**
A stream for an enum.
*/
pub trait StreamEnum<'sval> {
    /**
    The type of result produced by this stream on completion.
    */
    type Ok;

    /**
    Stream a tuple variant.
    */
    type Tuple: StreamTuple<'sval, Ok = Self::Ok>;

    /**
    Stream a record variant.
    */
    type Record: StreamRecord<'sval, Ok = Self::Ok>;

    /**
    Stream a nested enum variant.
    */
    type Nested: StreamEnum<'sval, Ok = Self::Ok, Nested = Self::Nested>;

    /**
    Stream a tag variant.
    */
    fn tag(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> Result<Self::Ok>;

    /**
    Stream a tagged value variant.
    */
    fn tagged<V: sval_ref::ValueRef<'sval>>(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        value: V,
    ) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::enum_tagged(self, tag, label, index, value)
    }

    /**
    Stream a reference to a tagged value variant.
    */
    fn tagged_ref<V: sval::Value + ?Sized>(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        value: &'sval V,
    ) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::enum_tagged_ref(self, tag, label, index, value)
    }

    /**
    Stream a tagged value variant, borrowed for an arbitrarily short lifetime.
    */
    fn tagged_computed<V: sval::Value>(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        value: V,
    ) -> Result<Self::Ok>;

    /**
    Stream a tuple variant.
    */
    fn tuple_begin(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        num_entries: Option<usize>,
    ) -> Result<Self::Tuple>;

    /**
    Stream a record variant.
    */
    fn record_begin(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        num_entries: Option<usize>,
    ) -> Result<Self::Record>;

    /**
    Recurse into a nested variant.
    */
    fn nested<F: FnOnce(Self::Nested) -> Result<<Self::Nested as StreamEnum<'sval>>::Ok>>(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        variant: F,
    ) -> Result<Self::Ok>;

    /**
    Stream an empty variant.
    */
    fn empty(self) -> Result<Self::Ok>;
}

/**
A placeholder for a kind of value that isn't supported by a particular stream.
*/
pub struct Unsupported<Ok>(Void, PhantomData<Result<Ok, Error>>);

// Ensure `Unsupported` can't be constructed
enum Void {}

impl<'sval, Ok> Stream<'sval> for Unsupported<Ok> {
    type Ok = Ok;

    type Seq = Self;
    type Map = Self;
    type Tuple = Self;
    type Record = Self;
    type Enum = Self;

    fn null(self) -> Result<Self::Ok> {
        Err(Error::invalid_value("null is unsupported"))
    }

    fn bool(self, _: bool) -> Result<Self::Ok> {
        Err(Error::invalid_value("booleans are unsupported"))
    }

    fn i64(self, _: i64) -> Result<Self::Ok> {
        Err(Error::invalid_value("numbers are unsupported"))
    }

    fn f64(self, _: f64) -> Result<Self::Ok> {
        Err(Error::invalid_value("numbers are unsupported"))
    }

    fn text_computed(self, _: &str) -> Result<Self::Ok> {
        Err(Error::invalid_value("text is unsupported"))
    }

    fn tag(
        self,
        _: Option<sval::Tag>,
        _: Option<sval::Label>,
        _: Option<sval::Index>,
    ) -> Result<Self::Ok> {
        Err(Error::invalid_value("tags are unsupported"))
    }

    fn tagged_computed<V: sval::Value>(
        self,
        _: Option<sval::Tag>,
        _: Option<sval::Label>,
        _: Option<sval::Index>,
        _: V,
    ) -> Result<Self::Ok> {
        Err(Error::invalid_value("tagged values are unsupported"))
    }

    fn seq_begin(self, _: Option<usize>) -> Result<Self::Seq> {
        Err(Error::invalid_value("sequences are unsupported"))
    }

    fn map_begin(self, _: Option<usize>) -> Result<Self::Map> {
        Err(Error::invalid_value("maps are unsupported"))
    }

    fn tuple_begin(
        self,
        _: Option<sval::Tag>,
        _: Option<sval::Label>,
        _: Option<sval::Index>,
        _: Option<usize>,
    ) -> Result<Self::Tuple> {
        Err(Error::invalid_value("records are unsupported"))
    }

    fn record_begin(
        self,
        _: Option<sval::Tag>,
        _: Option<sval::Label>,
        _: Option<sval::Index>,
        _: Option<usize>,
    ) -> Result<Self::Record> {
        Err(Error::invalid_value("records are unsupported"))
    }

    fn enum_begin(
        self,
        _: Option<sval::Tag>,
        _: Option<sval::Label>,
        _: Option<sval::Index>,
    ) -> Result<Self::Enum> {
        Err(Error::invalid_value("enums are unsupported"))
    }
}

impl<'sval, Ok> StreamSeq<'sval> for Unsupported<Ok> {
    type Ok = Ok;

    fn value_computed<V: sval::Value>(&mut self, _: V) -> Result {
        Err(Error::invalid_value("sequences are unsupported"))
    }

    fn end(self) -> Result<Self::Ok> {
        Err(Error::invalid_value("sequences are unsupported"))
    }
}

impl<'sval, Ok> StreamMap<'sval> for Unsupported<Ok> {
    type Ok = Ok;

    fn key_computed<V: sval::Value>(&mut self, _: V) -> Result {
        Err(Error::invalid_value("maps are unsupported"))
    }

    fn value_computed<V: sval::Value>(&mut self, _: V) -> Result {
        Err(Error::invalid_value("maps are unsupported"))
    }

    fn end(self) -> Result<Self::Ok> {
        Err(Error::invalid_value("maps are unsupported"))
    }
}

impl<'sval, Ok> StreamTuple<'sval> for Unsupported<Ok> {
    type Ok = Ok;

    fn value_computed<V: sval::Value>(
        &mut self,
        _: Option<sval::Tag>,
        _: sval::Index,
        _: V,
    ) -> Result {
        Err(Error::invalid_value("tuples are unsupported"))
    }

    fn end(self) -> Result<Self::Ok> {
        Err(Error::invalid_value("tuples are unsupported"))
    }
}

impl<'sval, Ok> StreamRecord<'sval> for Unsupported<Ok> {
    type Ok = Ok;

    fn value_computed<V: sval::Value>(
        &mut self,
        _: Option<sval::Tag>,
        _: sval::Label,
        _: V,
    ) -> Result {
        Err(Error::invalid_value("records are unsupported"))
    }

    fn end(self) -> Result<Self::Ok> {
        Err(Error::invalid_value("records are unsupported"))
    }
}

impl<'sval, Ok> StreamEnum<'sval> for Unsupported<Ok> {
    type Ok = Ok;

    type Tuple = Self;
    type Record = Self;
    type Nested = Self;

    fn tag(
        self,
        _: Option<sval::Tag>,
        _: Option<sval::Label>,
        _: Option<sval::Index>,
    ) -> Result<Self::Ok> {
        Err(Error::invalid_value("enums are unsupported"))
    }

    fn tagged_computed<V: sval::Value>(
        self,
        _: Option<sval::Tag>,
        _: Option<sval::Label>,
        _: Option<sval::Index>,
        _: V,
    ) -> Result<Self::Ok> {
        Err(Error::invalid_value("enums are unsupported"))
    }

    fn tuple_begin(
        self,
        _: Option<sval::Tag>,
        _: Option<sval::Label>,
        _: Option<sval::Index>,
        _: Option<usize>,
    ) -> Result<Self::Record> {
        Err(Error::invalid_value("enums are unsupported"))
    }

    fn record_begin(
        self,
        _: Option<sval::Tag>,
        _: Option<sval::Label>,
        _: Option<sval::Index>,
        _: Option<usize>,
    ) -> Result<Self::Record> {
        Err(Error::invalid_value("enums are unsupported"))
    }

    fn nested<F: FnOnce(Self::Nested) -> Result<<Self::Nested as StreamEnum<'sval>>::Ok>>(
        self,
        _: Option<sval::Tag>,
        _: Option<sval::Label>,
        _: Option<sval::Index>,
        _: F,
    ) -> Result<Self::Ok> {
        Err(Error::invalid_value("enums are unsupported"))
    }

    fn empty(self) -> Result<Self::Ok> {
        Err(Error::invalid_value("enums are unsupported"))
    }
}

fn owned_label(label: sval::Label) -> Result<sval::Label<'static>> {
    owned_label_ref(&label)
}

fn owned_label_ref(label: &sval::Label) -> Result<sval::Label<'static>> {
    #[cfg(feature = "alloc")]
    {
        Ok(label.to_owned())
    }
    #[cfg(not(feature = "alloc"))]
    {
        if let Some(label) = label.as_static_str() {
            Ok(sval::Label::new(label))
        } else {
            Err(Error::no_alloc("streaming value"))
        }
    }
}

pub mod default_stream {
    /*!
    Default method implementations for the [`Stream`] trait.
    */

    use super::*;

    /**
    Recurse into a value.
    */
    pub fn value<'sval, S: Stream<'sval>, V: sval_ref::ValueRef<'sval>>(
        stream: S,
        value: V,
    ) -> Result<S::Ok> {
        let mut stream = FlatStream::new(stream);
        let _ = sval_ref::stream_ref(&mut stream, value);
        stream.finish()
    }

    /**
    Recurse into a value by reference.
    */
    pub fn value_ref<'sval, S: Stream<'sval>, V: sval::Value + ?Sized>(
        stream: S,
        value: &'sval V,
    ) -> Result<S::Ok> {
        stream.value(sval_ref::to_ref(value))
    }

    /**
    Recurse into a value for an arbitrarily short lifetime.
    */
    pub fn value_computed<'sval, S: Stream<'sval>, V: sval::Value>(
        stream: S,
        value: V,
    ) -> Result<S::Ok> {
        let mut stream = FlatStream::new(stream);
        let _ = sval::default_stream::value_computed(&mut stream, &value);
        stream.finish()
    }

    /**
    Stream a signed 8bit integer.
    */
    pub fn i8<'sval, S: Stream<'sval>>(stream: S, value: i8) -> Result<S::Ok> {
        stream.i16(value as i16)
    }

    /**
    Stream a signed 16bit integer.
    */
    pub fn i16<'sval, S: Stream<'sval>>(stream: S, value: i16) -> Result<S::Ok> {
        stream.i32(value as i32)
    }

    /**
    Stream a signed 32bit integer.
    */
    pub fn i32<'sval, S: Stream<'sval>>(stream: S, value: i32) -> Result<S::Ok> {
        stream.i64(value as i64)
    }

    /**
    Stream a signed 128bit integer.
    */
    pub fn i128<'sval, S: Stream<'sval>>(stream: S, value: i128) -> Result<S::Ok> {
        if let Ok(value) = value.try_into() {
            stream.i64(value)
        } else {
            let mut stream = FlatStream::new(stream);
            let _ = sval::stream_number(&mut stream, value);
            stream.finish()
        }
    }

    /**
    Stream an unsigned 8bit integer.
    */
    pub fn u8<'sval, S: Stream<'sval>>(stream: S, value: u8) -> Result<S::Ok> {
        stream.u16(value as u16)
    }

    /**
    Stream an unsigned 16bit integer.
    */
    pub fn u16<'sval, S: Stream<'sval>>(stream: S, value: u16) -> Result<S::Ok> {
        stream.u32(value as u32)
    }

    /**
    Stream an unsigned 32bit integer.
    */
    pub fn u32<'sval, S: Stream<'sval>>(stream: S, value: u32) -> Result<S::Ok> {
        stream.u64(value as u64)
    }

    /**
    Stream an unsigned 64bit integer.
    */
    pub fn u64<'sval, S: Stream<'sval>>(stream: S, value: u64) -> Result<S::Ok> {
        stream.u128(value as u128)
    }

    /**
    Stream an unsigned 128bit integer.
    */
    pub fn u128<'sval, S: Stream<'sval>>(stream: S, value: u128) -> Result<S::Ok> {
        if let Ok(value) = value.try_into() {
            stream.i64(value)
        } else {
            let mut stream = FlatStream::new(stream);
            let _ = sval::stream_number(&mut stream, value);
            stream.finish()
        }
    }

    /**
    Stream a 32bit floating point number.
    */
    pub fn f32<'sval, S: Stream<'sval>>(stream: S, value: f32) -> Result<S::Ok> {
        stream.f64(value as f64)
    }

    /**
    Stream UTF8 text.
    */
    pub fn text<'sval, S: Stream<'sval>>(stream: S, text: &'sval str) -> Result<S::Ok> {
        stream.text_computed(text)
    }

    /**
    Streeam a bitstring.
    */
    pub fn binary<'sval, S: Stream<'sval>>(stream: S, binary: &'sval [u8]) -> Result<S::Ok> {
        stream.binary_computed(binary)
    }

    /**
    Stream a bitstring, borrowed for an arbitrarily short lifetime.
    */
    pub fn binary_computed<'sval, S: Stream<'sval>>(stream: S, binary: &[u8]) -> Result<S::Ok> {
        let mut seq = stream.seq_begin(Some(binary.len()))?;

        for b in binary {
            StreamSeq::value_computed(&mut seq, b)?;
        }

        seq.end()
    }

    /**
    Stream a tag.
    */
    pub fn tag<'sval, S: Stream<'sval>>(
        stream: S,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> Result<S::Ok> {
        struct Tag<'a> {
            tag: Option<&'a sval::Tag>,
            label: Option<&'a sval::Label<'a>>,
        }

        impl<'a> sval::Value for Tag<'a> {
            fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
                &'sval self,
                stream: &mut S,
            ) -> sval::Result {
                self.stream_ref(stream)
            }
        }

        impl<'a, 'sval> ValueRef<'sval> for Tag<'a> {
            fn stream_ref<S: sval::Stream<'sval> + ?Sized>(&self, stream: &mut S) -> sval::Result {
                // Rust's `Option` is fundamental enough that we handle it specially here
                if let Some(&sval::tags::RUST_OPTION_NONE) = self.tag {
                    stream.null()
                }
                // If the tag has a label then stream it as its value
                else if let Some(ref label) = self.label {
                    if let Some(label) = label.as_static_str() {
                        stream.value(label)
                    } else {
                        stream.value_computed(label.as_str())
                    }
                }
                // If the tag doesn't have a label then stream null
                else {
                    stream.null()
                }
            }
        }

        stream.tagged(
            tag.clone(),
            label.clone(),
            index,
            Tag {
                tag: tag.as_ref(),
                label: label.as_ref(),
            },
        )
    }

    /**
    Stream a tagged value.
    */
    pub fn tagged<'sval, S: Stream<'sval>, V: sval_ref::ValueRef<'sval>>(
        stream: S,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        value: V,
    ) -> Result<S::Ok> {
        stream.tagged_computed(tag, label, index, value)
    }

    /**
    Stream a reference to a tagged value.
    */
    pub fn tagged_ref<'sval, S: Stream<'sval>, V: sval::Value + ?Sized>(
        stream: S,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        value: &'sval V,
    ) -> Result<S::Ok> {
        stream.tagged(tag, label, index, sval_ref::to_ref(value))
    }

    /**
    Stream a reference to a tagged value.
    */
    pub fn tagged_computed<'sval, S: Stream<'sval>, V: sval::Value>(
        stream: S,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        value: V,
    ) -> Result<S::Ok> {
        let _ = (tag, label, index);

        stream.value_computed(value)
    }

    /**
    Stream a sequence element.
    */
    pub fn seq_value<'sval, S: StreamSeq<'sval> + ?Sized, V: sval_ref::ValueRef<'sval>>(
        seq: &mut S,
        value: V,
    ) -> Result {
        seq.value_computed(value)
    }

    /**
    Stream a reference to a sequence element.
    */
    pub fn seq_value_ref<'sval, S: StreamSeq<'sval> + ?Sized, V: sval::Value + ?Sized>(
        seq: &mut S,
        value: &'sval V,
    ) -> Result {
        seq.value(sval_ref::to_ref(value))
    }

    /**
    Stream a map key.
    */
    pub fn map_key<'sval, S: StreamMap<'sval> + ?Sized, V: sval_ref::ValueRef<'sval>>(
        map: &mut S,
        key: V,
    ) -> Result {
        map.key_computed(key)
    }

    /**
    Stream a reference to a map key.
    */
    pub fn map_key_ref<'sval, S: StreamMap<'sval> + ?Sized, V: sval::Value + ?Sized>(
        map: &mut S,
        key: &'sval V,
    ) -> Result {
        map.key(sval_ref::to_ref(key))
    }

    /**
    Stream a map value.
    */
    pub fn map_value<'sval, S: StreamMap<'sval> + ?Sized, V: sval_ref::ValueRef<'sval>>(
        map: &mut S,
        value: V,
    ) -> Result {
        map.value_computed(value)
    }

    /**
    Stream a reference to a map value.
    */
    pub fn map_value_ref<'sval, S: StreamMap<'sval> + ?Sized, V: sval::Value + ?Sized>(
        map: &mut S,
        value: &'sval V,
    ) -> Result {
        map.value(sval_ref::to_ref(value))
    }

    /**
    Stream a tuple field.
    */
    pub fn tuple_value<'sval, S: StreamTuple<'sval> + ?Sized, V: sval_ref::ValueRef<'sval>>(
        tuple: &mut S,
        tag: Option<sval::Tag>,
        index: sval::Index,
        value: V,
    ) -> Result {
        tuple.value_computed(tag, index, value)
    }

    /**
    Stream a reference to a tuple field.
    */
    pub fn tuple_value_ref<'sval, S: StreamTuple<'sval> + ?Sized, V: sval::Value + ?Sized>(
        tuple: &mut S,
        tag: Option<sval::Tag>,
        index: sval::Index,
        value: &'sval V,
    ) -> Result {
        tuple.value(tag, index, sval_ref::to_ref(value))
    }

    /**
    Stream a record field.
    */
    pub fn record_value<'sval, S: StreamRecord<'sval> + ?Sized, V: sval_ref::ValueRef<'sval>>(
        record: &mut S,
        tag: Option<sval::Tag>,
        label: sval::Label,
        value: V,
    ) -> Result {
        record.value_computed(tag, label, value)
    }

    /**
    Stream a reference to a record field.
    */
    pub fn record_value_ref<'sval, S: StreamRecord<'sval> + ?Sized, V: sval::Value + ?Sized>(
        record: &mut S,
        tag: Option<sval::Tag>,
        label: sval::Label,
        value: &'sval V,
    ) -> Result {
        record.value(tag, label, sval_ref::to_ref(value))
    }

    /**
    Stream a tagged value variant.
    */
    pub fn enum_tagged<'sval, S: StreamEnum<'sval>, V: sval_ref::ValueRef<'sval>>(
        stream: S,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        value: V,
    ) -> Result<S::Ok> {
        stream.tagged_computed(tag, label, index, value)
    }

    /**
    Stream a reference to a tagged value variant.
    */
    pub fn enum_tagged_ref<'sval, S: StreamEnum<'sval>, V: sval::Value + ?Sized>(
        stream: S,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        value: &'sval V,
    ) -> Result<S::Ok> {
        stream.tagged(tag, label, index, sval_ref::to_ref(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::borrow::Cow;

    use sval_derive_macros::*;

    #[test]
    fn stream_derive() {
        #[derive(Value)]
        struct DeriveRecord {
            a: i32,
            b: DeriveTuple,
        }

        #[derive(Value)]
        struct DeriveTuple(i32, DeriveEnum);

        #[derive(Value)]
        enum DeriveEnum {
            Record { a: i32, b: DeriveTagged },
        }

        #[derive(Value)]
        struct DeriveTagged(bool);

        assert_eq!(
            Value::Record(Record {
                tag: Tag::new(None, Some(sval::Label::new("DeriveRecord")), None).unwrap(),
                entries: vec![
                    (sval::Label::new("a"), Value::I64(1)),
                    (
                        sval::Label::new("b"),
                        Value::Tuple(Tuple {
                            tag: Tag::new(None, Some(sval::Label::new("DeriveTuple")), None)
                                .unwrap(),
                            entries: vec![
                                (sval::Index::new(0), Value::I64(2)),
                                (
                                    sval::Index::new(1),
                                    Value::Enum(Enum {
                                        tag: Tag::new(
                                            None,
                                            Some(sval::Label::new("DeriveEnum")),
                                            None
                                        )
                                        .unwrap(),
                                        variant: Some(Variant::Record(Record {
                                            tag: Tag::new(
                                                None,
                                                Some(sval::Label::new("Record")),
                                                Some(sval::Index::new(0))
                                            )
                                            .unwrap(),
                                            entries: vec![
                                                (sval::Label::new("a"), Value::I64(3)),
                                                (
                                                    sval::Label::new("b"),
                                                    Value::Tagged(Tagged {
                                                        tag: Tag::new(
                                                            None,
                                                            Some(sval::Label::new("DeriveTagged")),
                                                            None
                                                        )
                                                        .unwrap(),
                                                        value: Box::new(Value::Bool(true)),
                                                    })
                                                )
                                            ]
                                        }))
                                    })
                                )
                            ]
                        })
                    )
                ]
            }),
            ToValue::default()
                .value_ref(&DeriveRecord {
                    a: 1,
                    b: DeriveTuple(
                        2,
                        DeriveEnum::Record {
                            a: 3,
                            b: DeriveTagged(true)
                        }
                    )
                })
                .unwrap()
        );
    }

    #[test]
    fn stream_primitive() {
        assert_eq!(
            Value::Null,
            ToValue::default().value_ref(&sval::Null).unwrap()
        );
        assert_eq!(
            Value::I64(42),
            ToValue::default().value_ref(&42i64).unwrap()
        );
        assert_eq!(
            Value::F64(42.1),
            ToValue::default().value_ref(&42.1).unwrap()
        );
        assert_eq!(
            Value::Bool(true),
            ToValue::default().value_ref(&true).unwrap()
        );
    }

    #[test]
    fn stream_option() {
        #[derive(Value)]
        struct Inner {
            a: i32,
            b: bool,
        }

        assert_eq!(
            Value::Tag(
                Tag::new(
                    Some(sval::tags::RUST_OPTION_NONE),
                    Some(sval::Label::new("None")),
                    Some(sval::Index::new(0))
                )
                .unwrap()
            ),
            ToValue::default().value_ref(&None::<Inner>).unwrap()
        );

        assert_eq!(
            Value::Tagged(Tagged {
                tag: Tag::new(
                    Some(sval::tags::RUST_OPTION_SOME),
                    Some(sval::Label::new("Some")),
                    Some(sval::Index::new(1)),
                )
                .unwrap(),
                value: Box::new(Value::Record(Record {
                    tag: Tag::new(None, Some(sval::Label::new("Inner")), None,).unwrap(),
                    entries: vec![
                        (sval::Label::new("a"), Value::I64(42)),
                        (sval::Label::new("b"), Value::Bool(true)),
                    ]
                }))
            }),
            ToValue::default()
                .value_ref(&Some(Inner { a: 42, b: true }))
                .unwrap()
        );
    }

    #[test]
    fn stream_text_borrowed() {
        assert_eq!(
            Value::Text(Cow::Borrowed("borrowed")),
            ToValue::default().value_ref("borrowed").unwrap()
        );
    }

    #[test]
    fn stream_binary_borrowed() {
        assert_eq!(
            Value::Binary(Cow::Borrowed(b"borrowed")),
            ToValue::default()
                .value_ref(sval::BinarySlice::new(b"borrowed"))
                .unwrap()
        );
    }

    #[test]
    fn stream_array() {
        assert_eq!(
            Value::Tagged(Tagged {
                tag: Tag::new(Some(sval::tags::CONSTANT_SIZE), None, None).unwrap(),
                value: Box::new(Value::Seq(Seq {
                    entries: vec![Value::I64(1), Value::I64(2), Value::I64(3),]
                })),
            }),
            ToValue::default().value_ref(&[1, 2, 3] as &[_; 3]).unwrap()
        );
    }

    #[test]
    fn stream_seq() {
        assert_eq!(
            Value::Seq(Seq {
                entries: vec![Value::I64(1), Value::I64(2), Value::I64(3),]
            }),
            ToValue::default().value_ref(&[1, 2, 3] as &[_]).unwrap()
        );
    }

    #[test]
    fn stream_map() {
        assert_eq!(
            Value::Map(Map {
                entries: vec![
                    (Value::Text(Cow::Borrowed("a")), Value::I64(1)),
                    (Value::Text(Cow::Borrowed("b")), Value::I64(2)),
                    (Value::Text(Cow::Borrowed("c")), Value::I64(3)),
                ]
            }),
            ToValue::default()
                .value_ref(sval::MapSlice::new(&[("a", 1), ("b", 2), ("c", 3),]))
                .unwrap()
        );
    }

    #[test]
    fn stream_tuple() {
        assert_eq!(
            Value::Tuple(Tuple {
                tag: Tag::new(None, Some(sval::Label::new("Tuple")), None).unwrap(),
                entries: vec![
                    (sval::Index::new(0), Value::I64(1)),
                    (sval::Index::new(1), Value::Bool(true)),
                ]
            }),
            ToValue::default()
                .value_ref(&{
                    struct Tuple;

                    impl sval::Value for Tuple {
                        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
                            &'sval self,
                            stream: &mut S,
                        ) -> sval::Result {
                            stream.tuple_begin(
                                None,
                                Some(&sval::Label::new("Tuple")),
                                None,
                                None,
                            )?;

                            stream.tuple_value_begin(None, &sval::Index::new(0))?;
                            stream.i64(1)?;
                            stream.tuple_value_end(None, &sval::Index::new(0))?;

                            stream.tuple_value_begin(None, &sval::Index::new(1))?;
                            stream.bool(true)?;
                            stream.tuple_value_end(None, &sval::Index::new(1))?;

                            stream.tuple_end(None, Some(&sval::Label::new("Tuple")), None)
                        }
                    }

                    Tuple
                })
                .unwrap(),
        )
    }

    #[test]
    fn stream_enum_tuple_variant() {
        assert_eq!(
            Value::Enum(Enum {
                tag: Tag::new(None, Some(sval::Label::new("Enum")), None).unwrap(),
                variant: Some(Variant::Tuple(Tuple {
                    tag: Tag::new(
                        None,
                        Some(sval::Label::new("Tuple")),
                        Some(sval::Index::new(0))
                    )
                    .unwrap(),
                    entries: vec![
                        (sval::Index::new(0), Value::I64(1)),
                        (sval::Index::new(1), Value::Bool(true)),
                    ]
                })),
            }),
            ToValue::default()
                .value_ref(&{
                    struct TupleVariant;

                    impl sval::Value for TupleVariant {
                        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
                            &'sval self,
                            stream: &mut S,
                        ) -> sval::Result {
                            stream.enum_begin(None, Some(&sval::Label::new("Enum")), None)?;

                            stream.tuple_begin(
                                None,
                                Some(&sval::Label::new("Tuple")),
                                Some(&sval::Index::new(0)),
                                None,
                            )?;

                            stream.tuple_value_begin(None, &sval::Index::new(0))?;
                            stream.i64(1)?;
                            stream.tuple_value_end(None, &sval::Index::new(0))?;

                            stream.tuple_value_begin(None, &sval::Index::new(1))?;
                            stream.bool(true)?;
                            stream.tuple_value_end(None, &sval::Index::new(1))?;

                            stream.tuple_end(
                                None,
                                Some(&sval::Label::new("Tuple")),
                                Some(&sval::Index::new(0)),
                            )?;

                            stream.enum_end(None, Some(&sval::Label::new("Enum")), None)
                        }
                    }

                    TupleVariant
                })
                .unwrap(),
        )
    }

    #[test]
    fn stream_record() {
        assert_eq!(
            Value::Record(Record {
                tag: Tag::new(None, Some(sval::Label::new("Record")), None).unwrap(),
                entries: vec![
                    (sval::Label::new("a"), Value::I64(1)),
                    (sval::Label::new("b"), Value::Bool(true)),
                ]
            }),
            ToValue::default()
                .value_ref(&{
                    struct Record;

                    impl sval::Value for Record {
                        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
                            &'sval self,
                            stream: &mut S,
                        ) -> sval::Result {
                            stream.record_begin(
                                None,
                                Some(&sval::Label::new("Record")),
                                None,
                                None,
                            )?;

                            stream.record_value_begin(None, &sval::Label::new("a"))?;
                            stream.i64(1)?;
                            stream.record_value_end(None, &sval::Label::new("a"))?;

                            stream.record_value_begin(None, &sval::Label::new("b"))?;
                            stream.bool(true)?;
                            stream.record_value_end(None, &sval::Label::new("b"))?;

                            stream.record_end(None, Some(&sval::Label::new("Record")), None)
                        }
                    }

                    Record
                })
                .unwrap(),
        )
    }

    #[test]
    fn stream_enum_record_variant() {
        assert_eq!(
            Value::Enum(Enum {
                tag: Tag::new(None, Some(sval::Label::new("Enum")), None).unwrap(),
                variant: Some(Variant::Record(Record {
                    tag: Tag::new(
                        None,
                        Some(sval::Label::new("Record")),
                        Some(sval::Index::new(0))
                    )
                    .unwrap(),
                    entries: vec![
                        (sval::Label::new("a"), Value::I64(1)),
                        (sval::Label::new("b"), Value::Bool(true)),
                    ]
                })),
            }),
            ToValue::default()
                .value_ref(&{
                    struct RecordVariant;

                    impl sval::Value for RecordVariant {
                        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
                            &'sval self,
                            stream: &mut S,
                        ) -> sval::Result {
                            stream.enum_begin(None, Some(&sval::Label::new("Enum")), None)?;

                            stream.record_begin(
                                None,
                                Some(&sval::Label::new("Record")),
                                Some(&sval::Index::new(0)),
                                None,
                            )?;

                            stream.record_value_begin(None, &sval::Label::new("a"))?;
                            stream.i64(1)?;
                            stream.record_value_end(None, &sval::Label::new("a"))?;

                            stream.record_value_begin(None, &sval::Label::new("b"))?;
                            stream.bool(true)?;
                            stream.record_value_end(None, &sval::Label::new("b"))?;

                            stream.record_end(
                                None,
                                Some(&sval::Label::new("Record")),
                                Some(&sval::Index::new(0)),
                            )?;

                            stream.enum_end(None, Some(&sval::Label::new("Enum")), None)
                        }
                    }

                    RecordVariant
                })
                .unwrap(),
        )
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn stream_text_owned() {
        assert_eq!(
            Value::Text(Cow::Owned("owned".into())),
            ToValue::default()
                .value_ref(&{
                    struct Text;

                    impl sval::Value for Text {
                        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
                            &'sval self,
                            stream: &mut S,
                        ) -> sval::Result {
                            stream.text_begin(None)?;

                            stream.text_fragment("ow")?;
                            stream.text_fragment("ned")?;

                            stream.text_end()
                        }
                    }

                    Text
                })
                .unwrap()
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn stream_binary_owned() {
        assert_eq!(
            Value::Binary(Cow::Owned(b"owned".to_vec())),
            ToValue::default()
                .value_ref(&{
                    struct Binary;

                    impl sval::Value for Binary {
                        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
                            &'sval self,
                            stream: &mut S,
                        ) -> sval::Result {
                            stream.binary_begin(None)?;

                            stream.binary_fragment(b"ow")?;
                            stream.binary_fragment(b"ned")?;

                            stream.binary_end()
                        }
                    }

                    Binary
                })
                .unwrap()
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn stream_number() {
        struct Number<N>(N);

        impl<N: std::fmt::Display> sval::Value for Number<N> {
            fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
                &'sval self,
                stream: &mut S,
            ) -> sval::Result {
                sval::stream_number(stream, &self.0)
            }
        }

        assert_eq!(
            Value::Tagged(Tagged {
                tag: Tag::new(Some(sval::tags::NUMBER), None, None).unwrap(),
                value: Box::new(Value::Text(Cow::Owned("42".into()))),
            }),
            ToValue::default().value_ref(&Number(42)).unwrap()
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn stream_enum_nested_value() {
        struct Layer;

        impl sval::Value for Layer {
            fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
                &'sval self,
                stream: &mut S,
            ) -> sval::Result {
                struct Layer;

                impl sval::Value for Layer {
                    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
                        &'sval self,
                        stream: &mut S,
                    ) -> sval::Result {
                        stream.enum_begin(None, Some(&sval::Label::new("Layer2")), None)?;
                        stream.tagged_begin(None, Some(&sval::Label::new("Value")), None)?;
                        stream.i64(42)?;
                        stream.tagged_end(None, Some(&sval::Label::new("Value")), None)?;
                        stream.enum_end(None, Some(&sval::Label::new("Layer2")), None)
                    }
                }

                stream.enum_begin(None, Some(&sval::Label::new("Layer1")), None)?;
                stream.value(&Layer)?;
                stream.enum_end(None, Some(&sval::Label::new("Layer1")), None)
            }
        }

        assert_eq!(
            Value::Enum(Enum {
                tag: Tag::new(None, Some(sval::Label::new("Layer1")), None).unwrap(),
                variant: Some(Variant::Enum(Box::new(Enum {
                    tag: Tag::new(None, Some(sval::Label::new("Layer2")), None).unwrap(),
                    variant: Some(Variant::Tagged(Tagged {
                        tag: Tag::new(None, Some(sval::Label::new("Value")), None).unwrap(),
                        value: Box::new(Value::I64(42)),
                    }))
                })))
            }),
            ToValue::default().value_ref(&Layer).unwrap()
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn stream_deeply_nested_enum() {
        struct Layer;

        impl sval::Value for Layer {
            fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
                &'sval self,
                stream: &mut S,
            ) -> sval::Result {
                stream.enum_begin(None, Some(&sval::Label::new("Layer1")), None)?;
                stream.enum_begin(None, Some(&sval::Label::new("Layer2")), None)?;
                stream.enum_begin(None, Some(&sval::Label::new("Layer3")), None)?;
                stream.enum_begin(None, Some(&sval::Label::new("Layer4")), None)?;
                stream.enum_begin(None, Some(&sval::Label::new("Layer5")), None)?;
                stream.enum_begin(None, Some(&sval::Label::new("Layer6")), None)?;
                stream.enum_begin(None, Some(&sval::Label::new("Layer7")), None)?;
                stream.tagged_begin(None, Some(&sval::Label::new("Value")), None)?;
                stream.i64(42)?;
                stream.tagged_end(None, Some(&sval::Label::new("Value")), None)?;
                stream.enum_end(None, Some(&sval::Label::new("Layer7")), None)?;
                stream.enum_end(None, Some(&sval::Label::new("Layer6")), None)?;
                stream.enum_end(None, Some(&sval::Label::new("Layer5")), None)?;
                stream.enum_end(None, Some(&sval::Label::new("Layer4")), None)?;
                stream.enum_end(None, Some(&sval::Label::new("Layer3")), None)?;
                stream.enum_end(None, Some(&sval::Label::new("Layer2")), None)?;
                stream.enum_end(None, Some(&sval::Label::new("Layer1")), None)
            }
        }

        assert_eq!(
            Value::Enum(Enum {
                tag: Tag::new(None, Some(sval::Label::new("Layer1")), None).unwrap(),
                variant: Some(Variant::Enum(Box::new(Enum {
                    tag: Tag::new(None, Some(sval::Label::new("Layer2")), None).unwrap(),
                    variant: Some(Variant::Enum(Box::new(Enum {
                        tag: Tag::new(None, Some(sval::Label::new("Layer3")), None).unwrap(),
                        variant: Some(Variant::Enum(Box::new(Enum {
                            tag: Tag::new(None, Some(sval::Label::new("Layer4")), None).unwrap(),
                            variant: Some(Variant::Enum(Box::new(Enum {
                                tag: Tag::new(None, Some(sval::Label::new("Layer5")), None)
                                    .unwrap(),
                                variant: Some(Variant::Enum(Box::new(Enum {
                                    tag: Tag::new(None, Some(sval::Label::new("Layer6")), None)
                                        .unwrap(),
                                    variant: Some(Variant::Enum(Box::new(Enum {
                                        tag: Tag::new(None, Some(sval::Label::new("Layer7")), None)
                                            .unwrap(),
                                        variant: Some(Variant::Tagged(Tagged {
                                            tag: Tag::new(
                                                None,
                                                Some(sval::Label::new("Value")),
                                                None
                                            )
                                            .unwrap(),
                                            value: Box::new(Value::I64(42)),
                                        }))
                                    })))
                                })))
                            })))
                        })))
                    })))
                })))
            }),
            ToValue::default().value_ref(&Layer).unwrap()
        );
    }

    #[derive(Debug, PartialEq)]
    enum Value<'sval> {
        Null,
        Bool(bool),
        I64(i64),
        F64(f64),
        Binary(Cow<'sval, [u8]>),
        Text(Cow<'sval, str>),
        Tag(Tag),
        Tagged(Tagged<'sval>),
        Seq(Seq<'sval>),
        Map(Map<'sval>),
        Tuple(Tuple<'sval>),
        Record(Record<'sval>),
        Enum(Enum<'sval>),
    }

    impl<'sval> Value<'sval> {
        fn try_into_variant(self) -> Result<Variant<'sval>, Error> {
            match self {
                Value::Tag(variant) => Ok(Variant::Tag(variant)),
                Value::Tagged(variant) => Ok(Variant::Tagged(variant)),
                Value::Tuple(variant) => Ok(Variant::Tuple(variant)),
                Value::Record(variant) => Ok(Variant::Record(variant)),
                Value::Enum(variant) => Ok(Variant::Enum(Box::new(variant))),
                _ => Err(Error::invalid_value("expected an enum variant")),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    struct Tag {
        tag: Option<sval::Tag>,
        label: Option<sval::Label<'static>>,
        index: Option<sval::Index>,
    }

    impl Tag {
        fn new(
            tag: Option<sval::Tag>,
            label: Option<sval::Label>,
            index: Option<sval::Index>,
        ) -> Result<Self, Error> {
            Ok(Tag {
                tag,
                label: label.map(owned_label).transpose()?,
                index: index,
            })
        }
    }

    #[derive(Debug, PartialEq)]
    struct Tagged<'sval> {
        tag: Tag,
        value: Box<Value<'sval>>,
    }

    #[derive(Debug, PartialEq)]
    struct Seq<'sval> {
        entries: Vec<Value<'sval>>,
    }

    #[derive(Debug, PartialEq)]
    struct Map<'sval> {
        entries: Vec<(Value<'sval>, Value<'sval>)>,
    }

    #[derive(Debug, PartialEq)]
    struct Tuple<'sval> {
        tag: Tag,
        entries: Vec<(sval::Index, Value<'sval>)>,
    }

    #[derive(Debug, PartialEq)]
    struct Record<'sval> {
        tag: Tag,
        entries: Vec<(sval::Label<'static>, Value<'sval>)>,
    }

    #[derive(Debug, PartialEq)]
    struct Enum<'sval> {
        tag: Tag,
        variant: Option<Variant<'sval>>,
    }

    #[derive(Debug, PartialEq)]
    enum Variant<'sval> {
        Tag(Tag),
        Tagged(Tagged<'sval>),
        Tuple(Tuple<'sval>),
        Record(Record<'sval>),
        Enum(Box<Enum<'sval>>),
    }

    #[derive(Default)]
    struct ToValue<'sval>(PhantomData<Value<'sval>>);

    struct ToMap<'sval> {
        key: Option<Value<'sval>>,
        map: Map<'sval>,
    }

    struct ToSeq<'sval> {
        seq: Seq<'sval>,
    }

    struct ToTuple<'sval> {
        tuple: Tuple<'sval>,
    }

    struct ToRecord<'sval> {
        record: Record<'sval>,
    }

    struct ToEnum<'sval> {
        tag: Tag,
        _marker: PhantomData<Enum<'sval>>,
    }

    struct ToVariant<S> {
        tag: Tag,
        stream: S,
    }

    impl<'sval> Stream<'sval> for ToValue<'sval> {
        type Ok = Value<'sval>;

        type Seq = ToSeq<'sval>;
        type Map = ToMap<'sval>;

        type Tuple = ToTuple<'sval>;
        type Record = ToRecord<'sval>;
        type Enum = ToEnum<'sval>;

        fn null(self) -> Result<Self::Ok> {
            Ok(Value::Null)
        }

        fn bool(self, value: bool) -> Result<Self::Ok> {
            Ok(Value::Bool(value))
        }

        fn i64(self, value: i64) -> Result<Self::Ok> {
            Ok(Value::I64(value))
        }

        fn f64(self, value: f64) -> Result<Self::Ok> {
            Ok(Value::F64(value))
        }

        fn text(self, text: &'sval str) -> Result<Self::Ok> {
            Ok(Value::Text(Cow::Borrowed(text)))
        }

        fn text_computed(self, text: &str) -> Result<Self::Ok> {
            Ok(Value::Text(Cow::Owned(text.into())))
        }

        fn binary(self, binary: &'sval [u8]) -> Result<Self::Ok> {
            Ok(Value::Binary(Cow::Borrowed(binary)))
        }

        fn binary_computed(self, binary: &[u8]) -> Result<Self::Ok> {
            Ok(Value::Binary(Cow::Owned(binary.into())))
        }

        fn tag(
            self,
            tag: Option<sval::Tag>,
            label: Option<sval::Label>,
            index: Option<sval::Index>,
        ) -> Result<Self::Ok> {
            let tag = Tag::new(tag, label, index)?;

            Ok(Value::Tag(tag))
        }

        fn tagged_computed<V: sval::Value>(
            self,
            tag: Option<sval::Tag>,
            label: Option<sval::Label>,
            index: Option<sval::Index>,
            value: V,
        ) -> Result<Self::Ok> {
            let tag = Tag::new(tag, label, index)?;
            let value = ToValue::default().value_computed(value)?;

            Ok(Value::Tagged(Tagged {
                tag,
                value: Box::new(value),
            }))
        }

        fn seq_begin(self, _: Option<usize>) -> Result<Self::Seq> {
            Ok(ToSeq {
                seq: Seq {
                    entries: Vec::new(),
                },
            })
        }

        fn map_begin(self, _: Option<usize>) -> Result<Self::Map> {
            Ok(ToMap {
                key: None,
                map: Map {
                    entries: Vec::new(),
                },
            })
        }

        fn tuple_begin(
            self,
            tag: Option<sval::Tag>,
            label: Option<sval::Label>,
            index: Option<sval::Index>,
            _: Option<usize>,
        ) -> Result<Self::Tuple> {
            Ok(ToTuple {
                tuple: Tuple {
                    tag: Tag::new(tag, label, index)?,
                    entries: Default::default(),
                },
            })
        }

        fn record_begin(
            self,
            tag: Option<sval::Tag>,
            label: Option<sval::Label>,
            index: Option<sval::Index>,
            _: Option<usize>,
        ) -> Result<Self::Record> {
            Ok(ToRecord {
                record: Record {
                    tag: Tag::new(tag, label, index)?,
                    entries: Default::default(),
                },
            })
        }

        fn enum_begin(
            self,
            tag: Option<sval::Tag>,
            label: Option<sval::Label>,
            index: Option<sval::Index>,
        ) -> Result<Self::Enum> {
            Ok(ToEnum {
                tag: Tag::new(tag, label, index)?,
                _marker: Default::default(),
            })
        }
    }

    impl<'sval> StreamSeq<'sval> for ToSeq<'sval> {
        type Ok = Value<'sval>;

        fn value<V: sval_ref::ValueRef<'sval>>(&mut self, value: V) -> Result {
            let value = ToValue::default().value(value)?;

            self.seq.entries.push(value);

            Ok(())
        }

        fn value_computed<V: sval::Value>(&mut self, value: V) -> Result {
            let value = ToValue::default().value_computed(value)?;

            self.seq.entries.push(value);

            Ok(())
        }

        fn end(self) -> Result<Self::Ok> {
            Ok(Value::Seq(self.seq))
        }
    }

    impl<'sval> StreamMap<'sval> for ToMap<'sval> {
        type Ok = Value<'sval>;

        fn key<V: sval_ref::ValueRef<'sval>>(&mut self, key: V) -> Result {
            self.key = Some(ToValue::default().value(key)?);

            Ok(())
        }

        fn key_computed<V: sval::Value>(&mut self, key: V) -> Result {
            self.key = Some(ToValue::default().value_computed(key)?);

            Ok(())
        }

        fn value<V: sval_ref::ValueRef<'sval>>(&mut self, value: V) -> Result {
            let key = self.key.take().unwrap();
            let value = ToValue::default().value(value)?;

            self.map.entries.push((key, value));

            Ok(())
        }

        fn value_computed<V: sval::Value>(&mut self, value: V) -> Result {
            let key = self.key.take().unwrap();
            let value = ToValue::default().value_computed(value)?;

            self.map.entries.push((key, value));

            Ok(())
        }

        fn end(self) -> Result<Self::Ok> {
            Ok(Value::Map(self.map))
        }
    }

    impl<'sval> StreamTuple<'sval> for ToTuple<'sval> {
        type Ok = Value<'sval>;

        fn value<V: sval_ref::ValueRef<'sval>>(
            &mut self,
            _: Option<sval::Tag>,
            index: sval::Index,
            value: V,
        ) -> Result {
            let value = ToValue::default().value(value)?;

            self.tuple.entries.push((index.clone(), value));

            Ok(())
        }

        fn value_computed<V: sval::Value>(
            &mut self,
            _: Option<sval::Tag>,
            index: sval::Index,
            value: V,
        ) -> Result {
            let value = ToValue::default().value_computed(value)?;

            self.tuple.entries.push((index.clone(), value));

            Ok(())
        }

        fn end(self) -> Result<Self::Ok> {
            Ok(Value::Tuple(self.tuple))
        }
    }

    impl<'sval> StreamRecord<'sval> for ToRecord<'sval> {
        type Ok = Value<'sval>;

        fn value<V: sval_ref::ValueRef<'sval>>(
            &mut self,
            _: Option<sval::Tag>,
            label: sval::Label,
            value: V,
        ) -> Result {
            let label = owned_label(label)?;
            let value = ToValue::default().value(value)?;

            self.record.entries.push((label, value));

            Ok(())
        }

        fn value_computed<V: sval::Value>(
            &mut self,
            _: Option<sval::Tag>,
            label: sval::Label,
            value: V,
        ) -> Result {
            let label = owned_label(label)?;
            let value = ToValue::default().value_computed(value)?;

            self.record.entries.push((label, value));

            Ok(())
        }

        fn end(self) -> Result<Self::Ok> {
            Ok(Value::Record(self.record))
        }
    }

    impl<'sval> StreamEnum<'sval> for ToEnum<'sval> {
        type Ok = Value<'sval>;

        type Tuple = ToVariant<ToTuple<'sval>>;
        type Record = ToVariant<ToRecord<'sval>>;
        type Nested = Self;

        fn tag(
            self,
            tag: Option<sval::Tag>,
            label: Option<sval::Label>,
            index: Option<sval::Index>,
        ) -> Result<Self::Ok> {
            let tag = Tag::new(tag, label, index)?;

            Ok(Value::Enum(Enum {
                tag: self.tag,
                variant: Some(Variant::Tag(tag)),
            }))
        }

        fn tagged_computed<V: sval::Value>(
            self,
            tag: Option<sval::Tag>,
            label: Option<sval::Label>,
            index: Option<sval::Index>,
            value: V,
        ) -> Result<Self::Ok> {
            let tag = Tag::new(tag, label, index)?;
            let value = ToValue::default().value_computed(value)?;

            Ok(Value::Enum(Enum {
                tag: self.tag,
                variant: Some(Variant::Tagged(Tagged {
                    tag,
                    value: Box::new(value),
                })),
            }))
        }

        fn tuple_begin(
            self,
            tag: Option<sval::Tag>,
            label: Option<sval::Label>,
            index: Option<sval::Index>,
            _: Option<usize>,
        ) -> Result<Self::Tuple> {
            Ok(ToVariant {
                tag: self.tag,
                stream: ToTuple {
                    tuple: Tuple {
                        tag: Tag::new(tag, label, index)?,
                        entries: Vec::new(),
                    },
                },
            })
        }

        fn record_begin(
            self,
            tag: Option<sval::Tag>,
            label: Option<sval::Label>,
            index: Option<sval::Index>,
            _: Option<usize>,
        ) -> Result<Self::Record> {
            Ok(ToVariant {
                tag: self.tag,
                stream: ToRecord {
                    record: Record {
                        tag: Tag::new(tag, label, index)?,
                        entries: Vec::new(),
                    },
                },
            })
        }

        fn nested<F: FnOnce(Self::Nested) -> Result<<Self::Nested as StreamEnum<'sval>>::Ok>>(
            self,
            tag: Option<sval::Tag>,
            label: Option<sval::Label>,
            index: Option<sval::Index>,
            variant: F,
        ) -> Result<Self::Ok> {
            let variant = variant(ToEnum {
                tag: Tag::new(tag, label, index)?,
                _marker: PhantomData,
            })?
            .try_into_variant()?;

            Ok(Value::Enum(Enum {
                tag: self.tag,
                variant: Some(variant),
            }))
        }

        fn empty(self) -> Result<Self::Ok> {
            Ok(Value::Enum(Enum {
                tag: self.tag,
                variant: None,
            }))
        }
    }

    impl<'sval> StreamTuple<'sval> for ToVariant<ToTuple<'sval>> {
        type Ok = Value<'sval>;

        fn value<V: sval_ref::ValueRef<'sval>>(
            &mut self,
            tag: Option<sval::Tag>,
            index: sval::Index,
            value: V,
        ) -> Result {
            self.stream.value(tag, index, value)
        }

        fn value_computed<V: sval::Value>(
            &mut self,
            tag: Option<sval::Tag>,
            index: sval::Index,
            value: V,
        ) -> Result {
            self.stream.value_computed(tag, index, value)
        }

        fn end(self) -> Result<Self::Ok> {
            Ok(Value::Enum(Enum {
                tag: self.tag,
                variant: Some(Variant::Tuple(self.stream.tuple)),
            }))
        }
    }

    impl<'sval> StreamRecord<'sval> for ToVariant<ToRecord<'sval>> {
        type Ok = Value<'sval>;

        fn value<V: sval_ref::ValueRef<'sval>>(
            &mut self,
            tag: Option<sval::Tag>,
            label: sval::Label,
            value: V,
        ) -> Result {
            self.stream.value(tag, label, value)
        }

        fn value_computed<V: sval::Value>(
            &mut self,
            tag: Option<sval::Tag>,
            label: sval::Label,
            value: V,
        ) -> Result {
            self.stream.value_computed(tag, label, value)
        }

        fn end(self) -> Result<Self::Ok> {
            Ok(Value::Enum(Enum {
                tag: self.tag,
                variant: Some(Variant::Record(self.stream.record)),
            }))
        }
    }
}
