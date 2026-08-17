use super::write_document;
use super::{Error, Serializer};
use crate::fmt::DocumentFormatter;

type InnerSerializeDocumentTable =
    <toml_edit::ser::ValueSerializer as serde::Serializer>::SerializeMap;

#[doc(hidden)]
pub struct SerializeDocumentTable<'d> {
    inner: InnerSerializeDocumentTable,
    dst: &'d mut String,
    settings: DocumentFormatter,
}

impl<'d> SerializeDocumentTable<'d> {
    pub(crate) fn new(ser: Serializer<'d>, inner: InnerSerializeDocumentTable) -> Self {
        Self {
            inner,
            dst: ser.dst,
            settings: ser.settings,
        }
    }
}

impl serde::ser::SerializeMap for SerializeDocumentTable<'_> {
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
        write_document(self.dst, self.settings, self.inner.end())
    }
}

impl serde::ser::SerializeStruct for SerializeDocumentTable<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        self.inner.serialize_field(key, value).map_err(Error::wrap)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        write_document(self.dst, self.settings, self.inner.end())
    }
}

impl serde::ser::SerializeStructVariant for SerializeDocumentTable<'_> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        serde::ser::SerializeStruct::serialize_field(self, key, value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeStruct::end(self)
    }
}

type InnerSerializeDocumentStructVariant =
    <toml_edit::ser::ValueSerializer as serde::Serializer>::SerializeStructVariant;

#[doc(hidden)]
pub struct SerializeDocumentStructVariant<'d> {
    inner: InnerSerializeDocumentStructVariant,
    dst: &'d mut String,
    settings: DocumentFormatter,
}

impl<'d> SerializeDocumentStructVariant<'d> {
    pub(crate) fn new(ser: Serializer<'d>, inner: InnerSerializeDocumentStructVariant) -> Self {
        Self {
            inner,
            dst: ser.dst,
            settings: ser.settings,
        }
    }
}

impl serde::ser::SerializeStructVariant for SerializeDocumentStructVariant<'_> {
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
        write_document(self.dst, self.settings, self.inner.end())
    }
}
