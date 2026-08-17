use crate::any::{Any, AnyArguments, AnyColumn, AnyTypeInfo};
use crate::column::ColumnIndex;
use crate::database::Database;
use crate::error::Error;
use crate::ext::ustr::UStr;
use crate::statement::Statement;
use crate::HashMap;
use either::Either;
use std::borrow::Cow;
use std::sync::Arc;

pub struct AnyStatement<'q> {
    #[doc(hidden)]
    pub sql: Cow<'q, str>,
    #[doc(hidden)]
    pub parameters: Option<Either<Vec<AnyTypeInfo>, usize>>,
    #[doc(hidden)]
    pub column_names: Arc<HashMap<UStr, usize>>,
    #[doc(hidden)]
    pub columns: Vec<AnyColumn>,
}

impl<'q> Statement<'q> for AnyStatement<'q> {
    type Database = Any;

    fn to_owned(&self) -> AnyStatement<'static> {
        AnyStatement::<'static> {
            sql: Cow::Owned(self.sql.clone().into_owned()),
            column_names: self.column_names.clone(),
            parameters: self.parameters.clone(),
            columns: self.columns.clone(),
        }
    }

    fn sql(&self) -> &str {
        &self.sql
    }

    fn parameters(&self) -> Option<Either<&[AnyTypeInfo], usize>> {
        match &self.parameters {
            Some(Either::Left(types)) => Some(Either::Left(types)),
            Some(Either::Right(count)) => Some(Either::Right(*count)),
            None => None,
        }
    }

    fn columns(&self) -> &[AnyColumn] {
        &self.columns
    }

    impl_statement_query!(AnyArguments<'_>);
}

impl<'i> ColumnIndex<AnyStatement<'_>> for &'i str {
    fn index(&self, statement: &AnyStatement<'_>) -> Result<usize, Error> {
        statement
            .column_names
            .get(*self)
            .ok_or_else(|| Error::ColumnNotFound((*self).into()))
            .copied()
    }
}

impl<'q> AnyStatement<'q> {
    #[doc(hidden)]
    pub fn try_from_statement<S>(
        query: &'q str,
        statement: &S,
        column_names: Arc<HashMap<UStr, usize>>,
    ) -> crate::Result<Self>
    where
        S: Statement<'q>,
        AnyTypeInfo: for<'a> TryFrom<&'a <S::Database as Database>::TypeInfo, Error = Error>,
        AnyColumn: for<'a> TryFrom<&'a <S::Database as Database>::Column, Error = Error>,
    {
        let parameters = match statement.parameters() {
            Some(Either::Left(parameters)) => Some(Either::Left(
                parameters
                    .iter()
                    .map(AnyTypeInfo::try_from)
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            Some(Either::Right(count)) => Some(Either::Right(count)),
            None => None,
        };

        let columns = statement
            .columns()
            .iter()
            .map(AnyColumn::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            sql: query.into(),
            columns,
            column_names,
            parameters,
        })
    }
}
