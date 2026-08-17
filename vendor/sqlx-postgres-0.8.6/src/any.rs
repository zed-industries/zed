use crate::{
    Either, PgColumn, PgConnectOptions, PgConnection, PgQueryResult, PgRow, PgTransactionManager,
    PgTypeInfo, Postgres,
};
use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;
use futures_util::{stream, StreamExt, TryFutureExt, TryStreamExt};
use std::borrow::Cow;
use std::{future, pin::pin};

use sqlx_core::any::{
    Any, AnyArguments, AnyColumn, AnyConnectOptions, AnyConnectionBackend, AnyQueryResult, AnyRow,
    AnyStatement, AnyTypeInfo, AnyTypeInfoKind,
};

use crate::type_info::PgType;
use sqlx_core::connection::Connection;
use sqlx_core::database::Database;
use sqlx_core::describe::Describe;
use sqlx_core::executor::Executor;
use sqlx_core::ext::ustr::UStr;
use sqlx_core::transaction::TransactionManager;

sqlx_core::declare_driver_with_optional_migrate!(DRIVER = Postgres);

impl AnyConnectionBackend for PgConnection {
    fn name(&self) -> &str {
        <Postgres as Database>::NAME
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, sqlx_core::Result<()>> {
        Connection::close(*self)
    }

    fn close_hard(self: Box<Self>) -> BoxFuture<'static, sqlx_core::Result<()>> {
        Connection::close_hard(*self)
    }

    fn ping(&mut self) -> BoxFuture<'_, sqlx_core::Result<()>> {
        Connection::ping(self)
    }

    fn begin(
        &mut self,
        statement: Option<Cow<'static, str>>,
    ) -> BoxFuture<'_, sqlx_core::Result<()>> {
        PgTransactionManager::begin(self, statement)
    }

    fn commit(&mut self) -> BoxFuture<'_, sqlx_core::Result<()>> {
        PgTransactionManager::commit(self)
    }

    fn rollback(&mut self) -> BoxFuture<'_, sqlx_core::Result<()>> {
        PgTransactionManager::rollback(self)
    }

    fn start_rollback(&mut self) {
        PgTransactionManager::start_rollback(self)
    }

    fn get_transaction_depth(&self) -> usize {
        PgTransactionManager::get_transaction_depth(self)
    }

    fn shrink_buffers(&mut self) {
        Connection::shrink_buffers(self);
    }

    fn flush(&mut self) -> BoxFuture<'_, sqlx_core::Result<()>> {
        Connection::flush(self)
    }

    fn should_flush(&self) -> bool {
        Connection::should_flush(self)
    }

    #[cfg(feature = "migrate")]
    fn as_migrate(
        &mut self,
    ) -> sqlx_core::Result<&mut (dyn sqlx_core::migrate::Migrate + Send + 'static)> {
        Ok(self)
    }

    fn fetch_many<'q>(
        &'q mut self,
        query: &'q str,
        persistent: bool,
        arguments: Option<AnyArguments<'q>>,
    ) -> BoxStream<'q, sqlx_core::Result<Either<AnyQueryResult, AnyRow>>> {
        let persistent = persistent && arguments.is_some();
        let arguments = match arguments.as_ref().map(AnyArguments::convert_to).transpose() {
            Ok(arguments) => arguments,
            Err(error) => {
                return stream::once(future::ready(Err(sqlx_core::Error::Encode(error)))).boxed()
            }
        };

        Box::pin(
            self.run(query, arguments, persistent, None)
                .try_flatten_stream()
                .map(
                    move |res: sqlx_core::Result<Either<PgQueryResult, PgRow>>| match res? {
                        Either::Left(result) => Ok(Either::Left(map_result(result))),
                        Either::Right(row) => Ok(Either::Right(AnyRow::try_from(&row)?)),
                    },
                ),
        )
    }

    fn fetch_optional<'q>(
        &'q mut self,
        query: &'q str,
        persistent: bool,
        arguments: Option<AnyArguments<'q>>,
    ) -> BoxFuture<'q, sqlx_core::Result<Option<AnyRow>>> {
        let persistent = persistent && arguments.is_some();
        let arguments = arguments
            .as_ref()
            .map(AnyArguments::convert_to)
            .transpose()
            .map_err(sqlx_core::Error::Encode);

        Box::pin(async move {
            let arguments = arguments?;
            let mut stream = pin!(self.run(query, arguments, persistent, None).await?);

            if let Some(Either::Right(row)) = stream.try_next().await? {
                return Ok(Some(AnyRow::try_from(&row)?));
            }

            Ok(None)
        })
    }

    fn prepare_with<'c, 'q: 'c>(
        &'c mut self,
        sql: &'q str,
        _parameters: &[AnyTypeInfo],
    ) -> BoxFuture<'c, sqlx_core::Result<AnyStatement<'q>>> {
        Box::pin(async move {
            let statement = Executor::prepare_with(self, sql, &[]).await?;
            AnyStatement::try_from_statement(
                sql,
                &statement,
                statement.metadata.column_names.clone(),
            )
        })
    }

    fn describe<'q>(&'q mut self, sql: &'q str) -> BoxFuture<'q, sqlx_core::Result<Describe<Any>>> {
        Box::pin(async move {
            let describe = Executor::describe(self, sql).await?;

            let columns = describe
                .columns
                .iter()
                .map(AnyColumn::try_from)
                .collect::<Result<Vec<_>, _>>()?;

            let parameters = match describe.parameters {
                Some(Either::Left(parameters)) => Some(Either::Left(
                    parameters
                        .iter()
                        .enumerate()
                        .map(|(i, type_info)| {
                            AnyTypeInfo::try_from(type_info).map_err(|_| {
                                sqlx_core::Error::AnyDriverError(
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
                nullable: describe.nullable,
            })
        })
    }
}

impl<'a> TryFrom<&'a PgTypeInfo> for AnyTypeInfo {
    type Error = sqlx_core::Error;

    fn try_from(pg_type: &'a PgTypeInfo) -> Result<Self, Self::Error> {
        Ok(AnyTypeInfo {
            kind: match &pg_type.0 {
                PgType::Bool => AnyTypeInfoKind::Bool,
                PgType::Void => AnyTypeInfoKind::Null,
                PgType::Int2 => AnyTypeInfoKind::SmallInt,
                PgType::Int4 => AnyTypeInfoKind::Integer,
                PgType::Int8 => AnyTypeInfoKind::BigInt,
                PgType::Float4 => AnyTypeInfoKind::Real,
                PgType::Float8 => AnyTypeInfoKind::Double,
                PgType::Bytea => AnyTypeInfoKind::Blob,
                PgType::Text | PgType::Varchar => AnyTypeInfoKind::Text,
                PgType::DeclareWithName(UStr::Static("citext")) => AnyTypeInfoKind::Text,
                _ => {
                    return Err(sqlx_core::Error::AnyDriverError(
                        format!("Any driver does not support the Postgres type {pg_type:?}").into(),
                    ))
                }
            },
        })
    }
}

impl<'a> TryFrom<&'a PgColumn> for AnyColumn {
    type Error = sqlx_core::Error;

    fn try_from(col: &'a PgColumn) -> Result<Self, Self::Error> {
        let type_info =
            AnyTypeInfo::try_from(&col.type_info).map_err(|e| sqlx_core::Error::ColumnDecode {
                index: col.name.to_string(),
                source: e.into(),
            })?;

        Ok(AnyColumn {
            ordinal: col.ordinal,
            name: col.name.clone(),
            type_info,
        })
    }
}

impl<'a> TryFrom<&'a PgRow> for AnyRow {
    type Error = sqlx_core::Error;

    fn try_from(row: &'a PgRow) -> Result<Self, Self::Error> {
        AnyRow::map_from(row, row.metadata.column_names.clone())
    }
}

impl<'a> TryFrom<&'a AnyConnectOptions> for PgConnectOptions {
    type Error = sqlx_core::Error;

    fn try_from(value: &'a AnyConnectOptions) -> Result<Self, Self::Error> {
        let mut opts = PgConnectOptions::parse_from_url(&value.database_url)?;
        opts.log_settings = value.log_settings.clone();
        Ok(opts)
    }
}

fn map_result(res: PgQueryResult) -> AnyQueryResult {
    AnyQueryResult {
        rows_affected: res.rows_affected(),
        last_insert_id: None,
    }
}
