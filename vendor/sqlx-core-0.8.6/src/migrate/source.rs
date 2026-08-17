use crate::error::BoxDynError;
use crate::migrate::{Migration, MigrationType};
use futures_core::future::BoxFuture;

use std::borrow::Cow;
use std::fmt::Debug;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// In the default implementation, a MigrationSource is a directory which
/// contains the migration SQL scripts. All these scripts must be stored in
/// files with names using the format `<VERSION>_<DESCRIPTION>.sql`, where
/// `<VERSION>` is a string that can be parsed into `i64` and its value is
/// greater than zero, and `<DESCRIPTION>` is a string.
///
/// Files that don't match this format are silently ignored.
///
/// You can create a new empty migration script using sqlx-cli:
/// `sqlx migrate add <DESCRIPTION>`.
///
/// Note that migrations for each database are tracked using the
/// `_sqlx_migrations` table (stored in the database). If a migration's hash
/// changes and it has already been run, this will cause an error.
pub trait MigrationSource<'s>: Debug {
    fn resolve(self) -> BoxFuture<'s, Result<Vec<Migration>, BoxDynError>>;
}

impl<'s> MigrationSource<'s> for &'s Path {
    fn resolve(self) -> BoxFuture<'s, Result<Vec<Migration>, BoxDynError>> {
        Box::pin(async move {
            let canonical = self.canonicalize()?;
            let migrations_with_paths =
                crate::rt::spawn_blocking(move || resolve_blocking(&canonical)).await?;

            Ok(migrations_with_paths.into_iter().map(|(m, _p)| m).collect())
        })
    }
}

impl MigrationSource<'static> for PathBuf {
    fn resolve(self) -> BoxFuture<'static, Result<Vec<Migration>, BoxDynError>> {
        Box::pin(async move { self.as_path().resolve().await })
    }
}

#[derive(thiserror::Error, Debug)]
#[error("{message}")]
pub struct ResolveError {
    message: String,
    #[source]
    source: Option<io::Error>,
}

// FIXME: paths should just be part of `Migration` but we can't add a field backwards compatibly
// since it's `#[non_exhaustive]`.
pub fn resolve_blocking(path: &Path) -> Result<Vec<(Migration, PathBuf)>, ResolveError> {
    let s = fs::read_dir(path).map_err(|e| ResolveError {
        message: format!("error reading migration directory {}: {e}", path.display()),
        source: Some(e),
    })?;

    let mut migrations = Vec::new();

    for res in s {
        let entry = res.map_err(|e| ResolveError {
            message: format!(
                "error reading contents of migration directory {}: {e}",
                path.display()
            ),
            source: Some(e),
        })?;

        let entry_path = entry.path();

        let metadata = fs::metadata(&entry_path).map_err(|e| ResolveError {
            message: format!(
                "error getting metadata of migration path {}",
                entry_path.display()
            ),
            source: Some(e),
        })?;

        if !metadata.is_file() {
            // not a file; ignore
            continue;
        }

        let file_name = entry.file_name();
        // This is arguably the wrong choice,
        // but it really only matters for parsing the version and description.
        //
        // Using `.to_str()` and returning an error if the filename is not UTF-8
        // would be a breaking change.
        let file_name = file_name.to_string_lossy();

        let parts = file_name.splitn(2, '_').collect::<Vec<_>>();

        if parts.len() != 2 || !parts[1].ends_with(".sql") {
            // not of the format: <VERSION>_<DESCRIPTION>.<REVERSIBLE_DIRECTION>.sql; ignore
            continue;
        }

        let version: i64 = parts[0].parse()
            .map_err(|_e| ResolveError {
                message: format!("error parsing migration filename {file_name:?}; expected integer version prefix (e.g. `01_foo.sql`)"),
                source: None,
            })?;

        let migration_type = MigrationType::from_filename(parts[1]);

        // remove the `.sql` and replace `_` with ` `
        let description = parts[1]
            .trim_end_matches(migration_type.suffix())
            .replace('_', " ")
            .to_owned();

        let sql = fs::read_to_string(&entry_path).map_err(|e| ResolveError {
            message: format!(
                "error reading contents of migration {}: {e}",
                entry_path.display()
            ),
            source: Some(e),
        })?;

        // opt-out of migration transaction
        let no_tx = sql.starts_with("-- no-transaction");

        migrations.push((
            Migration::new(
                version,
                Cow::Owned(description),
                migration_type,
                Cow::Owned(sql),
                no_tx,
            ),
            entry_path,
        ));
    }

    // Ensure that we are sorted by version in ascending order.
    migrations.sort_by_key(|(m, _)| m.version);

    Ok(migrations)
}
