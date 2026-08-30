use anyhow::Result;
use db::{
    query,
    sqlez::{domain::Domain, statement::Statement, thread_safe_connection::ThreadSafeConnection},
    sqlez_macros::sql,
};
use gpui::{App, Task};
use workspace::{ItemId, WorkspaceDb, WorkspaceId, delete_unloaded_items};

pub struct AgentCenterPaneDb(ThreadSafeConnection);

impl Domain for AgentCenterPaneDb {
    const NAME: &str = stringify!(AgentCenterPaneDb);

    const MIGRATIONS: &[&str] = &[sql!(
        CREATE TABLE agent_conversation_panes (
            workspace_id INTEGER NOT NULL,
            item_id INTEGER NOT NULL,
            thread_id TEXT NOT NULL,
            PRIMARY KEY(workspace_id, item_id),
            FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
            ON DELETE CASCADE
        ) STRICT;

        CREATE TABLE agent_terminal_panes (
            workspace_id INTEGER NOT NULL,
            item_id INTEGER NOT NULL,
            terminal_id TEXT NOT NULL,
            PRIMARY KEY(workspace_id, item_id),
            FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
            ON DELETE CASCADE
        ) STRICT;
    )];
}

db::static_connection!(AgentCenterPaneDb, [WorkspaceDb]);

impl AgentCenterPaneDb {
    pub async fn save_thread(
        &self,
        workspace_id: WorkspaceId,
        item_id: ItemId,
        thread_id: String,
    ) -> Result<()> {
        self.write(move |connection| {
            let mut statement = Statement::prepare(
                connection,
                "INSERT INTO agent_conversation_panes(workspace_id, item_id, thread_id) \
                 VALUES (?1, ?2, ?3) \
                 ON CONFLICT(workspace_id, item_id) DO UPDATE SET thread_id = excluded.thread_id",
            )?;
            let mut index = statement.bind(&workspace_id, 1)?;
            index = statement.bind(&item_id, index)?;
            statement.bind(&thread_id, index)?;
            statement.exec()
        })
        .await
    }

    query! {
        pub fn thread_id(workspace_id: WorkspaceId, item_id: ItemId) -> Result<Option<String>> {
            SELECT thread_id
            FROM agent_conversation_panes
            WHERE workspace_id = ? AND item_id = ?
        }
    }

    pub async fn save_terminal(
        &self,
        workspace_id: WorkspaceId,
        item_id: ItemId,
        terminal_id: String,
    ) -> Result<()> {
        self.write(move |connection| {
            let mut statement = Statement::prepare(
                connection,
                "INSERT INTO agent_terminal_panes(workspace_id, item_id, terminal_id) \
                 VALUES (?1, ?2, ?3) \
                 ON CONFLICT(workspace_id, item_id) DO UPDATE SET terminal_id = excluded.terminal_id",
            )?;
            let mut index = statement.bind(&workspace_id, 1)?;
            index = statement.bind(&item_id, index)?;
            statement.bind(&terminal_id, index)?;
            statement.exec()
        })
        .await
    }

    query! {
        pub fn terminal_id(workspace_id: WorkspaceId, item_id: ItemId) -> Result<Option<String>> {
            SELECT terminal_id
            FROM agent_terminal_panes
            WHERE workspace_id = ? AND item_id = ?
        }
    }

    pub fn cleanup_threads(
        &self,
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
        cx: &mut App,
    ) -> Task<Result<()>> {
        delete_unloaded_items(
            alive_items,
            workspace_id,
            "agent_conversation_panes",
            &self.0,
            cx,
        )
    }

    pub fn cleanup_terminals(
        &self,
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
        cx: &mut App,
    ) -> Task<Result<()>> {
        delete_unloaded_items(
            alive_items,
            workspace_id,
            "agent_terminal_panes",
            &self.0,
            cx,
        )
    }
}
