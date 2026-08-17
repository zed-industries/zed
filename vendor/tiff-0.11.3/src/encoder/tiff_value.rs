use std::{borrow::Cow, io::Write, slice::from_ref};

use crate::{bytecast, tags, tags::Type, TiffError, TiffFormatError, TiffResult};

use super::writer::TiffWriter;

/// Trait for types that can be encoded in a tiff file
#[diagnostic::on_unimplemented(
    message = "the type `{Self}` does not implement `TiffValue`",
    note = "the trait is implemented for primitive types (`u8`, `i16`, `f32`, etc.)",
    note = "the trait is implemented for shared references to values",
    note = "the trait is implemented for slices, pass them by reference (e.g. `&[u8]`)",
    note = "the trait is implemented for arrays if it is implemented for a slice",
    note = "values in a `Vec` or `Box` should be dereferenced, e.g. `&vec[..]`"
)]
pub trait TiffValue {
    const BYTE_LEN: u8;
    const FIELD_TYPE: Type;

    // FIXME: If we want this to mean 'count' in the sense of `Entry` it should return `u32` or
    // `u64`, probably the latter with `convert_offset` taking care of the check against `u32::MAX`
    // for classic tiff.
    fn count(&self) -> usize;

    fn bytes(&self) -> usize {
        self.count() * usize::from(Self::BYTE_LEN)
    }

    /// Access this value as an contiguous sequence of bytes.
    /// If their is no trivial representation, allocate it on the heap.
    fn data(&self) -> Cow<'_, [u8]>;

    /// Write this value to a TiffWriter.
    /// While the default implementation will work in all cases, it may require unnecessary allocations.
    /// The written bytes of any custom implementation MUST be the same as yielded by `self.data()`.
    fn write<W: Write>(&self, writer: &mut TiffWriter<W>) -> TiffResult<()> {
        writer.write_bytes(&self.data())?;
        Ok(())
    }
}

impl TiffValue for [u8] {
    const BYTE_LEN: u8 = 1;
    const FIELD_TYPE: Type = Type::BYTE;

    fn count(&self) -> usize {
        self.len()
    }

    fn data(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(self)
    }
}

impl TiffValue for [i8] {
    const BYTE_LEN: u8 = 1;
    const FIELD_TYPE: Type = Type::SBYTE;

    fn count(&self) -> usize {
        self.len()
    }

    fn data(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(bytecast::i8_as_ne_bytes(self))
    }
}

impl TiffValue for [u16] {
    const BYTE_LEN: u8 = 2;
    const FIELD_TYPE: Type = Type::SHORT;

    fn count(&self) -> usize {
        self.len()
    }

    fn data(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(bytecast::u16_as_ne_bytes(self))
    }
}

impl TiffValue for [i16] {
    const BYTE_LEN: u8 = 2;
    const FIELD_TYPE: Type = Type::SSHORT;

    fn count(&self) -> usize {
        self.len()
    }

    fn data(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(bytecast::i16_as_ne_bytes(self))
    }
}

impl TiffValue for [u32] {
    const BYTE_LEN: u8 = 4;
    const FIELD_TYPE: Type = Type::LONG;

    fn count(&self) -> usize {
        self.len()
    }

    fn data(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(bytecast::u32_as_ne_bytes(self))
    }
}

impl TiffValue for [i32] {
    const BYTE_LEN: u8 = 4;
    const FIELD_TYPE: Type = Type::SLONG;

    fn count(&self) -> usize {
        self.len()
    }

    fn data(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(bytecast::i32_as_ne_bytes(self))
    }
}

impl TiffValue for [u64] {
    const BYTE_LEN: u8 = 8;
    const FIELD_TYPE: Type = Type::LONG8;

    fn count(&self) -> usize {
        self.len()
    }

    fn data(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(bytecast::u64_as_ne_bytes(self))
    }
}

impl TiffValue for [i64] {
    const BYTE_LEN: u8 = 8;
    const FIELD_TYPE: Type = Type::SLONG8;

    fn count(&self) -> usize {
        self.len()
    }

    fn data(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(bytecast::i64_as_ne_bytes(self))
    }
}

impl TiffValue for [f32] {
    const BYTE_LEN: u8 = 4;
    const FIELD_TYPE: Type = Type::FLOAT;

    fn count(&self) -> usize {
        self.len()
    }

    fn data(&self) -> Cow<'_, [u8]> {
        // We write using native endian so this should be safe
        Cow::Borrowed(bytecast::f32_as_ne_bytes(self))
    }
}

impl TiffValue for [f64] {
    const BYTE_LEN: u8 = 8;
    const FIELD_TYPE: Type = Type::DOUBLE;

    fn count(&self) -> usize {
        self.len()
    }

    fn data(&self) -> Cow<'_, [u8]> {
        // We write using native endian so this should be safe
        Cow::Borrowed(bytecast::f64_as_ne_bytes(self))
    }
}

impl TiffValue for u8 {
    const BYTE_LEN: u8 = 1;
    const FIELD_TYPE: Type = Type::BYTE;

    fn count(&self) -> usize {
        1
    }

    fn write<W: Write>(&self, writer: &mut TiffWriter<W>) -> TiffResult<()> {
        writer.write_u8(*self)?;
        Ok(())
    }

    fn data(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(from_ref(self))
    }
}

impl TiffValue for i8 {
    const BYTE_LEN: u8 = 1;
    const FIELD_TYPE: Type = Type::SBYTE;

    fn count(&self) -> usize {
        1
    }

    fn write<W: Write>(&self, writer: &mut TiffWriter<W>) -> TiffResult<()> {
        writer.write_i8(*self)?;
        Ok(())
    }

    fn data(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(bytecast::i8_as_ne_bytes(from_ref(self)))
    }
}

impl TiffValue for u16 {
    const BYTE_LEN: u8 = 2;
    const FIELD_TYPE: Type = Type::SHORT;

    fn count(&self) -> usize {
        1
    }

    fn write<W: Write>(&self, writer: &mut TiffWriter<W>) -> TiffResult<()> {
        writer.write_u16(*self)?;
        Ok(())
    }

    fn data(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(bytecast::u16_as_ne_bytes(from_ref(self)))
    }
}

impl TiffValue for i16 {
    const BYTE_LEN: u8 = 2;
    const FIELD_TYPE: Type = Type::SSHORT;

    fn count(&self) -> usize {
        1
    }

    fn write<W: Write>(&self, writer: &mut TiffWriter<W>) -> TiffResult<()> {
        writer.write_i16(*self)?;
        Ok(())
    }

    fn data(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(bytecast::i16_as_ne_bytes(from_ref(self)))
    }
}

impl TiffValue for u32 {
    const BYTE_LEN: u8 = 4;
    const FIELD_TYPE: Type = Type::LONG;

    fn count(&self) -> usize {
        1
    }

    fn write<W: Write>(&self, writer: &mut TiffWriter<W>) -> TiffResult<()> {
        writer.write_u32(*self)?;
        Ok(())
    }

    fn data(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(bytecast::u32_as_ne_bytes(from_ref(self)))
    }
}

impl TiffValue for i32 {
    const BYTE_LEN: u8 = 4;
    const FIELD_TYPE: Type = Type::SLONG;

    fn count(&self) -> usize {
        1
    }

    fn write<W: Write>(&self, writer: &mut TiffWriter<W>) -> TiffResult<()> {
        writer.write_i32(*self)?;
        Ok(())
    }

    fn data(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(bytecast::i32_as_ne_bytes(from_ref(self)))
    }
}

impl TiffValue for u64 {
    const BYTE_LEN: u8 = 8;
    const FIELD_TYPE: Type = Type::LONG8;

    fn count(&self) -> usize {
        1
    }

    fn write<W: Write>(&self, writer: &mut TiffWriter<W>) -> TiffResult<()> {
        writer.write_u64(*self)?;
        Ok(())
    }

    fn data(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(bytecast::u64_as_ne_bytes(from_ref(self)))
    }
}

impl TiffValue for i64 {
    const BYTE_LEN: u8 = 8;
    const FIELD_TYPE: Type = Type::SLONG8;

    fn count(&self) -> usize {
        1
    }

    fn write<W: Write>(&self, writer: &mut TiffWriter<W>) -> TiffResult<()> {
        writer.write_i64(*self)?;
        Ok(())
    }

    fn data(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(bytecast::i64_as_ne_bytes(from_ref(self)))
    }
}

impl TiffValue for f32 {
    const BYTE_LEN: u8 = 4;
    const FIELD_TYPE: Type = Type::FLOAT;

    fn count(&self) -> usize {
        1
    }

    fn write<W: Write>(&self, writer: &mut TiffWriter<W>) -> TiffResult<()> {
        writer.write_f32(*self)?;
        Ok(())
    }

    fn data(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(bytecast::f32_as_ne_bytes(from_ref(self)))
    }
}

impl TiffValue for f64 {
    const BYTE_LEN: u8 = 8;
    const FIELD_TYPE: Type = Type::DOUBLE;

    fn count(&self) -> usize {
        1
    }

    fn write<W: Write>(&self, writer: &mut TiffWriter<W>) -> TiffResult<()> {
        writer.write_f64(*self)?;
        Ok(())
    }

    fn data(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(bytecast::f64_as_ne_bytes(from_ref(self)))
    }
}

impl TiffValue for Ifd {
    const BYTE_LEN: u8 = 4;
    const FIELD_TYPE: Type = Type::IFD;

    fn count(&self) -> usize {
        1
    }

    fn write<W: Write>(&self, writer: &mut TiffWriter<W>) -> TiffResult<()> {
        writer.write_u32(self.0)?;
        Ok(())
    }

    fn data(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(bytecast::u32_as_ne_bytes(from_ref(&self.0)))
    }
}

impl TiffValue for Ifd8 {
    const BYTE_LEN: u8 = 8;
    const FIELD_TYPE: Type = Type::IFD8;

    fn count(&self) -> usize {
        1
    }

    fn write<W: Write>(&self, writer: &mut TiffWriter<W>) -> TiffResult<()> {
        writer.write_u64(self.0)?;
        Ok(())
    }

    fn data(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(bytecast::u64_as_ne_bytes(from_ref(&self.0)))
    }
}

impl TiffValue for Rational {
    const BYTE_LEN: u8 = 8;
    const FIELD_TYPE: Type = Type::RATIONAL;

    fn count(&self) -> usize {
        1
    }

    fn write<W: Write>(&self, writer: &mut TiffWriter<W>) -> TiffResult<()> {
        writer.write_u32(self.n)?;
        writer.write_u32(self.d)?;
        Ok(())
    }

    fn data(&self) -> Cow<'_, [u8]> {
        Cow::Owned({
            let first_dword = bytecast::u32_as_ne_bytes(from_ref(&self.n));
            let second_dword = bytecast::u32_as_ne_bytes(from_ref(&self.d));
            [first_dword, second_dword].concat()
        })
    }
}

impl TiffValue for SRational {
    const BYTE_LEN: u8 = 8;
    const FIELD_TYPE: Type = Type::SRATIONAL;

    fn count(&self) -> usize {
        1
    }

    fn write<W: Write>(&self, writer: &mut TiffWriter<W>) -> TiffResult<()> {
        writer.write_i32(self.n)?;
        writer.write_i32(self.d)?;
        Ok(())
    }

    fn data(&self) -> Cow<'_, [u8]> {
        Cow::Owned({
            let first_dword = bytecast::i32_as_ne_bytes(from_ref(&self.n));
            let second_dword = bytecast::i32_as_ne_bytes(from_ref(&self.d));
            [first_dword, second_dword].concat()
        })
    }
}

impl TiffValue for str {
    const BYTE_LEN: u8 = 1;
    const FIELD_TYPE: Type = Type::ASCII;

    fn count(&self) -> usize {
        self.len() + 1
    }

    fn write<W: Write>(&self, writer: &mut TiffWriter<W>) -> TiffResult<()> {
        if self.is_ascii() && !self.bytes().any(|b| b == 0) {
            writer.write_bytes(self.as_bytes())?;
            writer.write_u8(0)?;
            Ok(())
        } else {
            Err(TiffError::FormatError(TiffFormatError::InvalidTag))
        }
    }

    fn data(&self) -> Cow<'_, [u8]> {
        Cow::Owned({
            if self.is_ascii() && !self.bytes().any(|b| b == 0) {
                let bytes: &[u8] = self.as_bytes();
                [bytes, &[0]].concat()
            } else {
                vec![]
            }
        })
    }
}

// FIXME: held up on reading the discriminant for all but the unknown variant.
//
// It needs to have `repr(u16)` to have a defined layout and then we may read the tag from its
// value representation by casting a u16 pointer to a pointer to the enum type.
//     unsafe { *<*const _>::from(self).cast::<u16>() }
// But that is quite unsafe and needs careful review to maintain the safety requirements. Maybe we
// could get `bytemuck` to add a trait for this (it has `TransparentWrapper`) with a derive for
// enum types that have an explicit repr.
//
// This would allow returning borrowed data in more cases. For all types without an unknown, in all
// cases including as slices, and for all types with and unknown variant when we have a single
// value (either returning a borrowed discriminant or a borrowed value within `Unknown`).
impl TiffValue for tags::CompressionMethod {
    const BYTE_LEN: u8 = <u16 as TiffValue>::BYTE_LEN;
    const FIELD_TYPE: Type = Type::SHORT;

    fn count(&self) -> usize {
        1
    }

    fn data(&self) -> Cow<'_, [u8]> {
        let bytes = self.to_u16().to_ne_bytes();
        Cow::Owned(bytes.to_vec())
    }
}

impl TiffValue for tags::PhotometricInterpretation {
    const BYTE_LEN: u8 = <u16 as TiffValue>::BYTE_LEN;
    const FIELD_TYPE: Type = Type::SHORT;

    fn count(&self) -> usize {
        1
    }

    fn data(&self) -> Cow<'_, [u8]> {
        self.to_u16().to_ne_bytes().to_vec().into()
    }
}

impl TiffValue for tags::PlanarConfiguration {
    const BYTE_LEN: u8 = 2;
    const FIELD_TYPE: Type = Type::SHORT;

    fn count(&self) -> usize {
        1
    }

    fn data(&self) -> Cow<'_, [u8]> {
        self.to_u16().to_ne_bytes().to_vec().into()
    }
}

impl TiffValue for tags::Predictor {
    const BYTE_LEN: u8 = <u16 as TiffValue>::BYTE_LEN;
    const FIELD_TYPE: Type = Type::SHORT;

    fn count(&self) -> usize {
        1
    }

    fn data(&self) -> Cow<'_, [u8]> {
        self.to_u16().to_ne_bytes().to_vec().into()
    }
}

impl TiffValue for tags::ResolutionUnit {
    const BYTE_LEN: u8 = <u16 as TiffValue>::BYTE_LEN;
    const FIELD_TYPE: Type = Type::SHORT;

    fn count(&self) -> usize {
        1
    }

    fn data(&self) -> Cow<'_, [u8]> {
        self.to_u16().to_ne_bytes().to_vec().into()
    }
}

/// This is implemented for slices as the `SampleFormat` tag takes a count of `N`.
///
/// Use `core::slice::from_ref` if you really need to write a single element.
///
/// See: <https://web.archive.org/web/20191120220815/https://www.awaresystems.be/imaging/tiff/tifftags/sampleformat.html>
impl TiffValue for [tags::SampleFormat] {
    const BYTE_LEN: u8 = <u16 as TiffValue>::BYTE_LEN;
    const FIELD_TYPE: Type = Type::SHORT;

    fn count(&self) -> usize {
        self.len()
    }

    fn data(&self) -> Cow<'_, [u8]> {
        let mut buf: Vec<u8> = Vec::with_capacity(self.len() * 2);
        for x in self {
            buf.extend_from_slice(&x.to_u16().to_ne_bytes());
        }
        Cow::Owned(buf)
    }
}

/// This is implemented for slices as the `ExtraSamples` tag takes a count of `N`.
///
/// Use `core::slice::from_ref` if you really need to write a single element.
///
/// See: <https://web.archive.org/web/20191120220815/https://www.awaresystems.be/imaging/tiff/tifftags/extrasamples.html>
impl TiffValue for [tags::ExtraSamples] {
    const BYTE_LEN: u8 = <u16 as TiffValue>::BYTE_LEN;
    const FIELD_TYPE: Type = Type::SHORT;

    fn count(&self) -> usize {
        self.len()
    }

    fn data(&self) -> Cow<'_, [u8]> {
        let mut buf: Vec<u8> = Vec::with_capacity(self.len() * 2);
        for x in self {
            buf.extend_from_slice(&x.to_u16().to_ne_bytes());
        }
        Cow::Owned(buf)
    }
}

// If you pass `&Vec<_>` then you'd get at first sight the  complaint:
//
//  `Vec<T>` does not implement `TiffValue`
//
// and a list of implementations of `TiffValue` that are quite unrelated. That is not very helpful.
// We do not *want* `Vec` to implement the trait as an owning type, instead you should pass a
// reference to a slice. The error message is further customized in the diagnostics of the trait.
#[diagnostic::do_not_recommend]
impl<T: TiffValue + ?Sized> TiffValue for &'_ T {
    const BYTE_LEN: u8 = T::BYTE_LEN;
    const FIELD_TYPE: Type = T::FIELD_TYPE;

    fn count(&self) -> usize {
        (*self).count()
    }

    fn write<W: Write>(&self, writer: &mut TiffWriter<W>) -> TiffResult<()> {
        (*self).write(writer)
    }

    fn data(&self) -> Cow<'_, [u8]> {
        T::data(self)
    }
}

/// An array is treated like a slice of the same length.
impl<T, const N: usize> TiffValue for [T; N]
where
    [T]: TiffValue,
{
    const BYTE_LEN: u8 = <[T]>::BYTE_LEN;
    const FIELD_TYPE: Type = <[T]>::FIELD_TYPE;

    fn count(&self) -> usize {
        self.as_slice().count()
    }

    fn write<W: Write>(&self, writer: &mut TiffWriter<W>) -> TiffResult<()> {
        self.as_slice().write(writer)
    }

    fn data(&self) -> Cow<'_, [u8]> {
        self.as_slice().data()
    }
}

macro_rules! impl_tiff_value_for_contiguous_sequence {
    ($inner_type:ty; $bytes:expr; $field_type:expr) => {
        impl $crate::encoder::TiffValue for [$inner_type] {
            const BYTE_LEN: u8 = $bytes;
            const FIELD_TYPE: Type = $field_type;

            fn count(&self) -> usize {
                self.len()
            }

            fn write<W: Write>(&self, writer: &mut TiffWriter<W>) -> TiffResult<()> {
                for x in self {
                    x.write(writer)?;
                }
                Ok(())
            }

            fn data(&self) -> Cow<'_, [u8]> {
                let mut buf: Vec<u8> = Vec::with_capacity(Self::BYTE_LEN as usize * self.len());
                for x in self {
                    buf.extend_from_slice(&x.data());
                }
                Cow::Owned(buf)
            }
        }
    };
}

impl_tiff_value_for_contiguous_sequence!(Ifd; 4; Type::IFD);
impl_tiff_value_for_contiguous_sequence!(Ifd8; 8; Type::IFD8);
impl_tiff_value_for_contiguous_sequence!(Rational; 8; Type::RATIONAL);
impl_tiff_value_for_contiguous_sequence!(SRational; 8; Type::SRATIONAL);

/// Type to represent tiff values of type `IFD`
#[derive(Clone)]
pub struct Ifd(pub u32);

/// Type to represent tiff values of type `IFD8`
#[derive(Clone)]
pub struct Ifd8(pub u64);

/// Type to represent tiff values of type `RATIONAL`
#[derive(Clone)]
pub struct Rational {
    pub n: u32,
    pub d: u32,
}

/// Type to represent tiff values of type `SRATIONAL`
#[derive(Clone)]
pub struct SRational {
    pub n: i32,
    pub d: i32,
}
