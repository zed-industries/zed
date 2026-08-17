use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize, Serializer,
};

/// Serializes a color with an attached alpha value. The alpha value is added
/// alongside the other values in a flattened structure.
pub(crate) struct AlphaSerializer<'a, S, A> {
    pub inner: S,
    pub alpha: &'a A,
}

impl<'a, S, A> Serializer for AlphaSerializer<'a, S, A>
where
    S: Serializer,
    A: Serialize,
{
    type Ok = S::Ok;

    type Error = S::Error;

    type SerializeSeq = AlphaSerializer<'a, S::SerializeSeq, A>;

    type SerializeTuple = AlphaSerializer<'a, S::SerializeTuple, A>;

    type SerializeTupleStruct = AlphaSerializer<'a, S::SerializeTupleStruct, A>;

    type SerializeTupleVariant = AlphaSerializer<'a, S::SerializeTupleVariant, A>;

    type SerializeMap = AlphaSerializer<'a, S::SerializeMap, A>;

    type SerializeStruct = AlphaSerializer<'a, S::SerializeStruct, A>;

    type SerializeStructVariant = AlphaSerializer<'a, S::SerializeStructVariant, A>;

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(AlphaSerializer {
            inner: self.inner.serialize_seq(len.map(|len| len + 1))?,
            alpha: self.alpha,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(AlphaSerializer {
            inner: self.inner.serialize_tuple(len + 1)?,
            alpha: self.alpha,
        })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(AlphaSerializer {
            inner: self.inner.serialize_tuple_struct(name, len + 1)?,
            alpha: self.alpha,
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(AlphaSerializer {
            inner: self.inner.serialize_map(len.map(|len| len + 1))?,
            alpha: self.alpha,
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(AlphaSerializer {
            inner: self.inner.serialize_struct(name, len + 1)?,
            alpha: self.alpha,
        })
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize + ?Sized,
    {
        let mut serializer = self.serialize_tuple_struct(name, 1)?;
        serializer.serialize_field(value)?;
        serializer.end()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_newtype_struct(name, self.alpha)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_tuple(0)?.end()
    }

    // Unsupported methods:

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        alpha_serializer_error()
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        alpha_serializer_error()
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        alpha_serializer_error()
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        alpha_serializer_error()
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        alpha_serializer_error()
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        alpha_serializer_error()
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        alpha_serializer_error()
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        alpha_serializer_error()
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        alpha_serializer_error()
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        alpha_serializer_error()
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        alpha_serializer_error()
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        alpha_serializer_error()
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        alpha_serializer_error()
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        alpha_serializer_error()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        alpha_serializer_error()
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize + ?Sized,
    {
        alpha_serializer_error()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        alpha_serializer_error()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        alpha_serializer_error()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        alpha_serializer_error()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize + ?Sized,
    {
        alpha_serializer_error()
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        alpha_serializer_error()
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        alpha_serializer_error()
    }

    fn is_human_readable(&self) -> bool {
        self.inner.is_human_readable()
    }
}

fn alpha_serializer_error() -> ! {
    unimplemented!("AlphaSerializer can only serialize structs, maps and sequences")
}

impl<'a, S, A> SerializeSeq for AlphaSerializer<'a, S, A>
where
    S: SerializeSeq,
    A: Serialize,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.inner.serialize_element(value)
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_element(self.alpha)?;
        self.inner.end()
    }
}

impl<'a, S, A> SerializeTuple for AlphaSerializer<'a, S, A>
where
    S: SerializeTuple,
    A: Serialize,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.inner.serialize_element(value)
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_element(self.alpha)?;
        self.inner.end()
    }
}

impl<'a, S, A> SerializeTupleStruct for AlphaSerializer<'a, S, A>
where
    S: SerializeTupleStruct,
    A: Serialize,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.inner.serialize_field(value)
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_field(self.alpha)?;
        self.inner.end()
    }
}

impl<'a, S, A> SerializeTupleVariant for AlphaSerializer<'a, S, A>
where
    S: SerializeTupleVariant,
    A: Serialize,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.inner.serialize_field(value)
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_field(self.alpha)?;
        self.inner.end()
    }
}

impl<'a, S, A> SerializeMap for AlphaSerializer<'a, S, A>
where
    S: SerializeMap,
    A: Serialize,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.inner.serialize_key(key)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.inner.serialize_value(value)
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: Serialize + ?Sized,
        V: Serialize + ?Sized,
    {
        self.inner.serialize_entry(key, value)
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_entry("alpha", self.alpha)?;
        self.inner.end()
    }
}

impl<'a, S, A> SerializeStruct for AlphaSerializer<'a, S, A>
where
    S: SerializeStruct,
    A: Serialize,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.inner.serialize_field(key, value)
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.inner.skip_field(key)
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_field("alpha", self.alpha)?;
        self.inner.end()
    }
}

impl<'a, S, A> SerializeStructVariant for AlphaSerializer<'a, S, A>
where
    S: SerializeStructVariant,
    A: Serialize,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.inner.serialize_field(key, value)
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.inner.skip_field(key)
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_field("alpha", self.alpha)?;
        self.inner.end()
    }
}
