use super::{PgColumn, PgTypeInfo};
use crate::column::ColumnIndex;
use crate::error::Error;
use crate::ext::ustr::UStr;
use crate::{PgArguments, Postgres};
use std::borrow::Cow;
use std::sync::Arc;

pub(crate) use sqlx_core::statement::Statement;
use sqlx_core::{Either, HashMap};

#[derive(Debug, Clone)]
pub struct PgStatement<'q> {
    pub(crate) sql: Cow<'q, str>,
    pub(crate) metadata: Arc<PgStatementMetadata>,
}

#[derive(Debug, Default)]
pub(crate) struct PgStatementMetadata {
    pub(crate) columns: Vec<PgColumn>,
    // This `Arc` is not redundant; it's used to avoid deep-copying this map for the `Any` backend.
    // See `sqlx-postgres/src/any.rs`
    pub(crate) column_names: Arc<HashMap<UStr, usize>>,
    pub(crate) parameters: Vec<PgTypeInfo>,
}

impl<'q> Statement<'q> for PgStatement<'q> {
    type Database = Postgres;

    fn to_owned(&self) -> PgStatement<'static> {
        PgStatement::<'static> {
            sql: Cow::Owned(self.sql.clone().into_owned()),
            metadata: self.metadata.clone(),
        }
    }

    fn sql(&self) -> &str {
        &self.sql
    }

    fn parameters(&self) -> Option<Either<&[PgTypeInfo], usize>> {
        Some(Either::Left(&self.metadata.parameters))
    }

    fn columns(&self) -> &[PgColumn] {
        &self.metadata.columns
    }

    impl_statement_query!(PgArguments);
}

impl ColumnIndex<PgStatement<'_>> for &'_ str {
    fn index(&self, statement: &PgStatement<'_>) -> Result<usize, Error> {
        statement
            .metadata
            .column_names
            .get(*self)
            .ok_or_else(|| Error::ColumnNotFound((*self).into()))
            .copied()
    }
}

// #[cfg(feature = "any")]
// impl<'q> From<PgStatement<'q>> for crate::any::AnyStatement<'q> {
//     #[inline]
//     fn from(statement: PgStatement<'q>) -> Self {
//         crate::any::AnyStatement::<'q> {
//             columns: statement
//                 .metadata
//                 .columns
//                 .iter()
//                 .map(|col| col.clone().into())
//                 .collect(),
//             column_names: statement.metadata.column_names.clone(),
//             parameters: Some(Either::Left(
//                 statement
//                     .metadata
//                     .parameters
//                     .iter()
//                     .map(|ty| ty.clone().into())
//                     .collect(),
//             )),
//             sql: statement.sql,
//         }
//     }
// }
