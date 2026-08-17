use crate::error::Error;
use crate::migrate::{AppliedMigration, MigrateError, Migration};
use futures_core::future::BoxFuture;
use std::time::Duration;

pub trait MigrateDatabase {
    // create database in url
    // uses a maintenance database depending on driver
    fn create_database(url: &str) -> BoxFuture<'_, Result<(), Error>>;

    // check if the database in url exists
    // uses a maintenance database depending on driver
    fn database_exists(url: &str) -> BoxFuture<'_, Result<bool, Error>>;

    // drop database in url
    // uses a maintenance database depending on driver
    fn drop_database(url: &str) -> BoxFuture<'_, Result<(), Error>>;

    // force drop database in url
    // uses a maintenance database depending on driver
    fn force_drop_database(_url: &str) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async { Err(MigrateError::ForceNotSupported)? })
    }
}

// 'e = Executor
pub trait Migrate {
    // ensure migrations table exists
    // will create or migrate it if needed
    fn ensure_migrations_table(&mut self) -> BoxFuture<'_, Result<(), MigrateError>>;

    // Return the version on which the database is dirty or None otherwise.
    // "dirty" means there is a partially applied migration that failed.
    fn dirty_version(&mut self) -> BoxFuture<'_, Result<Option<i64>, MigrateError>>;

    // Return the ordered list of applied migrations
    fn list_applied_migrations(
        &mut self,
    ) -> BoxFuture<'_, Result<Vec<AppliedMigration>, MigrateError>>;

    // Should acquire a database lock so that only one migration process
    // can run at a time. [`Migrate`] will call this function before applying
    // any migrations.
    fn lock(&mut self) -> BoxFuture<'_, Result<(), MigrateError>>;

    // Should release the lock. [`Migrate`] will call this function after all
    // migrations have been run.
    fn unlock(&mut self) -> BoxFuture<'_, Result<(), MigrateError>>;

    // run SQL from migration in a DDL transaction
    // insert new row to [_migrations] table on completion (success or failure)
    // returns the time taking to run the migration SQL
    fn apply<'e: 'm, 'm>(
        &'e mut self,
        migration: &'m Migration,
    ) -> BoxFuture<'m, Result<Duration, MigrateError>>;

    // run a revert SQL from migration in a DDL transaction
    // deletes the row in [_migrations] table with specified migration version on completion (success or failure)
    // returns the time taking to run the migration SQL
    fn revert<'e: 'm, 'm>(
        &'e mut self,
        migration: &'m Migration,
    ) -> BoxFuture<'m, Result<Duration, MigrateError>>;
}
