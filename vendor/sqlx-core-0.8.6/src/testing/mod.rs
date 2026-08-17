use std::future::Future;
use std::time::Duration;

use futures_core::future::BoxFuture;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
pub use fixtures::FixtureSnapshot;
use sha2::{Digest, Sha512};

use crate::connection::{ConnectOptions, Connection};
use crate::database::Database;
use crate::error::Error;
use crate::executor::Executor;
use crate::migrate::{Migrate, Migrator};
use crate::pool::{Pool, PoolConnection, PoolOptions};

mod fixtures;

pub trait TestSupport: Database {
    /// Get parameters to construct a `Pool` suitable for testing.
    ///
    /// This `Pool` instance will behave somewhat specially:
    /// * all handles share a single global semaphore to avoid exceeding the connection limit
    ///   on the database server.
    /// * each invocation results in a different temporary database.
    ///
    /// The implementation may require `DATABASE_URL` to be set in order to manage databases.
    /// The user credentials it contains must have the privilege to create and drop databases.
    fn test_context(args: &TestArgs) -> BoxFuture<'_, Result<TestContext<Self>, Error>>;

    fn cleanup_test(db_name: &str) -> BoxFuture<'_, Result<(), Error>>;

    /// Cleanup any test databases that are no longer in-use.
    ///
    /// Returns a count of the databases deleted, if possible.
    ///
    /// The implementation may require `DATABASE_URL` to be set in order to manage databases.
    /// The user credentials it contains must have the privilege to create and drop databases.
    fn cleanup_test_dbs() -> BoxFuture<'static, Result<Option<usize>, Error>>;

    /// Take a snapshot of the current state of the database (data only).
    ///
    /// This snapshot can then be used to generate test fixtures.
    fn snapshot(conn: &mut Self::Connection)
        -> BoxFuture<'_, Result<FixtureSnapshot<Self>, Error>>;

    /// Generate a unique database name for the given test path.
    fn db_name(args: &TestArgs) -> String {
        let mut hasher = Sha512::new();
        hasher.update(args.test_path.as_bytes());
        let hash = hasher.finalize();
        let hash = URL_SAFE.encode(&hash[..39]);
        let db_name = format!("_sqlx_test_{}", hash).replace('-', "_");
        debug_assert!(db_name.len() == 63);
        db_name
    }
}

pub struct TestFixture {
    pub path: &'static str,
    pub contents: &'static str,
}

pub struct TestArgs {
    pub test_path: &'static str,
    pub migrator: Option<&'static Migrator>,
    pub fixtures: &'static [TestFixture],
}

pub trait TestFn {
    type Output;

    fn run_test(self, args: TestArgs) -> Self::Output;
}

pub trait TestTermination {
    fn is_success(&self) -> bool;
}

pub struct TestContext<DB: Database> {
    pub pool_opts: PoolOptions<DB>,
    pub connect_opts: <DB::Connection as Connection>::Options,
    pub db_name: String,
}

impl<DB, Fut> TestFn for fn(Pool<DB>) -> Fut
where
    DB: TestSupport + Database,
    DB::Connection: Migrate,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB>,
    Fut: Future,
    Fut::Output: TestTermination,
{
    type Output = Fut::Output;

    fn run_test(self, args: TestArgs) -> Self::Output {
        run_test_with_pool(args, self)
    }
}

impl<DB, Fut> TestFn for fn(PoolConnection<DB>) -> Fut
where
    DB: TestSupport + Database,
    DB::Connection: Migrate,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB>,
    Fut: Future,
    Fut::Output: TestTermination,
{
    type Output = Fut::Output;

    fn run_test(self, args: TestArgs) -> Self::Output {
        run_test_with_pool(args, |pool| async move {
            let conn = pool
                .acquire()
                .await
                .expect("failed to acquire test pool connection");
            let res = (self)(conn).await;
            pool.close().await;
            res
        })
    }
}

impl<DB, Fut> TestFn for fn(PoolOptions<DB>, <DB::Connection as Connection>::Options) -> Fut
where
    DB: Database + TestSupport,
    DB::Connection: Migrate,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB>,
    Fut: Future,
    Fut::Output: TestTermination,
{
    type Output = Fut::Output;

    fn run_test(self, args: TestArgs) -> Self::Output {
        run_test(args, self)
    }
}

impl<Fut> TestFn for fn() -> Fut
where
    Fut: Future,
{
    type Output = Fut::Output;

    fn run_test(self, args: TestArgs) -> Self::Output {
        assert!(
            args.fixtures.is_empty(),
            "fixtures cannot be applied for a bare function"
        );
        crate::rt::test_block_on(self())
    }
}

impl TestArgs {
    pub fn new(test_path: &'static str) -> Self {
        TestArgs {
            test_path,
            migrator: None,
            fixtures: &[],
        }
    }

    pub fn migrator(&mut self, migrator: &'static Migrator) {
        self.migrator = Some(migrator);
    }

    pub fn fixtures(&mut self, fixtures: &'static [TestFixture]) {
        self.fixtures = fixtures;
    }
}

impl TestTermination for () {
    fn is_success(&self) -> bool {
        true
    }
}

impl<T, E> TestTermination for Result<T, E> {
    fn is_success(&self) -> bool {
        self.is_ok()
    }
}

fn run_test_with_pool<DB, F, Fut>(args: TestArgs, test_fn: F) -> Fut::Output
where
    DB: TestSupport,
    DB::Connection: Migrate,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB>,
    F: FnOnce(Pool<DB>) -> Fut,
    Fut: Future,
    Fut::Output: TestTermination,
{
    let test_path = args.test_path;
    run_test::<DB, _, _>(args, |pool_opts, connect_opts| async move {
        let pool = pool_opts
            .connect_with(connect_opts)
            .await
            .expect("failed to connect test pool");

        let res = test_fn(pool.clone()).await;

        let close_timed_out = crate::rt::timeout(Duration::from_secs(10), pool.close())
            .await
            .is_err();

        if close_timed_out {
            eprintln!("test {test_path} held onto Pool after exiting");
        }

        res
    })
}

fn run_test<DB, F, Fut>(args: TestArgs, test_fn: F) -> Fut::Output
where
    DB: TestSupport,
    DB::Connection: Migrate,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB>,
    F: FnOnce(PoolOptions<DB>, <DB::Connection as Connection>::Options) -> Fut,
    Fut: Future,
    Fut::Output: TestTermination,
{
    crate::rt::test_block_on(async move {
        let test_context = DB::test_context(&args)
            .await
            .expect("failed to connect to setup test database");

        setup_test_db::<DB>(&test_context.connect_opts, &args).await;

        let res = test_fn(test_context.pool_opts, test_context.connect_opts).await;

        if res.is_success() {
            if let Err(e) = DB::cleanup_test(&DB::db_name(&args)).await {
                eprintln!(
                    "failed to delete database {:?}: {}",
                    test_context.db_name, e
                );
            }
        }

        res
    })
}

async fn setup_test_db<DB: Database>(
    copts: &<DB::Connection as Connection>::Options,
    args: &TestArgs,
) where
    DB::Connection: Migrate + Sized,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB>,
{
    let mut conn = copts
        .connect()
        .await
        .expect("failed to connect to test database");

    if let Some(migrator) = args.migrator {
        migrator
            .run_direct(&mut conn)
            .await
            .expect("failed to apply migrations");
    }

    for fixture in args.fixtures {
        (&mut conn)
            .execute(fixture.contents)
            .await
            .unwrap_or_else(|e| panic!("failed to apply test fixture {:?}: {:?}", fixture.path, e));
    }

    conn.close()
        .await
        .expect("failed to close setup connection");
}
