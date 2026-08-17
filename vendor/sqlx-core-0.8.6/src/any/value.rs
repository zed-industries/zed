use std::borrow::Cow;

use crate::any::{Any, AnyTypeInfo, AnyTypeInfoKind};
use crate::database::Database;
use crate::error::BoxDynError;
use crate::types::Type;
use crate::value::{Value, ValueRef};

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum AnyValueKind<'a> {
    Null(AnyTypeInfoKind),
    Bool(bool),
    SmallInt(i16),
    Integer(i32),
    BigInt(i64),
    Real(f32),
    Double(f64),
    Text(Cow<'a, str>),
    Blob(Cow<'a, [u8]>),
}

impl AnyValueKind<'_> {
    fn type_info(&self) -> AnyTypeInfo {
        AnyTypeInfo {
            kind: match self {
                AnyValueKind::Null(_) => AnyTypeInfoKind::Null,
                AnyValueKind::Bool(_) => AnyTypeInfoKind::Bool,
                AnyValueKind::SmallInt(_) => AnyTypeInfoKind::SmallInt,
                AnyValueKind::Integer(_) => AnyTypeInfoKind::Integer,
                AnyValueKind::BigInt(_) => AnyTypeInfoKind::BigInt,
                AnyValueKind::Real(_) => AnyTypeInfoKind::Real,
                AnyValueKind::Double(_) => AnyTypeInfoKind::Double,
                AnyValueKind::Text(_) => AnyTypeInfoKind::Text,
                AnyValueKind::Blob(_) => AnyTypeInfoKind::Blob,
            },
        }
    }

    pub(in crate::any) fn unexpected<Expected: Type<Any>>(&self) -> Result<Expected, BoxDynError> {
        Err(format!("expected {}, got {:?}", Expected::type_info(), self).into())
    }

    pub(in crate::any) fn try_integer<T>(&self) -> Result<T, BoxDynError>
    where
        T: Type<Any> + TryFrom<i16> + TryFrom<i32> + TryFrom<i64>,
        BoxDynError: From<<T as TryFrom<i16>>::Error>,
        BoxDynError: From<<T as TryFrom<i32>>::Error>,
        BoxDynError: From<<T as TryFrom<i64>>::Error>,
    {
        Ok(match self {
            AnyValueKind::SmallInt(i) => (*i).try_into()?,
            AnyValueKind::Integer(i) => (*i).try_into()?,
            AnyValueKind::BigInt(i) => (*i).try_into()?,
            _ => return self.unexpected(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct AnyValue {
    #[doc(hidden)]
    pub kind: AnyValueKind<'static>,
}

#[derive(Clone, Debug)]
pub struct AnyValueRef<'a> {
    pub(crate) kind: AnyValueKind<'a>,
}

impl Value for AnyValue {
    type Database = Any;

    fn as_ref(&self) -> <Self::Database as Database>::ValueRef<'_> {
        AnyValueRef {
            kind: match &self.kind {
                AnyValueKind::Null(k) => AnyValueKind::Null(*k),
                AnyValueKind::Bool(b) => AnyValueKind::Bool(*b),
                AnyValueKind::SmallInt(i) => AnyValueKind::SmallInt(*i),
                AnyValueKind::Integer(i) => AnyValueKind::Integer(*i),
                AnyValueKind::BigInt(i) => AnyValueKind::BigInt(*i),
                AnyValueKind::Real(r) => AnyValueKind::Real(*r),
                AnyValueKind::Double(d) => AnyValueKind::Double(*d),
                AnyValueKind::Text(t) => AnyValueKind::Text(Cow::Borrowed(t)),
                AnyValueKind::Blob(b) => AnyValueKind::Blob(Cow::Borrowed(b)),
            },
        }
    }

    fn type_info(&self) -> Cow<'_, <Self::Database as Database>::TypeInfo> {
        Cow::Owned(self.kind.type_info())
    }

    fn is_null(&self) -> bool {
        matches!(self.kind, AnyValueKind::Null(_))
    }
}

impl<'a> ValueRef<'a> for AnyValueRef<'a> {
    type Database = Any;

    fn to_owned(&self) -> <Self::Database as Database>::Value {
        AnyValue {
            kind: match &self.kind {
                AnyValueKind::Null(k) => AnyValueKind::Null(*k),
                AnyValueKind::Bool(b) => AnyValueKind::Bool(*b),
                AnyValueKind::SmallInt(i) => AnyValueKind::SmallInt(*i),
                AnyValueKind::Integer(i) => AnyValueKind::Integer(*i),
                AnyValueKind::BigInt(i) => AnyValueKind::BigInt(*i),
                AnyValueKind::Real(r) => AnyValueKind::Real(*r),
                AnyValueKind::Double(d) => AnyValueKind::Double(*d),
                AnyValueKind::Text(t) => AnyValueKind::Text(Cow::Owned(t.to_string())),
                AnyValueKind::Blob(b) => AnyValueKind::Blob(Cow::Owned(b.to_vec())),
            },
        }
    }

    fn type_info(&self) -> Cow<'_, <Self::Database as Database>::TypeInfo> {
        Cow::Owned(self.kind.type_info())
    }

    fn is_null(&self) -> bool {
        matches!(self.kind, AnyValueKind::Null(_))
    }
}
