// Migrations are constructed by domain, and stored in a table in the connection db with domain name,
// effected tables, actual query text, and order.
// If a migration is run and any of the query texts don't match, the app panics on startup (maybe fallback
// to creating a new db?)
// Otherwise any missing migrations are run on the connection

use anyhow::{anyhow, Result};
use indoc::{formatdoc, indoc};

use crate::connection::Connection;

const MIGRATIONS_MIGRATION: Migration = Migration::new(
    "migrations",
    // The migrations migration must be infallable because it runs to completion
    // with every call to migration run and is run unchecked.
    &[indoc! {"
        CREATE TABLE IF NOT EXISTS migrations (
            domain TEXT,
            step INTEGER,
            migration TEXT
        );
    "}],
);

pub struct Migration {
    domain: &'static str,
    migrations: &'static [&'static str],
}

impl Migration {
    pub const fn new(domain: &'static str, migrations: &'static [&'static str]) -> Self {
        Self { domain, migrations }
    }

    fn run_unchecked(&self, connection: &Connection) -> Result<()> {
        connection.exec(self.migrations.join(";\n"))
    }

    pub fn run(&self, connection: &Connection) -> Result<()> {
        // Setup the migrations table unconditionally
        MIGRATIONS_MIGRATION.run_unchecked(connection)?;

        let completed_migrations = connection
            .prepare(indoc! {"
                SELECT domain, step, migration FROM migrations
                WHERE domain = ?
                ORDER BY step
                "})?
            .bound(self.domain)?
            .rows::<(String, usize, String)>()?;

        let mut store_completed_migration = connection
            .prepare("INSERT INTO migrations (domain, step, migration) VALUES (?, ?, ?)")?;

        for (index, migration) in self.migrations.iter().enumerate() {
            if let Some((_, _, completed_migration)) = completed_migrations.get(index) {
                if completed_migration != migration {
                    return Err(anyhow!(formatdoc! {"
                        Migration changed for {} at step {}
                        
                        Stored migration:
                        {}
                        
                        Proposed migration:
                        {}", self.domain, index, completed_migration, migration}));
                } else {
                    // Migration already run. Continue
                    continue;
                }
            }

            connection.exec(migration)?;
            store_completed_migration
                .bound((self.domain, index, *migration))?
                .run()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::{connection::Connection, migrations::Migration};

    #[test]
    fn test_migrations_are_added_to_table() {
        let connection = Connection::open_memory("migrations_are_added_to_table");

        // Create first migration with a single step and run it
        let mut migration = Migration::new(
            "test",
            &[indoc! {"
            CREATE TABLE test1 (
                a TEXT,
                b TEXT
            );"}],
        );
        migration.run(&connection).unwrap();

        // Verify it got added to the migrations table
        assert_eq!(
            &connection
                .prepare("SELECT (migration) FROM migrations")
                .unwrap()
                .rows::<String>()
                .unwrap()[..],
            migration.migrations
        );

        // Add another step to the migration and run it again
        migration.migrations = &[
            indoc! {"
                CREATE TABLE test1 (
                    a TEXT,
                    b TEXT
                );"},
            indoc! {"
                CREATE TABLE test2 (
                    c TEXT,
                    d TEXT
                );"},
        ];
        migration.run(&connection).unwrap();

        // Verify it is also added to the migrations table
        assert_eq!(
            &connection
                .prepare("SELECT (migration) FROM migrations")
                .unwrap()
                .rows::<String>()
                .unwrap()[..],
            migration.migrations
        );
    }

    #[test]
    fn test_migration_setup_works() {
        let connection = Connection::open_memory("migration_setup_works");

        connection
            .exec(indoc! {"CREATE TABLE IF NOT EXISTS migrations (
                    domain TEXT,
                    step INTEGER,
                    migration TEXT
                );"})
            .unwrap();

        let mut store_completed_migration = connection
            .prepare(indoc! {"
                INSERT INTO migrations (domain, step, migration)
                VALUES (?, ?, ?)"})
            .unwrap();

        let domain = "test_domain";
        for i in 0..5 {
            // Create a table forcing a schema change
            connection
                .exec(format!("CREATE TABLE table{} ( test TEXT );", i))
                .unwrap();

            store_completed_migration
                .bound((domain, i, i.to_string()))
                .unwrap()
                .run()
                .unwrap();
        }
    }

    #[test]
    fn migrations_dont_rerun() {
        let connection = Connection::open_memory("migrations_dont_rerun");

        // Create migration which clears a table
        let migration = Migration::new("test", &["DELETE FROM test_table"]);

        // Manually create the table for that migration with a row
        connection
            .exec(indoc! {"
            CREATE TABLE test_table (
                test_column INTEGER
            );
            INSERT INTO test_table (test_column) VALUES (1)"})
            .unwrap();

        assert_eq!(
            connection
                .prepare("SELECT * FROM test_table")
                .unwrap()
                .row::<usize>()
                .unwrap(),
            1
        );

        // Run the migration verifying that the row got dropped
        migration.run(&connection).unwrap();
        assert_eq!(
            connection
                .prepare("SELECT * FROM test_table")
                .unwrap()
                .rows::<usize>()
                .unwrap(),
            Vec::new()
        );

        // Recreate the dropped row
        connection
            .exec("INSERT INTO test_table (test_column) VALUES (2)")
            .unwrap();

        // Run the same migration again and verify that the table was left unchanged
        migration.run(&connection).unwrap();
        assert_eq!(
            connection
                .prepare("SELECT * FROM test_table")
                .unwrap()
                .row::<usize>()
                .unwrap(),
            2
        );
    }

    #[test]
    fn changed_migration_fails() {
        let connection = Connection::open_memory("changed_migration_fails");

        // Create a migration with two steps and run it
        Migration::new(
            "test migration",
            &[
                indoc! {"
                CREATE TABLE test (
                    col INTEGER
                )"},
                indoc! {"
                INSERT INTO test (col) VALUES (1)"},
            ],
        )
        .run(&connection)
        .unwrap();

        // Create another migration with the same domain but different steps
        let second_migration_result = Migration::new(
            "test migration",
            &[
                indoc! {"
                CREATE TABLE test (
                    color INTEGER
                )"},
                indoc! {"
                INSERT INTO test (color) VALUES (1)"},
            ],
        )
        .run(&connection);

        // Verify new migration returns error when run
        assert!(second_migration_result.is_err())
    }
}
