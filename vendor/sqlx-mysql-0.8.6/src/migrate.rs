use std::str::FromStr;
use std::time::Duration;
use std::time::Instant;

use futures_core::future::BoxFuture;
pub(crate) use sqlx_core::migrate::*;

use crate::connection::{ConnectOptions, Connection};
use crate::error::Error;
use crate::executor::Executor;
use crate::query::query;
use crate::query_as::query_as;
use crate::query_scalar::query_scalar;
use crate::{MySql, MySqlConnectOptions, MySqlConnection};

fn parse_for_maintenance(url: &str) -> Result<(MySqlConnectOptions, String), Error> {
    let mut options = MySqlConnectOptions::from_str(url)?;

    let database = if let Some(database) = &options.database {
        database.to_owned()
    } else {
        return Err(Error::Configuration(
            "DATABASE_URL does not specify a database".into(),
        ));
    };

    // switch us to <no> database for create/drop commands
    options.database = None;

    Ok((options, database))
}

impl MigrateDatabase for MySql {
    fn create_database(url: &str) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async move {
            let (options, database) = parse_for_maintenance(url)?;
            let mut conn = options.connect().await?;

            let _ = conn
                .execute(&*format!("CREATE DATABASE `{database}`"))
                .await?;

            Ok(())
        })
    }

    fn database_exists(url: &str) -> BoxFuture<'_, Result<bool, Error>> {
        Box::pin(async move {
            let (options, database) = parse_for_maintenance(url)?;
            let mut conn = options.connect().await?;

            let exists: bool = query_scalar(
                "select exists(SELECT 1 from INFORMATION_SCHEMA.SCHEMATA WHERE SCHEMA_NAME = ?)",
            )
            .bind(database)
            .fetch_one(&mut conn)
            .await?;

            Ok(exists)
        })
    }

    fn drop_database(url: &str) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async move {
            let (options, database) = parse_for_maintenance(url)?;
            let mut conn = options.connect().await?;

            let _ = conn
                .execute(&*format!("DROP DATABASE IF EXISTS `{database}`"))
                .await?;

            Ok(())
        })
    }
}

impl Migrate for MySqlConnection {
    fn ensure_migrations_table(&mut self) -> BoxFuture<'_, Result<(), MigrateError>> {
        Box::pin(async move {
            // language=MySQL
            self.execute(
                r#"
CREATE TABLE IF NOT EXISTS _sqlx_migrations (
    version BIGINT PRIMARY KEY,
    description TEXT NOT NULL,
    installed_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    success BOOLEAN NOT NULL,
    checksum BLOB NOT NULL,
    execution_time BIGINT NOT NULL
);
                "#,
            )
            .await?;

            Ok(())
        })
    }

    fn dirty_version(&mut self) -> BoxFuture<'_, Result<Option<i64>, MigrateError>> {
        Box::pin(async move {
            // language=SQL
            let row: Option<(i64,)> = query_as(
                "SELECT version FROM _sqlx_migrations WHERE success = false ORDER BY version LIMIT 1",
            )
            .fetch_optional(self)
            .await?;

            Ok(row.map(|r| r.0))
        })
    }

    fn list_applied_migrations(
        &mut self,
    ) -> BoxFuture<'_, Result<Vec<AppliedMigration>, MigrateError>> {
        Box::pin(async move {
            // language=SQL
            let rows: Vec<(i64, Vec<u8>)> =
                query_as("SELECT version, checksum FROM _sqlx_migrations ORDER BY version")
                    .fetch_all(self)
                    .await?;

            let migrations = rows
                .into_iter()
                .map(|(version, checksum)| AppliedMigration {
                    version,
                    checksum: checksum.into(),
                })
                .collect();

            Ok(migrations)
        })
    }

    fn lock(&mut self) -> BoxFuture<'_, Result<(), MigrateError>> {
        Box::pin(async move {
            let database_name = current_database(self).await?;
            let lock_id = generate_lock_id(&database_name);

            // create an application lock over the database
            // this function will not return until the lock is acquired

            // https://www.postgresql.org/docs/current/explicit-locking.html#ADVISORY-LOCKS
            // https://www.postgresql.org/docs/current/functions-admin.html#FUNCTIONS-ADVISORY-LOCKS-TABLE

            // language=MySQL
            let _ = query("SELECT GET_LOCK(?, -1)")
                .bind(lock_id)
                .execute(self)
                .await?;

            Ok(())
        })
    }

    fn unlock(&mut self) -> BoxFuture<'_, Result<(), MigrateError>> {
        Box::pin(async move {
            let database_name = current_database(self).await?;
            let lock_id = generate_lock_id(&database_name);

            // language=MySQL
            let _ = query("SELECT RELEASE_LOCK(?)")
                .bind(lock_id)
                .execute(self)
                .await?;

            Ok(())
        })
    }

    fn apply<'e: 'm, 'm>(
        &'e mut self,
        migration: &'m Migration,
    ) -> BoxFuture<'m, Result<Duration, MigrateError>> {
        Box::pin(async move {
            // Use a single transaction for the actual migration script and the essential bookeeping so we never
            // execute migrations twice. See https://github.com/launchbadge/sqlx/issues/1966.
            // The `execution_time` however can only be measured for the whole transaction. This value _only_ exists for
            // data lineage and debugging reasons, so it is not super important if it is lost. So we initialize it to -1
            // and update it once the actual transaction completed.
            let mut tx = self.begin().await?;
            let start = Instant::now();

            // For MySQL we cannot really isolate migrations due to implicit commits caused by table modification, see
            // https://dev.mysql.com/doc/refman/8.0/en/implicit-commit.html
            //
            // To somewhat try to detect this, we first insert the migration into the migration table with
            // `success=FALSE` and later modify the flag.
            //
            // language=MySQL
            let _ = query(
                r#"
    INSERT INTO _sqlx_migrations ( version, description, success, checksum, execution_time )
    VALUES ( ?, ?, FALSE, ?, -1 )
                "#,
            )
            .bind(migration.version)
            .bind(&*migration.description)
            .bind(&*migration.checksum)
            .execute(&mut *tx)
            .await?;

            let _ = tx
                .execute(&*migration.sql)
                .await
                .map_err(|e| MigrateError::ExecuteMigration(e, migration.version))?;

            // language=MySQL
            let _ = query(
                r#"
    UPDATE _sqlx_migrations
    SET success = TRUE
    WHERE version = ?
                "#,
            )
            .bind(migration.version)
            .execute(&mut *tx)
            .await?;

            tx.commit().await?;

            // Update `elapsed_time`.
            // NOTE: The process may disconnect/die at this point, so the elapsed time value might be lost. We accept
            //       this small risk since this value is not super important.

            let elapsed = start.elapsed();

            #[allow(clippy::cast_possible_truncation)]
            let _ = query(
                r#"
    UPDATE _sqlx_migrations
    SET execution_time = ?
    WHERE version = ?
                "#,
            )
            .bind(elapsed.as_nanos() as i64)
            .bind(migration.version)
            .execute(self)
            .await?;

            Ok(elapsed)
        })
    }

    fn revert<'e: 'm, 'm>(
        &'e mut self,
        migration: &'m Migration,
    ) -> BoxFuture<'m, Result<Duration, MigrateError>> {
        Box::pin(async move {
            // Use a single transaction for the actual migration script and the essential bookeeping so we never
            // execute migrations twice. See https://github.com/launchbadge/sqlx/issues/1966.
            let mut tx = self.begin().await?;
            let start = Instant::now();

            // For MySQL we cannot really isolate migrations due to implicit commits caused by table modification, see
            // https://dev.mysql.com/doc/refman/8.0/en/implicit-commit.html
            //
            // To somewhat try to detect this, we first insert the migration into the migration table with
            // `success=FALSE` and later remove the migration altogether.
            //
            // language=MySQL
            let _ = query(
                r#"
    UPDATE _sqlx_migrations
    SET success = FALSE
    WHERE version = ?
                "#,
            )
            .bind(migration.version)
            .execute(&mut *tx)
            .await?;

            tx.execute(&*migration.sql).await?;

            // language=SQL
            let _ = query(r#"DELETE FROM _sqlx_migrations WHERE version = ?"#)
                .bind(migration.version)
                .execute(&mut *tx)
                .await?;

            tx.commit().await?;

            let elapsed = start.elapsed();

            Ok(elapsed)
        })
    }
}

async fn current_database(conn: &mut MySqlConnection) -> Result<String, MigrateError> {
    // language=MySQL
    Ok(query_scalar("SELECT DATABASE()").fetch_one(conn).await?)
}

// inspired from rails: https://github.com/rails/rails/blob/6e49cc77ab3d16c06e12f93158eaf3e507d4120e/activerecord/lib/active_record/migration.rb#L1308
fn generate_lock_id(database_name: &str) -> String {
    const CRC_IEEE: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
    // 0x3d32ad9e chosen by fair dice roll
    format!(
        "{:x}",
        0x3d32ad9e * (CRC_IEEE.checksum(database_name.as_bytes()) as i64)
    )
}
