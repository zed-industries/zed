use serde::ser;
use std::{
    borrow::Cow,
    fmt::Display,
    fs::File,
    io::{BufWriter, Write},
    mem,
    path::Path,
};

use crate::{
    date::serde_impls::DATE_NEWTYPE_STRUCT_NAME,
    error::{self, Error, ErrorKind},
    stream::{self, Writer},
    uid::serde_impls::UID_NEWTYPE_STRUCT_NAME,
    Date, Integer, Uid, Value, XmlWriteOptions,
};

#[doc(hidden)]
impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        ErrorKind::Serde(msg.to_string()).without_position()
    }
}

enum OptionMode {
    Root,
    StructField(&'static str),
    StructFieldNameWritten,
    Explicit,
}

/// A structure that serializes Rust values plist event streams.
pub struct Serializer<W: Writer> {
    writer: W,
    option_mode: OptionMode,
}

impl<W: Writer> Serializer<W> {
    pub fn new(writer: W) -> Serializer<W> {
        Serializer {
            writer,
            option_mode: OptionMode::Root,
        }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }

    fn serialize_with_option_mode<T: ?Sized + ser::Serialize>(
        &mut self,
        option_mode: OptionMode,
        value: &T,
    ) -> Result<(), Error> {
        let prev_option_mode = mem::replace(&mut self.option_mode, option_mode);
        let result = value.serialize(&mut *self);
        self.option_mode = prev_option_mode;
        result
    }

    fn maybe_write_pending_struct_field_name(&mut self) -> Result<(), Error> {
        if let OptionMode::StructField(field_name) = self.option_mode {
            self.option_mode = OptionMode::StructFieldNameWritten;
            self.writer.write_string(Cow::Borrowed(field_name))?;
        }
        Ok(())
    }

    fn write_start_array(&mut self, len: Option<u64>) -> Result<(), Error> {
        self.maybe_write_pending_struct_field_name()?;
        self.writer.write_start_array(len)
    }

    fn write_start_dictionary(&mut self, len: Option<u64>) -> Result<(), Error> {
        self.maybe_write_pending_struct_field_name()?;
        self.writer.write_start_dictionary(len)
    }

    fn write_end_collection(&mut self) -> Result<(), Error> {
        self.maybe_write_pending_struct_field_name()?;
        self.writer.write_end_collection()
    }

    fn write_boolean(&mut self, value: bool) -> Result<(), Error> {
        self.maybe_write_pending_struct_field_name()?;
        self.writer.write_boolean(value)
    }

    fn write_data(&mut self, value: Cow<[u8]>) -> Result<(), Error> {
        self.maybe_write_pending_struct_field_name()?;
        self.writer.write_data(value)
    }

    fn write_date(&mut self, value: Date) -> Result<(), Error> {
        self.maybe_write_pending_struct_field_name()?;
        self.writer.write_date(value)
    }

    fn write_integer(&mut self, value: Integer) -> Result<(), Error> {
        self.maybe_write_pending_struct_field_name()?;
        self.writer.write_integer(value)
    }

    fn write_real(&mut self, value: f64) -> Result<(), Error> {
        self.maybe_write_pending_struct_field_name()?;
        self.writer.write_real(value)
    }

    fn write_string<'a, T: Into<Cow<'a, str>>>(&mut self, value: T) -> Result<(), Error> {
        self.maybe_write_pending_struct_field_name()?;
        self.writer.write_string(value.into())
    }

    fn write_uid(&mut self, value: Uid) -> Result<(), Error> {
        self.maybe_write_pending_struct_field_name()?;
        self.writer.write_uid(value)
    }
}

impl<'a, W: Writer> ser::Serializer for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Compound<'a, W>;
    type SerializeTuple = Compound<'a, W>;
    type SerializeTupleStruct = Compound<'a, W>;
    type SerializeTupleVariant = Compound<'a, W>;
    type SerializeMap = Compound<'a, W>;
    type SerializeStruct = Compound<'a, W>;
    type SerializeStructVariant = Compound<'a, W>;

    fn serialize_bool(self, v: bool) -> Result<(), Self::Error> {
        self.write_boolean(v)
    }

    fn serialize_i8(self, v: i8) -> Result<(), Error> {
        self.serialize_i64(v.into())
    }

    fn serialize_i16(self, v: i16) -> Result<(), Error> {
        self.serialize_i64(v.into())
    }

    fn serialize_i32(self, v: i32) -> Result<(), Error> {
        self.serialize_i64(v.into())
    }

    fn serialize_i64(self, v: i64) -> Result<(), Self::Error> {
        self.write_integer(v.into())
    }

    fn serialize_u8(self, v: u8) -> Result<(), Error> {
        self.serialize_u64(v.into())
    }

    fn serialize_u16(self, v: u16) -> Result<(), Error> {
        self.serialize_u64(v.into())
    }

    fn serialize_u32(self, v: u32) -> Result<(), Error> {
        self.serialize_u64(v.into())
    }

    fn serialize_u64(self, v: u64) -> Result<(), Self::Error> {
        self.write_integer(v.into())
    }

    fn serialize_f32(self, v: f32) -> Result<(), Error> {
        self.serialize_f64(v.into())
    }

    fn serialize_f64(self, v: f64) -> Result<(), Error> {
        self.write_real(v)
    }

    fn serialize_char(self, v: char) -> Result<(), Self::Error> {
        let mut buf = [0; 4];
        let v = v.encode_utf8(&mut buf);
        self.write_string(&*v)
    }

    fn serialize_str(self, v: &str) -> Result<(), Error> {
        self.write_string(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<(), Error> {
        self.write_data(Cow::Borrowed(v))
    }

    fn serialize_none(self) -> Result<(), Error> {
        match self.option_mode {
            OptionMode::Root | OptionMode::StructField(_) => (),
            OptionMode::StructFieldNameWritten => unreachable!(),
            OptionMode::Explicit => {
                self.write_start_dictionary(Some(1))?;
                self.write_string("None")?;
                self.serialize_unit()?;
                self.write_end_collection()?;
            }
        }
        Ok(())
    }

    fn serialize_some<T: ?Sized + ser::Serialize>(self, value: &T) -> Result<(), Error> {
        match self.option_mode {
            OptionMode::Root => self.serialize_with_option_mode(OptionMode::Explicit, value)?,
            OptionMode::StructField(field_name) => {
                self.option_mode = OptionMode::StructFieldNameWritten;
                self.write_string(field_name)?;
                self.serialize_with_option_mode(OptionMode::Explicit, value)?;
            }
            OptionMode::StructFieldNameWritten => unreachable!(),
            OptionMode::Explicit => {
                self.write_start_dictionary(Some(1))?;
                self.write_string("Some")?;
                value.serialize(&mut *self)?;
                self.write_end_collection()?;
            }
        }
        Ok(())
    }

    fn serialize_unit(self) -> Result<(), Error> {
        self.write_string("")
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<(), Error> {
        // `plist` since v1.1 serialises unit enum variants as plain strings.
        self.write_string(variant)
    }

    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        match name {
            DATE_NEWTYPE_STRUCT_NAME => value.serialize(DateSerializer { ser: &mut *self }),
            UID_NEWTYPE_STRUCT_NAME => value.serialize(UidSerializer { ser: &mut *self }),
            _ => value.serialize(self),
        }
    }

    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        self.write_start_dictionary(Some(1))?;
        self.write_string(variant)?;
        value.serialize(&mut *self)?;
        self.write_end_collection()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Error> {
        let len = len.map(|len| len as u64);
        self.write_start_array(len)?;
        Ok(Compound { ser: self })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Error> {
        self.serialize_tuple(len)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        self.write_start_dictionary(Some(1))?;
        self.write_string(variant)?;
        self.serialize_tuple(len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Error> {
        let len = len.map(|len| len as u64);
        self.write_start_dictionary(len)?;
        Ok(Compound { ser: self })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Error> {
        // The number of struct fields is not known as fields with None values are ignored.
        self.serialize_map(None)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        self.write_start_dictionary(Some(1))?;
        self.write_string(variant)?;
        self.serialize_struct(name, len)
    }
}

struct DateSerializer<'a, W: 'a + Writer> {
    ser: &'a mut Serializer<W>,
}

impl<W: Writer> DateSerializer<'_, W> {
    fn expecting_date_error() -> Error {
        ser::Error::custom("plist date string expected")
    }
}

impl<W: Writer> ser::Serializer for DateSerializer<'_, W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = ser::Impossible<(), Error>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = ser::Impossible<(), Error>;
    type SerializeStruct = ser::Impossible<(), Error>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    fn serialize_bool(self, _: bool) -> Result<(), Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_i8(self, _: i8) -> Result<(), Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_i16(self, _: i16) -> Result<(), Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_i32(self, _: i32) -> Result<(), Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_i64(self, _: i64) -> Result<(), Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_u8(self, _: u8) -> Result<(), Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_u16(self, _: u16) -> Result<(), Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_u32(self, _: u32) -> Result<(), Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_u64(self, _: u64) -> Result<(), Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_f32(self, _: f32) -> Result<(), Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_f64(self, _: f64) -> Result<(), Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_char(self, _: char) -> Result<(), Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_str(self, v: &str) -> Result<(), Error> {
        let date = Date::from_xml_format(v).map_err(|_| Self::expecting_date_error())?;
        self.ser.write_date(date)
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<(), Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_none(self) -> Result<(), Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_some<T: ?Sized + ser::Serialize>(self, _: &T) -> Result<(), Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_unit(self) -> Result<(), Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<(), Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<(), Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(
        self,
        _: &'static str,
        _: &T,
    ) -> Result<(), Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<(), Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Error> {
        Err(Self::expecting_date_error())
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        Err(Self::expecting_date_error())
    }
}

struct UidSerializer<'a, W: 'a + Writer> {
    ser: &'a mut Serializer<W>,
}

impl<W: Writer> UidSerializer<'_, W> {
    fn expecting_uid_error() -> Error {
        ser::Error::custom("plist uid expected")
    }
}

impl<W: Writer> ser::Serializer for UidSerializer<'_, W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = ser::Impossible<(), Error>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = ser::Impossible<(), Error>;
    type SerializeStruct = ser::Impossible<(), Error>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    fn serialize_bool(self, _: bool) -> Result<(), Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_i8(self, _: i8) -> Result<(), Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_i16(self, _: i16) -> Result<(), Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_i32(self, _: i32) -> Result<(), Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_i64(self, _: i64) -> Result<(), Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_u8(self, _: u8) -> Result<(), Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_u16(self, _: u16) -> Result<(), Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_u32(self, _: u32) -> Result<(), Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_u64(self, v: u64) -> Result<(), Error> {
        self.ser.write_uid(Uid::new(v))
    }

    fn serialize_f32(self, _: f32) -> Result<(), Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_f64(self, _: f64) -> Result<(), Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_char(self, _: char) -> Result<(), Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_str(self, _: &str) -> Result<(), Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<(), Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_none(self) -> Result<(), Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_some<T: ?Sized + ser::Serialize>(self, _: &T) -> Result<(), Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_unit(self) -> Result<(), Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<(), Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<(), Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(
        self,
        _: &'static str,
        _: &T,
    ) -> Result<(), Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<(), Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Error> {
        Err(Self::expecting_uid_error())
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        Err(Self::expecting_uid_error())
    }
}

#[doc(hidden)]
pub struct Compound<'a, W: 'a + Writer> {
    ser: &'a mut Serializer<W>,
}

impl<W: Writer> ser::SerializeSeq for Compound<'_, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<(), Error> {
        self.ser
            .serialize_with_option_mode(OptionMode::Explicit, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.write_end_collection()
    }
}

impl<W: Writer> ser::SerializeTuple for Compound<'_, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<(), Error> {
        self.ser
            .serialize_with_option_mode(OptionMode::Explicit, value)
    }

    fn end(self) -> Result<(), Error> {
        self.ser.write_end_collection()
    }
}

impl<W: Writer> ser::SerializeTupleStruct for Compound<'_, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<(), Error> {
        self.ser
            .serialize_with_option_mode(OptionMode::Explicit, value)
    }

    fn end(self) -> Result<(), Error> {
        self.ser.write_end_collection()
    }
}

impl<W: Writer> ser::SerializeTupleVariant for Compound<'_, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<(), Error> {
        self.ser
            .serialize_with_option_mode(OptionMode::Explicit, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.write_end_collection()?;
        self.ser.write_end_collection()
    }
}

impl<W: Writer> ser::SerializeMap for Compound<'_, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + ser::Serialize>(&mut self, key: &T) -> Result<(), Error> {
        self.ser
            .serialize_with_option_mode(OptionMode::Explicit, key)
    }

    fn serialize_value<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<(), Error> {
        self.ser
            .serialize_with_option_mode(OptionMode::Explicit, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.write_end_collection()
    }
}

impl<W: Writer> ser::SerializeStruct for Compound<'_, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        // We don't want to serialize None if the Option is a struct field as this is how null
        // fields are represented in plists.
        self.ser
            .serialize_with_option_mode(OptionMode::StructField(key), value)
    }

    fn end(self) -> Result<(), Error> {
        self.ser.write_end_collection()
    }
}

impl<W: Writer> ser::SerializeStructVariant for Compound<'_, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        self.ser
            .serialize_with_option_mode(OptionMode::StructField(key), value)
    }

    fn end(self) -> Result<(), Error> {
        self.ser.write_end_collection()?;
        self.ser.write_end_collection()
    }
}

/// Serializes the given data structure to a file as a binary encoded plist.
pub fn to_file_binary<P: AsRef<Path>, T: ser::Serialize>(path: P, value: &T) -> Result<(), Error> {
    let mut file = File::create(path).map_err(error::from_io_without_position)?;
    to_writer_binary(BufWriter::new(&mut file), value)?;
    file.sync_all().map_err(error::from_io_without_position)?;
    Ok(())
}

/// Serializes the given data structure to a file as an XML encoded plist.
pub fn to_file_xml<P: AsRef<Path>, T: ser::Serialize>(path: P, value: &T) -> Result<(), Error> {
    let mut file = File::create(path).map_err(error::from_io_without_position)?;
    to_writer_xml(BufWriter::new(&mut file), value)?;
    file.sync_all().map_err(error::from_io_without_position)?;
    Ok(())
}

/// Serializes the given data structure to a byte stream as a binary encoded plist.
pub fn to_writer_binary<W: Write, T: ser::Serialize>(writer: W, value: &T) -> Result<(), Error> {
    let writer = stream::BinaryWriter::new(writer);
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser)
}

/// Serializes the given data structure to a byte stream as an XML encoded plist.
pub fn to_writer_xml<W: Write, T: ser::Serialize>(writer: W, value: &T) -> Result<(), Error> {
    to_writer_xml_with_options(writer, value, &XmlWriteOptions::default())
}

/// Serializes to a byte stream as an XML encoded plist, using custom [`XmlWriteOptions`].
pub fn to_writer_xml_with_options<W: Write, T: ser::Serialize>(
    writer: W,
    value: &T,
    options: &XmlWriteOptions,
) -> Result<(), Error> {
    let writer = stream::XmlWriter::new_with_options(writer, options);
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser)
}

/// Converts a `T` into a [`Value`] which can represent any valid plist.
pub fn to_value<T: ser::Serialize>(value: &T) -> Result<Value, Error> {
    let writer = crate::value::Builder::default();
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser)?;
    ser.into_inner().finish()
}
