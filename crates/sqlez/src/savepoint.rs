use anyhow::Result;
use indoc::formatdoc;

use crate::connection::Connection;

impl Connection {
    // Run a set of commands within the context of a `SAVEPOINT name`. If the callback
    // returns Err(_), the savepoint will be rolled back. Otherwise, the save
    // point is released.
    pub fn with_savepoint<R, F>(&self, name: impl AsRef<str>, f: F) -> Result<R>
    where
        F: FnOnce() -> Result<R>,
    {
        let name = name.as_ref();
        self.exec(&format!("SAVEPOINT {name}"))?()?;
        let result = f();
        match result {
            Ok(_) => {
                self.exec(&format!("RELEASE {name}"))?()?;
            }
            Err(_) => {
                self.exec(&formatdoc! {"
                    ROLLBACK TO {name};
                    RELEASE {name}"})?()?;
            }
        }
        result
    }

    // Run a set of commands within the context of a `SAVEPOINT name`. If the callback
    // returns Ok(None) or Err(_), the savepoint will be rolled back. Otherwise, the save
    // point is released.
    pub fn with_savepoint_rollback<R, F>(&self, name: impl AsRef<str>, f: F) -> Result<Option<R>>
    where
        F: FnOnce() -> Result<Option<R>>,
    {
        let name = name.as_ref();
        self.exec(&format!("SAVEPOINT {name}"))?()?;
        let result = f();
        match result {
            Ok(Some(_)) => {
                self.exec(&format!("RELEASE {name}"))?()?;
            }
            Ok(None) | Err(_) => {
                self.exec(&formatdoc! {"
                    ROLLBACK TO {name};
                    RELEASE {name}"})?()?;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::connection::Connection;
    use anyhow::Result;
    use indoc::indoc;

    #[test]
    fn test_nested_savepoints() -> Result<()> {
        let connection = Connection::open_memory(Some("nested_savepoints"));

        connection
            .exec(indoc! {"
            CREATE TABLE text (
                text TEXT,
                idx INTEGER
            );"})
            .unwrap()()
        .unwrap();

        let save1_text = "test save1";
        let save2_text = "test save2";

        connection.with_savepoint("first", || {
            connection.exec_bound("INSERT INTO text(text, idx) VALUES (?, ?)")?((save1_text, 1))?;

            assert!(
                connection
                    .with_savepoint("second", || -> Result<Option<()>, anyhow::Error> {
                        connection.exec_bound("INSERT INTO text(text, idx) VALUES (?, ?)")?((
                            save2_text, 2,
                        ))?;

                        assert_eq!(
                            connection
                                .select::<String>("SELECT text FROM text ORDER BY text.idx ASC")?(
                            )?,
                            vec![save1_text, save2_text],
                        );

                        anyhow::bail!("Failed second save point :(")
                    })
                    .err()
                    .is_some()
            );

            assert_eq!(
                connection.select::<String>("SELECT text FROM text ORDER BY text.idx ASC")?()?,
                vec![save1_text],
            );

            connection.with_savepoint_rollback::<(), _>("second", || {
                connection.exec_bound("INSERT INTO text(text, idx) VALUES (?, ?)")?((
                    save2_text, 2,
                ))?;

                assert_eq!(
                    connection.select::<String>("SELECT text FROM text ORDER BY text.idx ASC")?()?,
                    vec![save1_text, save2_text],
                );

                Ok(None)
            })?;

            assert_eq!(
                connection.select::<String>("SELECT text FROM text ORDER BY text.idx ASC")?()?,
                vec![save1_text],
            );

            connection.with_savepoint_rollback("second", || {
                connection.exec_bound("INSERT INTO text(text, idx) VALUES (?, ?)")?((
                    save2_text, 2,
                ))?;

                assert_eq!(
                    connection.select::<String>("SELECT text FROM text ORDER BY text.idx ASC")?()?,
                    vec![save1_text, save2_text],
                );

                Ok(Some(()))
            })?;

            assert_eq!(
                connection.select::<String>("SELECT text FROM text ORDER BY text.idx ASC")?()?,
                vec![save1_text, save2_text],
            );

            Ok(())
        })?;

        assert_eq!(
            connection.select::<String>("SELECT text FROM text ORDER BY text.idx ASC")?()?,
            vec![save1_text, save2_text],
        );

        Ok(())
    }
}
