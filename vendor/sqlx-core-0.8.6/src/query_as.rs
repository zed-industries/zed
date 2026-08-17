use std::marker::PhantomData;

use either::Either;
use futures_core::stream::BoxStream;
use futures_util::{StreamExt, TryStreamExt};

use crate::arguments::IntoArguments;
use crate::database::{Database, HasStatementCache};
use crate::encode::Encode;
use crate::error::{BoxDynError, Error};
use crate::executor::{Execute, Executor};
use crate::from_row::FromRow;
use crate::query::{query, query_statement, query_statement_with, query_with_result, Query};
use crate::types::Type;

/// A single SQL query as a prepared statement, mapping results using [`FromRow`].
/// Returned by [`query_as()`].
#[must_use = "query must be executed to affect database"]
pub struct QueryAs<'q, DB: Database, O, A> {
    pub(crate) inner: Query<'q, DB, A>,
    pub(crate) output: PhantomData<O>,
}

impl<'q, DB, O: Send, A: Send> Execute<'q, DB> for QueryAs<'q, DB, O, A>
where
    DB: Database,
    A: 'q + IntoArguments<'q, DB>,
{
    #[inline]
    fn sql(&self) -> &'q str {
        self.inner.sql()
    }

    #[inline]
    fn statement(&self) -> Option<&DB::Statement<'q>> {
        self.inner.statement()
    }

    #[inline]
    fn take_arguments(&mut self) -> Result<Option<<DB as Database>::Arguments<'q>>, BoxDynError> {
        self.inner.take_arguments()
    }

    #[inline]
    fn persistent(&self) -> bool {
        self.inner.persistent()
    }
}

impl<'q, DB: Database, O> QueryAs<'q, DB, O, <DB as Database>::Arguments<'q>> {
    /// Bind a value for use with this SQL query.
    ///
    /// See [`Query::bind`](Query::bind).
    pub fn bind<T: 'q + Encode<'q, DB> + Type<DB>>(mut self, value: T) -> Self {
        self.inner = self.inner.bind(value);
        self
    }
}

impl<'q, DB, O, A> QueryAs<'q, DB, O, A>
where
    DB: Database + HasStatementCache,
{
    /// If `true`, the statement will get prepared once and cached to the
    /// connection's statement cache.
    ///
    /// If queried once with the flag set to `true`, all subsequent queries
    /// matching the one with the flag will use the cached statement until the
    /// cache is cleared.
    ///
    /// If `false`, the prepared statement will be closed after execution.
    ///
    /// Default: `true`.
    pub fn persistent(mut self, value: bool) -> Self {
        self.inner = self.inner.persistent(value);
        self
    }
}

// FIXME: This is very close, nearly 1:1 with `Map`
// noinspection DuplicatedCode
impl<'q, DB, O, A> QueryAs<'q, DB, O, A>
where
    DB: Database,
    A: 'q + IntoArguments<'q, DB>,
    O: Send + Unpin + for<'r> FromRow<'r, DB::Row>,
{
    /// Execute the query and return the generated results as a stream.
    pub fn fetch<'e, 'c: 'e, E>(self, executor: E) -> BoxStream<'e, Result<O, Error>>
    where
        'q: 'e,
        E: 'e + Executor<'c, Database = DB>,
        DB: 'e,
        O: 'e,
        A: 'e,
    {
        // FIXME: this should have used `executor.fetch()` but that's a breaking change
        // because this technically allows multiple statements in one query string.
        #[allow(deprecated)]
        self.fetch_many(executor)
            .try_filter_map(|step| async move { Ok(step.right()) })
            .boxed()
    }

    /// Execute multiple queries and return the generated results as a stream
    /// from each query, in a stream.
    #[deprecated = "Only the SQLite driver supports multiple statements in one prepared statement and that behavior is deprecated. Use `sqlx::raw_sql()` instead. See https://github.com/launchbadge/sqlx/issues/3108 for discussion."]
    pub fn fetch_many<'e, 'c: 'e, E>(
        self,
        executor: E,
    ) -> BoxStream<'e, Result<Either<DB::QueryResult, O>, Error>>
    where
        'q: 'e,
        E: 'e + Executor<'c, Database = DB>,
        DB: 'e,
        O: 'e,
        A: 'e,
    {
        executor
            .fetch_many(self.inner)
            .map(|v| match v {
                Ok(Either::Right(row)) => O::from_row(&row).map(Either::Right),
                Ok(Either::Left(v)) => Ok(Either::Left(v)),
                Err(e) => Err(e),
            })
            .boxed()
    }

    /// Execute the query and return all the resulting rows collected into a [`Vec`].
    ///
    /// ### Note: beware result set size.
    /// This will attempt to collect the full result set of the query into memory.
    ///
    /// To avoid exhausting available memory, ensure the result set has a known upper bound,
    /// e.g. using `LIMIT`.
    #[inline]
    pub async fn fetch_all<'e, 'c: 'e, E>(self, executor: E) -> Result<Vec<O>, Error>
    where
        'q: 'e,
        E: 'e + Executor<'c, Database = DB>,
        DB: 'e,
        O: 'e,
        A: 'e,
    {
        self.fetch(executor).try_collect().await
    }

    /// Execute the query, returning the first row or [`Error::RowNotFound`] otherwise.
    ///
    /// ### Note: for best performance, ensure the query returns at most one row.
    /// Depending on the driver implementation, if your query can return more than one row,
    /// it may lead to wasted CPU time and bandwidth on the database server.
    ///
    /// Even when the driver implementation takes this into account, ensuring the query returns at most one row
    /// can result in a more optimal query plan.
    ///
    /// If your query has a `WHERE` clause filtering a unique column by a single value, you're good.
    ///
    /// Otherwise, you might want to add `LIMIT 1` to your query.
    pub async fn fetch_one<'e, 'c: 'e, E>(self, executor: E) -> Result<O, Error>
    where
        'q: 'e,
        E: 'e + Executor<'c, Database = DB>,
        DB: 'e,
        O: 'e,
        A: 'e,
    {
        self.fetch_optional(executor)
            .await
            .and_then(|row| row.ok_or(Error::RowNotFound))
    }

    /// Execute the query, returning the first row or `None` otherwise.
    ///
    /// ### Note: for best performance, ensure the query returns at most one row.
    /// Depending on the driver implementation, if your query can return more than one row,
    /// it may lead to wasted CPU time and bandwidth on the database server.
    ///
    /// Even when the driver implementation takes this into account, ensuring the query returns at most one row
    /// can result in a more optimal query plan.
    ///
    /// If your query has a `WHERE` clause filtering a unique column by a single value, you're good.
    ///
    /// Otherwise, you might want to add `LIMIT 1` to your query.
    pub async fn fetch_optional<'e, 'c: 'e, E>(self, executor: E) -> Result<Option<O>, Error>
    where
        'q: 'e,
        E: 'e + Executor<'c, Database = DB>,
        DB: 'e,
        O: 'e,
        A: 'e,
    {
        let row = executor.fetch_optional(self.inner).await?;
        if let Some(row) = row {
            O::from_row(&row).map(Some)
        } else {
            Ok(None)
        }
    }
}

/// Execute a single SQL query as a prepared statement (transparently cached).
/// Maps rows to Rust types using [`FromRow`].
///
/// For details about prepared statements and allowed SQL syntax, see [`query()`][crate::query::query].
///
/// ### Example: Map Rows using Tuples
/// [`FromRow`] is implemented for tuples of up to 16 elements<sup>1</sup>.
/// Using a tuple of N elements will extract the first N columns from each row using [`Decode`][crate::decode::Decode].
/// Any extra columns are ignored.
///
/// See [`sqlx::types`][crate::types] for the types that can be used.
///
/// The `FromRow` implementation will check [`Type::compatible()`] for each column to ensure a compatible type mapping
/// is used. If an incompatible mapping is detected, an error is returned.
/// To statically assert compatible types at compile time, see the `query!()` family of macros.
///
/// **NOTE**: `SELECT *` is not recommended with this approach because the ordering of returned columns may be different
/// than expected, especially when using joins.
///
/// ```rust,no_run
/// # async fn example1() -> sqlx::Result<()> {
/// use sqlx::Connection;
/// use sqlx::PgConnection;
///
/// // This example can be applied to any database as it only uses standard types and syntax.
/// let mut conn: PgConnection = PgConnection::connect("<Database URL>").await?;
///
/// sqlx::raw_sql(
///     "CREATE TABLE users(id INTEGER PRIMARY KEY, username TEXT UNIQUE, created_at TIMESTAMPTZ DEFAULT (now()))"
/// )
///     .execute(&mut conn)
///     .await?;
///
/// sqlx::query("INSERT INTO users(id, username) VALUES (1, 'alice'), (2, 'bob');")
///     .execute(&mut conn)
///     .await?;
///
/// // Get the first row of the result (note the `LIMIT 1` for efficiency)
/// // This assumes the `time` feature of SQLx is enabled.
/// let oldest_user: (i32, String, time::OffsetDateTime) = sqlx::query_as(
///     "SELECT id, username, created_at FROM users ORDER BY created_at LIMIT 1"
/// )
///     .fetch_one(&mut conn)
///     .await?;
///
/// assert_eq!(oldest_user.0, 1);
/// assert_eq!(oldest_user.1, "alice");
///
/// // Get at most one row
/// let maybe_charlie: Option<(i32, String, time::OffsetDateTime)> = sqlx::query_as(
///     "SELECT id, username, created_at FROM users WHERE username = 'charlie'"
/// )
///     .fetch_optional(&mut conn)
///     .await?;
///
/// assert_eq!(maybe_charlie, None);
///
/// // Get all rows in result (Beware of the size of the result set! Consider using `LIMIT`)
/// let users: Vec<(i32, String, time::OffsetDateTime)> = sqlx::query_as(
///     "SELECT id, username, created_at FROM users ORDER BY id"
/// )
///     .fetch_all(&mut conn)
///     .await?;
///
/// println!("{users:?}");
/// # Ok(())
/// # }
/// ```
///
/// <sup>1</sup>: It's impossible in Rust to implement a trait for tuples of arbitrary size.
/// For larger result sets, either use an explicit struct (see below) or use [`query()`][crate::query::query]
/// instead and extract columns dynamically.
///
/// ### Example: Map Rows using `#[derive(FromRow)]`
/// Using `#[derive(FromRow)]`, we can create a Rust struct to represent our row type
/// so we can look up fields by name instead of tuple index.
///
/// When querying this way, columns will be matched up to the corresponding fields by name, so `SELECT *` is safe to use.
/// However, you will still want to be aware of duplicate column names in your query when using joins.
///
/// The derived `FromRow` implementation will check [`Type::compatible()`] for each column to ensure a compatible type
/// mapping is used. If an incompatible mapping is detected, an error is returned.
/// To statically assert compatible types at compile time, see the `query!()` family of macros.
///
/// An error will also be returned if an expected column is missing from the result set.
///
/// `#[derive(FromRow)]` supports several control attributes which can be used to change how column names and types
/// are mapped. See [`FromRow`] for details.
///
/// Using our previous table definition, we can convert our queries like so:
/// ```rust,no_run
/// # async fn example2() -> sqlx::Result<()> {
/// use sqlx::Connection;
/// use sqlx::PgConnection;
///
/// use time::OffsetDateTime;
///
/// #[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
/// struct User {
///     id: i64,
///     username: String,
///     // Note: the derive won't compile if the `time` feature of SQLx is not enabled.
///     created_at: OffsetDateTime,
/// }
///
/// let mut conn: PgConnection = PgConnection::connect("<Database URL>").await?;
///
/// // Get the first row of the result (note the `LIMIT 1` for efficiency)
/// let oldest_user: User = sqlx::query_as(
///     "SELECT id, username, created_at FROM users ORDER BY created_at LIMIT 1"
/// )
///     .fetch_one(&mut conn)
///     .await?;
///
/// assert_eq!(oldest_user.id, 1);
/// assert_eq!(oldest_user.username, "alice");
///
/// // Get at most one row
/// let maybe_charlie: Option<User> = sqlx::query_as(
///     "SELECT id, username, created_at FROM users WHERE username = 'charlie'"
/// )
///     .fetch_optional(&mut conn)
///     .await?;
///
/// assert_eq!(maybe_charlie, None);
///
/// // Get all rows in result (Beware of the size of the result set! Consider using `LIMIT`)
/// let users: Vec<User> = sqlx::query_as(
///     "SELECT id, username, created_at FROM users ORDER BY id"
/// )
///     .fetch_all(&mut conn)
///     .await?;
///
/// assert_eq!(users[1].id, 2);
/// assert_eq!(users[1].username, "bob");
/// # Ok(())
/// # }
///
/// ```
#[inline]
pub fn query_as<'q, DB, O>(sql: &'q str) -> QueryAs<'q, DB, O, <DB as Database>::Arguments<'q>>
where
    DB: Database,
    O: for<'r> FromRow<'r, DB::Row>,
{
    QueryAs {
        inner: query(sql),
        output: PhantomData,
    }
}

/// Execute a single SQL query, with the given arguments as a prepared statement (transparently cached).
/// Maps rows to Rust types using [`FromRow`].
///
/// For details about prepared statements and allowed SQL syntax, see [`query()`][crate::query::query].
///
/// For details about type mapping from [`FromRow`], see [`query_as()`].
#[inline]
pub fn query_as_with<'q, DB, O, A>(sql: &'q str, arguments: A) -> QueryAs<'q, DB, O, A>
where
    DB: Database,
    A: IntoArguments<'q, DB>,
    O: for<'r> FromRow<'r, DB::Row>,
{
    query_as_with_result(sql, Ok(arguments))
}

/// Same as [`query_as_with`] but takes arguments as a Result
#[inline]
pub fn query_as_with_result<'q, DB, O, A>(
    sql: &'q str,
    arguments: Result<A, BoxDynError>,
) -> QueryAs<'q, DB, O, A>
where
    DB: Database,
    A: IntoArguments<'q, DB>,
    O: for<'r> FromRow<'r, DB::Row>,
{
    QueryAs {
        inner: query_with_result(sql, arguments),
        output: PhantomData,
    }
}

// Make a SQL query from a statement, that is mapped to a concrete type.
pub fn query_statement_as<'q, DB, O>(
    statement: &'q DB::Statement<'q>,
) -> QueryAs<'q, DB, O, <DB as Database>::Arguments<'_>>
where
    DB: Database,
    O: for<'r> FromRow<'r, DB::Row>,
{
    QueryAs {
        inner: query_statement(statement),
        output: PhantomData,
    }
}

// Make a SQL query from a statement, with the given arguments, that is mapped to a concrete type.
pub fn query_statement_as_with<'q, DB, O, A>(
    statement: &'q DB::Statement<'q>,
    arguments: A,
) -> QueryAs<'q, DB, O, A>
where
    DB: Database,
    A: IntoArguments<'q, DB>,
    O: for<'r> FromRow<'r, DB::Row>,
{
    QueryAs {
        inner: query_statement_with(statement, arguments),
        output: PhantomData,
    }
}
