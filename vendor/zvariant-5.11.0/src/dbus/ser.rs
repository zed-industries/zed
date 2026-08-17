use serde::{
    Serialize,
    ser::{self, SerializeMap, SerializeSeq, SerializeTuple},
};
use std::{
    io::{Seek, Write},
    str::{self, FromStr},
};

use crate::{
    Basic, Error, ObjectPath, Result, Signature, WriteBytes,
    container_depths::ContainerDepths,
    serialized::{Context, Format},
    utils::*,
};

/// Our D-Bus serialization implementation.
pub(crate) struct Serializer<'ser, W>(pub(crate) crate::SerializerCommon<'ser, W>);

impl<'ser, W> Serializer<'ser, W>
where
    W: Write + Seek,
{
    /// Create a D-Bus Serializer struct instance.
    ///
    /// On Windows, there is no `fds` argument.
    pub fn new<'w: 'ser, 'f: 'ser>(
        signature: &'ser Signature,
        writer: &'w mut W,
        #[cfg(unix)] fds: &'f mut crate::ser::FdList,
        ctxt: Context,
    ) -> Result<Self> {
        assert_eq!(ctxt.format(), Format::DBus);

        Ok(Self(crate::SerializerCommon {
            ctxt,
            signature,
            writer,
            #[cfg(unix)]
            fds,
            bytes_written: 0,
            value_sign: None,
            container_depths: Default::default(),
        }))
    }
}

macro_rules! serialize_basic {
    ($method:ident($type:ty) $write_method:ident) => {
        serialize_basic!($method($type) $write_method($type));
    };
    ($method:ident($type:ty) $write_method:ident($as:ty)) => {
        fn $method(self, v: $type) -> Result<()> {
            self.0.prep_serialize_basic::<$type>()?;
            self.0.$write_method(self.0.ctxt.endian(), v as $as).map_err(|e| Error::InputOutput(e.into()))
        }
    };
}

impl<'ser, 'b, W> ser::Serializer for &'b mut Serializer<'ser, W>
where
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqSerializer<'ser, 'b, W>;
    type SerializeTuple = StructSeqSerializer<'ser, 'b, W>;
    type SerializeTupleStruct = StructSeqSerializer<'ser, 'b, W>;
    type SerializeTupleVariant = StructSeqSerializer<'ser, 'b, W>;
    type SerializeMap = MapSerializer<'ser, 'b, W>;
    type SerializeStruct = StructSeqSerializer<'ser, 'b, W>;
    type SerializeStructVariant = StructSeqSerializer<'ser, 'b, W>;

    serialize_basic!(serialize_bool(bool) write_u32(u32));
    // No i8 type in D-Bus/GVariant, let's pretend it's i16
    serialize_basic!(serialize_i8(i8) write_i16(i16));
    serialize_basic!(serialize_i16(i16) write_i16);
    serialize_basic!(serialize_i64(i64) write_i64);

    fn serialize_i32(self, v: i32) -> Result<()> {
        match &self.0.signature {
            #[cfg(unix)]
            Signature::Fd => {
                self.0.add_padding(u32::alignment(Format::DBus))?;
                let idx = self.0.add_fd(v)?;
                self.0
                    .write_u32(self.0.ctxt.endian(), idx)
                    .map_err(|e| Error::InputOutput(e.into()))
            }
            _ => {
                self.0.prep_serialize_basic::<i32>()?;
                self.0
                    .write_i32(self.0.ctxt.endian(), v)
                    .map_err(|e| Error::InputOutput(e.into()))
            }
        }
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.0.prep_serialize_basic::<u8>()?;
        // Endianness is irrelevant for single bytes.
        self.0
            .write_u8(self.0.ctxt.endian(), v)
            .map_err(|e| Error::InputOutput(e.into()))
    }

    serialize_basic!(serialize_u16(u16) write_u16);
    serialize_basic!(serialize_u32(u32) write_u32);
    serialize_basic!(serialize_u64(u64) write_u64);
    // No f32 type in D-Bus/GVariant, let's pretend it's f64
    serialize_basic!(serialize_f32(f32) write_f64(f64));
    serialize_basic!(serialize_f64(f64) write_f64);

    fn serialize_char(self, v: char) -> Result<()> {
        // No char type in D-Bus, let's pretend it's a string
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.0
            .add_padding(self.0.signature.alignment(Format::DBus))?;

        let signature = self.0.signature;
        if matches!(signature, Signature::Variant) {
            self.0.value_sign = Some(Signature::from_str(v)?);
        }

        match signature {
            Signature::ObjectPath | Signature::Str => {
                self.0
                    .write_u32(self.0.ctxt.endian(), usize_to_u32(v.len()))
                    .map_err(|e| Error::InputOutput(e.into()))?;
            }
            Signature::Signature | Signature::Variant => {
                self.0
                    .write_u8(self.0.ctxt.endian(), usize_to_u8(v.len()))
                    .map_err(|e| Error::InputOutput(e.into()))?;
            }
            _ => {
                let expected = format!(
                    "`{}`, `{}`, `{}` or `{}`",
                    <&str>::SIGNATURE_STR,
                    Signature::SIGNATURE_STR,
                    ObjectPath::SIGNATURE_STR,
                    VARIANT_SIGNATURE_CHAR,
                );
                return Err(Error::SignatureMismatch(signature.clone(), expected));
            }
        }

        self.0
            .write_all(v.as_bytes())
            .map_err(|e| Error::InputOutput(e.into()))?;
        self.0
            .write_all(&b"\0"[..])
            .map_err(|e| Error::InputOutput(e.into()))?;

        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.0.add_padding(ARRAY_ALIGNMENT_DBUS)?;
        self.0
            .write_u32(self.0.ctxt.endian(), v.len() as u32)
            .map_err(|e| Error::InputOutput(e.into()))?;
        self.0
            .write(v)
            .map(|_| ())
            .map_err(|e| Error::InputOutput(e.into()))
    }

    fn serialize_none(self) -> Result<()> {
        #[cfg(feature = "option-as-array")]
        {
            let seq = self.serialize_seq(Some(0))?;
            seq.end()
        }

        #[cfg(not(feature = "option-as-array"))]
        unreachable!(
            "Can only encode Option<T> in D-Bus format if `option-as-array` feature is enabled",
        );
    }

    fn serialize_some<T>(self, #[allow(unused)] value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        #[cfg(feature = "option-as-array")]
        {
            let mut seq = self.serialize_seq(Some(1))?;
            seq.serialize_element(value)?;
            seq.end()
        }

        #[cfg(not(feature = "option-as-array"))]
        unreachable!(
            "Can only encode Option<T> in D-Bus format if `option-as-array` feature is enabled",
        );
    }

    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        if matches!(self.0.signature, Signature::Str) {
            variant.serialize(self)
        } else {
            variant_index.serialize(self)
        }
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)?;

        Ok(())
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        StructSerializer::enum_variant(self, variant_index)
            .and_then(|mut ser| ser.serialize_element(value))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.0.add_padding(ARRAY_ALIGNMENT_DBUS)?;
        // Length in bytes (unfortunately not the same as len passed to us here) which we
        // initially set to 0.
        self.0
            .write_u32(self.0.ctxt.endian(), 0_u32)
            .map_err(|e| Error::InputOutput(e.into()))?;

        // D-Bus expects us to add padding for the first element even when there is no first
        // element (i-e empty array) so we add padding already.
        let (alignment, child_signature) = match self.0.signature {
            Signature::Array(child) => (child.alignment(self.0.ctxt.format()), child.signature()),
            Signature::Dict { key, .. } => (DICT_ENTRY_ALIGNMENT_DBUS, key.signature()),
            _ => {
                return Err(Error::SignatureMismatch(
                    self.0.signature.clone(),
                    "an array or dict".to_string(),
                ));
            }
        };

        // In case of an array, we'll only be serializing the array's child elements from now on and
        // in case of a dict, we'll swap key and value signatures during serlization of each entry,
        // so let's assume the element signature for array and key signature for dict, from now on.
        // We restore the original signature at the end of serialization.
        let array_signature = self.0.signature;
        self.0.signature = child_signature;
        let first_padding = self.0.add_padding(alignment)?;
        let start = self.0.bytes_written;
        self.0.container_depths = self.0.container_depths.inc_array()?;

        Ok(SeqSerializer {
            ser: self,
            start,
            first_padding,
            array_signature,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_struct("", len)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_struct(name, len)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        StructSerializer::enum_variant(self, variant_index).map(StructSeqSerializer::Struct)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        let (key_signature, value_signature) = match self.0.signature {
            Signature::Dict { key, value } => (key.signature(), value.signature()),
            _ => {
                return Err(Error::SignatureMismatch(
                    self.0.signature.clone(),
                    "a dict".to_string(),
                ));
            }
        };

        let seq = self.serialize_seq(len)?;

        Ok(MapSerializer {
            seq,
            key_signature,
            value_signature,
        })
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.0
            .add_padding(self.0.signature.alignment(self.0.ctxt.format()))?;
        match &self.0.signature {
            Signature::Variant => StructSerializer::variant(self).map(StructSeqSerializer::Struct),
            Signature::Array(_) => self.serialize_seq(Some(len)).map(StructSeqSerializer::Seq),
            Signature::U8 => StructSerializer::unit(self).map(StructSeqSerializer::Struct),
            Signature::Structure(_) => {
                StructSerializer::structure(self).map(StructSeqSerializer::Struct)
            }
            Signature::Dict { .. } => self.serialize_map(Some(len)).map(StructSeqSerializer::Map),
            _ => Err(Error::SignatureMismatch(
                self.0.signature.clone(),
                "a struct, array, u8 or variant".to_string(),
            )),
        }
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        StructSerializer::enum_variant(self, variant_index).map(StructSeqSerializer::Struct)
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

#[doc(hidden)]
pub struct SeqSerializer<'ser, 'b, W> {
    ser: &'b mut Serializer<'ser, W>,
    start: usize,
    // First element's padding
    first_padding: usize,
    array_signature: &'ser Signature,
}

impl<W> SeqSerializer<'_, '_, W>
where
    W: Write + Seek,
{
    pub(self) fn end_seq(self) -> Result<()> {
        // Set size of array in bytes
        let array_len = self.ser.0.bytes_written - self.start;
        let len = usize_to_u32(array_len);
        let total_array_len = (array_len + self.first_padding + 4) as i64;
        self.ser
            .0
            .writer
            .seek(std::io::SeekFrom::Current(-total_array_len))
            .map_err(|e| Error::InputOutput(e.into()))?;
        self.ser
            .0
            .writer
            .write_u32(self.ser.0.ctxt.endian(), len)
            .map_err(|e| Error::InputOutput(e.into()))?;
        self.ser
            .0
            .writer
            .seek(std::io::SeekFrom::Current(total_array_len - 4))
            .map_err(|e| Error::InputOutput(e.into()))?;

        self.ser.0.container_depths = self.ser.0.container_depths.dec_array();
        self.ser.0.signature = self.array_signature;

        Ok(())
    }
}

impl<W> ser::SerializeSeq for SeqSerializer<'_, '_, W>
where
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<()> {
        self.end_seq()
    }
}

#[doc(hidden)]
pub struct StructSerializer<'ser, 'b, W> {
    ser: &'b mut Serializer<'ser, W>,
    // The original container depths. We restore to that at the end.
    container_depths: ContainerDepths,
    // Index of the next field to serialize.
    field_idx: usize,
}

impl<'ser, 'b, W> StructSerializer<'ser, 'b, W>
where
    W: Write + Seek,
{
    fn variant(ser: &'b mut Serializer<'ser, W>) -> Result<Self> {
        let container_depths = ser.0.container_depths;
        ser.0.container_depths = ser.0.container_depths.inc_variant()?;

        Ok(Self {
            ser,
            container_depths,
            field_idx: 0,
        })
    }

    fn structure(ser: &'b mut Serializer<'ser, W>) -> Result<Self> {
        let container_depths = ser.0.container_depths;
        ser.0.container_depths = ser.0.container_depths.inc_structure()?;

        Ok(Self {
            ser,
            container_depths,
            field_idx: 0,
        })
    }

    fn unit(ser: &'b mut Serializer<'ser, W>) -> Result<Self> {
        // serialize as a `0u8`
        serde::Serializer::serialize_u8(&mut *ser, 0)?;

        let container_depths = ser.0.container_depths;
        Ok(Self {
            ser,
            container_depths,
            field_idx: 0,
        })
    }

    fn enum_variant(ser: &'b mut Serializer<'ser, W>, variant_index: u32) -> Result<Self> {
        // Encode enum variants as a struct with first field as variant index
        let Signature::Structure(fields) = ser.0.signature else {
            return Err(Error::SignatureMismatch(
                ser.0.signature.clone(),
                "a struct".to_string(),
            ));
        };
        let struct_field = fields.iter().nth(1).and_then(|f| {
            if matches!(f, Signature::Structure(_)) {
                Some(f)
            } else {
                None
            }
        });

        ser.0.add_padding(STRUCT_ALIGNMENT_DBUS)?;
        let mut struct_ser = Self::structure(ser)?;
        struct_ser.serialize_struct_element(&variant_index)?;

        if let Some(field) = struct_field {
            // Add struct padding for inner struct and pretend we're the inner struct.
            struct_ser.ser.0.add_padding(STRUCT_ALIGNMENT_DBUS)?;
            struct_ser.field_idx = 0;
            struct_ser.ser.0.signature = field;
        }

        Ok(struct_ser)
    }

    fn serialize_struct_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let signature = self.ser.0.signature;
        let field_signature = match signature {
            Signature::Variant => {
                match &self.ser.0.value_sign {
                    // Serializing the value of a Value, which means signature was serialized
                    // already, and also put aside for us to be picked here.
                    Some(signature) => signature,
                    // Serializing the signature of a Value.
                    None => &Signature::Variant,
                }
            }
            Signature::Structure(fields) => {
                let signature = fields.iter().nth(self.field_idx).ok_or_else(|| {
                    Error::SignatureMismatch(signature.clone(), "a struct".to_string())
                })?;
                self.field_idx += 1;

                signature
            }
            _ => unreachable!("Incorrect signature for struct"),
        };
        let bytes_written = self.ser.0.bytes_written;
        let mut ser = Serializer(crate::SerializerCommon::<W> {
            ctxt: self.ser.0.ctxt,
            signature: field_signature,
            writer: self.ser.0.writer,
            #[cfg(unix)]
            fds: self.ser.0.fds,
            bytes_written,
            value_sign: None,
            container_depths: self.ser.0.container_depths,
        });

        value.serialize(&mut ser)?;
        self.ser.0.bytes_written = ser.0.bytes_written;
        self.ser.0.value_sign = ser.0.value_sign;

        Ok(())
    }

    fn end_struct(self) -> Result<()> {
        // Restore the original container depths.
        self.ser.0.container_depths = self.container_depths;

        Ok(())
    }
}

#[doc(hidden)]
/// Allows us to serialize a struct as an ARRAY.
pub enum StructSeqSerializer<'ser, 'b, W> {
    Struct(StructSerializer<'ser, 'b, W>),
    Seq(SeqSerializer<'ser, 'b, W>),
    Map(MapSerializer<'ser, 'b, W>),
}

macro_rules! serialize_struct_anon_fields {
    ($trait:ident $method:ident) => {
        impl<'ser, 'b, W> ser::$trait for StructSerializer<'ser, 'b, W>
        where
            W: Write + Seek,
        {
            type Ok = ();
            type Error = Error;

            fn $method<T>(&mut self, value: &T) -> Result<()>
            where
                T: ?Sized + Serialize,
            {
                self.serialize_struct_element(value)
            }

            fn end(self) -> Result<()> {
                self.end_struct()
            }
        }

        impl<'ser, 'b, W> ser::$trait for StructSeqSerializer<'ser, 'b, W>
        where
            W: Write + Seek,
        {
            type Ok = ();
            type Error = Error;

            fn $method<T>(&mut self, value: &T) -> Result<()>
            where
                T: ?Sized + Serialize,
            {
                match self {
                    StructSeqSerializer::Struct(ser) => ser.$method(value),
                    StructSeqSerializer::Seq(ser) => ser.serialize_element(value),
                    StructSeqSerializer::Map(_) => unreachable!(),
                }
            }

            fn end(self) -> Result<()> {
                match self {
                    StructSeqSerializer::Struct(ser) => ser.end_struct(),
                    StructSeqSerializer::Seq(ser) => ser.end_seq(),
                    StructSeqSerializer::Map(_) => unreachable!(),
                }
            }
        }
    };
}
serialize_struct_anon_fields!(SerializeTuple serialize_element);
serialize_struct_anon_fields!(SerializeTupleStruct serialize_field);
serialize_struct_anon_fields!(SerializeTupleVariant serialize_field);

pub struct MapSerializer<'ser, 'b, W> {
    seq: SeqSerializer<'ser, 'b, W>,
    key_signature: &'ser Signature,
    value_signature: &'ser Signature,
}

impl<W> SerializeMap for MapSerializer<'_, '_, W>
where
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.seq.ser.0.add_padding(DICT_ENTRY_ALIGNMENT_DBUS)?;

        key.serialize(&mut *self.seq.ser)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.seq.ser.0.signature = self.value_signature;
        value.serialize(&mut *self.seq.ser)?;
        self.seq.ser.0.signature = self.key_signature;

        Ok(())
    }

    fn end(self) -> Result<()> {
        self.seq.end_seq()
    }
}

macro_rules! serialize_struct_named_fields {
    ($trait:ident) => {
        impl<'ser, 'b, W> ser::$trait for StructSerializer<'ser, 'b, W>
        where
            W: Write + Seek,
        {
            type Ok = ();
            type Error = Error;

            fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
            where
                T: ?Sized + Serialize,
            {
                self.serialize_struct_element(value)
            }

            fn end(self) -> Result<()> {
                self.end_struct()
            }
        }

        impl<'ser, 'b, W> ser::$trait for StructSeqSerializer<'ser, 'b, W>
        where
            W: Write + Seek,
        {
            type Ok = ();
            type Error = Error;

            fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
            where
                T: ?Sized + Serialize,
            {
                match self {
                    StructSeqSerializer::Struct(ser) => ser.serialize_field(key, value),
                    StructSeqSerializer::Seq(ser) => ser.serialize_element(value),
                    StructSeqSerializer::Map(ser) => {
                        ser.serialize_key(key)?;
                        ser.serialize_value(value)
                    }
                }
            }

            fn end(self) -> Result<()> {
                match self {
                    StructSeqSerializer::Struct(ser) => ser.end_struct(),
                    StructSeqSerializer::Seq(ser) => ser.end_seq(),
                    StructSeqSerializer::Map(ser) => ser.end(),
                }
            }
        }
    };
}
serialize_struct_named_fields!(SerializeStruct);
serialize_struct_named_fields!(SerializeStructVariant);
