use anyhow::Result;
use db::sqlez::bindable::{Bind, Column, StaticColumnCount};
use db::sqlez::statement::Statement;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use db::sqlez_macros::sql;
use db::{define_connection, query};

use workspace::{ItemId, WorkspaceDb, WorkspaceId};

#[derive(Clone, Debug, PartialEq, Default)]
pub(crate) struct SerializedEditor {
    pub(crate) path: Option<PathBuf>,
    pub(crate) contents: Option<String>,
    pub(crate) language: Option<String>,
    pub(crate) mtime: Option<SystemTime>,
}

impl StaticColumnCount for SerializedEditor {
    fn column_count() -> usize {
        5
    }
}

impl Bind for SerializedEditor {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        let start_index = statement.bind(&self.path, start_index)?;
        let start_index = statement.bind(&self.contents, start_index)?;
        let start_index = statement.bind(&self.language, start_index)?;

        let mtime = self.mtime.and_then(|mtime| {
            mtime
                .duration_since(UNIX_EPOCH)
                .ok()
                .map(|duration| (duration.as_secs() as i64, duration.subsec_nanos() as i32))
        });
        let start_index = match mtime {
            Some((seconds, nanos)) => {
                let start_index = statement.bind(&seconds, start_index)?;
                statement.bind(&nanos, start_index)?
            }
            None => {
                let start_index = statement.bind::<Option<i64>>(&None, start_index)?;
                statement.bind::<Option<i32>>(&None, start_index)?
            }
        };
        Ok(start_index)
    }
}

impl Column for SerializedEditor {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (path, start_index): (Option<PathBuf>, i32) = Column::column(statement, start_index)?;
        let (contents, start_index): (Option<String>, i32) =
            Column::column(statement, start_index)?;
        let (language, start_index): (Option<String>, i32) =
            Column::column(statement, start_index)?;
        let (mtime_seconds, start_index): (Option<i64>, i32) =
            Column::column(statement, start_index)?;
        let (mtime_nanos, start_index): (Option<i32>, i32) =
            Column::column(statement, start_index)?;

        let mtime = mtime_seconds
            .zip(mtime_nanos)
            .map(|(seconds, nanos)| UNIX_EPOCH + Duration::new(seconds as u64, nanos as u32));

        let editor = Self {
            path,
            contents,
            language,
            mtime,
        };
        Ok((editor, start_index))
    }
}

define_connection!(
    // Current schema shape using pseudo-rust syntax:
    // editors(
    //   item_id: usize,
    //   workspace_id: usize,
    //   path: Option<PathBuf>,
    //   scroll_top_row: usize,
    //   scroll_vertical_offset: f32,
    //   scroll_horizontal_offset: f32,
    //   content: Option<String>,
    //   language: Option<String>,
    //   mtime_seconds: Option<i64>,
    //   mtime_nanos: Option<i32>,
    // )
    pub static ref DB: EditorDb<WorkspaceDb> =
        &[sql! (
            CREATE TABLE editors(
                item_id INTEGER NOT NULL,
                workspace_id INTEGER NOT NULL,
                path BLOB NOT NULL,
                PRIMARY KEY(item_id, workspace_id),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
                ON UPDATE CASCADE
            ) STRICT;
        ),
        sql! (
            ALTER TABLE editors ADD COLUMN scroll_top_row INTEGER NOT NULL DEFAULT 0;
            ALTER TABLE editors ADD COLUMN scroll_horizontal_offset REAL NOT NULL DEFAULT 0;
            ALTER TABLE editors ADD COLUMN scroll_vertical_offset REAL NOT NULL DEFAULT 0;
        ),
        sql! (
            // Since sqlite3 doesn't support ALTER COLUMN, we create a new
            // table, move the data over, drop the old table, rename new table.
            CREATE TABLE new_editors_tmp (
                item_id INTEGER NOT NULL,
                workspace_id INTEGER NOT NULL,
                path BLOB, // <-- No longer "NOT NULL"
                scroll_top_row INTEGER NOT NULL DEFAULT 0,
                scroll_horizontal_offset REAL NOT NULL DEFAULT 0,
                scroll_vertical_offset REAL NOT NULL DEFAULT 0,
                contents TEXT, // New
                language TEXT, // New
                PRIMARY KEY(item_id, workspace_id),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
                ON UPDATE CASCADE
            ) STRICT;

            INSERT INTO new_editors_tmp(item_id, workspace_id, path, scroll_top_row, scroll_horizontal_offset, scroll_vertical_offset)
            SELECT item_id, workspace_id, path, scroll_top_row, scroll_horizontal_offset, scroll_vertical_offset
            FROM editors;

            DROP TABLE editors;

            ALTER TABLE new_editors_tmp RENAME TO editors;
        ),
        sql! (
            ALTER TABLE editors ADD COLUMN mtime_seconds INTEGER DEFAULT NULL;
            ALTER TABLE editors ADD COLUMN mtime_nanos INTEGER DEFAULT NULL;
        ),
        ];
);

impl EditorDb {
    query! {
        pub fn get_serialized_editor(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<SerializedEditor>> {
            SELECT path, contents, language, mtime_seconds, mtime_nanos FROM editors
            WHERE item_id = ? AND workspace_id = ?
        }
    }

    query! {
        pub async fn save_serialized_editor(item_id: ItemId, workspace_id: WorkspaceId, serialized_editor: SerializedEditor) -> Result<()> {
            INSERT INTO editors
                (item_id, workspace_id, path, contents, language, mtime_seconds, mtime_nanos)
            VALUES
                (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            ON CONFLICT DO UPDATE SET
                item_id = ?1,
                workspace_id = ?2,
                path = ?3,
                contents = ?4,
                language = ?5,
                mtime_seconds = ?6,
                mtime_nanos = ?7
        }
    }

    // Returns the scroll top row, and offset
    query! {
        pub fn get_scroll_position(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<(u32, f32, f32)>> {
            SELECT scroll_top_row, scroll_horizontal_offset, scroll_vertical_offset
            FROM editors
            WHERE item_id = ? AND workspace_id = ?
        }
    }

    query! {
        pub async fn save_scroll_position(
            item_id: ItemId,
            workspace_id: WorkspaceId,
            top_row: u32,
            vertical_offset: f32,
            horizontal_offset: f32
        ) -> Result<()> {
            UPDATE OR IGNORE editors
            SET
                scroll_top_row = ?3,
                scroll_horizontal_offset = ?4,
                scroll_vertical_offset = ?5
            WHERE item_id = ?1 AND workspace_id = ?2
        }
    }

    pub async fn delete_unloaded_items(
        &self,
        workspace: WorkspaceId,
        alive_items: Vec<ItemId>,
    ) -> Result<()> {
        let placeholders = alive_items
            .iter()
            .map(|_| "?")
            .collect::<Vec<&str>>()
            .join(", ");

        let query = format!(
            "DELETE FROM editors WHERE workspace_id = ? AND item_id NOT IN ({placeholders})"
        );

        self.write(move |conn| {
            let mut statement = Statement::prepare(conn, query)?;
            let mut next_index = statement.bind(&workspace, 1)?;
            for id in alive_items {
                next_index = statement.bind(&id, next_index)?;
            }
            statement.exec()
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui;

    #[gpui::test]
    async fn test_save_and_get_serialized_editor() {
        let workspace_id = workspace::WORKSPACE_DB.next_id().await.unwrap();

        let serialized_editor = SerializedEditor {
            path: Some(PathBuf::from("testing.txt")),
            contents: None,
            language: None,
            mtime: None,
        };

        DB.save_serialized_editor(1234, workspace_id, serialized_editor.clone())
            .await
            .unwrap();

        let have = DB
            .get_serialized_editor(1234, workspace_id)
            .unwrap()
            .unwrap();
        assert_eq!(have, serialized_editor);

        // Now update contents and language
        let serialized_editor = SerializedEditor {
            path: Some(PathBuf::from("testing.txt")),
            contents: Some("Test".to_owned()),
            language: Some("Go".to_owned()),
            mtime: None,
        };

        DB.save_serialized_editor(1234, workspace_id, serialized_editor.clone())
            .await
            .unwrap();

        let have = DB
            .get_serialized_editor(1234, workspace_id)
            .unwrap()
            .unwrap();
        assert_eq!(have, serialized_editor);

        // Now set all the fields to NULL
        let serialized_editor = SerializedEditor {
            path: None,
            contents: None,
            language: None,
            mtime: None,
        };

        DB.save_serialized_editor(1234, workspace_id, serialized_editor.clone())
            .await
            .unwrap();

        let have = DB
            .get_serialized_editor(1234, workspace_id)
            .unwrap()
            .unwrap();
        assert_eq!(have, serialized_editor);

        // Storing and retrieving mtime
        let now = SystemTime::now();
        let serialized_editor = SerializedEditor {
            path: None,
            contents: None,
            language: None,
            mtime: Some(now),
        };

        DB.save_serialized_editor(1234, workspace_id, serialized_editor.clone())
            .await
            .unwrap();

        let have = DB
            .get_serialized_editor(1234, workspace_id)
            .unwrap()
            .unwrap();
        assert_eq!(have, serialized_editor);
    }
}
