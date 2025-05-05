use anyhow::Result;
use db::{define_connection, query, sqlez::statement::Statement, sqlez_macros::sql};
use workspace::{ItemId, WorkspaceDb, WorkspaceId};

define_connection! {
    pub static ref COMPONENT_PREVIEW_DB: ComponentPreviewDb<WorkspaceDb> =
        &[sql!(
            CREATE TABLE component_previews (
                workspace_id INTEGER,
                item_id INTEGER UNIQUE,
                active_page_id TEXT,
                PRIMARY KEY(workspace_id, item_id),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
            ) STRICT;
        )];
}

impl ComponentPreviewDb {
    pub async fn save_active_page(
        &self,
        item_id: ItemId,
        workspace_id: WorkspaceId,
        active_page_id: String,
    ) -> Result<()> {
        log::debug!(
            "Saving active page: item_id={item_id:?}, workspace_id={workspace_id:?}, active_page_id={active_page_id}"
        );
        let query = "INSERT INTO component_previews(item_id, workspace_id, active_page_id)
            VALUES (?1, ?2, ?3)
            ON CONFLICT DO UPDATE SET
                active_page_id = ?3";
        self.write(move |conn| {
            let mut statement = Statement::prepare(conn, query)?;
            let mut next_index = statement.bind(&item_id, 1)?;
            next_index = statement.bind(&workspace_id, next_index)?;
            statement.bind(&active_page_id, next_index)?;
            statement.exec()
        })
        .await
    }

    query! {
        pub fn get_active_page(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<String>> {
            SELECT active_page_id
            FROM component_previews
            WHERE item_id = ? AND workspace_id = ?
        }
    }
}
