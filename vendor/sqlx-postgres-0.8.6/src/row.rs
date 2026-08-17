use crate::column::ColumnIndex;
use crate::error::Error;
use crate::message::DataRow;
use crate::statement::PgStatementMetadata;
use crate::value::PgValueFormat;
use crate::{PgColumn, PgValueRef, Postgres};
pub(crate) use sqlx_core::row::Row;
use sqlx_core::type_checking::TypeChecking;
use sqlx_core::value::ValueRef;
use std::fmt::Debug;
use std::sync::Arc;

/// Implementation of [`Row`] for PostgreSQL.
pub struct PgRow {
    pub(crate) data: DataRow,
    pub(crate) format: PgValueFormat,
    pub(crate) metadata: Arc<PgStatementMetadata>,
}

impl Row for PgRow {
    type Database = Postgres;

    fn columns(&self) -> &[PgColumn] {
        &self.metadata.columns
    }

    fn try_get_raw<I>(&self, index: I) -> Result<PgValueRef<'_>, Error>
    where
        I: ColumnIndex<Self>,
    {
        let index = index.index(self)?;
        let column = &self.metadata.columns[index];
        let value = self.data.get(index);

        Ok(PgValueRef {
            format: self.format,
            row: Some(&self.data.storage),
            type_info: column.type_info.clone(),
            value,
        })
    }
}

impl ColumnIndex<PgRow> for &'_ str {
    fn index(&self, row: &PgRow) -> Result<usize, Error> {
        row.metadata
            .column_names
            .get(*self)
            .ok_or_else(|| Error::ColumnNotFound((*self).into()))
            .copied()
    }
}

impl Debug for PgRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PgRow ")?;

        let mut debug_map = f.debug_map();
        for (index, column) in self.columns().iter().enumerate() {
            match self.try_get_raw(index) {
                Ok(value) => {
                    debug_map.entry(
                        &column.name,
                        &Postgres::fmt_value_debug(&<PgValueRef as ValueRef>::to_owned(&value)),
                    );
                }
                Err(error) => {
                    debug_map.entry(&column.name, &format!("decode error: {error:?}"));
                }
            }
        }

        debug_map.finish()
    }
}
