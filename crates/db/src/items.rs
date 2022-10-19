use std::{ffi::OsStr, fmt::Display, hash::Hash, os::unix::prelude::OsStrExt, path::PathBuf};

use anyhow::Result;
use collections::HashSet;
use rusqlite::{named_params, params};

use super::Db;

pub(crate) const ITEMS_M_1: &str = "
CREATE TABLE items(
    id INTEGER PRIMARY KEY,
    kind TEXT
) STRICT;
CREATE TABLE item_path(
    item_id INTEGER PRIMARY KEY,
    path BLOB
) STRICT;
CREATE TABLE item_query(
    item_id INTEGER PRIMARY KEY,
    query TEXT
) STRICT;
";

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum SerializedItemKind {
    Editor,
    Terminal,
    ProjectSearch,
    Diagnostics,
}

impl Display for SerializedItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SerializedItem {
    Editor(usize, PathBuf),
    Terminal(usize),
    ProjectSearch(usize, String),
    Diagnostics(usize),
}

impl SerializedItem {
    fn kind(&self) -> SerializedItemKind {
        match self {
            SerializedItem::Editor(_, _) => SerializedItemKind::Editor,
            SerializedItem::Terminal(_) => SerializedItemKind::Terminal,
            SerializedItem::ProjectSearch(_, _) => SerializedItemKind::ProjectSearch,
            SerializedItem::Diagnostics(_) => SerializedItemKind::Diagnostics,
        }
    }

    fn id(&self) -> usize {
        match self {
            SerializedItem::Editor(id, _)
            | SerializedItem::Terminal(id)
            | SerializedItem::ProjectSearch(id, _)
            | SerializedItem::Diagnostics(id) => *id,
        }
    }
}

impl Db {
    fn write_item(&self, serialized_item: SerializedItem) -> Result<()> {
        self.real()
            .map(|db| {
                let mut lock = db.connection.lock();
                let tx = lock.transaction()?;

                // Serialize the item
                let id = serialized_item.id();
                {
                    let mut stmt = tx.prepare_cached(
                        "INSERT OR REPLACE INTO items(id, kind) VALUES ((?), (?))",
                    )?;

                    dbg!("inserting item");
                    stmt.execute(params![id, serialized_item.kind().to_string()])?;
                }

                // Serialize item data
                match &serialized_item {
                    SerializedItem::Editor(_, path) => {
                        dbg!("inserting path");
                        let mut stmt = tx.prepare_cached(
                            "INSERT OR REPLACE INTO item_path(item_id, path) VALUES ((?), (?))",
                        )?;

                        let path_bytes = path.as_os_str().as_bytes();
                        stmt.execute(params![id, path_bytes])?;
                    }
                    SerializedItem::ProjectSearch(_, query) => {
                        dbg!("inserting query");
                        let mut stmt = tx.prepare_cached(
                            "INSERT OR REPLACE INTO item_query(item_id, query) VALUES ((?), (?))",
                        )?;

                        stmt.execute(params![id, query])?;
                    }
                    _ => {}
                }

                tx.commit()?;

                let mut stmt = lock.prepare_cached("SELECT id, kind FROM items")?;
                let _ = stmt
                    .query_map([], |row| {
                        let zero: usize = row.get(0)?;
                        let one: String = row.get(1)?;

                        dbg!(zero, one);
                        Ok(())
                    })?
                    .collect::<Vec<Result<(), _>>>();

                Ok(())
            })
            .unwrap_or(Ok(()))
    }

    fn delete_item(&self, item_id: usize) -> Result<()> {
        self.real()
            .map(|db| {
                let lock = db.connection.lock();

                let mut stmt = lock.prepare_cached(
                    r#"
                    DELETE FROM items WHERE id = (:id);
                    DELETE FROM item_path WHERE id = (:id);
                    DELETE FROM item_query WHERE id = (:id);
                    "#,
                )?;

                stmt.execute(named_params! {":id": item_id})?;

                Ok(())
            })
            .unwrap_or(Ok(()))
    }

    fn take_items(&self) -> Result<HashSet<SerializedItem>> {
        self.real()
            .map(|db| {
                let mut lock = db.connection.lock();

                let tx = lock.transaction()?;

                // When working with transactions in rusqlite, need to make this kind of scope
                // To make the borrow stuff work correctly. Don't know why, rust is wild.
                let result = {
                    let mut editors_stmt = tx.prepare_cached(
                        r#"
                        SELECT items.id, item_path.path
                        FROM items
                        LEFT JOIN item_path
                            ON items.id = item_path.item_id
                        WHERE items.kind = ?;
                        "#,
                    )?;

                    let editors_iter = editors_stmt.query_map(
                        [SerializedItemKind::Editor.to_string()],
                        |row| {
                            let id: usize = row.get(0)?;

                            let buf: Vec<u8> = row.get(1)?;
                            let path: PathBuf = OsStr::from_bytes(&buf).into();

                            Ok(SerializedItem::Editor(id, path))
                        },
                    )?;

                    let mut terminals_stmt = tx.prepare_cached(
                        r#"
                        SELECT items.id
                        FROM items
                        WHERE items.kind = ?;
                        "#,
                    )?;
                    let terminals_iter = terminals_stmt.query_map(
                        [SerializedItemKind::Terminal.to_string()],
                        |row| {
                            let id: usize = row.get(0)?;

                            Ok(SerializedItem::Terminal(id))
                        },
                    )?;

                    let mut search_stmt = tx.prepare_cached(
                        r#"
                        SELECT items.id, item_query.query
                        FROM items
                        LEFT JOIN item_query
                            ON items.id = item_query.item_id
                        WHERE items.kind = ?;
                        "#,
                    )?;
                    let searches_iter = search_stmt.query_map(
                        [SerializedItemKind::ProjectSearch.to_string()],
                        |row| {
                            let id: usize = row.get(0)?;
                            let query = row.get(1)?;

                            Ok(SerializedItem::ProjectSearch(id, query))
                        },
                    )?;

                    #[cfg(debug_assertions)]
                    let tmp =
                        searches_iter.collect::<Vec<Result<SerializedItem, rusqlite::Error>>>();
                    #[cfg(debug_assertions)]
                    debug_assert!(tmp.len() == 0 || tmp.len() == 1);
                    #[cfg(debug_assertions)]
                    let searches_iter = tmp.into_iter();

                    let mut diagnostic_stmt = tx.prepare_cached(
                        r#"
                        SELECT items.id
                        FROM items
                        WHERE items.kind = ?;
                        "#,
                    )?;

                    let diagnostics_iter = diagnostic_stmt.query_map(
                        [SerializedItemKind::Diagnostics.to_string()],
                        |row| {
                            let id: usize = row.get(0)?;

                            Ok(SerializedItem::Diagnostics(id))
                        },
                    )?;

                    #[cfg(debug_assertions)]
                    let tmp =
                        diagnostics_iter.collect::<Vec<Result<SerializedItem, rusqlite::Error>>>();
                    #[cfg(debug_assertions)]
                    debug_assert!(tmp.len() == 0 || tmp.len() == 1);
                    #[cfg(debug_assertions)]
                    let diagnostics_iter = tmp.into_iter();

                    let res = editors_iter
                        .chain(terminals_iter)
                        .chain(diagnostics_iter)
                        .chain(searches_iter)
                        .collect::<Result<HashSet<SerializedItem>, rusqlite::Error>>()?;

                    let mut delete_stmt = tx.prepare_cached(
                        r#"
                        DELETE FROM items;
                        DELETE FROM item_path;
                        DELETE FROM item_query;
                        "#,
                    )?;

                    delete_stmt.execute([])?;

                    res
                };

                tx.commit()?;

                Ok(result)
            })
            .unwrap_or(Ok(HashSet::default()))
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use super::*;

    #[test]
    fn test_items_round_trip() -> Result<()> {
        let db = Db::open_in_memory();

        let mut items = vec![
            SerializedItem::Editor(0, PathBuf::from("/tmp/test.txt")),
            SerializedItem::Terminal(1),
            SerializedItem::ProjectSearch(2, "Test query!".to_string()),
            SerializedItem::Diagnostics(3),
        ]
        .into_iter()
        .collect::<HashSet<_>>();

        for item in items.iter() {
            dbg!("Inserting... ");
            db.write_item(item.clone())?;
        }

        assert_eq!(items, db.take_items()?);

        // Check that it's empty, as expected
        assert_eq!(HashSet::default(), db.take_items()?);

        for item in items.iter() {
            db.write_item(item.clone())?;
        }

        items.remove(&SerializedItem::ProjectSearch(2, "Test query!".to_string()));
        db.delete_item(2)?;

        assert_eq!(items, db.take_items()?);

        Ok(())
    }
}
