use anyhow::Result;
use std::path::PathBuf;

use db::sqlez_macros::sql;
use db::{define_connection, query};

use workspace::{ItemId, WorkspaceDb, WorkspaceId};

define_connection!(
    // Current schema shape using pseudo-rust syntax:
    // editors(
    //   item_id: usize,
    //   workspace_id: usize,
    //   path: PathBuf,
    //   scroll_top_row: usize,
    //   scroll_vertical_offset: f32,
    //   scroll_horizontal_offset: f32,
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
            CREATE TABLE editor_contents (
                item_id INTEGER NOT NULL,
                workspace_id INTEGER NOT NULL,
                contents TEXT NOT NULL,
                PRIMARY KEY(item_id, workspace_id),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
                ON UPDATE CASCADE
            ) STRICT;
        )];
);

impl EditorDb {
    query! {
        pub fn get_path(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<PathBuf>> {
            SELECT path FROM editors
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
        pub fn get_contents(item_id: ItemId, workspace: WorkspaceId) -> Result<Option<String>> {
            SELECT contents
            FROM editor_contents
            WHERE item_id = ?1
            AND workspace_id = ?2
        }
    }

    query! {
        pub async fn save_contents(item_id: ItemId, workspace: WorkspaceId, contents: String) -> Result<()> {
            INSERT INTO editor_contents
                (item_id, workspace_id, contents)
            VALUES
                (?1, ?2, ?3)
            ON CONFLICT DO UPDATE SET
                item_id = ?1,
                workspace_id = ?2,
                contents = ?3
        }
    }

    query! {
        pub async fn delete_contents(workspace: WorkspaceId, item_id: ItemId) -> Result<()> {
            DELETE FROM editor_contents
            WHERE workspace_id = ?
            AND item_id = ?
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
        loaded_item_ids: Vec<ItemId>,
    ) -> Result<()> {
        let ids_string = loaded_item_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        let workspace_id: i64 = workspace.into();

        let query = format!("DELETE FROM editor_contents WHERE workspace_id = {workspace_id} AND item_id NOT IN ({ids_string})");
        self.write(move |conn| conn.exec(&query).unwrap()()).await
    }
}
