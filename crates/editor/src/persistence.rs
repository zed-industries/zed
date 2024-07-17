use anyhow::Result;
use db::sqlez::statement::Statement;
use std::path::PathBuf;

use db::sqlez_macros::sql;
use db::{define_connection, query};

use workspace::{ItemId, WorkspaceDb, WorkspaceId};

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
        )];
);

impl EditorDb {
    query! {
        pub fn get_path_and_contents(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<(Option<PathBuf>, Option<String>, Option<String>)>> {
            SELECT path, contents, language FROM editors
            WHERE item_id = ? AND workspace_id = ?
        }
    }

    query! {
        pub async fn save_path(item_id: ItemId, workspace_id: WorkspaceId, path: PathBuf) -> Result<()> {
            INSERT INTO editors
                (item_id, workspace_id, path)
            VALUES
                (?1, ?2, ?3)
            ON CONFLICT DO UPDATE SET
                item_id = ?1,
                workspace_id = ?2,
                path = ?3
        }
    }

    query! {
        pub async fn save_contents(item_id: ItemId, workspace: WorkspaceId, contents: Option<String>, language: Option<String>) -> Result<()> {
            INSERT INTO editors
                (item_id, workspace_id, contents, language)
            VALUES
                (?1, ?2, ?3, ?4)
            ON CONFLICT DO UPDATE SET
                item_id = ?1,
                workspace_id = ?2,
                contents = ?3,
                language = ?4
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
    async fn test_saving_content() {
        env_logger::try_init().ok();

        let workspace_id = workspace::WORKSPACE_DB.next_id().await.unwrap();

        // Sanity check: make sure there is no row in the `editors` table
        assert_eq!(DB.get_path_and_contents(1234, workspace_id).unwrap(), None);

        // Save content/language
        DB.save_contents(
            1234,
            workspace_id,
            Some("testing".into()),
            Some("Go".into()),
        )
        .await
        .unwrap();

        // Check that it can be read from DB
        let path_and_contents = DB.get_path_and_contents(1234, workspace_id).unwrap();
        let (path, contents, language) = path_and_contents.unwrap();
        assert!(path.is_none());
        assert_eq!(contents, Some("testing".to_owned()));
        assert_eq!(language, Some("Go".to_owned()));

        // Update it with NULL
        DB.save_contents(1234, workspace_id, None, None)
            .await
            .unwrap();

        // Check that it worked
        let path_and_contents = DB.get_path_and_contents(1234, workspace_id).unwrap();
        let (path, contents, language) = path_and_contents.unwrap();
        assert!(path.is_none());
        assert!(contents.is_none());
        assert!(language.is_none());
    }
}
