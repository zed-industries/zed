use crate::component;
use crate::prelude::*;
use std::borrow::Cow;

use super::{canonicalize_nan32, canonicalize_nan64, unwrap_2val, unwrap_val};
use component::wasm_wave::wasm::{
    DisplayValue, WasmFunc, WasmType, WasmTypeKind, WasmValue, WasmValueError, ensure_type_kind,
};

macro_rules! maybe_unwrap_type {
    ($ty:expr, $case:path) => {
        match $ty {
            $case(v) => Some(v),
            _ => None,
        }
    };
}

impl WasmType for component::Type {
    fn kind(&self) -> WasmTypeKind {
        match self {
            Self::Bool => WasmTypeKind::Bool,
            Self::S8 => WasmTypeKind::S8,
            Self::U8 => WasmTypeKind::U8,
            Self::S16 => WasmTypeKind::S16,
            Self::U16 => WasmTypeKind::U16,
            Self::S32 => WasmTypeKind::S32,
            Self::U32 => WasmTypeKind::U32,
            Self::S64 => WasmTypeKind::S64,
            Self::U64 => WasmTypeKind::U64,
            Self::Float32 => WasmTypeKind::F32,
            Self::Float64 => WasmTypeKind::F64,
            Self::Char => WasmTypeKind::Char,
            Self::String => WasmTypeKind::String,
            Self::List(_) => WasmTypeKind::List,
            Self::Record(_) => WasmTypeKind::Record,
            Self::Tuple(_) => WasmTypeKind::Tuple,
            Self::Variant(_) => WasmTypeKind::Variant,
            Self::Enum(_) => WasmTypeKind::Enum,
            Self::Option(_) => WasmTypeKind::Option,
            Self::Result(_) => WasmTypeKind::Result,
            Self::Flags(_) => WasmTypeKind::Flags,

            Self::Own(_)
            | Self::Borrow(_)
            | Self::Stream(_)
            | Self::Future(_)
            | Self::ErrorContext => WasmTypeKind::Unsupported,
        }
    }

    fn list_element_type(&self) -> Option<Self> {
        Some(maybe_unwrap_type!(self, Self::List)?.ty())
    }

    fn record_fields(&self) -> Box<dyn Iterator<Item = (Cow<'_, str>, Self)> + '_> {
        let Self::Record(record) = self else {
            return Box::new(std::iter::empty());
        };
        Box::new(record.fields().map(|f| (f.name.into(), f.ty.clone())))
    }

    fn tuple_element_types(&self) -> Box<dyn Iterator<Item = Self> + '_> {
        let Self::Tuple(tuple) = self else {
            return Box::new(std::iter::empty());
        };
        Box::new(tuple.types())
    }

    fn variant_cases(&self) -> Box<dyn Iterator<Item = (Cow<'_, str>, Option<Self>)> + '_> {
        let Self::Variant(variant) = self else {
            return Box::new(std::iter::empty());
        };
        Box::new(variant.cases().map(|case| (case.name.into(), case.ty)))
    }

    fn enum_cases(&self) -> Box<dyn Iterator<Item = Cow<'_, str>> + '_> {
        let Self::Enum(enum_) = self else {
            return Box::new(std::iter::empty());
        };
        Box::new(enum_.names().map(Into::into))
    }

    fn option_some_type(&self) -> Option<Self> {
        maybe_unwrap_type!(self, Self::Option).map(|o| o.ty())
    }

    fn result_types(&self) -> Option<(Option<Self>, Option<Self>)> {
        let result = maybe_unwrap_type!(self, Self::Result)?;
        Some((result.ok(), result.err()))
    }

    fn flags_names(&self) -> Box<dyn Iterator<Item = Cow<'_, str>> + '_> {
        let Self::Flags(flags) = self else {
            return Box::new(std::iter::empty());
        };
        Box::new(flags.names().map(Into::into))
    }
}

macro_rules! impl_primitives {
    ($Self:ident, $(($case:ident, $ty:ty, $make:ident, $unwrap:ident)),*) => {
        $(
            fn $make(val: $ty) -> $Self {
                $Self::$case(val)
            }

            fn $unwrap(&self) -> $ty {
                *unwrap_val!(self, $Self::$case, stringify!($case))
            }
        )*
    };
}

impl WasmValue for component::Val {
    type Type = component::Type;

    fn kind(&self) -> WasmTypeKind {
        match self {
            Self::Bool(_) => WasmTypeKind::Bool,
            Self::S8(_) => WasmTypeKind::S8,
            Self::U8(_) => WasmTypeKind::U8,
            Self::S16(_) => WasmTypeKind::S16,
            Self::U16(_) => WasmTypeKind::U16,
            Self::S32(_) => WasmTypeKind::S32,
            Self::U32(_) => WasmTypeKind::U32,
            Self::S64(_) => WasmTypeKind::S64,
            Self::U64(_) => WasmTypeKind::U64,
            Self::Float32(_) => WasmTypeKind::F32,
            Self::Float64(_) => WasmTypeKind::F64,
            Self::Char(_) => WasmTypeKind::Char,
            Self::String(_) => WasmTypeKind::String,
            Self::List(_) => WasmTypeKind::List,
            Self::Record(_) => WasmTypeKind::Record,
            Self::Tuple(_) => WasmTypeKind::Tuple,
            Self::Variant(..) => WasmTypeKind::Variant,
            Self::Enum(_) => WasmTypeKind::Enum,
            Self::Option(_) => WasmTypeKind::Option,
            Self::Result(_) => WasmTypeKind::Result,
            Self::Flags(_) => WasmTypeKind::Flags,
            Self::Resource(_) | Self::Stream(_) | Self::Future(_) | Self::ErrorContext(_) => {
                WasmTypeKind::Unsupported
            }
        }
    }

    impl_primitives!(
        Self,
        (Bool, bool, make_bool, unwrap_bool),
        (S8, i8, make_s8, unwrap_s8),
        (S16, i16, make_s16, unwrap_s16),
        (S32, i32, make_s32, unwrap_s32),
        (S64, i64, make_s64, unwrap_s64),
        (U8, u8, make_u8, unwrap_u8),
        (U16, u16, make_u16, unwrap_u16),
        (U32, u32, make_u32, unwrap_u32),
        (U64, u64, make_u64, unwrap_u64),
        (Char, char, make_char, unwrap_char)
    );

    fn make_f32(val: f32) -> Self {
        let val = canonicalize_nan32(val);
        Self::Float32(val)
    }
    fn make_f64(val: f64) -> Self {
        let val = canonicalize_nan64(val);
        Self::Float64(val)
    }
    fn make_string(val: Cow<str>) -> Self {
        Self::String(val.into())
    }
    fn make_list(
        ty: &Self::Type,
        vals: impl IntoIterator<Item = Self>,
    ) -> Result<Self, WasmValueError> {
        ensure_type_kind(ty, WasmTypeKind::List)?;
        let val = Self::List(vals.into_iter().collect());
        ensure_type_val(ty, &val)?;
        Ok(val)
    }
    fn make_record<'a>(
        ty: &Self::Type,
        fields: impl IntoIterator<Item = (&'a str, Self)>,
    ) -> Result<Self, WasmValueError> {
        ensure_type_kind(ty, WasmTypeKind::Record)?;
        let values: Vec<(String, Self)> = fields
            .into_iter()
            .map(|(name, val)| (name.to_string(), val))
            .collect();
        let val = Self::Record(values);
        ensure_type_val(ty, &val)?;
        Ok(val)
    }
    fn make_tuple(
        ty: &Self::Type,
        vals: impl IntoIterator<Item = Self>,
    ) -> Result<Self, WasmValueError> {
        ensure_type_kind(ty, WasmTypeKind::Tuple)?;
        let val = Self::Tuple(vals.into_iter().collect());
        ensure_type_val(ty, &val)?;
        Ok(val)
    }
    fn make_variant(
        ty: &Self::Type,
        case: &str,
        val: Option<Self>,
    ) -> Result<Self, WasmValueError> {
        ensure_type_kind(ty, WasmTypeKind::Variant)?;
        let val = Self::Variant(case.to_string(), val.map(Box::new));
        ensure_type_val(ty, &val)?;
        Ok(val)
    }
    fn make_enum(ty: &Self::Type, case: &str) -> Result<Self, WasmValueError> {
        ensure_type_kind(ty, WasmTypeKind::Enum)?;
        let val = Self::Enum(case.to_string());
        ensure_type_val(ty, &val)?;
        Ok(val)
    }
    fn make_option(ty: &Self::Type, val: Option<Self>) -> Result<Self, WasmValueError> {
        ensure_type_kind(ty, WasmTypeKind::Option)?;
        let val = Self::Option(val.map(Box::new));
        ensure_type_val(ty, &val)?;
        Ok(val)
    }
    fn make_result(
        ty: &Self::Type,
        val: Result<Option<Self>, Option<Self>>,
    ) -> Result<Self, WasmValueError> {
        ensure_type_kind(ty, WasmTypeKind::Result)?;
        let val = match val {
            Ok(val) => Self::Result(Ok(val.map(Box::new))),
            Err(val) => Self::Result(Err(val.map(Box::new))),
        };
        ensure_type_val(ty, &val)?;
        Ok(val)
    }
    fn make_flags<'a>(
        ty: &Self::Type,
        names: impl IntoIterator<Item = &'a str>,
    ) -> Result<Self, WasmValueError> {
        ensure_type_kind(ty, WasmTypeKind::Flags)?;
        let val = Self::Flags(names.into_iter().map(|n| n.to_string()).collect());
        ensure_type_val(ty, &val)?;
        Ok(val)
    }

    fn unwrap_f32(&self) -> f32 {
        let val = *unwrap_val!(self, Self::Float32, "f32");
        canonicalize_nan32(val)
    }
    fn unwrap_f64(&self) -> f64 {
        let val = *unwrap_val!(self, Self::Float64, "f64");
        canonicalize_nan64(val)
    }
    fn unwrap_string(&self) -> Cow<'_, str> {
        unwrap_val!(self, Self::String, "string").into()
    }
    fn unwrap_list(&self) -> Box<dyn Iterator<Item = Cow<'_, Self>> + '_> {
        let list = unwrap_val!(self, Self::List, "list");
        Box::new(list.iter().map(cow))
    }
    fn unwrap_record(&self) -> Box<dyn Iterator<Item = (Cow<'_, str>, Cow<'_, Self>)> + '_> {
        let record = unwrap_val!(self, Self::Record, "record");
        Box::new(record.iter().map(|(name, val)| (name.into(), cow(val))))
    }
    fn unwrap_tuple(&self) -> Box<dyn Iterator<Item = Cow<'_, Self>> + '_> {
        let tuple = unwrap_val!(self, Self::Tuple, "tuple");
        Box::new(tuple.iter().map(cow))
    }
    fn unwrap_variant(&self) -> (Cow<'_, str>, Option<Cow<'_, Self>>) {
        let (discriminant, payload) = unwrap_2val!(self, Self::Variant, "variant");
        (discriminant.into(), payload.as_deref().map(cow))
    }
    fn unwrap_enum(&self) -> Cow<'_, str> {
        unwrap_val!(self, Self::Enum, "enum").into()
    }
    fn unwrap_option(&self) -> Option<Cow<'_, Self>> {
        unwrap_val!(self, Self::Option, "option")
            .as_deref()
            .map(cow)
    }
    fn unwrap_result(&self) -> Result<Option<Cow<'_, Self>>, Option<Cow<'_, Self>>> {
        match unwrap_val!(self, Self::Result, "result") {
            Ok(t) => Ok(t.as_deref().map(cow)),
            Err(e) => Err(e.as_deref().map(cow)),
        }
    }
    fn unwrap_flags(&self) -> Box<dyn Iterator<Item = Cow<'_, str>> + '_> {
        let flags = unwrap_val!(self, Self::Flags, "flags");
        Box::new(flags.iter().map(Into::into))
    }
}

// Returns an error if the given component::Val is not of the given component::Type.
//
// The component::Val::Resource(_) variant results in an unsupported error at this time.
fn ensure_type_val(ty: &component::Type, val: &component::Val) -> Result<(), WasmValueError> {
    let wrong_value_type = || -> Result<(), WasmValueError> {
        Err(WasmValueError::WrongValueType {
            ty: wasm_wave::wasm::DisplayType(ty).to_string(),
            val: wasm_wave::wasm::DisplayValue(val).to_string(),
        })
    };

    if ty.kind() != val.kind() {
        return wrong_value_type();
    }

    match val {
        component::Val::List(vals) => {
            let list_type = ty.unwrap_list().ty();
            for val in vals {
                ensure_type_val(&list_type, val)?;
            }
        }
        component::Val::Record(vals) => {
            let record_handle = ty.unwrap_record();
            // Check that every non option field type is found in the Vec
            for field in record_handle.fields() {
                if !matches!(field.ty, component::Type::Option(_))
                    && !vals.iter().any(|(n, _)| n == field.name)
                {
                    return wrong_value_type();
                }
            }
            // Check that every (String, Val) of the given Vec is a correct field_type
            for (name, field_val) in vals.iter() {
                // N.B. The `fields` call in each iteration is non-trivial, perhaps a cleaner way
                // using the loop above will present itself.
                if let Some(field) = record_handle.fields().find(|field| field.name == name) {
                    ensure_type_val(&field.ty, field_val)?;
                } else {
                    return wrong_value_type();
                }
            }
        }
        component::Val::Tuple(vals) => {
            let field_types = ty.unwrap_tuple().types();
            if field_types.len() != vals.len() {
                return wrong_value_type();
            }
            for (ty, val) in field_types.into_iter().zip(vals.iter()) {
                ensure_type_val(&ty, val)?;
            }
        }
        component::Val::Variant(name, optional_payload) => {
            if let Some(case) = ty.unwrap_variant().cases().find(|case| case.name == name) {
                match (optional_payload, case.ty) {
                    (None, None) => {}
                    (Some(payload), Some(payload_ty)) => ensure_type_val(&payload_ty, payload)?,
                    _ => return wrong_value_type(),
                }
            } else {
                return wrong_value_type();
            }
        }
        component::Val::Enum(name) => {
            if !ty.unwrap_enum().names().any(|n| n == name) {
                return wrong_value_type();
            }
        }
        component::Val::Option(Some(some_val)) => {
            ensure_type_val(&ty.unwrap_option().ty(), some_val.as_ref())?;
        }
        component::Val::Result(res_val) => {
            let result_handle = ty.unwrap_result();
            match res_val {
                Ok(ok) => match (ok, result_handle.ok()) {
                    (None, None) => {}
                    (Some(ok_val), Some(ok_ty)) => ensure_type_val(&ok_ty, ok_val.as_ref())?,
                    _ => return wrong_value_type(),
                },
                Err(err) => match (err, result_handle.err()) {
                    (None, None) => {}
                    (Some(err_val), Some(err_ty)) => ensure_type_val(&err_ty, err_val.as_ref())?,
                    _ => return wrong_value_type(),
                },
            }
        }
        component::Val::Flags(flags) => {
            let flags_handle = ty.unwrap_flags();
            for flag in flags {
                if !flags_handle.names().any(|n| n == flag) {
                    return wrong_value_type();
                }
            }
        }
        component::Val::Resource(_) => {
            return Err(WasmValueError::UnsupportedType(
                DisplayValue(val).to_string(),
            ));
        }

        // Any leaf variant type has already had its kind compared above; nothing further to check.
        // Likewise, the component::Option(None) arm would have nothing left to check.
        _ => {}
    }
    Ok(())
}

impl WasmFunc for component::types::ComponentFunc {
    type Type = component::Type;

    fn params(&self) -> Box<dyn Iterator<Item = Self::Type> + '_> {
        Box::new(self.params().map(|(_n, t)| t))
    }

    fn results(&self) -> Box<dyn Iterator<Item = Self::Type> + '_> {
        Box::new(self.results())
    }
}

fn cow<T: Clone>(t: &T) -> Cow<'_, T> {
    Cow::Borrowed(t)
}

#[cfg(test)]
mod tests {
    #[test]
    fn component_vals_smoke_test() {
        use crate::component::Val;
        for (val, want) in [
            (Val::Bool(false), "false"),
            (Val::Bool(true), "true"),
            (Val::S8(10), "10"),
            (Val::S16(-10), "-10"),
            (Val::S32(1_000_000), "1000000"),
            (Val::S64(0), "0"),
            (Val::U8(255), "255"),
            (Val::U16(0), "0"),
            (Val::U32(1_000_000), "1000000"),
            (Val::U64(9), "9"),
            (Val::Float32(1.5), "1.5"),
            (Val::Float32(f32::NAN), "nan"),
            (Val::Float32(f32::INFINITY), "inf"),
            (Val::Float32(f32::NEG_INFINITY), "-inf"),
            (Val::Float64(-1.5e-10), "-0.00000000015"),
            (Val::Float64(f64::NAN), "nan"),
            (Val::Float64(f64::INFINITY), "inf"),
            (Val::Float64(f64::NEG_INFINITY), "-inf"),
            (Val::Char('x'), "'x'"),
            (Val::Char('☃'), "'☃'"),
            (Val::Char('\''), r"'\''"),
            (Val::Char('\0'), r"'\u{0}'"),
            (Val::Char('\x1b'), r"'\u{1b}'"),
            (Val::Char('😂'), r"'😂'"),
            (Val::String("abc".into()), r#""abc""#),
            (Val::String(r#"\☃""#.into()), r#""\\☃\"""#),
            (Val::String("\t\r\n\0".into()), r#""\t\r\n\u{0}""#),
        ] {
            let got = wasm_wave::to_string(&val)
                .unwrap_or_else(|err| panic!("failed to serialize {val:?}: {err}"));
            assert_eq!(got, want, "for {val:?}");
        }
    }

    #[test]
    fn test_round_trip_floats() {
        use crate::component::{Type, Val};
        use std::fmt::Debug;

        fn round_trip<V: wasm_wave::wasm::WasmValue + PartialEq + Debug>(ty: &V::Type, val: &V) {
            let val_str = wasm_wave::to_string(val).unwrap();
            let result: V = wasm_wave::from_str::<V>(ty, &val_str).unwrap();
            assert_eq!(val, &result);
        }

        for i in 0..100 {
            for j in 0..100 {
                round_trip(&Type::Float32, &Val::Float32(i as f32 / j as f32));
                round_trip(&Type::Float64, &Val::Float64(i as f64 / j as f64));
            }
        }

        round_trip(&Type::Float32, &Val::Float32(f32::EPSILON));
        round_trip(&Type::Float64, &Val::Float64(f64::EPSILON));
    }
}
