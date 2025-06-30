use anyhow::Result;
use db::{define_connection, query, sqlez::statement::Statement, sqlez_macros::sql};

use workspace::{WorkspaceDb, WorkspaceId};

define_connection! {
    pub static ref ONBOARDING_DB: OnboardingDb<WorkspaceDb> =
        &[sql!(
            CREATE TABLE onboarding_state (
                workspace_id INTEGER,
                item_id INTEGER UNIQUE,
                current_page TEXT,
                completed_pages TEXT,
                PRIMARY KEY(workspace_id, item_id),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
            ) STRICT;
        )];
}

impl OnboardingDb {
    pub async fn save_state(
        &self,
        item_id: u64,
        workspace_id: WorkspaceId,
        current_page: String,
        completed_pages: String,
    ) -> Result<()> {
        let query =
            "INSERT INTO onboarding_state(item_id, workspace_id, current_page, completed_pages)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT DO UPDATE SET
                current_page = ?3,
                completed_pages = ?4";
        self.write(move |conn| {
            let mut statement = Statement::prepare(conn, query)?;
            let mut next_index = statement.bind(&item_id, 1)?;
            next_index = statement.bind(&workspace_id, next_index)?;
            next_index = statement.bind(&current_page, next_index)?;
            statement.bind(&completed_pages, next_index)?;
            statement.exec()
        })
        .await
    }

    query! {
        pub fn get_state(item_id: u64, workspace_id: WorkspaceId) -> Result<Option<(String, String)>> {
            SELECT current_page, completed_pages
            FROM onboarding_state
            WHERE item_id = ? AND workspace_id = ?
        }
    }
}
