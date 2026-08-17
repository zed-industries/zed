use crate::database::Database;
use either::Either;
use std::convert::identity;

/// Provides extended information on a statement.
///
/// Returned from [`Executor::describe`].
///
/// The query macros (e.g., `query!`, `query_as!`, etc.) use the information here to validate
/// output and parameter types; and, generate an anonymous record.
#[derive(Debug)]
#[cfg_attr(feature = "offline", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "offline",
    serde(bound(
        serialize = "DB::TypeInfo: serde::Serialize, DB::Column: serde::Serialize",
        deserialize = "DB::TypeInfo: serde::de::DeserializeOwned, DB::Column: serde::de::DeserializeOwned",
    ))
)]
#[doc(hidden)]
pub struct Describe<DB: Database> {
    pub columns: Vec<DB::Column>,
    pub parameters: Option<Either<Vec<DB::TypeInfo>, usize>>,
    pub nullable: Vec<Option<bool>>,
}

impl<DB: Database> Describe<DB> {
    /// Gets all columns in this statement.
    pub fn columns(&self) -> &[DB::Column] {
        &self.columns
    }

    /// Gets the column information at `index`.
    ///
    /// Panics if `index` is out of bounds.
    pub fn column(&self, index: usize) -> &DB::Column {
        &self.columns[index]
    }

    /// Gets the available information for parameters in this statement.
    ///
    /// Some drivers may return more or less than others. As an example, **PostgreSQL** will
    /// return `Some(Either::Left(_))` with a full list of type information for each parameter.
    /// However, **MSSQL** will return `None` as there is no information available.
    pub fn parameters(&self) -> Option<Either<&[DB::TypeInfo], usize>> {
        self.parameters.as_ref().map(|p| match p {
            Either::Left(params) => Either::Left(&**params),
            Either::Right(count) => Either::Right(*count),
        })
    }

    /// Gets whether a column may be `NULL`, if this information is available.
    pub fn nullable(&self, column: usize) -> Option<bool> {
        self.nullable.get(column).copied().and_then(identity)
    }
}

#[cfg(feature = "any")]
impl<DB: Database> Describe<DB> {
    #[doc(hidden)]
    pub fn try_into_any(self) -> crate::Result<Describe<crate::any::Any>>
    where
        crate::any::AnyColumn: for<'a> TryFrom<&'a DB::Column, Error = crate::Error>,
        crate::any::AnyTypeInfo: for<'a> TryFrom<&'a DB::TypeInfo, Error = crate::Error>,
    {
        use crate::any::AnyTypeInfo;

        let columns = self
            .columns
            .iter()
            .map(crate::any::AnyColumn::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let parameters = match self.parameters {
            Some(Either::Left(parameters)) => Some(Either::Left(
                parameters
                    .iter()
                    .enumerate()
                    .map(|(i, type_info)| {
                        AnyTypeInfo::try_from(type_info).map_err(|_| {
                            crate::Error::AnyDriverError(
                                format!(
                                    "Any driver does not support type {type_info} of parameter {i}"
                                )
                                .into(),
                            )
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            Some(Either::Right(count)) => Some(Either::Right(count)),
            None => None,
        };

        Ok(Describe {
            columns,
            parameters,
            nullable: self.nullable,
        })
    }
}
