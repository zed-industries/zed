use std::path::Path;
use std::time::Duration;

use anyhow::{Result, anyhow};
use collections::HashMap;
use sea_orm::ConnectOptions;
use sqlx::Connection;
use sqlx::migrate::{Migrate, Migration, MigrationSource};

/// Runs the database migrations for the specified database.
pub async fn run_database_migrations(
    database_options: &ConnectOptions,
    migrations_path: impl AsRef<Path>,
) -> Result<Vec<(Migration, Duration)>> {
    let migrations = MigrationSource::resolve(migrations_path.as_ref())
        .await
        .map_err(|err| anyhow!("failed to load migrations: {err:?}"))?;

    let mut connection = sqlx::AnyConnection::connect(database_options.get_url()).await?;

    connection.ensure_migrations_table().await?;
    let applied_migrations: HashMap<_, _> = connection
        .list_applied_migrations()
        .await?
        .into_iter()
        .map(|migration| (migration.version, migration))
        .collect();

    let mut new_migrations = Vec::new();
    for migration in migrations {
        match applied_migrations.get(&migration.version) {
            Some(applied_migration) => {
                if migration.checksum != applied_migration.checksum {
                    Err(anyhow!(
                        "checksum mismatch for applied migration {}",
                        migration.description
                    ))?;
                }
            }
            None => {
                let elapsed = connection.apply(&migration).await?;
                new_migrations.push((migration, elapsed));
            }
        }
    }

    Ok(new_migrations)
}
