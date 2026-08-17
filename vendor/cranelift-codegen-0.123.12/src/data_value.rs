//! This module gives users to instantiate values that Cranelift understands. These values are used,
//! for example, during interpretation and for wrapping immediates.
use crate::ir::immediates::{Ieee16, Ieee32, Ieee64, Ieee128, Offset32};
use crate::ir::{ConstantData, Type, types};
use core::cmp::Ordering;
use core::fmt::{self, Display, Formatter};

/// Represent a data value. Where [Value] is an SSA reference, [DataValue] is the type + value
/// that would be referred to by a [Value].
///
/// [Value]: crate::ir::Value
#[expect(missing_docs, reason = "self-describing variants")]
#[derive(Clone, Debug, PartialOrd)]
pub enum DataValue {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    F16(Ieee16),
    F32(Ieee32),
    F64(Ieee64),
    F128(Ieee128),
    V128([u8; 16]),
    V64([u8; 8]),
    V32([u8; 4]),
    V16([u8; 2]),
}

impl PartialEq for DataValue {
    fn eq(&self, other: &Self) -> bool {
        use DataValue::*;
        match (self, other) {
            (I8(l), I8(r)) => l == r,
            (I8(_), _) => false,
            (I16(l), I16(r)) => l == r,
            (I16(_), _) => false,
            (I32(l), I32(r)) => l == r,
            (I32(_), _) => false,
            (I64(l), I64(r)) => l == r,
            (I64(_), _) => false,
            (I128(l), I128(r)) => l == r,
            (I128(_), _) => false,
            (F16(l), F16(r)) => l.partial_cmp(&r) == Some(Ordering::Equal),
            (F16(_), _) => false,
            (F32(l), F32(r)) => l.as_f32() == r.as_f32(),
            (F32(_), _) => false,
            (F64(l), F64(r)) => l.as_f64() == r.as_f64(),
            (F64(_), _) => false,
            (F128(l), F128(r)) => l.partial_cmp(&r) == Some(Ordering::Equal),
            (F128(_), _) => false,
            (V128(l), V128(r)) => l == r,
            (V128(_), _) => false,
            (V64(l), V64(r)) => l == r,
            (V64(_), _) => false,
            (V32(l), V32(r)) => l == r,
            (V32(_), _) => false,
            (V16(l), V16(r)) => l == r,
            (V16(_), _) => false,
        }
    }
}

impl DataValue {
    /// Try to cast an immediate integer (a wrapped `i64` on most Cranelift instructions) to the
    /// given Cranelift [Type].
    pub fn from_integer(imm: i128, ty: Type) -> Result<DataValue, DataValueCastFailure> {
        match ty {
            types::I8 => Ok(DataValue::I8(imm as i8)),
            types::I16 => Ok(DataValue::I16(imm as i16)),
            types::I32 => Ok(DataValue::I32(imm as i32)),
            types::I64 => Ok(DataValue::I64(imm as i64)),
            types::I128 => Ok(DataValue::I128(imm)),
            _ => Err(DataValueCastFailure::FromInteger(imm, ty)),
        }
    }

    /// Return the Cranelift IR [Type] for this [DataValue].
    pub fn ty(&self) -> Type {
        match self {
            DataValue::I8(_) => types::I8,
            DataValue::I16(_) => types::I16,
            DataValue::I32(_) => types::I32,
            DataValue::I64(_) => types::I64,
            DataValue::I128(_) => types::I128,
            DataValue::F16(_) => types::F16,
            DataValue::F32(_) => types::F32,
            DataValue::F64(_) => types::F64,
            DataValue::F128(_) => types::F128,
            DataValue::V128(_) => types::I8X16, // A default type.
            DataValue::V64(_) => types::I8X8,   // A default type.
            DataValue::V32(_) => types::I8X4,   // A default type.
            DataValue::V16(_) => types::I8X2,   // A default type.
        }
    }

    /// Return true if the value is a vector (i.e. `DataValue::V128`).
    pub fn is_vector(&self) -> bool {
        match self {
            DataValue::V128(_) | DataValue::V64(_) | DataValue::V32(_) | DataValue::V16(_) => true,
            _ => false,
        }
    }

    fn swap_bytes(self) -> Self {
        match self {
            DataValue::I8(i) => DataValue::I8(i.swap_bytes()),
            DataValue::I16(i) => DataValue::I16(i.swap_bytes()),
            DataValue::I32(i) => DataValue::I32(i.swap_bytes()),
            DataValue::I64(i) => DataValue::I64(i.swap_bytes()),
            DataValue::I128(i) => DataValue::I128(i.swap_bytes()),
            DataValue::F16(f) => DataValue::F16(Ieee16::with_bits(f.bits().swap_bytes())),
            DataValue::F32(f) => DataValue::F32(Ieee32::with_bits(f.bits().swap_bytes())),
            DataValue::F64(f) => DataValue::F64(Ieee64::with_bits(f.bits().swap_bytes())),
            DataValue::F128(f) => DataValue::F128(Ieee128::with_bits(f.bits().swap_bytes())),
            DataValue::V128(mut v) => {
                v.reverse();
                DataValue::V128(v)
            }
            DataValue::V64(mut v) => {
                v.reverse();
                DataValue::V64(v)
            }
            DataValue::V32(mut v) => {
                v.reverse();
                DataValue::V32(v)
            }
            DataValue::V16(mut v) => {
                v.reverse();
                DataValue::V16(v)
            }
        }
    }

    /// Converts `self` to big endian from target's endianness.
    pub fn to_be(self) -> Self {
        if cfg!(target_endian = "big") {
            self
        } else {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from target's endianness.
    pub fn to_le(self) -> Self {
        if cfg!(target_endian = "little") {
            self
        } else {
            self.swap_bytes()
        }
    }

    /// Write a [DataValue] to a slice in native-endian byte order.
    ///
    /// # Panics:
    ///
    /// Panics if the slice does not have enough space to accommodate the [DataValue]
    pub fn write_to_slice_ne(&self, dst: &mut [u8]) {
        match self {
            DataValue::I8(i) => dst[..1].copy_from_slice(&i.to_ne_bytes()[..]),
            DataValue::I16(i) => dst[..2].copy_from_slice(&i.to_ne_bytes()[..]),
            DataValue::I32(i) => dst[..4].copy_from_slice(&i.to_ne_bytes()[..]),
            DataValue::I64(i) => dst[..8].copy_from_slice(&i.to_ne_bytes()[..]),
            DataValue::I128(i) => dst[..16].copy_from_slice(&i.to_ne_bytes()[..]),
            DataValue::F16(f) => dst[..2].copy_from_slice(&f.bits().to_ne_bytes()[..]),
            DataValue::F32(f) => dst[..4].copy_from_slice(&f.bits().to_ne_bytes()[..]),
            DataValue::F64(f) => dst[..8].copy_from_slice(&f.bits().to_ne_bytes()[..]),
            DataValue::F128(f) => dst[..16].copy_from_slice(&f.bits().to_ne_bytes()[..]),
            DataValue::V128(v) => dst[..16].copy_from_slice(&v[..]),
            DataValue::V64(v) => dst[..8].copy_from_slice(&v[..]),
            DataValue::V32(v) => dst[..4].copy_from_slice(&v[..]),
            DataValue::V16(v) => dst[..2].copy_from_slice(&v[..]),
        };
    }

    /// Write a [DataValue] to a slice in big-endian byte order.
    ///
    /// # Panics:
    ///
    /// Panics if the slice does not have enough space to accommodate the [DataValue]
    pub fn write_to_slice_be(&self, dst: &mut [u8]) {
        self.clone().to_be().write_to_slice_ne(dst);
    }

    /// Write a [DataValue] to a slice in little-endian byte order.
    ///
    /// # Panics:
    ///
    /// Panics if the slice does not have enough space to accommodate the [DataValue]
    pub fn write_to_slice_le(&self, dst: &mut [u8]) {
        self.clone().to_le().write_to_slice_ne(dst);
    }

    /// Read a [DataValue] from a slice using a given [Type] with native-endian byte order.
    ///
    /// # Panics:
    ///
    /// Panics if the slice does not have enough space to accommodate the [DataValue]
    pub fn read_from_slice_ne(src: &[u8], ty: Type) -> Self {
        match ty {
            types::I8 => DataValue::I8(i8::from_ne_bytes(src[..1].try_into().unwrap())),
            types::I16 => DataValue::I16(i16::from_ne_bytes(src[..2].try_into().unwrap())),
            types::I32 => DataValue::I32(i32::from_ne_bytes(src[..4].try_into().unwrap())),
            types::I64 => DataValue::I64(i64::from_ne_bytes(src[..8].try_into().unwrap())),
            types::I128 => DataValue::I128(i128::from_ne_bytes(src[..16].try_into().unwrap())),
            types::F16 => DataValue::F16(Ieee16::with_bits(u16::from_ne_bytes(
                src[..2].try_into().unwrap(),
            ))),
            types::F32 => DataValue::F32(Ieee32::with_bits(u32::from_ne_bytes(
                src[..4].try_into().unwrap(),
            ))),
            types::F64 => DataValue::F64(Ieee64::with_bits(u64::from_ne_bytes(
                src[..8].try_into().unwrap(),
            ))),
            types::F128 => DataValue::F128(Ieee128::with_bits(u128::from_ne_bytes(
                src[..16].try_into().unwrap(),
            ))),
            _ if ty.is_vector() => match ty.bytes() {
                16 => DataValue::V128(src[..16].try_into().unwrap()),
                8 => DataValue::V64(src[..8].try_into().unwrap()),
                4 => DataValue::V32(src[..4].try_into().unwrap()),
                2 => DataValue::V16(src[..2].try_into().unwrap()),
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }

    /// Read a [DataValue] from a slice using a given [Type] in big-endian byte order.
    ///
    /// # Panics:
    ///
    /// Panics if the slice does not have enough space to accommodate the [DataValue]
    pub fn read_from_slice_be(src: &[u8], ty: Type) -> Self {
        DataValue::read_from_slice_ne(src, ty).to_be()
    }

    /// Read a [DataValue] from a slice using a given [Type] in little-endian byte order.
    ///
    /// # Panics:
    ///
    /// Panics if the slice does not have enough space to accommodate the [DataValue]
    pub fn read_from_slice_le(src: &[u8], ty: Type) -> Self {
        DataValue::read_from_slice_ne(src, ty).to_le()
    }

    /// Write a [DataValue] to a memory location in native-endian byte order.
    pub unsafe fn write_value_to(&self, p: *mut u128) {
        let size = self.ty().bytes() as usize;
        self.write_to_slice_ne(unsafe { std::slice::from_raw_parts_mut(p as *mut u8, size) });
    }

    /// Read a [DataValue] from a memory location using a given [Type] in native-endian byte order.
    pub unsafe fn read_value_from(p: *const u128, ty: Type) -> Self {
        DataValue::read_from_slice_ne(
            unsafe { std::slice::from_raw_parts(p as *const u8, ty.bytes() as usize) },
            ty,
        )
    }

    /// Performs a bitwise comparison over the contents of [DataValue].
    ///
    /// Returns true if all bits are equal.
    ///
    /// This behaviour is different from PartialEq for NaN floats.
    pub fn bitwise_eq(&self, other: &DataValue) -> bool {
        match (self, other) {
            // We need to bit compare the floats to ensure that we produce the correct values
            // on NaN's. The test suite expects to assert the precise bit pattern on NaN's or
            // works around it in the tests themselves.
            (DataValue::F16(a), DataValue::F16(b)) => a.bits() == b.bits(),
            (DataValue::F32(a), DataValue::F32(b)) => a.bits() == b.bits(),
            (DataValue::F64(a), DataValue::F64(b)) => a.bits() == b.bits(),
            (DataValue::F128(a), DataValue::F128(b)) => a.bits() == b.bits(),

            // We don't need to worry about F32x4 / F64x2 Since we compare V128 which is already the
            // raw bytes anyway
            (a, b) => a == b,
        }
    }
}

/// Record failures to cast [DataValue].
#[derive(Debug, PartialEq)]
#[expect(missing_docs, reason = "self-describing variants")]
pub enum DataValueCastFailure {
    TryInto(Type, Type),
    FromInteger(i128, Type),
}

// This is manually implementing Error and Display instead of using thiserror to reduce the amount
// of dependencies used by Cranelift.
impl std::error::Error for DataValueCastFailure {}

impl Display for DataValueCastFailure {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            DataValueCastFailure::TryInto(from, to) => {
                write!(f, "unable to cast data value of type {from} to type {to}")
            }
            DataValueCastFailure::FromInteger(val, to) => {
                write!(f, "unable to cast i64({val}) to a data value of type {to}")
            }
        }
    }
}

/// Helper for creating conversion implementations for [DataValue].
macro_rules! build_conversion_impl {
    ( $rust_ty:ty, $data_value_ty:ident, $cranelift_ty:ident ) => {
        impl From<$rust_ty> for DataValue {
            fn from(data: $rust_ty) -> Self {
                DataValue::$data_value_ty(data)
            }
        }

        impl TryInto<$rust_ty> for DataValue {
            type Error = DataValueCastFailure;
            fn try_into(self) -> Result<$rust_ty, Self::Error> {
                if let DataValue::$data_value_ty(v) = self {
                    Ok(v)
                } else {
                    Err(DataValueCastFailure::TryInto(
                        self.ty(),
                        types::$cranelift_ty,
                    ))
                }
            }
        }
    };
}
build_conversion_impl!(i8, I8, I8);
build_conversion_impl!(i16, I16, I16);
build_conversion_impl!(i32, I32, I32);
build_conversion_impl!(i64, I64, I64);
build_conversion_impl!(i128, I128, I128);
build_conversion_impl!(Ieee16, F16, F16);
build_conversion_impl!(Ieee32, F32, F32);
build_conversion_impl!(Ieee64, F64, F64);
build_conversion_impl!(Ieee128, F128, F128);
build_conversion_impl!([u8; 16], V128, I8X16);
build_conversion_impl!([u8; 8], V64, I8X8);
build_conversion_impl!([u8; 4], V32, I8X4);
build_conversion_impl!([u8; 2], V16, I8X2);
impl From<Offset32> for DataValue {
    fn from(o: Offset32) -> Self {
        DataValue::from(Into::<i32>::into(o))
    }
}

impl Display for DataValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DataValue::I8(dv) => write!(f, "{dv}"),
            DataValue::I16(dv) => write!(f, "{dv}"),
            DataValue::I32(dv) => write!(f, "{dv}"),
            DataValue::I64(dv) => write!(f, "{dv}"),
            DataValue::I128(dv) => write!(f, "{dv}"),
            // The Ieee* wrappers here print the expected syntax.
            DataValue::F16(dv) => write!(f, "{dv}"),
            DataValue::F32(dv) => write!(f, "{dv}"),
            DataValue::F64(dv) => write!(f, "{dv}"),
            DataValue::F128(dv) => write!(f, "{dv}"),
            // Again, for syntax consistency, use ConstantData, which in this case displays as hex.
            DataValue::V128(dv) => write!(f, "{}", ConstantData::from(&dv[..])),
            DataValue::V64(dv) => write!(f, "{}", ConstantData::from(&dv[..])),
            DataValue::V32(dv) => write!(f, "{}", ConstantData::from(&dv[..])),
            DataValue::V16(dv) => write!(f, "{}", ConstantData::from(&dv[..])),
        }
    }
}

/// Helper structure for printing bracket-enclosed vectors of [DataValue]s.
/// - for empty vectors, display `[]`
/// - for single item vectors, display `42`, e.g.
/// - for multiple item vectors, display `[42, 43, 44]`, e.g.
pub struct DisplayDataValues<'a>(pub &'a [DataValue]);

impl<'a> Display for DisplayDataValues<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.0.len() == 1 {
            write!(f, "{}", self.0[0])
        } else {
            write!(f, "[")?;
            write_data_value_list(f, &self.0)?;
            write!(f, "]")
        }
    }
}

/// Helper function for displaying `Vec<DataValue>`.
pub fn write_data_value_list(f: &mut Formatter<'_>, list: &[DataValue]) -> fmt::Result {
    match list.len() {
        0 => Ok(()),
        1 => write!(f, "{}", list[0]),
        _ => {
            write!(f, "{}", list[0])?;
            for dv in list.iter().skip(1) {
                write!(f, ", {dv}")?;
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn type_conversions() {
        assert_eq!(DataValue::V128([0; 16]).ty(), types::I8X16);
        assert_eq!(
            TryInto::<[u8; 16]>::try_into(DataValue::V128([0; 16])).unwrap(),
            [0; 16]
        );
        assert_eq!(
            TryInto::<i32>::try_into(DataValue::V128([0; 16])).unwrap_err(),
            DataValueCastFailure::TryInto(types::I8X16, types::I32)
        );
    }
}
