use crate::prelude::*;
use std::borrow::Cow;

use super::{canonicalize_nan32, canonicalize_nan64, unwrap_val};
use wasm_wave::wasm::{WasmFunc, WasmType, WasmTypeKind, WasmValue, WasmValueError};

impl WasmType for crate::ValType {
    fn kind(&self) -> WasmTypeKind {
        match self {
            Self::I32 => WasmTypeKind::S32,
            Self::I64 => WasmTypeKind::S64,
            Self::F32 => WasmTypeKind::F32,
            Self::F64 => WasmTypeKind::F64,
            Self::V128 => WasmTypeKind::Tuple,

            Self::Ref(_) => WasmTypeKind::Unsupported,
        }
    }

    fn tuple_element_types(&self) -> Box<dyn Iterator<Item = Self> + '_> {
        match *self {
            Self::V128 => {}
            _ => panic!("tuple_element_types called on non-tuple type"),
        }
        Box::new([Self::I64, Self::I64].into_iter())
    }
}

impl WasmValue for crate::Val {
    type Type = crate::ValType;

    fn kind(&self) -> WasmTypeKind {
        match self {
            Self::I32(_) => WasmTypeKind::S32,
            Self::I64(_) => WasmTypeKind::S64,
            Self::F32(_) => WasmTypeKind::F32,
            Self::F64(_) => WasmTypeKind::F64,
            Self::V128(_) => WasmTypeKind::Tuple,
            Self::FuncRef(_) => WasmTypeKind::Unsupported,
            Self::ExternRef(_) => WasmTypeKind::Unsupported,
            Self::AnyRef(_) => WasmTypeKind::Unsupported,
            Self::ExnRef(_) => WasmTypeKind::Unsupported,
        }
    }

    fn make_s32(val: i32) -> Self {
        Self::I32(val)
    }
    fn make_s64(val: i64) -> Self {
        Self::I64(val)
    }
    fn make_f32(val: f32) -> Self {
        let val = canonicalize_nan32(val);
        Self::F32(val.to_bits())
    }
    fn make_f64(val: f64) -> Self {
        let val = canonicalize_nan64(val);
        Self::F64(val.to_bits())
    }
    fn make_tuple(
        ty: &Self::Type,
        vals: impl IntoIterator<Item = Self>,
    ) -> Result<Self, WasmValueError> {
        match *ty {
            Self::Type::V128 => {}
            _ => {
                return Err(WasmValueError::Other(
                    "tuples only used for v128 (v64x2)".to_string(),
                ));
            }
        }
        let [l_val, h_val]: [Self; 2] = vals
            .into_iter()
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| WasmValueError::Other("expected 2 values".to_string()))?;

        let (Some(l), Some(h)) = (l_val.i64(), h_val.i64()) else {
            return Err(WasmValueError::Other("expected 2 i64s (v64x2)".to_string()));
        };
        Ok(Self::V128(((h as u128) << 64 | (l as u128)).into()))
    }

    fn unwrap_s32(&self) -> i32 {
        *unwrap_val!(self, Self::I32, "s32")
    }

    fn unwrap_s64(&self) -> i64 {
        *unwrap_val!(self, Self::I64, "s64")
    }

    fn unwrap_f32(&self) -> f32 {
        let val = f32::from_bits(*unwrap_val!(self, Self::F32, "f32"));
        canonicalize_nan32(val)
    }

    fn unwrap_f64(&self) -> f64 {
        let val = f64::from_bits(*unwrap_val!(self, Self::F64, "f64"));
        canonicalize_nan64(val)
    }
    #[expect(clippy::cast_possible_truncation, reason = "handled losslessly here")]
    fn unwrap_tuple(&self) -> Box<dyn Iterator<Item = Cow<'_, Self>> + '_> {
        let v = unwrap_val!(self, Self::V128, "tuple").as_u128();
        let low = v as i64;
        let high = (v >> 64) as i64;
        Box::new(
            [Self::I64(low), Self::I64(high)]
                .into_iter()
                .map(Cow::Owned),
        )
    }
}

impl WasmFunc for crate::FuncType {
    type Type = crate::ValType;

    fn params(&self) -> Box<dyn Iterator<Item = Self::Type> + '_> {
        Box::new(self.params())
    }

    fn results(&self) -> Box<dyn Iterator<Item = Self::Type> + '_> {
        Box::new(self.results())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn core_vals_smoke_test() {
        use crate::Val;
        for (val, want) in [
            (Val::I32(10), "10"),
            (Val::I64(-10), "-10"),
            (1.5f32.into(), "1.5"),
            (f32::NAN.into(), "nan"),
            (f32::INFINITY.into(), "inf"),
            (f32::NEG_INFINITY.into(), "-inf"),
            ((-1.5f64).into(), "-1.5"),
            (f32::NAN.into(), "nan"),
            (f32::INFINITY.into(), "inf"),
            (f32::NEG_INFINITY.into(), "-inf"),
            (
                Val::V128(0x1234567890abcdef1122334455667788.into()),
                "(1234605616436508552, 1311768467294899695)",
            ),
        ] {
            let got = wasm_wave::to_string(&val)
                .unwrap_or_else(|err| panic!("failed to serialize {val:?}: {err}"));
            assert_eq!(got, want, "for {val:?}");
        }
    }
}
