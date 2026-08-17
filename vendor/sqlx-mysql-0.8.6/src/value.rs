use std::borrow::Cow;
use std::str::from_utf8;

use bytes::Bytes;
pub(crate) use sqlx_core::value::*;

use crate::error::{BoxDynError, UnexpectedNullError};
use crate::protocol::text::ColumnType;
use crate::{MySql, MySqlTypeInfo};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum MySqlValueFormat {
    Text,
    Binary,
}

/// Implementation of [`Value`] for MySQL.
#[derive(Clone)]
pub struct MySqlValue {
    value: Option<Bytes>,
    type_info: MySqlTypeInfo,
    format: MySqlValueFormat,
}

/// Implementation of [`ValueRef`] for MySQL.
#[derive(Clone)]
pub struct MySqlValueRef<'r> {
    pub(crate) value: Option<&'r [u8]>,
    pub(crate) row: Option<&'r Bytes>,
    pub(crate) type_info: MySqlTypeInfo,
    pub(crate) format: MySqlValueFormat,
}

impl<'r> MySqlValueRef<'r> {
    pub(crate) fn format(&self) -> MySqlValueFormat {
        self.format
    }

    pub(crate) fn as_bytes(&self) -> Result<&'r [u8], BoxDynError> {
        match &self.value {
            Some(v) => Ok(v),
            None => Err(UnexpectedNullError.into()),
        }
    }

    pub(crate) fn as_str(&self) -> Result<&'r str, BoxDynError> {
        Ok(from_utf8(self.as_bytes()?)?)
    }
}

impl Value for MySqlValue {
    type Database = MySql;

    fn as_ref(&self) -> MySqlValueRef<'_> {
        MySqlValueRef {
            value: self.value.as_deref(),
            row: None,
            type_info: self.type_info.clone(),
            format: self.format,
        }
    }

    fn type_info(&self) -> Cow<'_, MySqlTypeInfo> {
        Cow::Borrowed(&self.type_info)
    }

    fn is_null(&self) -> bool {
        is_null(self.value.as_deref(), &self.type_info)
    }
}

impl<'r> ValueRef<'r> for MySqlValueRef<'r> {
    type Database = MySql;

    fn to_owned(&self) -> MySqlValue {
        let value = match (self.row, self.value) {
            (Some(row), Some(value)) => Some(row.slice_ref(value)),

            (None, Some(value)) => Some(Bytes::copy_from_slice(value)),

            _ => None,
        };

        MySqlValue {
            value,
            format: self.format,
            type_info: self.type_info.clone(),
        }
    }

    fn type_info(&self) -> Cow<'_, MySqlTypeInfo> {
        Cow::Borrowed(&self.type_info)
    }

    #[inline]
    fn is_null(&self) -> bool {
        is_null(self.value, &self.type_info)
    }
}

fn is_null(value: Option<&[u8]>, ty: &MySqlTypeInfo) -> bool {
    if let Some(value) = value {
        // zero dates and date times should be treated the same as NULL
        if matches!(
            ty.r#type,
            ColumnType::Date | ColumnType::Timestamp | ColumnType::Datetime
        ) && value.starts_with(b"\0")
        {
            return true;
        }
    }

    value.is_none()
}
