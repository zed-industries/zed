use super::write_value;
use super::{Error, ValueSerializer};

type InnerSerializeValueTable =
    <toml_edit::ser::ValueSerializer as serde::Serializer>::SerializeMap;

#[doc(hidden)]
pub struct SerializeValueTable<'d> {
    inner: InnerSerializeValueTable,
    dst: &'d mut String,
}

impl<'d> SerializeValueTable<'d> {
    pub(crate) fn new(ser: ValueSerializer<'d>, inner: InnerSerializeValueTable) -> Self {
        Self {
            inner,
            dst: ser.dst,
        }
    }
}

impl serde::ser::SerializeMap for SerializeValueTable<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, input: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        self.inner.serialize_key(input).map_err(Error::wrap)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        self.inner.serialize_value(value).map_err(Error::wrap)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, self.inner.end())
    }
}

impl serde::ser::SerializeStruct for SerializeValueTable<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        self.inner.serialize_field(key, value).map_err(Error::wrap)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, self.inner.end())
    }
}

type InnerSerializeValueStructVariant =
    <toml_edit::ser::ValueSerializer as serde::Serializer>::SerializeStructVariant;

#[doc(hidden)]
pub struct SerializeValueStructVariant<'d> {
    inner: InnerSerializeValueStructVariant,
    dst: &'d mut String,
}

impl<'d> SerializeValueStructVariant<'d> {
    pub(crate) fn new(ser: ValueSerializer<'d>, inner: InnerSerializeValueStructVariant) -> Self {
        Self {
            inner,
            dst: ser.dst,
        }
    }
}

impl serde::ser::SerializeStructVariant for SerializeValueStructVariant<'_> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        self.inner.serialize_field(key, value).map_err(Error::wrap)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, self.inner.end())
    }
}
