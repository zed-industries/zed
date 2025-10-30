// Migrations are constructed by domain, and stored in a table in the connection db with domain name,
// effected tables, actual query text, and order.
// If a migration is run and any of the query texts don't match, the app panics on startup (maybe fallback
// to creating a new db?)
// Otherwise any missing migrations are run on the connection

use std::ffi::CString;

use anyhow::{Context as _, Result};
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
    pub fn migrate(
        &self,
        domain: &'static str,
        migrations: &[&'static str],
        mut should_allow_migration_change: impl FnMut(usize, &str, &str) -> bool,
    ) -> Result<()> {
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

            let mut did_migrate = false;
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
                    } else if should_allow_migration_change(index, &completed_migration, &migration)
                    {
                        continue;
                    } else {
                        anyhow::bail!(formatdoc! {"
                            Migration changed for {domain} at step {index}

                            Stored migration:
                            {completed_migration}

                            Proposed migration:
                            {migration}"});
                    }
                }

                self.eager_exec(&migration)?;
                did_migrate = true;
                store_completed_migration((domain, index, migration))?;
            }

            if did_migrate {
                self.delete_rows_with_orphaned_foreign_key_references()?;
                self.exec("PRAGMA foreign_key_check;")?()?;
            }

            Ok(())
        })
    }

    /// Delete any rows that were orphaned by a migration. This is needed
    /// because we disable foreign key constraints during migrations, so
    /// that it's possible to re-create a table with the same name, without
    /// deleting all associated data.
    fn delete_rows_with_orphaned_foreign_key_references(&self) -> Result<()> {
        let foreign_key_info: Vec<(String, String, String, String)> = self.select(
            r#"
                SELECT DISTINCT
                    schema.name as child_table,
                    foreign_keys.[from] as child_key,
                    foreign_keys.[table] as parent_table,
                    foreign_keys.[to] as parent_key
                FROM sqlite_schema schema
                JOIN pragma_foreign_key_list(schema.name) foreign_keys
                WHERE
                    schema.type = 'table' AND
                    schema.name NOT LIKE "sqlite_%"
            "#,
        )?()?;

        if !foreign_key_info.is_empty() {
            log::info!(
                "Found {} foreign key relationships to check",
                foreign_key_info.len()
            );
        }

        for (child_table, child_key, parent_table, parent_key) in foreign_key_info {
            self.exec(&format!(
                "
                DELETE FROM {child_table}
                WHERE {child_key} IS NOT NULL and {child_key} NOT IN
                (SELECT {parent_key} FROM {parent_table})
                "
            ))?()?;
        }

        Ok(())
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
                disallow_migration_change,
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
                disallow_migration_change,
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
            .migrate(
                "test",
                &["DELETE FROM test_table"],
                disallow_migration_change,
            )
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
            .migrate(
                "test",
                &["DELETE FROM test_table"],
                disallow_migration_change,
            )
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
                    "CREATE TABLE test (col INTEGER)",
                    "INSERT INTO test (col) VALUES (1)",
                ],
                disallow_migration_change,
            )
            .unwrap();

        let mut migration_changed = false;

        // Create another migration with the same domain but different steps
        let second_migration_result = connection.migrate(
            "test migration",
            &[
                "CREATE TABLE test (color INTEGER )",
                "INSERT INTO test (color) VALUES (1)",
            ],
            |_, old, new| {
                assert_eq!(old, "CREATE TABLE test (col INTEGER)");
                assert_eq!(new, "CREATE TABLE test (color INTEGER)");
                migration_changed = true;
                false
            },
        );

        // Verify new migration returns error when run
        assert!(second_migration_result.is_err())
    }

    #[test]
    fn test_create_alter_drop() {
        let connection = Connection::open_memory(Some("test_create_alter_drop"));

        connection
            .migrate(
                "first_migration",
                &["CREATE TABLE table1(a TEXT) STRICT;"],
                disallow_migration_change,
            )
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
                disallow_migration_change,
            )
            .unwrap();

        let res = &connection.select::<String>("SELECT b FROM table1").unwrap()().unwrap()[0];

        assert_eq!(res, "test text");
    }

    fn disallow_migration_change(_: usize, _: &str, _: &str) -> bool {
        false
    }
}
