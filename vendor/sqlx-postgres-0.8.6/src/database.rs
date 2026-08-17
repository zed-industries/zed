use crate::arguments::PgArgumentBuffer;
use crate::value::{PgValue, PgValueRef};
use crate::{
    PgArguments, PgColumn, PgConnection, PgQueryResult, PgRow, PgStatement, PgTransactionManager,
    PgTypeInfo,
};

pub(crate) use sqlx_core::database::{Database, HasStatementCache};

/// PostgreSQL database driver.
#[derive(Debug)]
pub struct Postgres;

impl Database for Postgres {
    type Connection = PgConnection;

    type TransactionManager = PgTransactionManager;

    type Row = PgRow;

    type QueryResult = PgQueryResult;

    type Column = PgColumn;

    type TypeInfo = PgTypeInfo;

    type Value = PgValue;
    type ValueRef<'r> = PgValueRef<'r>;

    type Arguments<'q> = PgArguments;
    type ArgumentBuffer<'q> = PgArgumentBuffer;

    type Statement<'q> = PgStatement<'q>;

    const NAME: &'static str = "PostgreSQL";

    const URL_SCHEMES: &'static [&'static str] = &["postgres", "postgresql"];
}

impl HasStatementCache for Postgres {}
