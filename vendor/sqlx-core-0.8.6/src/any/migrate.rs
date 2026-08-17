use crate::any::driver;
use crate::any::{Any, AnyConnection};
use crate::error::Error;
use crate::migrate::{AppliedMigration, Migrate, MigrateDatabase, MigrateError, Migration};
use futures_core::future::BoxFuture;
use std::time::Duration;

impl MigrateDatabase for Any {
    fn create_database(url: &str) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            driver::from_url_str(url)?
                .get_migrate_database()?
                .create_database(url)
                .await
        })
    }

    fn database_exists(url: &str) -> BoxFuture<'_, Result<bool, Error>> {
        Box::pin(async {
            driver::from_url_str(url)?
                .get_migrate_database()?
                .database_exists(url)
                .await
        })
    }

    fn drop_database(url: &str) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            driver::from_url_str(url)?
                .get_migrate_database()?
                .drop_database(url)
                .await
        })
    }

    fn force_drop_database(url: &str) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            driver::from_url_str(url)?
                .get_migrate_database()?
                .force_drop_database(url)
                .await
        })
    }
}

impl Migrate for AnyConnection {
    fn ensure_migrations_table(&mut self) -> BoxFuture<'_, Result<(), MigrateError>> {
        Box::pin(async { self.get_migrate()?.ensure_migrations_table().await })
    }

    fn dirty_version(&mut self) -> BoxFuture<'_, Result<Option<i64>, MigrateError>> {
        Box::pin(async { self.get_migrate()?.dirty_version().await })
    }

    fn list_applied_migrations(
        &mut self,
    ) -> BoxFuture<'_, Result<Vec<AppliedMigration>, MigrateError>> {
        Box::pin(async { self.get_migrate()?.list_applied_migrations().await })
    }

    fn lock(&mut self) -> BoxFuture<'_, Result<(), MigrateError>> {
        Box::pin(async { self.get_migrate()?.lock().await })
    }

    fn unlock(&mut self) -> BoxFuture<'_, Result<(), MigrateError>> {
        Box::pin(async { self.get_migrate()?.unlock().await })
    }

    fn apply<'e: 'm, 'm>(
        &'e mut self,
        migration: &'m Migration,
    ) -> BoxFuture<'m, Result<Duration, MigrateError>> {
        Box::pin(async { self.get_migrate()?.apply(migration).await })
    }

    fn revert<'e: 'm, 'm>(
        &'e mut self,
        migration: &'m Migration,
    ) -> BoxFuture<'m, Result<Duration, MigrateError>> {
        Box::pin(async { self.get_migrate()?.revert(migration).await })
    }
}
