// Migrations are constructed by domain, and stored in a table in the connection db with domain name,
// effected tables, actual query text, and order.
// If a migration is run and any of the query texts don't match, the app panics on startup (maybe fallback
// to creating a new db?)
// Otherwise any missing migrations are run on the connection

use std::ffi::CString;

use anyhow::{Context as _, Result, anyhow};
use indoc::{formatdoc, indoc};
use libsqlite3_sys::sqlite3_exec;

use crate::connection::Connection;

impl Connection {
    fn eager_exec(&self, sql: &str) -> anyhow::Result<()> {
        let sql_str = CString::new(sql).context("Error creating cstr")?;
        unsafe {
            sqlite3_exec(
                self.sqlite3,
                sql_str.as_c_str().as_ptr(),
                None,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
        }
        self.last_error()
            .with_context(|| format!("Prepare call failed for query:\n{}", sql))?;

        Ok(())
    }

    /// Migrate the database, for the given domain.
    /// Note: Unlike everything else in SQLez, migrations are run eagerly, without first
    /// preparing the SQL statements. This makes it possible to do multi-statement schema
    /// updates in a single string without running into prepare errors.
    pub fn migrate(&self, domain: &'static str, migrations: &[&'static str]) -> Result<()> {
        self.with_savepoint("migrating", || {
            // Setup the migrations table unconditionally
            self.exec(indoc! {"
                CREATE TABLE IF NOT EXISTS migrations (
                    domain TEXT,
                    step INTEGER,
                    migration TEXT
                )"})?()?;

            let completed_migrations =
                self.select_bound::<&str, (String, usize, String)>(indoc! {"
                    SELECT domain, step, migration FROM migrations
                    WHERE domain = ?
                    ORDER BY step
                    "})?(domain)?;

            let mut store_completed_migration = self
                .exec_bound("INSERT INTO migrations (domain, step, migration) VALUES (?, ?, ?)")?;

            for (index, migration) in migrations.iter().enumerate() {
                let migration =
                    sqlformat::format(migration, &sqlformat::QueryParams::None, Default::default());
                if let Some((_, _, completed_migration)) = completed_migrations.get(index) {
                    // Reformat completed migrations with the current `sqlformat` version, so that past migrations stored
                    // conform to the new formatting rules.
                    let completed_migration = sqlformat::format(
                        completed_migration,
                        &sqlformat::QueryParams::None,
                        Default::default(),
                    );
                    if completed_migration == migration {
                        // Migration already run. Continue
                        continue;
                    } else {
                        return Err(anyhow!(formatdoc! {"
                            Migration changed for {} at step {}

                            Stored migration:
                            {}

                            Proposed migration:
                            {}", domain, index, completed_migration, migration}));
                    }
                }

                self.eager_exec(&migration)?;
                store_completed_migration((domain, index, migration))?;
            }

            Ok(())
        })
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::connection::Connection;

    #[test]
    fn test_migrations_are_added_to_table() {
        let connection = Connection::open_memory(Some("migrations_are_added_to_table"));

        // Create first migration with a single step and run it
        connection
            .migrate(
                "test",
                &[indoc! {"
                CREATE TABLE test1 (
                    a TEXT,
                    b TEXT
                )"}],
            )
            .unwrap();

        // Verify it got added to the migrations table
        assert_eq!(
            &connection
                .select::<String>("SELECT (migration) FROM migrations")
                .unwrap()()
            .unwrap()[..],
            &[indoc! {"CREATE TABLE test1 (a TEXT, b TEXT)"}],
        );

        // Add another step to the migration and run it again
        connection
            .migrate(
                "test",
                &[
                    indoc! {"
                    CREATE TABLE test1 (
                        a TEXT,
                        b TEXT
                    )"},
                    indoc! {"
                    CREATE TABLE test2 (
                        c TEXT,
                        d TEXT
                    )"},
                ],
            )
            .unwrap();

        // Verify it is also added to the migrations table
        assert_eq!(
            &connection
                .select::<String>("SELECT (migration) FROM migrations")
                .unwrap()()
            .unwrap()[..],
            &[
                indoc! {"CREATE TABLE test1 (a TEXT, b TEXT)"},
                indoc! {"CREATE TABLE test2 (c TEXT, d TEXT)"},
            ],
        );
    }

    #[test]
    fn test_migration_setup_works() {
        let connection = Connection::open_memory(Some("migration_setup_works"));

        connection
            .exec(indoc! {"
                CREATE TABLE IF NOT EXISTS migrations (
                    domain TEXT,
                    step INTEGER,
                    migration TEXT
                );"})
            .unwrap()()
        .unwrap();

        let mut store_completed_migration = connection
            .exec_bound::<(&str, usize, String)>(indoc! {"
                INSERT INTO migrations (domain, step, migration)
                VALUES (?, ?, ?)"})
            .unwrap();

        let domain = "test_domain";
        for i in 0..5 {
            // Create a table forcing a schema change
            connection
                .exec(&format!("CREATE TABLE table{} ( test TEXT );", i))
                .unwrap()()
            .unwrap();

            store_completed_migration((domain, i, i.to_string())).unwrap();
        }
    }

    #[test]
    fn migrations_dont_rerun() {
        let connection = Connection::open_memory(Some("migrations_dont_rerun"));

        // Create migration which clears a table

        // Manually create the table for that migration with a row
        connection
            .exec(indoc! {"
                CREATE TABLE test_table (
                    test_column INTEGER
                );"})
            .unwrap()()
        .unwrap();
        connection
            .exec(indoc! {"
            INSERT INTO test_table (test_column) VALUES (1);"})
            .unwrap()()
        .unwrap();

        assert_eq!(
            connection
                .select_row::<usize>("SELECT * FROM test_table")
                .unwrap()()
            .unwrap(),
            Some(1)
        );

        // Run the migration verifying that the row got dropped
        connection
            .migrate("test", &["DELETE FROM test_table"])
            .unwrap();
        assert_eq!(
            connection
                .select_row::<usize>("SELECT * FROM test_table")
                .unwrap()()
            .unwrap(),
            None
        );

        // Recreate the dropped row
        connection
            .exec("INSERT INTO test_table (test_column) VALUES (2)")
            .unwrap()()
        .unwrap();

        // Run the same migration again and verify that the table was left unchanged
        connection
            .migrate("test", &["DELETE FROM test_table"])
            .unwrap();
        assert_eq!(
            connection
                .select_row::<usize>("SELECT * FROM test_table")
                .unwrap()()
            .unwrap(),
            Some(2)
        );
    }

    #[test]
    fn changed_migration_fails() {
        let connection = Connection::open_memory(Some("changed_migration_fails"));

        // Create a migration with two steps and run it
        connection
            .migrate(
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
            .unwrap();

        // Create another migration with the same domain but different steps
        let second_migration_result = connection.migrate(
            "test migration",
            &[
                indoc! {"
                CREATE TABLE test (
                    color INTEGER
                )"},
                indoc! {"
                INSERT INTO test (color) VALUES (1)"},
            ],
        );

        // Verify new migration returns error when run
        assert!(second_migration_result.is_err())
    }

    #[test]
    fn test_create_alter_drop() {
        let connection = Connection::open_memory(Some("test_create_alter_drop"));

        connection
            .migrate("first_migration", &["CREATE TABLE table1(a TEXT) STRICT;"])
            .unwrap();

        connection
            .exec("INSERT INTO table1(a) VALUES (\"test text\");")
            .unwrap()()
        .unwrap();

        connection
            .migrate(
                "second_migration",
                &[indoc! {"
                    CREATE TABLE table2(b TEXT) STRICT;

                    INSERT INTO table2 (b)
                    SELECT a FROM table1;

                    DROP TABLE table1;

                    ALTER TABLE table2 RENAME TO table1;
                "}],
            )
            .unwrap();

        let res = &connection.select::<String>("SELECT b FROM table1").unwrap()().unwrap()[0];

        assert_eq!(res, "test text");
    }
}
