use crate::ext::ustr::UStr;
use crate::{PgTypeInfo, Postgres};

pub(crate) use sqlx_core::column::{Column, ColumnIndex};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "offline", derive(serde::Serialize, serde::Deserialize))]
pub struct PgColumn {
    pub(crate) ordinal: usize,
    pub(crate) name: UStr,
    pub(crate) type_info: PgTypeInfo,
    #[cfg_attr(feature = "offline", serde(skip))]
    pub(crate) relation_id: Option<crate::types::Oid>,
    #[cfg_attr(feature = "offline", serde(skip))]
    pub(crate) relation_attribute_no: Option<i16>,
}

impl PgColumn {
    /// Returns the OID of the table this column is from, if applicable.
    ///
    /// This will be `None` if the column is the result of an expression.
    ///
    /// Corresponds to column `attrelid` of the `pg_catalog.pg_attribute` table:
    /// <https://www.postgresql.org/docs/current/catalog-pg-attribute.html>
    pub fn relation_id(&self) -> Option<crate::types::Oid> {
        self.relation_id
    }

    /// Returns the 1-based index of this column in its parent table, if applicable.
    ///
    /// This will be `None` if the column is the result of an expression.
    ///
    /// Corresponds to column `attnum` of the `pg_catalog.pg_attribute` table:
    /// <https://www.postgresql.org/docs/current/catalog-pg-attribute.html>
    pub fn relation_attribute_no(&self) -> Option<i16> {
        self.relation_attribute_no
    }
}

impl Column for PgColumn {
    type Database = Postgres;

    fn ordinal(&self) -> usize {
        self.ordinal
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn type_info(&self) -> &PgTypeInfo {
        &self.type_info
    }
}
