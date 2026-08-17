use super::MySqlColumn;
use crate::column::ColumnIndex;
use crate::error::Error;
use crate::ext::ustr::UStr;
use crate::HashMap;
use crate::{MySql, MySqlArguments, MySqlTypeInfo};
use either::Either;
use std::borrow::Cow;
use std::sync::Arc;

pub(crate) use sqlx_core::statement::*;

#[derive(Debug, Clone)]
pub struct MySqlStatement<'q> {
    pub(crate) sql: Cow<'q, str>,
    pub(crate) metadata: MySqlStatementMetadata,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct MySqlStatementMetadata {
    pub(crate) columns: Arc<Vec<MySqlColumn>>,
    pub(crate) column_names: Arc<HashMap<UStr, usize>>,
    pub(crate) parameters: usize,
}

impl<'q> Statement<'q> for MySqlStatement<'q> {
    type Database = MySql;

    fn to_owned(&self) -> MySqlStatement<'static> {
        MySqlStatement::<'static> {
            sql: Cow::Owned(self.sql.clone().into_owned()),
            metadata: self.metadata.clone(),
        }
    }

    fn sql(&self) -> &str {
        &self.sql
    }

    fn parameters(&self) -> Option<Either<&[MySqlTypeInfo], usize>> {
        Some(Either::Right(self.metadata.parameters))
    }

    fn columns(&self) -> &[MySqlColumn] {
        &self.metadata.columns
    }

    impl_statement_query!(MySqlArguments);
}

impl ColumnIndex<MySqlStatement<'_>> for &'_ str {
    fn index(&self, statement: &MySqlStatement<'_>) -> Result<usize, Error> {
        statement
            .metadata
            .column_names
            .get(*self)
            .ok_or_else(|| Error::ColumnNotFound((*self).into()))
            .copied()
    }
}
