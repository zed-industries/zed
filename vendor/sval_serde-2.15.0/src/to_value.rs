use core::fmt;

use serde_core::ser::Error as _;

/**
Stream a [`serde_core::Serialize`] into an [`sval::Stream`].
*/
pub fn stream<'sval>(
    stream: &mut (impl sval::Stream<'sval> + ?Sized),
    value: impl serde_core::Serialize,
) -> sval::Result {
    stream.value_computed(&ToValue::new(value))
}

/**
Adapt a [`serde_core::Serialize`] into a [`sval::Value`].
*/
#[repr(transparent)]
pub struct ToValue<V: ?Sized>(V);

impl<V: serde_core::Serialize> ToValue<V> {
    /**
    Adapt a [`serde_core::Serialize`] into a [`sval::Value`].
    */
    pub const fn new(value: V) -> ToValue<V> {
        ToValue(value)
    }
}

impl<V: serde_core::Serialize + ?Sized> ToValue<V> {
    /**
    Adapt a reference to a [`serde_core::Serialize`] into an [`sval::Value`].
    */
    pub const fn new_borrowed<'a>(value: &'a V) -> &'a ToValue<V> {
        // SAFETY: `&'a V` and `&'a ToValue<V>` have the same ABI
        unsafe { &*(value as *const _ as *const ToValue<V>) }
    }
}

impl<V: serde_core::Serialize> sval::Value for ToValue<V> {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        self.0.serialize(Stream { stream })?;

        Ok(())
    }
}

struct Stream<S> {
    stream: S,
}

struct StreamTuple<S> {
    stream: S,
    label: Option<sval::Label<'static>>,
    index: usize,
}

struct StreamTupleVariant<S> {
    stream: S,
    enum_label: sval::Label<'static>,
    variant_label: sval::Label<'static>,
    variant_index: sval::Index,
    index: usize,
}

struct StreamRecord<S> {
    stream: S,
    label: Option<sval::Label<'static>>,
}

struct StreamRecordVariant<S> {
    stream: S,
    enum_label: sval::Label<'static>,
    variant_label: sval::Label<'static>,
    variant_index: sval::Index,
}

#[derive(Debug)]
struct Error;

impl From<Error> for sval::Error {
    fn from(_: Error) -> sval::Error {
        sval::Error::new()
    }
}

impl From<sval::Error> for Error {
    fn from(_: sval::Error) -> Error {
        Error
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to stream a value")
    }
}

impl serde_core::ser::Error for Error {
    fn custom<T>(_: T) -> Self
    where
        T: fmt::Display,
    {
        Error
    }
}

impl serde_core::ser::StdError for Error {}

impl<'sval, S: sval::Stream<'sval>> Stream<S> {
    fn stream_value(&mut self, v: impl sval::Value) -> Result<(), Error> {
        self.stream.value_computed(&v)?;

        Ok(())
    }
}

impl<'sval, S: sval::Stream<'sval>> serde_core::Serializer for Stream<S> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = StreamTuple<S>;
    type SerializeTupleStruct = StreamTuple<S>;
    type SerializeTupleVariant = StreamTupleVariant<S>;
    type SerializeMap = Self;
    type SerializeStruct = StreamRecord<S>;
    type SerializeStructVariant = StreamRecordVariant<S>;

    fn serialize_bool(mut self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_i8(mut self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_i16(mut self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_i32(mut self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_i64(mut self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_i128(mut self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_u8(mut self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_u16(mut self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_u32(mut self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_u64(mut self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_u128(mut self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_f32(mut self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_f64(mut self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_char(mut self, v: char) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn serialize_str(mut self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.stream_value(v)
    }

    fn collect_str<T: ?Sized>(mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: fmt::Display,
    {
        sval::stream_display(&mut self.stream, value)
            .map_err(|_| Error::custom("failed to stream a string"))
    }

    fn serialize_bytes(mut self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.stream_value(sval::BinarySlice::new(v))
    }

    fn serialize_none(mut self) -> Result<Self::Ok, Self::Error> {
        self.stream_value(None::<()>)
    }

    fn serialize_some<T: ?Sized>(mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde_core::Serialize,
    {
        self.stream_value(Some(ToValue(value)))
    }

    fn serialize_unit(mut self) -> Result<Self::Ok, Self::Error> {
        self.stream_value(())
    }

    fn serialize_unit_struct(mut self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.stream
            .tag(None, Some(&sval::Label::new(name)), None)
            .map_err(|_| Error::custom("failed to stream a tag"))
    }

    fn serialize_unit_variant(
        mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.stream
            .enum_begin(None, Some(&sval::Label::new(name)), None)
            .map_err(|_| Error::custom("failed to stream a tag variant"))?;
        self.stream
            .tag(
                None,
                Some(&sval::Label::new(variant)),
                Some(&sval::Index::new_u32(variant_index)),
            )
            .map_err(|_| Error::custom("failed to stream a tag variant"))?;
        self.stream
            .enum_end(None, Some(&sval::Label::new(name)), None)
            .map_err(|_| Error::custom("failed to stream a tag variant"))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        mut self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde_core::Serialize,
    {
        self.stream
            .tagged_begin(None, Some(&sval::Label::new(name)), None)
            .map_err(|_| Error::custom("failed to stream a tagged value"))?;
        self.stream
            .value_computed(&ToValue(value))
            .map_err(|_| Error::custom("failed to stream a tagged value"))?;
        self.stream
            .tagged_end(None, Some(&sval::Label::new(name)), None)
            .map_err(|_| Error::custom("failed to stream a tagged value"))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde_core::Serialize,
    {
        self.stream
            .enum_begin(None, Some(&sval::Label::new(name)), None)
            .map_err(|_| Error::custom("failed to stream a tagged variant"))?;
        self.stream
            .tagged_begin(
                None,
                Some(&sval::Label::new(variant)),
                Some(&sval::Index::new_u32(variant_index)),
            )
            .map_err(|_| Error::custom("failed to stream a tagged variant"))?;
        self.stream
            .value_computed(&ToValue(value))
            .map_err(|_| Error::custom("failed to stream a tagged variant"))?;
        self.stream
            .tagged_end(
                None,
                Some(&sval::Label::new(variant)),
                Some(&sval::Index::new_u32(variant_index)),
            )
            .map_err(|_| Error::custom("failed to stream a tagged variant"))?;
        self.stream
            .enum_end(None, Some(&sval::Label::new(name)), None)
            .map_err(|_| Error::custom("failed to stream a tagged variant"))
    }

    fn serialize_seq(mut self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.stream
            .seq_begin(len)
            .map_err(|_| Error::custom("failed to stream a sequence"))?;

        Ok(self)
    }

    fn serialize_tuple(mut self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.stream
            .tuple_begin(None, None, None, Some(len))
            .map_err(|_| Error::custom("failed to stream a tuple"))?;

        Ok(StreamTuple {
            stream: self.stream,
            label: None,
            index: 0,
        })
    }

    fn serialize_tuple_struct(
        mut self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.stream
            .tuple_begin(None, Some(&sval::Label::new(name)), None, Some(len))
            .map_err(|_| Error::custom("failed to stream a tuple"))?;

        Ok(StreamTuple {
            stream: self.stream,
            label: Some(sval::Label::new(name)),
            index: 0,
        })
    }

    fn serialize_tuple_variant(
        mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.stream
            .enum_begin(None, Some(&sval::Label::new(name)), None)
            .map_err(|_| Error::custom("failed to stream a tuple variant"))?;
        self.stream
            .tuple_begin(
                None,
                Some(&sval::Label::new(variant)),
                Some(&sval::Index::new_u32(variant_index)),
                Some(len),
            )
            .map_err(|_| Error::custom("failed to stream a tuple variant"))?;

        Ok(StreamTupleVariant {
            stream: self.stream,
            enum_label: sval::Label::new(name),
            variant_label: sval::Label::new(variant),
            variant_index: sval::Index::new_u32(variant_index),
            index: 0,
        })
    }

    fn serialize_map(mut self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.stream
            .map_begin(len)
            .map_err(|_| Error::custom("failed to stream a map"))?;

        Ok(self)
    }

    fn serialize_struct(
        mut self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.stream
            .record_begin(None, Some(&sval::Label::new(name)), None, Some(len))
            .map_err(|_| Error::custom("failed to stream a record"))?;

        Ok(StreamRecord {
            stream: self.stream,
            label: Some(sval::Label::new(name)),
        })
    }

    fn serialize_struct_variant(
        mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.stream
            .enum_begin(None, Some(&sval::Label::new(name)), None)
            .map_err(|_| Error::custom("failed to stream a record variant"))?;
        self.stream
            .record_begin(
                None,
                Some(&sval::Label::new(variant)),
                Some(&sval::Index::new_u32(variant_index)),
                Some(len),
            )
            .map_err(|_| Error::custom("failed to stream a record variant"))?;

        Ok(StreamRecordVariant {
            stream: self.stream,
            enum_label: sval::Label::new(name),
            variant_label: sval::Label::new(variant),
            variant_index: sval::Index::new_u32(variant_index),
        })
    }
}

impl<'sval, S: sval::Stream<'sval>> serde_core::ser::SerializeSeq for Stream<S> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde_core::Serialize,
    {
        self.stream
            .seq_value_begin()
            .map_err(|_| Error::custom("failed to stream a sequence value"))?;
        self.stream
            .value_computed(&ToValue(value))
            .map_err(|_| Error::custom("failed to stream a sequence value"))?;
        self.stream
            .seq_value_end()
            .map_err(|_| Error::custom("failed to stream a sequence value"))
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.stream
            .seq_end()
            .map_err(|_| Error::custom("failed to stream a sequence"))
    }
}

impl<'sval, S: sval::Stream<'sval>> serde_core::ser::SerializeTuple for StreamTuple<S> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde_core::Serialize,
    {
        self.stream
            .tuple_value_begin(None, &sval::Index::new(self.index))
            .map_err(|_| Error::custom("failed to stream a tuple value"))?;
        self.stream
            .value_computed(&ToValue(value))
            .map_err(|_| Error::custom("failed to stream a tuple value"))?;
        self.stream
            .tuple_value_end(None, &sval::Index::new(self.index))
            .map_err(|_| Error::custom("failed to stream a tuple value"))?;

        self.index += 1;

        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.stream
            .tuple_end(None, self.label.as_ref(), None)
            .map_err(|_| Error::custom("failed to stream a tuple"))
    }
}

impl<'sval, S: sval::Stream<'sval>> serde_core::ser::SerializeTupleStruct for StreamTuple<S> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde_core::Serialize,
    {
        self.stream
            .tuple_value_begin(None, &sval::Index::new(self.index))
            .map_err(|_| Error::custom("failed to stream a tuple value"))?;
        self.stream
            .value_computed(&ToValue(value))
            .map_err(|_| Error::custom("failed to stream a tuple value"))?;
        self.stream
            .tuple_value_end(None, &sval::Index::new(self.index))
            .map_err(|_| Error::custom("failed to stream a tuple value"))?;

        self.index += 1;

        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.stream
            .tuple_end(None, self.label.as_ref(), None)
            .map_err(|_| Error::custom("failed to stream a tuple"))
    }
}

impl<'sval, S: sval::Stream<'sval>> serde_core::ser::SerializeTupleVariant
    for StreamTupleVariant<S>
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde_core::Serialize,
    {
        self.stream
            .tuple_value_begin(None, &sval::Index::new(self.index))
            .map_err(|_| Error::custom("failed to stream a tuple variant value"))?;
        self.stream
            .value_computed(&ToValue(value))
            .map_err(|_| Error::custom("failed to stream a tuple value"))?;
        self.stream
            .tuple_value_end(None, &sval::Index::new(self.index))
            .map_err(|_| Error::custom("failed to stream a tuple variant value"))?;

        self.index += 1;

        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.stream
            .tuple_end(None, Some(&self.variant_label), Some(&self.variant_index))
            .map_err(|_| Error::custom("failed to stream a tuple variant"))?;
        self.stream
            .enum_end(None, Some(&self.enum_label), None)
            .map_err(|_| Error::custom("failed to stream a tuple variant"))
    }
}

impl<'sval, S: sval::Stream<'sval>> serde_core::ser::SerializeMap for Stream<S> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde_core::Serialize,
    {
        self.stream
            .map_key_begin()
            .map_err(|_| Error::custom("failed to stream a map key"))?;
        self.stream
            .value_computed(&ToValue(key))
            .map_err(|_| Error::custom("failed to stream a map key"))?;
        self.stream
            .map_key_end()
            .map_err(|_| Error::custom("failed to stream a map key"))
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde_core::Serialize,
    {
        self.stream
            .map_value_begin()
            .map_err(|_| Error::custom("failed to stream a map value"))?;
        self.stream
            .value_computed(&ToValue(value))
            .map_err(|_| Error::custom("failed to stream a map value"))?;
        self.stream
            .map_value_end()
            .map_err(|_| Error::custom("failed to stream a map value"))
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.stream
            .map_end()
            .map_err(|_| Error::custom("failed to stream a map"))
    }
}

impl<'sval, S: sval::Stream<'sval>> serde_core::ser::SerializeStruct for StreamRecord<S> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde_core::Serialize,
    {
        self.stream
            .record_value_begin(None, &sval::Label::new(key))
            .map_err(|_| Error::custom("failed to stream a record value"))?;
        self.stream
            .value_computed(&ToValue(value))
            .map_err(|_| Error::custom("failed to stream a record value"))?;
        self.stream
            .record_value_end(None, &sval::Label::new(key))
            .map_err(|_| Error::custom("failed to stream a record value"))
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.stream
            .record_end(None, self.label.as_ref(), None)
            .map_err(|_| Error::custom("failed to stream a record"))
    }
}

impl<'sval, S: sval::Stream<'sval>> serde_core::ser::SerializeStructVariant
    for StreamRecordVariant<S>
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde_core::Serialize,
    {
        self.stream
            .record_value_begin(None, &sval::Label::new(key))
            .map_err(|_| Error::custom("failed to stream a record variant value"))?;
        self.stream
            .value_computed(&ToValue(value))
            .map_err(|_| Error::custom("failed to stream a record variant value"))?;
        self.stream
            .record_value_end(None, &sval::Label::new(key))
            .map_err(|_| Error::custom("failed to stream a record variant value"))
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.stream
            .record_end(None, Some(&self.variant_label), Some(&self.variant_index))
            .map_err(|_| Error::custom("failed to stream a record variant"))?;
        self.stream
            .enum_end(None, Some(&self.enum_label), None)
            .map_err(|_| Error::custom("failed to stream a record variant"))
    }
}
