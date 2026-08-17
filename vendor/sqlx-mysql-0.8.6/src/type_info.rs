use std::fmt::{self, Display, Formatter};

pub(crate) use sqlx_core::type_info::*;

use crate::protocol::text::{ColumnDefinition, ColumnFlags, ColumnType};

/// Type information for a MySql type.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "offline", derive(serde::Serialize, serde::Deserialize))]
pub struct MySqlTypeInfo {
    pub(crate) r#type: ColumnType,
    pub(crate) flags: ColumnFlags,

    // [max_size] for integer types, this is (M) in BIT(M) or TINYINT(M)
    #[cfg_attr(feature = "offline", serde(default))]
    pub(crate) max_size: Option<u32>,
}

impl MySqlTypeInfo {
    pub(crate) const fn binary(ty: ColumnType) -> Self {
        Self {
            r#type: ty,
            flags: ColumnFlags::BINARY,
            max_size: None,
        }
    }

    #[doc(hidden)]
    pub const fn __enum() -> Self {
        // Newer versions of MySQL seem to expect that a parameter binding of `MYSQL_TYPE_ENUM`
        // means that the value is encoded as an integer.
        //
        // For "strong" enums inputted as strings, we need to specify this type instead
        // for wider compatibility. This works on all covered versions of MySQL and MariaDB.
        //
        // Annoyingly, MySQL's developer documentation doesn't really explain this anywhere;
        // this had to be determined experimentally.
        Self {
            r#type: ColumnType::String,
            flags: ColumnFlags::ENUM,
            max_size: None,
        }
    }

    #[doc(hidden)]
    pub fn __type_feature_gate(&self) -> Option<&'static str> {
        match self.r#type {
            ColumnType::Date | ColumnType::Time | ColumnType::Timestamp | ColumnType::Datetime => {
                Some("time")
            }

            ColumnType::Json => Some("json"),
            ColumnType::NewDecimal => Some("bigdecimal"),

            _ => None,
        }
    }

    pub(crate) fn from_column(column: &ColumnDefinition) -> Self {
        Self {
            r#type: column.r#type,
            flags: column.flags,
            max_size: Some(column.max_size),
        }
    }
}

impl Display for MySqlTypeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(self.name())
    }
}

impl TypeInfo for MySqlTypeInfo {
    fn is_null(&self) -> bool {
        matches!(self.r#type, ColumnType::Null)
    }

    fn name(&self) -> &str {
        self.r#type.name(self.flags, self.max_size)
    }
}

impl PartialEq<MySqlTypeInfo> for MySqlTypeInfo {
    fn eq(&self, other: &MySqlTypeInfo) -> bool {
        if self.r#type != other.r#type {
            return false;
        }

        match self.r#type {
            ColumnType::Tiny
            | ColumnType::Short
            | ColumnType::Long
            | ColumnType::Int24
            | ColumnType::LongLong => {
                return self.flags.contains(ColumnFlags::UNSIGNED)
                    == other.flags.contains(ColumnFlags::UNSIGNED);
            }

            // for string types, check that our charset matches
            ColumnType::VarChar
            | ColumnType::Blob
            | ColumnType::TinyBlob
            | ColumnType::MediumBlob
            | ColumnType::LongBlob
            | ColumnType::String
            | ColumnType::VarString
            | ColumnType::Enum => {
                return self.flags == other.flags;
            }
            _ => {}
        }

        true
    }
}

impl Eq for MySqlTypeInfo {}
