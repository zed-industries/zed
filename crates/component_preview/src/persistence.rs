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

    const MIGRATIONS: &[&str] = &[
        sql!(
            CREATE TABLE component_previews (
                workspace_id INTEGER,
                item_id INTEGER UNIQUE,
                active_page_id TEXT,
                PRIMARY KEY(workspace_id, item_id),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
            ) STRICT;
        ),
        sql!(
            CREATE TABLE component_previews_new (
                workspace_id INTEGER,
                item_id INTEGER,
                active_page_id TEXT,
                PRIMARY KEY(workspace_id, item_id),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
            ) STRICT;
            INSERT INTO component_previews_new (workspace_id, item_id, active_page_id)
                SELECT workspace_id, item_id, active_page_id FROM component_previews;
            DROP TABLE component_previews;
            ALTER TABLE component_previews_new RENAME TO component_previews;
        ),
    ];
}

db::static_connection!(ComponentPreviewDb, [WorkspaceDb]);

#[cfg(test)]
mod schema_tests {
    use super::ComponentPreviewDb;
    use db::sqlez::{
        connection::Connection,
        domain::{Domain as _, Migrator as _},
    };

    #[test]
    fn migration_preserves_rows() -> anyhow::Result<()> {
        let connection = connection(1)?;
        connection.exec("INSERT INTO component_previews VALUES (1, 0, 'Button'), (2, 9223372036854775807, NULL)")?()?;
        let rows = connection.select::<(i64, i64, Option<String>)>(
            "SELECT * FROM component_previews ORDER BY workspace_id",
        )?()?;
        ComponentPreviewDb::migrate(&connection)?;
        ComponentPreviewDb::migrate(&connection)?;
        assert_eq!(
            connection.select::<(i64, i64, Option<String>)>(
                "SELECT * FROM component_previews ORDER BY workspace_id"
            )?()?,
            rows
        );
        assert_eq!(
            connection
                .select::<String>("SELECT origin FROM pragma_index_list('component_previews')")?(
            )?,
            ["pk"]
        );
        Ok(())
    }

    #[test]
    fn workspace_scoped_item_ids() -> anyhow::Result<()> {
        let connection = connection(ComponentPreviewDb::MIGRATIONS.len())?;
        connection.exec("PRAGMA foreign_keys = ON")?()?;
        connection.exec("INSERT INTO component_previews(item_id, workspace_id, active_page_id) VALUES (7, 1, 'Button'), (7, 2, 'Label') ON CONFLICT(workspace_id, item_id) DO UPDATE SET active_page_id = excluded.active_page_id")?()?;
        connection.exec("INSERT INTO component_previews(item_id, workspace_id, active_page_id) VALUES (7, 1, 'Icon') ON CONFLICT(workspace_id, item_id) DO UPDATE SET active_page_id = excluded.active_page_id")?()?;
        assert_eq!(connection.select::<(i64, i64, String)>("SELECT workspace_id, item_id, active_page_id FROM component_previews ORDER BY workspace_id")?()?, [(1, 7, String::from("Icon")), (2, 7, String::from("Label"))]);
        connection.exec("DELETE FROM workspaces WHERE workspace_id = 1")?()?;
        assert_eq!(
            connection.select::<(i64, i64, String)>(
                "SELECT workspace_id, item_id, active_page_id FROM component_previews"
            )?()?,
            [(2, 7, String::from("Label"))]
        );
        assert!(
            connection
                .exec("INSERT INTO component_previews(workspace_id, item_id) VALUES (3, 7)")?(
            )
            .is_err()
        );
        Ok(())
    }

    fn connection(migration_count: usize) -> anyhow::Result<Connection> {
        let connection = Connection::open_memory(None);
        connection.exec("CREATE TABLE workspaces(workspace_id INTEGER PRIMARY KEY) STRICT")?()?;
        connection.exec("INSERT INTO workspaces VALUES (1), (2)")?()?;
        connection.migrate(
            ComponentPreviewDb::NAME,
            &ComponentPreviewDb::MIGRATIONS[..migration_count],
            &mut |_, _, _| false,
        )?;
        Ok(connection)
    }
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
            ON CONFLICT(workspace_id, item_id) DO UPDATE SET
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
