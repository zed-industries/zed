use anyhow::Result;

use crate::connection::Connection;

impl Connection {
    // Run a set of commands within the context of a `SAVEPOINT name`. If the callback
    // returns Ok(None) or Err(_), the savepoint will be rolled back. Otherwise, the save
    // point is released.
    pub fn with_savepoint<F, R>(&mut self, name: impl AsRef<str>, f: F) -> Result<Option<R>>
    where
        F: FnOnce(&mut Connection) -> Result<Option<R>>,
    {
        let name = name.as_ref().to_owned();
        self.exec(format!("SAVEPOINT {}", &name))?;
        let result = f(self);
        match result {
            Ok(Some(_)) => {
                self.exec(format!("RELEASE {}", name))?;
            }
            Ok(None) | Err(_) => {
                self.exec(format!("ROLLBACK TO {}", name))?;
                self.exec(format!("RELEASE {}", name))?;
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
        let mut connection = Connection::open_memory("nested_savepoints");

        connection
            .exec(indoc! {"
            CREATE TABLE text (
                text TEXT,
                idx INTEGER
            );"})
            .unwrap();

        let save1_text = "test save1";
        let save2_text = "test save2";

        connection.with_savepoint("first", |save1| {
            save1
                .prepare("INSERT INTO text(text, idx) VALUES (?, ?)")?
                .bound((save1_text, 1))?
                .run()?;

            assert!(save1
                .with_savepoint("second", |save2| -> Result<Option<()>, anyhow::Error> {
                    save2
                        .prepare("INSERT INTO text(text, idx) VALUES (?, ?)")?
                        .bound((save2_text, 2))?
                        .run()?;

                    assert_eq!(
                        save2
                            .prepare("SELECT text FROM text ORDER BY text.idx ASC")?
                            .rows::<String>()?,
                        vec![save1_text, save2_text],
                    );

                    anyhow::bail!("Failed second save point :(")
                })
                .err()
                .is_some());

            assert_eq!(
                save1
                    .prepare("SELECT text FROM text ORDER BY text.idx ASC")?
                    .rows::<String>()?,
                vec![save1_text],
            );

            save1.with_savepoint("second", |save2| {
                save2
                    .prepare("INSERT INTO text(text, idx) VALUES (?, ?)")?
                    .bound((save2_text, 2))?
                    .run()?;

                assert_eq!(
                    save2
                        .prepare("SELECT text FROM text ORDER BY text.idx ASC")?
                        .rows::<String>()?,
                    vec![save1_text, save2_text],
                );

                Ok(Some(()))
            })?;

            assert_eq!(
                save1
                    .prepare("SELECT text FROM text ORDER BY text.idx ASC")?
                    .rows::<String>()?,
                vec![save1_text, save2_text],
            );

            Ok(Some(()))
        })?;

        Ok(())
    }
}
