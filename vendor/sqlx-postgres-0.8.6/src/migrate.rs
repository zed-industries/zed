use std::str::FromStr;
use std::time::Duration;
use std::time::Instant;

use futures_core::future::BoxFuture;

pub(crate) use sqlx_core::migrate::MigrateError;
pub(crate) use sqlx_core::migrate::{AppliedMigration, Migration};
pub(crate) use sqlx_core::migrate::{Migrate, MigrateDatabase};

use crate::connection::{ConnectOptions, Connection};
use crate::error::Error;
use crate::executor::Executor;
use crate::query::query;
use crate::query_as::query_as;
use crate::query_scalar::query_scalar;
use crate::{PgConnectOptions, PgConnection, Postgres};

fn parse_for_maintenance(url: &str) -> Result<(PgConnectOptions, String), Error> {
    let mut options = PgConnectOptions::from_str(url)?;

    // pull out the name of the database to create
    let database = options
        .database
        .as_deref()
        .unwrap_or(&options.username)
        .to_owned();

    // switch us to the maintenance database
    // use `postgres` _unless_ the database is postgres, in which case, use `template1`
    // this matches the behavior of the `createdb` util
    options.database = if database == "postgres" {
        Some("template1".into())
    } else {
        Some("postgres".into())
    };

    Ok((options, database))
}

impl MigrateDatabase for Postgres {
    fn create_database(url: &str) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async move {
            let (options, database) = parse_for_maintenance(url)?;
            let mut conn = options.connect().await?;

            let _ = conn
                .execute(&*format!(
                    "CREATE DATABASE \"{}\"",
                    database.replace('"', "\"\"")
                ))
                .await?;

            Ok(())
        })
    }

    fn database_exists(url: &str) -> BoxFuture<'_, Result<bool, Error>> {
        Box::pin(async move {
            let (options, database) = parse_for_maintenance(url)?;
            let mut conn = options.connect().await?;

            let exists: bool =
                query_scalar("select exists(SELECT 1 from pg_database WHERE datname = $1)")
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
                .execute(&*format!(
                    "DROP DATABASE IF EXISTS \"{}\"",
                    database.replace('"', "\"\"")
                ))
                .await?;

            Ok(())
        })
    }

    fn force_drop_database(url: &str) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async move {
            let (options, database) = parse_for_maintenance(url)?;
            let mut conn = options.connect().await?;

            let row: (String,) = query_as("SELECT current_setting('server_version_num')")
                .fetch_one(&mut conn)
                .await?;

            let version = row.0.parse::<i32>().unwrap();

            let pid_type = if version >= 90200 { "pid" } else { "procpid" };

            conn.execute(&*format!(
                "SELECT pg_terminate_backend(pg_stat_activity.{pid_type}) FROM pg_stat_activity \
                 WHERE pg_stat_activity.datname = '{database}' AND {pid_type} <> pg_backend_pid()"
            ))
            .await?;

            Self::drop_database(url).await
        })
    }
}

impl Migrate for PgConnection {
    fn ensure_migrations_table(&mut self) -> BoxFuture<'_, Result<(), MigrateError>> {
        Box::pin(async move {
            // language=SQL
            self.execute(
                r#"
CREATE TABLE IF NOT EXISTS _sqlx_migrations (
    version BIGINT PRIMARY KEY,
    description TEXT NOT NULL,
    installed_on TIMESTAMPTZ NOT NULL DEFAULT now(),
    success BOOLEAN NOT NULL,
    checksum BYTEA NOT NULL,
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

            // language=SQL
            let _ = query("SELECT pg_advisory_lock($1)")
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

            // language=SQL
            let _ = query("SELECT pg_advisory_unlock($1)")
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
            let start = Instant::now();

            // execute migration queries
            if migration.no_tx {
                execute_migration(self, migration).await?;
            } else {
                // Use a single transaction for the actual migration script and the essential bookeeping so we never
                // execute migrations twice. See https://github.com/launchbadge/sqlx/issues/1966.
                // The `execution_time` however can only be measured for the whole transaction. This value _only_ exists for
                // data lineage and debugging reasons, so it is not super important if it is lost. So we initialize it to -1
                // and update it once the actual transaction completed.
                let mut tx = self.begin().await?;
                execute_migration(&mut tx, migration).await?;
                tx.commit().await?;
            }

            // Update `elapsed_time`.
            // NOTE: The process may disconnect/die at this point, so the elapsed time value might be lost. We accept
            //       this small risk since this value is not super important.
            let elapsed = start.elapsed();

            // language=SQL
            #[allow(clippy::cast_possible_truncation)]
            let _ = query(
                r#"
    UPDATE _sqlx_migrations
    SET execution_time = $1
    WHERE version = $2
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
            let start = Instant::now();

            // execute migration queries
            if migration.no_tx {
                revert_migration(self, migration).await?;
            } else {
                // Use a single transaction for the actual migration script and the essential bookeeping so we never
                // execute migrations twice. See https://github.com/launchbadge/sqlx/issues/1966.
                let mut tx = self.begin().await?;
                revert_migration(&mut tx, migration).await?;
                tx.commit().await?;
            }

            let elapsed = start.elapsed();

            Ok(elapsed)
        })
    }
}

async fn execute_migration(
    conn: &mut PgConnection,
    migration: &Migration,
) -> Result<(), MigrateError> {
    let _ = conn
        .execute(&*migration.sql)
        .await
        .map_err(|e| MigrateError::ExecuteMigration(e, migration.version))?;

    // language=SQL
    let _ = query(
        r#"
    INSERT INTO _sqlx_migrations ( version, description, success, checksum, execution_time )
    VALUES ( $1, $2, TRUE, $3, -1 )
                "#,
    )
    .bind(migration.version)
    .bind(&*migration.description)
    .bind(&*migration.checksum)
    .execute(conn)
    .await?;

    Ok(())
}

async fn revert_migration(
    conn: &mut PgConnection,
    migration: &Migration,
) -> Result<(), MigrateError> {
    let _ = conn
        .execute(&*migration.sql)
        .await
        .map_err(|e| MigrateError::ExecuteMigration(e, migration.version))?;

    // language=SQL
    let _ = query(r#"DELETE FROM _sqlx_migrations WHERE version = $1"#)
        .bind(migration.version)
        .execute(conn)
        .await?;

    Ok(())
}

async fn current_database(conn: &mut PgConnection) -> Result<String, MigrateError> {
    // language=SQL
    Ok(query_scalar("SELECT current_database()")
        .fetch_one(conn)
        .await?)
}

// inspired from rails: https://github.com/rails/rails/blob/6e49cc77ab3d16c06e12f93158eaf3e507d4120e/activerecord/lib/active_record/migration.rb#L1308
fn generate_lock_id(database_name: &str) -> i64 {
    const CRC_IEEE: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
    // 0x3d32ad9e chosen by fair dice roll
    0x3d32ad9e * (CRC_IEEE.checksum(database_name.as_bytes()) as i64)
}
