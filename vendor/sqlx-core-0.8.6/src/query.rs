use std::marker::PhantomData;

use either::Either;
use futures_core::stream::BoxStream;
use futures_util::{future, StreamExt, TryFutureExt, TryStreamExt};

use crate::arguments::{Arguments, IntoArguments};
use crate::database::{Database, HasStatementCache};
use crate::encode::Encode;
use crate::error::{BoxDynError, Error};
use crate::executor::{Execute, Executor};
use crate::statement::Statement;
use crate::types::Type;

/// A single SQL query as a prepared statement. Returned by [`query()`].
#[must_use = "query must be executed to affect database"]
pub struct Query<'q, DB: Database, A> {
    pub(crate) statement: Either<&'q str, &'q DB::Statement<'q>>,
    pub(crate) arguments: Option<Result<A, BoxDynError>>,
    pub(crate) database: PhantomData<DB>,
    pub(crate) persistent: bool,
}

/// A single SQL query that will map its results to an owned Rust type.
///
/// Executes as a prepared statement.
///
/// Returned by [`Query::try_map`], `query!()`, etc. Has most of the same methods as [`Query`] but
/// the return types are changed to reflect the mapping. However, there is no equivalent of
/// [`Query::execute`] as it doesn't make sense to map the result type and then ignore it.
///
/// [`Query::bind`] is also omitted; stylistically we recommend placing your `.bind()` calls
/// before `.try_map()`. This is also to prevent adding superfluous binds to the result of
/// `query!()` et al.
#[must_use = "query must be executed to affect database"]
pub struct Map<'q, DB: Database, F, A> {
    inner: Query<'q, DB, A>,
    mapper: F,
}

impl<'q, DB, A> Execute<'q, DB> for Query<'q, DB, A>
where
    DB: Database,
    A: Send + IntoArguments<'q, DB>,
{
    #[inline]
    fn sql(&self) -> &'q str {
        match self.statement {
            Either::Right(statement) => statement.sql(),
            Either::Left(sql) => sql,
        }
    }

    fn statement(&self) -> Option<&DB::Statement<'q>> {
        match self.statement {
            Either::Right(statement) => Some(statement),
            Either::Left(_) => None,
        }
    }

    #[inline]
    fn take_arguments(&mut self) -> Result<Option<<DB as Database>::Arguments<'q>>, BoxDynError> {
        self.arguments
            .take()
            .transpose()
            .map(|option| option.map(IntoArguments::into_arguments))
    }

    #[inline]
    fn persistent(&self) -> bool {
        self.persistent
    }
}

impl<'q, DB: Database> Query<'q, DB, <DB as Database>::Arguments<'q>> {
    /// Bind a value for use with this SQL query.
    ///
    /// If the number of times this is called does not match the number of bind parameters that
    /// appear in the query (`?` for most SQL flavors, `$1 .. $N` for Postgres) then an error
    /// will be returned when this query is executed.
    ///
    /// There is no validation that the value is of the type expected by the query. Most SQL
    /// flavors will perform type coercion (Postgres will return a database error).
    ///
    /// If encoding the value fails, the error is stored and later surfaced when executing the query.
    pub fn bind<T: 'q + Encode<'q, DB> + Type<DB>>(mut self, value: T) -> Self {
        let Ok(arguments) = self.get_arguments() else {
            return self;
        };

        let argument_number = arguments.len() + 1;
        if let Err(error) = arguments.add(value) {
            self.arguments = Some(Err(format!(
                "Encoding argument ${argument_number} failed: {error}"
            )
            .into()));
        }

        self
    }

    /// Like [`Query::try_bind`] but immediately returns an error if encoding the value failed.
    pub fn try_bind<T: 'q + Encode<'q, DB> + Type<DB>>(
        &mut self,
        value: T,
    ) -> Result<(), BoxDynError> {
        let arguments = self.get_arguments()?;

        arguments.add(value)
    }

    fn get_arguments(&mut self) -> Result<&mut DB::Arguments<'q>, BoxDynError> {
        let Some(Ok(arguments)) = self.arguments.as_mut().map(Result::as_mut) else {
            return Err("A previous call to Query::bind produced an error"
                .to_owned()
                .into());
        };

        Ok(arguments)
    }
}

impl<'q, DB, A> Query<'q, DB, A>
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
        self.persistent = value;
        self
    }
}

impl<'q, DB, A: Send> Query<'q, DB, A>
where
    DB: Database,
    A: 'q + IntoArguments<'q, DB>,
{
    /// Map each row in the result to another type.
    ///
    /// See [`try_map`](Query::try_map) for a fallible version of this method.
    ///
    /// The [`query_as`](super::query_as::query_as) method will construct a mapped query using
    /// a [`FromRow`](super::from_row::FromRow) implementation.
    #[inline]
    pub fn map<F, O>(
        self,
        mut f: F,
    ) -> Map<'q, DB, impl FnMut(DB::Row) -> Result<O, Error> + Send, A>
    where
        F: FnMut(DB::Row) -> O + Send,
        O: Unpin,
    {
        self.try_map(move |row| Ok(f(row)))
    }

    /// Map each row in the result to another type.
    ///
    /// The [`query_as`](super::query_as::query_as) method will construct a mapped query using
    /// a [`FromRow`](super::from_row::FromRow) implementation.
    #[inline]
    pub fn try_map<F, O>(self, f: F) -> Map<'q, DB, F, A>
    where
        F: FnMut(DB::Row) -> Result<O, Error> + Send,
        O: Unpin,
    {
        Map {
            inner: self,
            mapper: f,
        }
    }

    /// Execute the query and return the total number of rows affected.
    #[inline]
    pub async fn execute<'e, 'c: 'e, E>(self, executor: E) -> Result<DB::QueryResult, Error>
    where
        'q: 'e,
        A: 'e,
        E: Executor<'c, Database = DB>,
    {
        executor.execute(self).await
    }

    /// Execute multiple queries and return the rows affected from each query, in a stream.
    #[inline]
    #[deprecated = "Only the SQLite driver supports multiple statements in one prepared statement and that behavior is deprecated. Use `sqlx::raw_sql()` instead. See https://github.com/launchbadge/sqlx/issues/3108 for discussion."]
    pub async fn execute_many<'e, 'c: 'e, E>(
        self,
        executor: E,
    ) -> BoxStream<'e, Result<DB::QueryResult, Error>>
    where
        'q: 'e,
        A: 'e,
        E: Executor<'c, Database = DB>,
    {
        executor.execute_many(self)
    }

    /// Execute the query and return the generated results as a stream.
    #[inline]
    pub fn fetch<'e, 'c: 'e, E>(self, executor: E) -> BoxStream<'e, Result<DB::Row, Error>>
    where
        'q: 'e,
        A: 'e,
        E: Executor<'c, Database = DB>,
    {
        executor.fetch(self)
    }

    /// Execute multiple queries and return the generated results as a stream.
    ///
    /// For each query in the stream, any generated rows are returned first,
    /// then the `QueryResult` with the number of rows affected.
    #[inline]
    #[deprecated = "Only the SQLite driver supports multiple statements in one prepared statement and that behavior is deprecated. Use `sqlx::raw_sql()` instead. See https://github.com/launchbadge/sqlx/issues/3108 for discussion."]
    // TODO: we'll probably still want a way to get the `DB::QueryResult` at the end of a `fetch()` stream.
    pub fn fetch_many<'e, 'c: 'e, E>(
        self,
        executor: E,
    ) -> BoxStream<'e, Result<Either<DB::QueryResult, DB::Row>, Error>>
    where
        'q: 'e,
        A: 'e,
        E: Executor<'c, Database = DB>,
    {
        executor.fetch_many(self)
    }

    /// Execute the query and return all the resulting rows collected into a [`Vec`].
    ///
    /// ### Note: beware result set size.
    /// This will attempt to collect the full result set of the query into memory.
    ///
    /// To avoid exhausting available memory, ensure the result set has a known upper bound,
    /// e.g. using `LIMIT`.
    #[inline]
    pub async fn fetch_all<'e, 'c: 'e, E>(self, executor: E) -> Result<Vec<DB::Row>, Error>
    where
        'q: 'e,
        A: 'e,
        E: Executor<'c, Database = DB>,
    {
        executor.fetch_all(self).await
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
    pub async fn fetch_one<'e, 'c: 'e, E>(self, executor: E) -> Result<DB::Row, Error>
    where
        'q: 'e,
        A: 'e,
        E: Executor<'c, Database = DB>,
    {
        executor.fetch_one(self).await
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
    pub async fn fetch_optional<'e, 'c: 'e, E>(self, executor: E) -> Result<Option<DB::Row>, Error>
    where
        'q: 'e,
        A: 'e,
        E: Executor<'c, Database = DB>,
    {
        executor.fetch_optional(self).await
    }
}

impl<'q, DB, F: Send, A: Send> Execute<'q, DB> for Map<'q, DB, F, A>
where
    DB: Database,
    A: IntoArguments<'q, DB>,
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
        self.inner.arguments.is_some()
    }
}

impl<'q, DB, F, O, A> Map<'q, DB, F, A>
where
    DB: Database,
    F: FnMut(DB::Row) -> Result<O, Error> + Send,
    O: Send + Unpin,
    A: 'q + Send + IntoArguments<'q, DB>,
{
    /// Map each row in the result to another type.
    ///
    /// See [`try_map`](Map::try_map) for a fallible version of this method.
    ///
    /// The [`query_as`](super::query_as::query_as) method will construct a mapped query using
    /// a [`FromRow`](super::from_row::FromRow) implementation.
    #[inline]
    pub fn map<G, P>(
        self,
        mut g: G,
    ) -> Map<'q, DB, impl FnMut(DB::Row) -> Result<P, Error> + Send, A>
    where
        G: FnMut(O) -> P + Send,
        P: Unpin,
    {
        self.try_map(move |data| Ok(g(data)))
    }

    /// Map each row in the result to another type.
    ///
    /// The [`query_as`](super::query_as::query_as) method will construct a mapped query using
    /// a [`FromRow`](super::from_row::FromRow) implementation.
    #[inline]
    pub fn try_map<G, P>(
        self,
        mut g: G,
    ) -> Map<'q, DB, impl FnMut(DB::Row) -> Result<P, Error> + Send, A>
    where
        G: FnMut(O) -> Result<P, Error> + Send,
        P: Unpin,
    {
        let mut f = self.mapper;
        Map {
            inner: self.inner,
            mapper: move |row| f(row).and_then(&mut g),
        }
    }

    /// Execute the query and return the generated results as a stream.
    pub fn fetch<'e, 'c: 'e, E>(self, executor: E) -> BoxStream<'e, Result<O, Error>>
    where
        'q: 'e,
        E: 'e + Executor<'c, Database = DB>,
        DB: 'e,
        F: 'e,
        O: 'e,
    {
        // FIXME: this should have used `executor.fetch()` but that's a breaking change
        // because this technically allows multiple statements in one query string.
        #[allow(deprecated)]
        self.fetch_many(executor)
            .try_filter_map(|step| async move {
                Ok(match step {
                    Either::Left(_) => None,
                    Either::Right(o) => Some(o),
                })
            })
            .boxed()
    }

    /// Execute multiple queries and return the generated results as a stream
    /// from each query, in a stream.
    #[deprecated = "Only the SQLite driver supports multiple statements in one prepared statement and that behavior is deprecated. Use `sqlx::raw_sql()` instead."]
    pub fn fetch_many<'e, 'c: 'e, E>(
        mut self,
        executor: E,
    ) -> BoxStream<'e, Result<Either<DB::QueryResult, O>, Error>>
    where
        'q: 'e,
        E: 'e + Executor<'c, Database = DB>,
        DB: 'e,
        F: 'e,
        O: 'e,
    {
        Box::pin(try_stream! {
            let mut s = executor.fetch_many(self.inner);

            while let Some(v) = s.try_next().await? {
                r#yield!(match v {
                    Either::Left(v) => Either::Left(v),
                    Either::Right(row) => {
                        Either::Right((self.mapper)(row)?)
                    }
                });
            }

            Ok(())
        })
    }

    /// Execute the query and return all the resulting rows collected into a [`Vec`].
    ///
    /// ### Note: beware result set size.
    /// This will attempt to collect the full result set of the query into memory.
    ///
    /// To avoid exhausting available memory, ensure the result set has a known upper bound,
    /// e.g. using `LIMIT`.
    pub async fn fetch_all<'e, 'c: 'e, E>(self, executor: E) -> Result<Vec<O>, Error>
    where
        'q: 'e,
        E: 'e + Executor<'c, Database = DB>,
        DB: 'e,
        F: 'e,
        O: 'e,
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
        F: 'e,
        O: 'e,
    {
        self.fetch_optional(executor)
            .and_then(|row| match row {
                Some(row) => future::ok(row),
                None => future::err(Error::RowNotFound),
            })
            .await
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
    pub async fn fetch_optional<'e, 'c: 'e, E>(mut self, executor: E) -> Result<Option<O>, Error>
    where
        'q: 'e,
        E: 'e + Executor<'c, Database = DB>,
        DB: 'e,
        F: 'e,
        O: 'e,
    {
        let row = executor.fetch_optional(self.inner).await?;

        if let Some(row) = row {
            (self.mapper)(row).map(Some)
        } else {
            Ok(None)
        }
    }
}

/// Execute a single SQL query as a prepared statement (explicitly created).
pub fn query_statement<'q, DB>(
    statement: &'q DB::Statement<'q>,
) -> Query<'q, DB, <DB as Database>::Arguments<'_>>
where
    DB: Database,
{
    Query {
        database: PhantomData,
        arguments: Some(Ok(Default::default())),
        statement: Either::Right(statement),
        persistent: true,
    }
}

/// Execute a single SQL query as a prepared statement (explicitly created), with the given arguments.
pub fn query_statement_with<'q, DB, A>(
    statement: &'q DB::Statement<'q>,
    arguments: A,
) -> Query<'q, DB, A>
where
    DB: Database,
    A: IntoArguments<'q, DB>,
{
    Query {
        database: PhantomData,
        arguments: Some(Ok(arguments)),
        statement: Either::Right(statement),
        persistent: true,
    }
}

/// Execute a single SQL query as a prepared statement (transparently cached).
///
/// The query string may only contain a single DML statement: `SELECT`, `INSERT`, `UPDATE`, `DELETE` and variants.
/// The SQLite driver does not currently follow this restriction, but that behavior is deprecated.
///
/// The connection will transparently prepare and cache the statement, which means it only needs to be parsed once
/// in the connection's lifetime, and any generated query plans can be retained.
/// Thus, the overhead of executing the statement is amortized.
///
/// Some third-party databases that speak a supported protocol, e.g. CockroachDB or PGBouncer that speak Postgres,
/// may have issues with the transparent caching of prepared statements. If you are having trouble,
/// try setting [`.persistent(false)`][Query::persistent].
///
/// See the [`Query`] type for the methods you may call.
///
/// ### Dynamic Input: Use Query Parameters (Prevents SQL Injection)
/// At some point, you'll likely want to include some form of dynamic input in your query, possibly from the user.
///
/// Your first instinct might be to do something like this:
/// ```rust,no_run
/// # async fn example() -> sqlx::Result<()> {
/// # let mut conn: sqlx::PgConnection = unimplemented!();
/// // Imagine this is input from the user, e.g. a search form on a website.
/// let user_input = "possibly untrustworthy input!";
///
/// // DO NOT DO THIS unless you're ABSOLUTELY CERTAIN it's what you need!
/// let query = format!("SELECT * FROM articles WHERE content LIKE '%{user_input}%'");
/// // where `conn` is `PgConnection` or `MySqlConnection`
/// // or some other type that implements `Executor`.
/// let results = sqlx::query(&query).fetch_all(&mut conn).await?;
/// # Ok(())
/// # }
/// ```
///
/// The example above showcases a **SQL injection vulnerability**, because it's trivial for a malicious user to craft
/// an input that can "break out" of the string literal.
///
/// For example, if they send the input `foo'; DELETE FROM articles; --`
/// then your application would send the following to the database server (line breaks added for clarity):
///
/// ```sql
/// SELECT * FROM articles WHERE content LIKE '%foo';
/// DELETE FROM articles;
/// --%'
/// ```
///
/// In this case, because this interface *always* uses prepared statements, you would likely be fine because prepared
/// statements _generally_ (see above) are only allowed to contain a single query. This would simply return an error.
///
/// However, it would also break on legitimate user input.
/// What if someone wanted to search for the string `Alice's Apples`? It would also return an error because
/// the database would receive a query with a broken string literal (line breaks added for clarity):
///
/// ```sql
/// SELECT * FROM articles WHERE content LIKE '%Alice'
/// s Apples%'
/// ```
///
/// Of course, it's possible to make this syntactically valid by escaping the apostrophe, but there's a better way.
///
/// ##### You should always prefer query parameters for dynamic input.
///
/// When using query parameters, you add placeholders to your query where a value
/// should be substituted at execution time, then call [`.bind()`][Query::bind] with that value.
///
/// The syntax for placeholders is unfortunately not standardized and depends on the database:
///
/// * Postgres and SQLite: use `$1`, `$2`, `$3`, etc.
///     * The number is the Nth bound value, starting from one.
///     * The same placeholder can be used arbitrarily many times to refer to the same bound value.
///     * SQLite technically supports MySQL's syntax as well as others, but we recommend using this syntax
///       as SQLx's SQLite driver is written with it in mind.
/// * MySQL and MariaDB: use `?`.
///     * Placeholders are purely positional, similar to `println!("{}, {}", foo, bar)`.
///     * The order of bindings must match the order of placeholders in the query.
///     * To use a value in multiple places, you must bind it multiple times.
///
/// In both cases, the placeholder syntax acts as a variable expression representing the bound value:
///
/// ```rust,no_run
/// # async fn example2() -> sqlx::Result<()> {
/// # let mut conn: sqlx::PgConnection = unimplemented!();
/// let user_input = "Alice's Apples";
///
/// // Postgres and SQLite
/// let results = sqlx::query(
///     // Notice how we only have to bind the argument once and we can use it multiple times:
///     "SELECT * FROM articles
///      WHERE title LIKE '%' || $1 || '%'
///      OR content LIKE '%' || $1 || '%'"
/// )
///     .bind(user_input)
///     .fetch_all(&mut conn)
///     .await?;
///
/// // MySQL and MariaDB
/// let results = sqlx::query(
///     "SELECT * FROM articles
///      WHERE title LIKE CONCAT('%', ?, '%')
///      OR content LIKE CONCAT('%', ?, '%')"
/// )
///     // If we want to reference the same value multiple times, we have to bind it multiple times:
///     .bind(user_input)
///     .bind(user_input)
///     .fetch_all(&mut conn)
///     .await?;
/// # Ok(())
/// # }
/// ```
/// ##### The value bound to a query parameter is entirely separate from the query and does not affect its syntax.
/// Thus, SQL injection is impossible (barring shenanigans like calling a SQL function that lets you execute a string
/// as a statement) and *all* strings are valid.
///
/// This also means you cannot use query parameters to add conditional SQL fragments.
///
/// **SQLx does not substitute placeholders on the client side**. It is done by the database server itself.
///
/// ##### SQLx supports many different types for parameter binding, not just strings.
/// Any type that implements [`Encode<DB>`][Encode] and [`Type<DB>`] can be bound as a parameter.
///
/// See [the `types` module][crate::types] (links to `sqlx_core::types` but you should use `sqlx::types`) for details.
///
/// As an additional benefit, query parameters are usually sent in a compact binary encoding instead of a human-readable
/// text encoding, which saves bandwidth.
pub fn query<DB>(sql: &str) -> Query<'_, DB, <DB as Database>::Arguments<'_>>
where
    DB: Database,
{
    Query {
        database: PhantomData,
        arguments: Some(Ok(Default::default())),
        statement: Either::Left(sql),
        persistent: true,
    }
}

/// Execute a SQL query as a prepared statement (transparently cached), with the given arguments.
///
/// See [`query()`][query] for details, such as supported syntax.
pub fn query_with<'q, DB, A>(sql: &'q str, arguments: A) -> Query<'q, DB, A>
where
    DB: Database,
    A: IntoArguments<'q, DB>,
{
    query_with_result(sql, Ok(arguments))
}

/// Same as [`query_with`] but is initialized with a Result of arguments instead
pub fn query_with_result<'q, DB, A>(
    sql: &'q str,
    arguments: Result<A, BoxDynError>,
) -> Query<'q, DB, A>
where
    DB: Database,
    A: IntoArguments<'q, DB>,
{
    Query {
        database: PhantomData,
        arguments: Some(arguments),
        statement: Either::Left(sql),
        persistent: true,
    }
}
