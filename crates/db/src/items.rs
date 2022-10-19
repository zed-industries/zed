use std::{ffi::OsStr, os::unix::prelude::OsStrExt, path::PathBuf, sync::Arc};

use anyhow::Result;
use rusqlite::{
    named_params, params,
    types::{FromSql, FromSqlError, FromSqlResult, ValueRef},
};

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SerializedItem {
    Editor(usize, PathBuf),
    Terminal(usize),
    ProjectSearch(usize, String),
    Diagnostics(usize),
}

impl FromSql for SerializedItemKind {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Null => Err(FromSqlError::InvalidType),
            ValueRef::Integer(_) => Err(FromSqlError::InvalidType),
            ValueRef::Real(_) => Err(FromSqlError::InvalidType),
            ValueRef::Text(bytes) => {
                let str = std::str::from_utf8(bytes).map_err(|_| FromSqlError::InvalidType)?;
                match str {
                    "Editor" => Ok(SerializedItemKind::Editor),
                    "Terminal" => Ok(SerializedItemKind::Terminal),
                    "ProjectSearch" => Ok(SerializedItemKind::ProjectSearch),
                    "Diagnostics" => Ok(SerializedItemKind::Diagnostics),
                    _ => Err(FromSqlError::InvalidType),
                }
            }
            ValueRef::Blob(_) => Err(FromSqlError::InvalidType),
        }
    }
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
        let mut lock = self.connection.lock();
        let tx = lock.transaction()?;

        // Serialize the item
        let id = serialized_item.id();
        {
            let kind = format!("{:?}", serialized_item.kind());

            let mut stmt =
                tx.prepare_cached("INSERT OR REPLACE INTO items(id, kind) VALUES ((?), (?))")?;

            stmt.execute(params![id, kind])?;
        }

        // Serialize item data
        match &serialized_item {
            SerializedItem::Editor(_, path) => {
                let mut stmt = tx.prepare_cached(
                    "INSERT OR REPLACE INTO item_path(item_id, path) VALUES ((?), (?))",
                )?;

                let path_bytes = path.as_os_str().as_bytes();
                stmt.execute(params![id, path_bytes])?;
            }
            SerializedItem::ProjectSearch(_, query) => {
                let mut stmt = tx.prepare_cached(
                    "INSERT OR REPLACE INTO item_query(item_id, query) VALUES ((?), (?))",
                )?;

                stmt.execute(params![id, query])?;
            }
            _ => {}
        }

        tx.commit()?;

        Ok(())
    }

    fn delete_item(&self, item_id: usize) -> Result<()> {
        let lock = self.connection.lock();

        let mut stmt = lock.prepare_cached(
            "
            DELETE FROM items WHERE id = (:id);
            DELETE FROM item_path WHERE id = (:id);
            DELETE FROM item_query WHERE id = (:id);
        ",
        )?;

        stmt.execute(named_params! {":id": item_id})?;

        Ok(())
    }

    fn take_items(&self) -> Result<Vec<SerializedItem>> {
        let mut lock = self.connection.lock();

        let tx = lock.transaction()?;

        // When working with transactions in rusqlite, need to make this kind of scope
        // To make the borrow stuff work correctly. Don't know why, rust is wild.
        let result = {
            let mut read_stmt = tx.prepare_cached(
                "
                    SELECT items.id, items.kind, item_path.path, item_query.query
                    FROM items
                    LEFT JOIN item_path
                        ON items.id = item_path.item_id
                    LEFT JOIN item_query
                        ON items.id = item_query.item_id
                    ORDER BY items.id
            ",
            )?;

            let result = read_stmt
                .query_map([], |row| {
                    let id: usize = row.get(0)?;
                    let kind: SerializedItemKind = row.get(1)?;

                    match kind {
                        SerializedItemKind::Editor => {
                            let buf: Vec<u8> = row.get(2)?;
                            let path: PathBuf = OsStr::from_bytes(&buf).into();

                            Ok(SerializedItem::Editor(id, path))
                        }
                        SerializedItemKind::Terminal => Ok(SerializedItem::Terminal(id)),
                        SerializedItemKind::ProjectSearch => {
                            let query: Arc<str> = row.get(3)?;
                            Ok(SerializedItem::ProjectSearch(id, query.to_string()))
                        }
                        SerializedItemKind::Diagnostics => Ok(SerializedItem::Diagnostics(id)),
                    }
                })?
                .collect::<Result<Vec<SerializedItem>, rusqlite::Error>>()?;

            let mut delete_stmt = tx.prepare_cached(
                "DELETE FROM items;
                DELETE FROM item_path;
                DELETE FROM item_query;",
            )?;

            delete_stmt.execute([])?;

            result
        };

        tx.commit()?;

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use super::*;

    #[test]
    fn test_items_round_trip() -> Result<()> {
        let db = Db::open_in_memory()?;

        let mut items = vec![
            SerializedItem::Editor(0, PathBuf::from("/tmp/test.txt")),
            SerializedItem::Terminal(1),
            SerializedItem::ProjectSearch(2, "Test query!".to_string()),
            SerializedItem::Diagnostics(3),
        ];

        for item in items.iter() {
            db.write_item(item.clone())?;
        }

        assert_eq!(items, db.take_items()?);

        // Check that it's empty, as expected
        assert_eq!(Vec::<SerializedItem>::new(), db.take_items()?);

        for item in items.iter() {
            db.write_item(item.clone())?;
        }

        items.remove(2);
        db.delete_item(2)?;

        assert_eq!(items, db.take_items()?);

        Ok(())
    }
}
