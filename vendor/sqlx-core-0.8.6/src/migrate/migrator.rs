use crate::acquire::Acquire;
use crate::migrate::{AppliedMigration, Migrate, MigrateError, Migration, MigrationSource};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::slice;

/// A resolved set of migrations, ready to be run.
///
/// Can be constructed statically using `migrate!()` or at runtime using [`Migrator::new()`].
#[derive(Debug)]
// Forbids `migrate!()` from constructing this:
// #[non_exhaustive]
pub struct Migrator {
    // NOTE: these fields are semver-exempt and may be changed or removed in any future version.
    // These have to be public for `migrate!()` to be able to initialize them in an implicitly
    // const-promotable context. A `const fn` constructor isn't implicitly const-promotable.
    #[doc(hidden)]
    pub migrations: Cow<'static, [Migration]>,
    #[doc(hidden)]
    pub ignore_missing: bool,
    #[doc(hidden)]
    pub locking: bool,
    #[doc(hidden)]
    pub no_tx: bool,
}

fn validate_applied_migrations(
    applied_migrations: &[AppliedMigration],
    migrator: &Migrator,
) -> Result<(), MigrateError> {
    if migrator.ignore_missing {
        return Ok(());
    }

    let migrations: HashSet<_> = migrator.iter().map(|m| m.version).collect();

    for applied_migration in applied_migrations {
        if !migrations.contains(&applied_migration.version) {
            return Err(MigrateError::VersionMissing(applied_migration.version));
        }
    }

    Ok(())
}

impl Migrator {
    #[doc(hidden)]
    pub const DEFAULT: Migrator = Migrator {
        migrations: Cow::Borrowed(&[]),
        ignore_missing: false,
        no_tx: false,
        locking: true,
    };

    /// Creates a new instance with the given source.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use sqlx_core::migrate::MigrateError;
    /// # fn main() -> Result<(), MigrateError> {
    /// # sqlx::__rt::test_block_on(async move {
    /// # use sqlx_core::migrate::Migrator;
    /// use std::path::Path;
    ///
    /// // Read migrations from a local folder: ./migrations
    /// let m = Migrator::new(Path::new("./migrations")).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    /// See [MigrationSource] for details on structure of the `./migrations` directory.
    pub async fn new<'s, S>(source: S) -> Result<Self, MigrateError>
    where
        S: MigrationSource<'s>,
    {
        Ok(Self {
            migrations: Cow::Owned(source.resolve().await.map_err(MigrateError::Source)?),
            ..Self::DEFAULT
        })
    }

    /// Specify whether applied migrations that are missing from the resolved migrations should be ignored.
    pub fn set_ignore_missing(&mut self, ignore_missing: bool) -> &Self {
        self.ignore_missing = ignore_missing;
        self
    }

    /// Specify whether or not to lock the database during migration. Defaults to `true`.
    ///
    /// ### Warning
    /// Disabling locking can lead to errors or data loss if multiple clients attempt to apply migrations simultaneously
    /// without some sort of mutual exclusion.
    ///
    /// This should only be used if the database does not support locking, e.g. CockroachDB which talks the Postgres
    /// protocol but does not support advisory locks used by SQLx's migrations support for Postgres.
    pub fn set_locking(&mut self, locking: bool) -> &Self {
        self.locking = locking;
        self
    }

    /// Get an iterator over all known migrations.
    pub fn iter(&self) -> slice::Iter<'_, Migration> {
        self.migrations.iter()
    }

    /// Check if a migration version exists.
    pub fn version_exists(&self, version: i64) -> bool {
        self.iter().any(|m| m.version == version)
    }

    /// Run any pending migrations against the database; and, validate previously applied migrations
    /// against the current migration source to detect accidental changes in previously-applied migrations.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use sqlx::migrate::MigrateError;
    /// # fn main() -> Result<(), MigrateError> {
    /// #     sqlx::__rt::test_block_on(async move {
    /// use sqlx::migrate::Migrator;
    /// use sqlx::sqlite::SqlitePoolOptions;
    ///
    /// let m = Migrator::new(std::path::Path::new("./migrations")).await?;
    /// let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await?;
    /// m.run(&pool).await
    /// #     })
    /// # }
    /// ```
    pub async fn run<'a, A>(&self, migrator: A) -> Result<(), MigrateError>
    where
        A: Acquire<'a>,
        <A::Connection as Deref>::Target: Migrate,
    {
        let mut conn = migrator.acquire().await?;
        self.run_direct(&mut *conn).await
    }

    // Getting around the annoying "implementation of `Acquire` is not general enough" error
    #[doc(hidden)]
    pub async fn run_direct<C>(&self, conn: &mut C) -> Result<(), MigrateError>
    where
        C: Migrate,
    {
        // lock the database for exclusive access by the migrator
        if self.locking {
            conn.lock().await?;
        }

        // creates [_migrations] table only if needed
        // eventually this will likely migrate previous versions of the table
        conn.ensure_migrations_table().await?;

        let version = conn.dirty_version().await?;
        if let Some(version) = version {
            return Err(MigrateError::Dirty(version));
        }

        let applied_migrations = conn.list_applied_migrations().await?;
        validate_applied_migrations(&applied_migrations, self)?;

        let applied_migrations: HashMap<_, _> = applied_migrations
            .into_iter()
            .map(|m| (m.version, m))
            .collect();

        for migration in self.iter() {
            if migration.migration_type.is_down_migration() {
                continue;
            }

            match applied_migrations.get(&migration.version) {
                Some(applied_migration) => {
                    if migration.checksum != applied_migration.checksum {
                        return Err(MigrateError::VersionMismatch(migration.version));
                    }
                }
                None => {
                    conn.apply(migration).await?;
                }
            }
        }

        // unlock the migrator to allow other migrators to run
        // but do nothing as we already migrated
        if self.locking {
            conn.unlock().await?;
        }

        Ok(())
    }

    /// Run down migrations against the database until a specific version.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use sqlx::migrate::MigrateError;
    /// # fn main() -> Result<(), MigrateError> {
    /// #     sqlx::__rt::test_block_on(async move {
    /// use sqlx::migrate::Migrator;
    /// use sqlx::sqlite::SqlitePoolOptions;
    ///
    /// let m = Migrator::new(std::path::Path::new("./migrations")).await?;
    /// let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await?;
    /// m.undo(&pool, 4).await
    /// #     })
    /// # }
    /// ```
    pub async fn undo<'a, A>(&self, migrator: A, target: i64) -> Result<(), MigrateError>
    where
        A: Acquire<'a>,
        <A::Connection as Deref>::Target: Migrate,
    {
        let mut conn = migrator.acquire().await?;

        // lock the database for exclusive access by the migrator
        if self.locking {
            conn.lock().await?;
        }

        // creates [_migrations] table only if needed
        // eventually this will likely migrate previous versions of the table
        conn.ensure_migrations_table().await?;

        let version = conn.dirty_version().await?;
        if let Some(version) = version {
            return Err(MigrateError::Dirty(version));
        }

        let applied_migrations = conn.list_applied_migrations().await?;
        validate_applied_migrations(&applied_migrations, self)?;

        let applied_migrations: HashMap<_, _> = applied_migrations
            .into_iter()
            .map(|m| (m.version, m))
            .collect();

        for migration in self
            .iter()
            .rev()
            .filter(|m| m.migration_type.is_down_migration())
            .filter(|m| applied_migrations.contains_key(&m.version))
            .filter(|m| m.version > target)
        {
            conn.revert(migration).await?;
        }

        // unlock the migrator to allow other migrators to run
        // but do nothing as we already migrated
        if self.locking {
            conn.unlock().await?;
        }

        Ok(())
    }
}
