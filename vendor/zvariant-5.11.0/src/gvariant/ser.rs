use serde::{
    Serialize,
    ser::{self, SerializeMap, SerializeSeq, SerializeTuple},
};
use std::{
    io::{Seek, Write},
    str::{self, FromStr},
};

use crate::{
    Error, Result, Signature,
    container_depths::ContainerDepths,
    framing_offset_size::FramingOffsetSize,
    framing_offsets::FramingOffsets,
    serialized::{Context, Format},
    utils::*,
};

/// Our serialization implementation.
pub(crate) struct Serializer<'ser, W>(pub(crate) crate::SerializerCommon<'ser, W>);

impl<'ser, W> Serializer<'ser, W>
where
    W: Write + Seek,
{
    /// Create a GVariant Serializer struct instance.
    ///
    /// On Windows, the method doesn't have `fds` argument.
    pub fn new<'w: 'ser, 'f: 'ser>(
        signature: &'ser Signature,
        writer: &'w mut W,
        #[cfg(unix)] fds: &'f mut crate::ser::FdList,
        ctxt: Context,
    ) -> Result<Self> {
        assert_eq!(ctxt.format(), Format::GVariant);

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

    fn serialize_maybe<T>(&mut self, value: Option<&T>) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let alignment = self.0.signature.alignment(self.0.ctxt.format());
        self.0.add_padding(alignment)?;

        let mut child_signature = match self.0.signature {
            Signature::Maybe(child) => child.signature(),
            _ => {
                return Err(Error::SignatureMismatch(
                    self.0.signature.clone(),
                    "a maybe".to_string(),
                ));
            }
        };
        let fixed_sized_child = child_signature.is_fixed_sized();

        let value = match value {
            Some(value) => value,
            None => return Ok(()),
        };

        std::mem::swap(&mut self.0.signature, &mut child_signature);
        self.0.container_depths = self.0.container_depths.inc_maybe()?;

        value.serialize(&mut *self)?;

        self.0.container_depths = self.0.container_depths.dec_maybe();
        std::mem::swap(&mut self.0.signature, &mut child_signature);

        if !fixed_sized_child {
            self.0
                .write_all(&b"\0"[..])
                .map_err(|e| Error::InputOutput(e.into()))?;
        }

        Ok(())
    }
}

macro_rules! serialize_basic {
    ($method:ident, $type:ty) => {
        fn $method(self, v: $type) -> Result<()> {
            let ctxt = Context::new_dbus(self.0.ctxt.endian(), self.0.ctxt.position());
            let bytes_written = self.0.bytes_written;
            let mut dbus_ser = crate::dbus::Serializer(crate::SerializerCommon::<W> {
                ctxt,
                signature: self.0.signature,
                writer: &mut self.0.writer,
                #[cfg(unix)]
                fds: self.0.fds,
                bytes_written,
                value_sign: None,
                container_depths: self.0.container_depths,
            });

            dbus_ser.$method(v)?;

            self.0.bytes_written = dbus_ser.0.bytes_written;

            Ok(())
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

    serialize_basic!(serialize_i16, i16);
    serialize_basic!(serialize_i32, i32);
    serialize_basic!(serialize_i64, i64);

    serialize_basic!(serialize_u8, u8);
    serialize_basic!(serialize_u16, u16);
    serialize_basic!(serialize_u32, u32);
    serialize_basic!(serialize_u64, u64);

    serialize_basic!(serialize_f64, f64);

    fn serialize_bool(self, v: bool) -> Result<()> {
        // bool is encoded as a single byte in GVariant
        self.0
            .write_all(&[v as u8])
            .map_err(|e| Error::InputOutput(e.into()))
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        // No i8 type in GVariant, let's pretend it's i16
        self.serialize_i16(v as i16)
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        // No f32 type in GVariant, let's pretend it's f64
        self.serialize_f64(v as f64)
    }

    fn serialize_char(self, v: char) -> Result<()> {
        // No char type in GVariant, let's pretend it's a string
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        // Strings in GVariant format require no alignment.

        if matches!(self.0.signature, Signature::Variant) {
            self.0.value_sign = Some(Signature::from_str(v)?);

            // signature is serialized after the value in GVariant
            return Ok(());
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
        let seq = self.serialize_seq(Some(v.len()))?;
        seq.ser
            .0
            .write(v)
            .map_err(|e| Error::InputOutput(e.into()))?;
        seq.end()
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_maybe::<()>(None)
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_maybe(Some(value))
    }

    fn serialize_unit(self) -> Result<()> {
        self.0
            .write_all(&b"\0"[..])
            .map_err(|e| Error::InputOutput(e.into()))
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
        let signature = self.0.signature;
        let alignment = signature.alignment(Format::GVariant);
        self.0.add_padding(alignment)?;

        let (child_signature, fixed_sized_element) = match signature {
            Signature::Array(child) => (child.signature(), child.is_fixed_sized()),
            Signature::Dict { key, value } => (
                key.signature(),
                key.is_fixed_sized() && value.is_fixed_sized(),
            ),
            _ => {
                return Err(Error::SignatureMismatch(
                    self.0.signature.clone(),
                    "an array or dict".to_string(),
                ));
            }
        };
        let offsets = (!fixed_sized_element).then(FramingOffsets::new);

        // In case of an array, we'll only be serializing the array's child elements from now on and
        // in case of a dict, we'll swap key and value signatures during serlization of each entry,
        // so let's assume the element signature for array and key signature for dict, from now on.
        // We restore the original signature at the end of serialization.
        let array_signature = self.0.signature;
        self.0.signature = child_signature;
        self.0.container_depths = self.0.container_depths.inc_array()?;

        let start = self.0.bytes_written;

        Ok(SeqSerializer {
            ser: self,
            start,
            element_alignment: alignment,
            offsets,
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
        let (key_signature, value_signature, key_start) = match self.0.signature {
            Signature::Dict { key, value } => (
                key.signature(),
                value.signature(),
                (!key.is_fixed_sized()).then_some(0),
            ),
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
            key_start,
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
    // alignment of element
    element_alignment: usize,
    // All offsets
    offsets: Option<FramingOffsets>,
    array_signature: &'ser Signature,
}

impl<'ser, 'b, W> SeqSerializer<'ser, 'b, W>
where
    W: Write + Seek,
{
    pub(self) fn end_seq(self) -> Result<()> {
        self.ser.0.container_depths = self.ser.0.container_depths.dec_array();
        self.ser.0.signature = self.array_signature;

        let offsets = match self.offsets {
            Some(offsets) => offsets,
            None => return Ok(()),
        };
        let array_len = self.ser.0.bytes_written - self.start;
        if array_len == 0 {
            // Empty sequence
            return Ok(());
        }

        offsets.write_all(&mut self.ser.0, array_len)?;

        Ok(())
    }
}

impl<'ser, 'b, W> ser::SerializeSeq for SeqSerializer<'ser, 'b, W>
where
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ser)?;

        if let Some(ref mut offsets) = self.offsets {
            let offset = self.ser.0.bytes_written - self.start;

            offsets.push(offset);
        }

        Ok(())
    }

    fn end(self) -> Result<()> {
        self.end_seq()
    }
}

#[doc(hidden)]
pub struct StructSerializer<'ser, 'b, W> {
    ser: &'b mut Serializer<'ser, W>,
    start: usize,
    // All offsets
    offsets: Option<FramingOffsets>,
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
        ser.0.add_padding(VARIANT_ALIGNMENT_GVARIANT)?;
        let offsets = match ser.0.signature {
            Signature::Structure(_) => Some(FramingOffsets::new()),
            _ => None,
        };
        let start = ser.0.bytes_written;
        let container_depths = ser.0.container_depths;
        ser.0.container_depths = ser.0.container_depths.inc_variant()?;

        Ok(Self {
            ser,
            offsets,
            start,
            container_depths,
            field_idx: 0,
        })
    }

    fn structure(ser: &'b mut Serializer<'ser, W>) -> Result<Self> {
        let alignment = ser.0.signature.alignment(Format::GVariant);
        ser.0.add_padding(alignment)?;

        let offsets = match ser.0.signature {
            Signature::Structure(_) => Some(FramingOffsets::new()),
            _ => None,
        };
        let start = ser.0.bytes_written;
        let container_depths = ser.0.container_depths;
        ser.0.container_depths = ser.0.container_depths.inc_structure()?;

        Ok(Self {
            ser,
            offsets,
            start,
            container_depths,
            field_idx: 0,
        })
    }

    fn unit(ser: &'b mut Serializer<'ser, W>) -> Result<Self> {
        // serialize as a `0u8`
        serde::Serializer::serialize_u8(&mut *ser, 0)?;

        let start = ser.0.bytes_written;
        let container_depths = ser.0.container_depths;
        Ok(Self {
            ser,
            offsets: None,
            start,
            container_depths,
            field_idx: 0,
        })
    }

    fn enum_variant(ser: &'b mut Serializer<'ser, W>, variant_index: u32) -> Result<Self> {
        // Encode enum variants as a struct with first field as variant index
        let Signature::Structure(fields) = &ser.0.signature else {
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

        let alignment = ser.0.signature.alignment(Format::GVariant);
        ser.0.add_padding(alignment)?;
        let mut struct_ser = Self::structure(ser)?;
        struct_ser.serialize_struct_element(&variant_index)?;

        if let Some(field) = struct_field {
            // Add struct padding for inner struct and pretend we're the inner struct.
            let alignment = field.alignment(Format::GVariant);
            struct_ser.ser.0.add_padding(alignment)?;
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
        let (field_signature, is_variant_value) = match signature {
            Signature::Variant => {
                match &self.ser.0.value_sign {
                    // Serializing the value of a Value, which means signature was serialized
                    // already, and also put aside for us to be picked here.
                    Some(signature) => (signature, true),
                    // Serializing the signature of a Value.
                    None => (&Signature::Variant, false),
                }
            }
            Signature::Structure(fields) => {
                let signature = fields.iter().nth(self.field_idx).ok_or_else(|| {
                    Error::SignatureMismatch(signature.clone(), "a struct".to_string())
                })?;
                self.field_idx += 1;

                (signature, false)
            }
            _ => unreachable!("Incorrect signature for struct or variant"),
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
        let field_signature = field_signature.clone();
        self.ser.0.value_sign = ser.0.value_sign;

        match signature {
            Signature::Variant if is_variant_value => {
                self.ser
                    .0
                    .write_all(&b"\0"[..])
                    .map_err(|e| Error::InputOutput(e.into()))?;
                write!(self.ser.0, "{field_signature}")
                    .map_err(|e| Error::InputOutput(e.into()))?;
            }
            Signature::Variant => (),
            Signature::Structure(_) => {
                if let Some(ref mut offsets) = self.offsets {
                    if !field_signature.is_fixed_sized() {
                        offsets.push_front(self.ser.0.bytes_written - self.start);
                    }
                }
            }
            _ => unreachable!("Incorrect signature for struct"),
        };

        Ok(())
    }

    fn end_struct(self) -> Result<()> {
        // Restore the original container depths.
        self.ser.0.container_depths = self.container_depths;

        let mut offsets = match self.offsets {
            Some(offsets) => offsets,
            None => return Ok(()),
        };
        let struct_len = self.ser.0.bytes_written - self.start;
        if struct_len == 0 {
            // Empty sequence
            return Ok(());
        }
        if offsets.peek() == Some(struct_len) {
            // For structs, we don't want offset of last element
            offsets.pop();
        }

        offsets.write_all(&mut self.ser.0, struct_len)?;

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
    // start of last dict-entry key written
    key_start: Option<usize>,
}

impl<'ser, 'b, W> SerializeMap for MapSerializer<'ser, 'b, W>
where
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.seq.ser.0.add_padding(self.seq.element_alignment)?;

        if self.key_start.is_some() {
            self.key_start.replace(self.seq.ser.0.bytes_written);
        }

        key.serialize(&mut *self.seq.ser)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // For non-fixed-sized keys, we must add the key offset after the value
        let key_offset = self
            .key_start
            .map(|start| self.seq.ser.0.bytes_written - start);

        self.seq.ser.0.signature = self.value_signature;
        value.serialize(&mut *self.seq.ser)?;
        self.seq.ser.0.signature = self.key_signature;

        if let Some(key_offset) = key_offset {
            let entry_size = self.seq.ser.0.bytes_written - self.key_start.unwrap_or(0);
            let offset_size = FramingOffsetSize::for_encoded_container(entry_size);
            offset_size.write_offset(&mut self.seq.ser.0, key_offset)?;
        }

        // And now the offset of the array element end (which is encoded later)
        if let Some(ref mut offsets) = self.seq.offsets {
            let offset = self.seq.ser.0.bytes_written - self.seq.start;

            offsets.push(offset);
        }

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
