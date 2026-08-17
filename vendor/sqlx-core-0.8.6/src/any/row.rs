use crate::any::error::mismatched_types;
use crate::any::{Any, AnyColumn, AnyTypeInfo, AnyTypeInfoKind, AnyValue, AnyValueKind};
use crate::column::{Column, ColumnIndex};
use crate::database::Database;
use crate::decode::Decode;
use crate::error::Error;
use crate::ext::ustr::UStr;
use crate::row::Row;
use crate::type_info::TypeInfo;
use crate::types::Type;
use crate::value::{Value, ValueRef};
use std::sync::Arc;

#[derive(Clone)]
pub struct AnyRow {
    #[doc(hidden)]
    pub column_names: Arc<crate::HashMap<UStr, usize>>,
    #[doc(hidden)]
    pub columns: Vec<AnyColumn>,
    #[doc(hidden)]
    pub values: Vec<AnyValue>,
}

impl Row for AnyRow {
    type Database = Any;

    fn columns(&self) -> &[AnyColumn] {
        &self.columns
    }

    fn try_get_raw<I>(&self, index: I) -> Result<<Self::Database as Database>::ValueRef<'_>, Error>
    where
        I: ColumnIndex<Self>,
    {
        let index = index.index(self)?;
        Ok(self
            .values
            .get(index)
            .ok_or_else(|| Error::ColumnIndexOutOfBounds {
                index,
                len: self.columns.len(),
            })?
            .as_ref())
    }

    fn try_get<'r, T, I>(&'r self, index: I) -> Result<T, Error>
    where
        I: ColumnIndex<Self>,
        T: Decode<'r, Self::Database> + Type<Self::Database>,
    {
        let value = self.try_get_raw(&index)?;
        let ty = value.type_info();

        if !value.is_null() && !ty.is_null() && !T::compatible(&ty) {
            Err(mismatched_types::<T>(&ty))
        } else {
            T::decode(value)
        }
        .map_err(|source| Error::ColumnDecode {
            index: format!("{index:?}"),
            source,
        })
    }
}

impl<'i> ColumnIndex<AnyRow> for &'i str {
    fn index(&self, row: &AnyRow) -> Result<usize, Error> {
        row.column_names
            .get(*self)
            .copied()
            .ok_or_else(|| Error::ColumnNotFound(self.to_string()))
    }
}

impl AnyRow {
    // This is not a `TryFrom` impl because trait impls are easy for users to accidentally
    // become reliant upon, even if hidden, but we want to be able to change the bounds
    // on this function as the `Any` driver gains support for more types.
    //
    // Also `column_names` needs to be passed by the driver to avoid making deep copies.
    #[doc(hidden)]
    pub fn map_from<'a, R: Row>(
        row: &'a R,
        column_names: Arc<crate::HashMap<UStr, usize>>,
    ) -> Result<Self, Error>
    where
        usize: ColumnIndex<R>,
        AnyTypeInfo: for<'b> TryFrom<&'b <R::Database as Database>::TypeInfo, Error = Error>,
        AnyColumn: for<'b> TryFrom<&'b <R::Database as Database>::Column, Error = Error>,
        bool: Type<R::Database> + Decode<'a, R::Database>,
        i16: Type<R::Database> + Decode<'a, R::Database>,
        i32: Type<R::Database> + Decode<'a, R::Database>,
        i64: Type<R::Database> + Decode<'a, R::Database>,
        f32: Type<R::Database> + Decode<'a, R::Database>,
        f64: Type<R::Database> + Decode<'a, R::Database>,
        String: Type<R::Database> + Decode<'a, R::Database>,
        Vec<u8>: Type<R::Database> + Decode<'a, R::Database>,
    {
        let mut row_out = AnyRow {
            column_names,
            columns: Vec::with_capacity(row.columns().len()),
            values: Vec::with_capacity(row.columns().len()),
        };

        for col in row.columns() {
            let i = col.ordinal();

            let any_col = AnyColumn::try_from(col)?;

            let value = row.try_get_raw(i)?;

            // Map based on the _value_ type info, not the column type info.
            let type_info =
                AnyTypeInfo::try_from(&value.type_info()).map_err(|e| Error::ColumnDecode {
                    index: col.ordinal().to_string(),
                    source: e.into(),
                })?;

            let value_kind = match type_info.kind {
                k if value.is_null() => AnyValueKind::Null(k),
                AnyTypeInfoKind::Null => AnyValueKind::Null(AnyTypeInfoKind::Null),
                AnyTypeInfoKind::Bool => AnyValueKind::Bool(decode(value)?),
                AnyTypeInfoKind::SmallInt => AnyValueKind::SmallInt(decode(value)?),
                AnyTypeInfoKind::Integer => AnyValueKind::Integer(decode(value)?),
                AnyTypeInfoKind::BigInt => AnyValueKind::BigInt(decode(value)?),
                AnyTypeInfoKind::Real => AnyValueKind::Real(decode(value)?),
                AnyTypeInfoKind::Double => AnyValueKind::Double(decode(value)?),
                AnyTypeInfoKind::Blob => AnyValueKind::Blob(decode::<_, Vec<u8>>(value)?.into()),
                AnyTypeInfoKind::Text => AnyValueKind::Text(decode::<_, String>(value)?.into()),
            };

            row_out.columns.push(any_col);
            row_out.values.push(AnyValue { kind: value_kind });
        }

        Ok(row_out)
    }
}

fn decode<'r, DB: Database, T: Decode<'r, DB>>(
    valueref: <DB as Database>::ValueRef<'r>,
) -> crate::Result<T> {
    Decode::decode(valueref).map_err(Error::decode)
}
