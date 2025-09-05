use anyhow::Result;
use db::{
    query,
    sqlez::{domain::Domain, statement::Statement, thread_safe_connection::ThreadSafeConnection},
    sqlez_macros::sql,
};
use workspace::{ItemId, WorkspaceDb, WorkspaceId};

pub struct ComponentPreviewDb(ThreadSafeConnection);

impl Domain for ComponentPreviewDb {
    const NAME: &str = stringify!(ComponentPreviewDb);

    const MIGRATIONS: &[&str] = &[sql!(
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

db::static_connection!(COMPONENT_PREVIEW_DB, ComponentPreviewDb, [WorkspaceDb]);

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
