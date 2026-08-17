#![cfg(unix)]
use sqlx::migrate::Migrator;
use std::path::Path;

static EMBEDDED_SIMPLE: Migrator = sqlx::migrate!("tests/migrate/migrations_simple");
static EMBEDDED_REVERSIBLE: Migrator = sqlx::migrate!("tests/migrate/migrations_reversible");
static EMBEDDED_SYMLINK: Migrator = sqlx::migrate!("tests/migrate/migrations_symlink");

#[sqlx_macros::test]
async fn same_output() -> anyhow::Result<()> {
    let runtime_simple = Migrator::new(Path::new("tests/migrate/migrations_simple")).await?;
    let runtime_reversible =
        Migrator::new(Path::new("tests/migrate/migrations_reversible")).await?;
    let runtime_symlink = Migrator::new(Path::new("tests/migrate/migrations_symlink")).await?;

    assert_same(&EMBEDDED_SIMPLE, &runtime_simple);
    assert_same(&EMBEDDED_REVERSIBLE, &runtime_reversible);
    assert_same(&EMBEDDED_SYMLINK, &runtime_symlink);

    Ok(())
}

fn assert_same(embedded: &Migrator, runtime: &Migrator) {
    assert_eq!(runtime.migrations.len(), embedded.migrations.len());

    for (e, r) in embedded.iter().zip(runtime.iter()) {
        assert_eq!(e.version, r.version);
        assert_eq!(e.description, r.description);
        assert_eq!(e.migration_type, r.migration_type);
        assert_eq!(e.sql, r.sql);
        assert_eq!(e.checksum, r.checksum);
    }
}
