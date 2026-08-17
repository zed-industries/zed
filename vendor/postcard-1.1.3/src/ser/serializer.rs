use serde::{ser, Serialize};

use crate::error::{Error, Result};
use crate::ser::flavors::Flavor;
use crate::varint::*;

/// A `serde` compatible serializer, generic over "Flavors" of serializing plugins.
///
/// It should rarely be necessary to directly use this type unless you are implementing your
/// own [`SerFlavor`].
///
/// See the docs for [`SerFlavor`] for more information about "flavors" of serialization
///
/// [`SerFlavor`]: crate::ser_flavors::Flavor
pub struct Serializer<F>
where
    F: Flavor,
{
    /// This is the Flavor(s) that will be used to modify or store any bytes generated
    /// by serialization
    pub output: F,
}

impl<F: Flavor> Serializer<F> {
    /// Attempt to push a variably encoded [usize] into the output data stream
    #[inline]
    pub(crate) fn try_push_varint_usize(&mut self, data: usize) -> Result<()> {
        let mut buf = [0u8; varint_max::<usize>()];
        let used_buf = varint_usize(data, &mut buf);
        self.output.try_extend(used_buf)
    }

    /// Attempt to push a variably encoded [u128] into the output data stream
    #[inline]
    pub(crate) fn try_push_varint_u128(&mut self, data: u128) -> Result<()> {
        let mut buf = [0u8; varint_max::<u128>()];
        let used_buf = varint_u128(data, &mut buf);
        self.output.try_extend(used_buf)
    }

    /// Attempt to push a variably encoded [u64] into the output data stream
    #[inline]
    pub(crate) fn try_push_varint_u64(&mut self, data: u64) -> Result<()> {
        let mut buf = [0u8; varint_max::<u64>()];
        let used_buf = varint_u64(data, &mut buf);
        self.output.try_extend(used_buf)
    }

    /// Attempt to push a variably encoded [u32] into the output data stream
    #[inline]
    pub(crate) fn try_push_varint_u32(&mut self, data: u32) -> Result<()> {
        let mut buf = [0u8; varint_max::<u32>()];
        let used_buf = varint_u32(data, &mut buf);
        self.output.try_extend(used_buf)
    }

    /// Attempt to push a variably encoded [u16] into the output data stream
    #[inline]
    pub(crate) fn try_push_varint_u16(&mut self, data: u16) -> Result<()> {
        let mut buf = [0u8; varint_max::<u16>()];
        let used_buf = varint_u16(data, &mut buf);
        self.output.try_extend(used_buf)
    }
}

impl<F> ser::Serializer for &mut Serializer<F>
where
    F: Flavor,
{
    type Ok = ();

    type Error = Error;

    // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // Serializer struct.
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    #[inline]
    fn is_human_readable(&self) -> bool {
        false
    }

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<()> {
        self.serialize_u8(if v { 1 } else { 0 })
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_u8(v.to_le_bytes()[0])
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<()> {
        let zzv = zig_zag_i16(v);
        self.try_push_varint_u16(zzv)
            .map_err(|_| Error::SerializeBufferFull)
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<()> {
        let zzv = zig_zag_i32(v);
        self.try_push_varint_u32(zzv)
            .map_err(|_| Error::SerializeBufferFull)
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<()> {
        let zzv = zig_zag_i64(v);
        self.try_push_varint_u64(zzv)
            .map_err(|_| Error::SerializeBufferFull)
    }

    #[inline]
    fn serialize_i128(self, v: i128) -> Result<()> {
        let zzv = zig_zag_i128(v);
        self.try_push_varint_u128(zzv)
            .map_err(|_| Error::SerializeBufferFull)
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<()> {
        self.output
            .try_push(v)
            .map_err(|_| Error::SerializeBufferFull)
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<()> {
        self.try_push_varint_u16(v)
            .map_err(|_| Error::SerializeBufferFull)
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<()> {
        self.try_push_varint_u32(v)
            .map_err(|_| Error::SerializeBufferFull)
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<()> {
        self.try_push_varint_u64(v)
            .map_err(|_| Error::SerializeBufferFull)
    }

    #[inline]
    fn serialize_u128(self, v: u128) -> Result<()> {
        self.try_push_varint_u128(v)
            .map_err(|_| Error::SerializeBufferFull)
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<()> {
        let buf = v.to_bits().to_le_bytes();
        self.output
            .try_extend(&buf)
            .map_err(|_| Error::SerializeBufferFull)
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<()> {
        let buf = v.to_bits().to_le_bytes();
        self.output
            .try_extend(&buf)
            .map_err(|_| Error::SerializeBufferFull)
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<()> {
        let mut buf = [0u8; 4];
        let strsl = v.encode_utf8(&mut buf);
        strsl.serialize(self)
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<()> {
        self.try_push_varint_usize(v.len())
            .map_err(|_| Error::SerializeBufferFull)?;
        self.output
            .try_extend(v.as_bytes())
            .map_err(|_| Error::SerializeBufferFull)?;
        Ok(())
    }

    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.try_push_varint_usize(v.len())
            .map_err(|_| Error::SerializeBufferFull)?;
        self.output
            .try_extend(v)
            .map_err(|_| Error::SerializeBufferFull)
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        self.serialize_u8(0)
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_u8(1)?;
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        self.try_push_varint_u32(variant_index)
            .map_err(|_| Error::SerializeBufferFull)
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    #[inline]
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
        self.try_push_varint_u32(variant_index)
            .map_err(|_| Error::SerializeBufferFull)?;
        value.serialize(self)
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.try_push_varint_usize(len.ok_or(Error::SerializeSeqLengthUnknown)?)
            .map_err(|_| Error::SerializeBufferFull)?;
        Ok(self)
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(self)
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.try_push_varint_u32(variant_index)
            .map_err(|_| Error::SerializeBufferFull)?;
        Ok(self)
    }

    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        self.try_push_varint_usize(len.ok_or(Error::SerializeSeqLengthUnknown)?)
            .map_err(|_| Error::SerializeBufferFull)?;
        Ok(self)
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.try_push_varint_u32(variant_index)
            .map_err(|_| Error::SerializeBufferFull)?;
        Ok(self)
    }

    #[inline]
    fn collect_str<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: core::fmt::Display + ?Sized,
    {
        use core::fmt::Write;

        // Unfortunately, we need to know the size of the serialized data before
        // we can place it into the output. In order to do this, we run the formatting
        // of the output data TWICE, the first time to determine the length, the
        // second time to actually format the data
        //
        // There are potentially other ways to do this, such as:
        //
        // * Reserving a fixed max size, such as 5 bytes, for the length field, and
        //     leaving non-canonical trailing zeroes at the end. This would work up
        //     to some reasonable length, but might have some portability vs max size
        //     tradeoffs, e.g. 64KiB if we pick 3 bytes, or 4GiB if we pick 5 bytes
        // * Expose some kind of "memmove" capability to flavors, to allow us to
        //     format into the buffer, then "scoot over" that many times.
        //
        // Despite the current approaches downside in speed, it is likely flexible
        // enough for the rare-ish case where formatting a Debug impl is necessary.
        // This is better than the previous panicking behavior, and can be improved
        // in the future.
        struct CountWriter {
            ct: usize,
        }
        impl Write for CountWriter {
            fn write_str(&mut self, s: &str) -> core::result::Result<(), core::fmt::Error> {
                self.ct += s.len();
                Ok(())
            }
        }

        let mut ctr = CountWriter { ct: 0 };

        // This is the first pass through, where we just count the length of the
        // data that we are given
        write!(&mut ctr, "{value}").map_err(|_| Error::CollectStrError)?;
        let len = ctr.ct;
        self.try_push_varint_usize(len)
            .map_err(|_| Error::SerializeBufferFull)?;

        struct FmtWriter<'a, IF>
        where
            IF: Flavor,
        {
            output: &'a mut IF,
        }
        impl<IF> Write for FmtWriter<'_, IF>
        where
            IF: Flavor,
        {
            fn write_str(&mut self, s: &str) -> core::result::Result<(), core::fmt::Error> {
                self.output
                    .try_extend(s.as_bytes())
                    .map_err(|_| core::fmt::Error)
            }
        }

        // This second pass actually inserts the data.
        let mut fw = FmtWriter {
            output: &mut self.output,
        };
        write!(&mut fw, "{value}").map_err(|_| Error::CollectStrError)?;

        Ok(())
    }
}

impl<F> ser::SerializeSeq for &mut Serializer<F>
where
    F: Flavor,
{
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = Error;

    // Serialize a single element of the sequence.
    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    // Close the sequence.
    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<F> ser::SerializeTuple for &mut Serializer<F>
where
    F: Flavor,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<F> ser::SerializeTupleStruct for &mut Serializer<F>
where
    F: Flavor,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<F> ser::SerializeTupleVariant for &mut Serializer<F>
where
    F: Flavor,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<F> ser::SerializeMap for &mut Serializer<F>
where
    F: Flavor,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)
    }

    #[inline]
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<F> ser::SerializeStruct for &mut Serializer<F>
where
    F: Flavor,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<F> ser::SerializeStructVariant for &mut Serializer<F>
where
    F: Flavor,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

fn zig_zag_i16(n: i16) -> u16 {
    ((n << 1) ^ (n >> 15)) as u16
}

fn zig_zag_i32(n: i32) -> u32 {
    ((n << 1) ^ (n >> 31)) as u32
}

fn zig_zag_i64(n: i64) -> u64 {
    ((n << 1) ^ (n >> 63)) as u64
}

fn zig_zag_i128(n: i128) -> u128 {
    ((n << 1) ^ (n >> 127)) as u128
}
