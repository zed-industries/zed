use rusqlite_migration::{Migrations, M};

lazy_static::lazy_static! {
    pub static ref MIGRATIONS: Migrations<'static> = Migrations::new(vec![M::up(
        "CREATE TABLE kv_store(
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        ) STRICT;",
    )]);
}
