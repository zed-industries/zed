use db::{define_connection, query, sqlez_macros::sql};
use workspace::{ItemId, WorkspaceDb};

define_connection! {
    pub static ref WALKTHROUGH_DB: WalkthroughDb<WorkspaceDb> =
        &[sql!(
            CREATE TABLE walkthroughs (
                workspace_id INTEGER,
                item_id INTEGER UNIQUE,
                PRIMARY KEY(workspace_id, item_id),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
            ) STRICT;
        )];
}

impl WalkthroughDb {
    query! {
        pub async fn save_walkthrough(item_id: ItemId, workspace_id: workspace::WorkspaceId) -> Result<()> {
            INSERT INTO walkthroughs(item_id, workspace_id)
            VALUES (?1, ?2)
            ON CONFLICT DO UPDATE SET
              item_id = ?1,
              workspace_id = ?2
        }
    }

    query! {
        pub fn get_walkthrough(item_id: ItemId, workspace_id: workspace::WorkspaceId) -> Result<ItemId> {
            SELECT item_id
            FROM walkthroughs
            WHERE item_id = ? AND workspace_id = ?
        }
    }
}
