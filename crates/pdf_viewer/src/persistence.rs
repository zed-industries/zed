use std::path::PathBuf;

use db::{
    query,
    sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
    sqlez_macros::sql,
};
use workspace::{ItemId, WorkspaceDb, WorkspaceId};

pub struct PdfViewerDb(ThreadSafeConnection);

impl Domain for PdfViewerDb {
    const NAME: &str = stringify!(PdfViewerDb);

    const MIGRATIONS: &[&str] = &[
        sql!(
            CREATE TABLE pdf_viewers (
                workspace_id INTEGER,
                item_id INTEGER UNIQUE,
                pdf_path BLOB,
                page_number INTEGER DEFAULT 0,
                zoom_level REAL DEFAULT 1.0,
                PRIMARY KEY(workspace_id, item_id),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
            ) STRICT;
        ),
        sql!(
            ALTER TABLE pdf_viewers ADD COLUMN scroll_x REAL DEFAULT 0.0;
            ALTER TABLE pdf_viewers ADD COLUMN scroll_y REAL DEFAULT 0.0;
        ),
        sql!(
            ALTER TABLE pdf_viewers ADD COLUMN rotation INTEGER DEFAULT 0;
        ),
    ];
}

db::static_connection!(PdfViewerDb, [WorkspaceDb]);

impl PdfViewerDb {
    query! {
        pub async fn save_pdf_state(
            item_id: ItemId,
            workspace_id: WorkspaceId,
            pdf_path: PathBuf,
            page_number: usize,
            zoom_level: f32,
            scroll_x: f32,
            scroll_y: f32,
            rotation: u16
        ) -> Result<()> {
            INSERT OR REPLACE INTO pdf_viewers(item_id, workspace_id, pdf_path, page_number, zoom_level, scroll_x, scroll_y, rotation)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        }
    }

    query! {
        pub fn get_pdf_state(
            item_id: ItemId,
            workspace_id: WorkspaceId
        ) -> Result<Option<(PathBuf, usize, f32, f32, f32, u16)>> {
            SELECT pdf_path, page_number, zoom_level, scroll_x, scroll_y, rotation
            FROM pdf_viewers
            WHERE item_id = ? AND workspace_id = ?
        }
    }
}
