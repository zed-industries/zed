use either::Either;
use futures_core::stream::BoxStream;
use futures_util::{StreamExt, TryFutureExt, TryStreamExt};

use crate::arguments::IntoArguments;
use crate::database::{Database, HasStatementCache};
use crate::encode::Encode;
use crate::error::{BoxDynError, Error};
use crate::executor::{Execute, Executor};
use crate::from_row::FromRow;
use crate::query_as::{
    query_as, query_as_with_result, query_statement_as, query_statement_as_with, QueryAs,
};
use crate::types::Type;

/// A single SQL query as a prepared statement which extracts only the first column of each row.
/// Returned by [`query_scalar()`].
#[must_use = "query must be executed to affect database"]
pub struct QueryScalar<'q, DB: Database, O, A> {
    pub(crate) inner: QueryAs<'q, DB, (O,), A>,
}

impl<'q, DB: Database, O: Send, A: Send> Execute<'q, DB> for QueryScalar<'q, DB, O, A>
where
    A: 'q + IntoArguments<'q, DB>,
{
    #[inline]
    fn sql(&self) -> &'q str {
        self.inner.sql()
    }

    fn statement(&self) -> Option<&DB::Statement<'q>> {
        self.inner.statement()
    }

    #[inline]
    fn take_arguments(&mut self) -> Result<Option<<DB as Database>::Arguments<'q>>, BoxDynError> {
        self.inner.take_arguments()
    }

    #[inline]
    fn persistent(&self) -> bool {
        Execute::persistent(&self.inner)
    }
}

impl<'q, DB: Database, O> QueryScalar<'q, DB, O, <DB as Database>::Arguments<'q>> {
    /// Bind a value for use with this SQL query.
    ///
    /// See [`Query::bind`](crate::query::Query::bind).
    pub fn bind<T: 'q + Encode<'q, DB> + Type<DB>>(mut self, value: T) -> Self {
        self.inner = self.inner.bind(value);
        self
    }
}

impl<'q, DB, O, A> QueryScalar<'q, DB, O, A>
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
impl<'q, DB, O, A> QueryScalar<'q, DB, O, A>
where
    DB: Database,
    O: Send + Unpin,
    A: 'q + IntoArguments<'q, DB>,
    (O,): Send + Unpin + for<'r> FromRow<'r, DB::Row>,
{
    /// Execute the query and return the generated results as a stream.
    #[inline]
    pub fn fetch<'e, 'c: 'e, E>(self, executor: E) -> BoxStream<'e, Result<O, Error>>
    where
        'q: 'e,
        E: 'e + Executor<'c, Database = DB>,
        DB: 'e,
        A: 'e,
        O: 'e,
    {
        self.inner.fetch(executor).map_ok(|it| it.0).boxed()
    }

    /// Execute multiple queries and return the generated results as a stream
    /// from each query, in a stream.
    #[inline]
    #[deprecated = "Only the SQLite driver supports multiple statements in one prepared statement and that behavior is deprecated. Use `sqlx::raw_sql()` instead. See https://github.com/launchbadge/sqlx/issues/3108 for discussion."]
    pub fn fetch_many<'e, 'c: 'e, E>(
        self,
        executor: E,
    ) -> BoxStream<'e, Result<Either<DB::QueryResult, O>, Error>>
    where
        'q: 'e,
        E: 'e + Executor<'c, Database = DB>,
        DB: 'e,
        A: 'e,
        O: 'e,
    {
        #[allow(deprecated)]
        self.inner
            .fetch_many(executor)
            .map_ok(|v| v.map_right(|it| it.0))
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
        (O,): 'e,
        A: 'e,
    {
        self.inner
            .fetch(executor)
            .map_ok(|it| it.0)
            .try_collect()
            .await
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
    #[inline]
    pub async fn fetch_one<'e, 'c: 'e, E>(self, executor: E) -> Result<O, Error>
    where
        'q: 'e,
        E: 'e + Executor<'c, Database = DB>,
        DB: 'e,
        O: 'e,
        A: 'e,
    {
        self.inner.fetch_one(executor).map_ok(|it| it.0).await
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
    #[inline]
    pub async fn fetch_optional<'e, 'c: 'e, E>(self, executor: E) -> Result<Option<O>, Error>
    where
        'q: 'e,
        E: 'e + Executor<'c, Database = DB>,
        DB: 'e,
        O: 'e,
        A: 'e,
    {
        Ok(self.inner.fetch_optional(executor).await?.map(|it| it.0))
    }
}

/// Execute a single SQL query as a prepared statement (transparently cached) and extract the first
/// column of each row.
///
/// Extracts the first column of each row. Additional columns are ignored.
/// Any type that implements `Type<DB> + Decode<DB>` may be used.
///
/// For details about prepared statements and allowed SQL syntax, see [`query()`][crate::query::query].
///
/// ### Example: Simple Lookup
/// If you just want to look up a single value with little fanfare, this API is perfect for you:
///
/// ```rust,no_run
/// # async fn example_lookup() -> Result<(), Box<dyn std::error::Error>> {
/// # let mut conn: sqlx::PgConnection = unimplemented!();
/// use uuid::Uuid;
///
/// // MySQL and MariaDB: use `?`
/// let user_id: Option<Uuid> = sqlx::query_scalar("SELECT user_id FROM users WHERE username = $1")
///     .bind("alice")
///     // Use `&mut` where `conn` is a connection or a transaction, or use `&` for a `Pool`.
///     .fetch_optional(&mut conn)
///     .await?;
///
/// let user_id = user_id.ok_or("unknown user")?;
///
/// # Ok(())
/// # }
/// ```
///
/// Note how we're using `.fetch_optional()` because the lookup may return no results,
/// in which case we need to be able to handle an empty result set.
/// Any rows after the first are ignored.
///
/// ### Example: `COUNT`
/// This API is the easiest way to invoke an aggregate query like `SELECT COUNT(*)`, because you
/// can conveniently extract the result:
///
/// ```rust,no_run
/// # async fn example_count() -> sqlx::Result<()> {
/// # let mut conn: sqlx::PgConnection = unimplemented!();
/// // Note that `usize` is not used here because unsigned integers are generally not supported,
/// // and `usize` doesn't even make sense as a mapping because the database server may have
/// // a completely different architecture.
/// //
/// // `i64` is generally a safe choice for `COUNT`.
/// let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE accepted_tos IS TRUE")
///     // Use `&mut` where `conn` is a connection or a transaction, or use `&` for a `Pool`.
///     .fetch_one(&mut conn)
///     .await?;
///
/// // The above is functionally equivalent to the following:
/// // Note the trailing comma, required for the compiler to recognize a 1-element tuple.
/// let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE accepted_tos IS TRUE")
///     .fetch_one(&mut conn)
///     .await?;
/// # Ok(())
/// # }
/// ```
///
/// ### Example: `EXISTS`
/// To test if a row exists or not, use `SELECT EXISTS(<query>)`:
///
/// ```rust,no_run
/// # async fn example_exists() -> sqlx::Result<()> {
/// # let mut conn: sqlx::PgConnection = unimplemented!();
/// // MySQL and MariaDB: use `?`
/// let username_taken: bool = sqlx::query_scalar(
///     "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)"
/// )
///     .bind("alice")
///     // Use `&mut` where `conn` is a connection or a transaction, or use `&` for a `Pool`.
///     .fetch_one(&mut conn)
///     .await?;
/// # Ok(())
/// # }
/// ```
///
/// ### Example: Other Aggregates
/// Be aware that most other aggregate functions return `NULL` if the query yields an empty set:
///
/// ```rust,no_run
/// # async fn example_aggregate() -> sqlx::Result<()> {
/// # let mut conn: sqlx::PgConnection = unimplemented!();
/// let max_upvotes: Option<i64> = sqlx::query_scalar("SELECT MAX(upvotes) FROM posts")
///     // Use `&mut` where `conn` is a connection or a transaction, or use `&` for a `Pool`.
///     .fetch_one(&mut conn)
///     .await?;
/// # Ok(())
/// # }
/// ```
///
/// Note how we're using `Option<i64>` with `.fetch_one()`, because we're always expecting one row
/// but the column value may be `NULL`. If no rows are returned, this will error.
///
/// This is in contrast to using `.fetch_optional()` with `Option<i64>`, which implies that
/// we're expecting _either_ a row with a `i64` (`BIGINT`), _or_ no rows at all.
///
/// Either way, any rows after the first are ignored.
///
/// ### Example: `Vec` of Scalars
/// If you want to collect a single column from a query into a vector,
/// try `.fetch_all()`:
///
/// ```rust,no_run
/// # async fn example_vec() -> sqlx::Result<()> {
/// # let mut conn: sqlx::PgConnection = unimplemented!();
/// let top_users: Vec<String> = sqlx::query_scalar(
///     // Note the `LIMIT` to ensure that this doesn't return *all* users:
///     "SELECT username
///      FROM (
///          SELECT SUM(upvotes) total, user_id
///          FROM posts
///          GROUP BY user_id
///      ) top_users
///      INNER JOIN users USING (user_id)
///      ORDER BY total DESC
///      LIMIT 10"
/// )
///     // Use `&mut` where `conn` is a connection or a transaction, or use `&` for a `Pool`.
///     .fetch_all(&mut conn)
///     .await?;
///
/// // `top_users` could be empty, too.
/// assert!(top_users.len() <= 10);
/// # Ok(())
/// # }
/// ```
#[inline]
pub fn query_scalar<'q, DB, O>(
    sql: &'q str,
) -> QueryScalar<'q, DB, O, <DB as Database>::Arguments<'q>>
where
    DB: Database,
    (O,): for<'r> FromRow<'r, DB::Row>,
{
    QueryScalar {
        inner: query_as(sql),
    }
}

/// Execute a SQL query as a prepared statement (transparently cached), with the given arguments,
/// and extract the first column of each row.
///
/// See [`query_scalar()`] for details.
///
/// For details about prepared statements and allowed SQL syntax, see [`query()`][crate::query::query].
#[inline]
pub fn query_scalar_with<'q, DB, O, A>(sql: &'q str, arguments: A) -> QueryScalar<'q, DB, O, A>
where
    DB: Database,
    A: IntoArguments<'q, DB>,
    (O,): for<'r> FromRow<'r, DB::Row>,
{
    query_scalar_with_result(sql, Ok(arguments))
}

/// Same as [`query_scalar_with`] but takes arguments as Result
#[inline]
pub fn query_scalar_with_result<'q, DB, O, A>(
    sql: &'q str,
    arguments: Result<A, BoxDynError>,
) -> QueryScalar<'q, DB, O, A>
where
    DB: Database,
    A: IntoArguments<'q, DB>,
    (O,): for<'r> FromRow<'r, DB::Row>,
{
    QueryScalar {
        inner: query_as_with_result(sql, arguments),
    }
}

// Make a SQL query from a statement, that is mapped to a concrete value.
pub fn query_statement_scalar<'q, DB, O>(
    statement: &'q DB::Statement<'q>,
) -> QueryScalar<'q, DB, O, <DB as Database>::Arguments<'_>>
where
    DB: Database,
    (O,): for<'r> FromRow<'r, DB::Row>,
{
    QueryScalar {
        inner: query_statement_as(statement),
    }
}

// Make a SQL query from a statement, with the given arguments, that is mapped to a concrete value.
pub fn query_statement_scalar_with<'q, DB, O, A>(
    statement: &'q DB::Statement<'q>,
    arguments: A,
) -> QueryScalar<'q, DB, O, A>
where
    DB: Database,
    A: IntoArguments<'q, DB>,
    (O,): for<'r> FromRow<'r, DB::Row>,
{
    QueryScalar {
        inner: query_statement_as_with(statement, arguments),
    }
}
