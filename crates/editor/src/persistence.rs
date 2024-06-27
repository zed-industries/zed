use std::path::PathBuf;

use anyhow::Result;
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
        pub fn get_contents_metadata(workspace: WorkspaceId) -> Result<Vec<(ItemId, WorkspaceId)>> {
            SELECT item_id, workspace_id
            FROM editor_contents
            WHERE workspace_id = ?
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

    //TODO pass Vec<ItemId> instead of String, unsure how to do that with sqlez
    pub fn delete_outdated_contents(
        &self,
        workspace: WorkspaceId,
        item_ids: Vec<ItemId>,
    ) -> Result<()> {
        let ids_string = item_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        let query = format!("DELETE FROM editor_contents WHERE workspace_id = {workspace:?} AND item_id NOT IN ({ids_string})");
        self.exec(&query).unwrap()()
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

// pub struct BufferVersion(pub clock::Global);

// impl StaticColumnCount for BufferVersion {}

// impl Bind for BufferVersion {
//     fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
//         let data: Vec<u32> = self.0.clone().into();
//         statement.bind(&bincode::serialize(&data)?, start_index)
//     }
// }

// impl Column for BufferVersion {
//     fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
//         let version_blob = statement.column_blob(start_index)?;

//         let version: Vec<u32> = if version_blob.is_empty() {
//             Default::default()
//         } else {
//             bincode::deserialize(version_blob).context("Bincode deserialization of paths failed")?
//         };

//         Ok((
//             BufferVersion(clock::Global::from(version.as_slice())),
//             start_index + 1,
//         ))
//     }
// }
