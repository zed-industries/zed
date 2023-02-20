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
}
