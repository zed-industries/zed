use anyhow::{Context, Result};
use indoc::indoc;
use sqlez::migrations::Migration;

use crate::{
    model::{ItemId, PaneId, SerializedItem, SerializedItemKind, WorkspaceId},
    Db,
};

// 1) Move all of this into Workspace crate
// 2) Deserialize items fully
// 3) Typed prepares (including how you expect to pull data out)
// 4) Investigate Tree column impls
pub(crate) const ITEM_MIGRATIONS: Migration = Migration::new(
    "item",
    &[indoc! {"
        CREATE TABLE items(
            item_id INTEGER NOT NULL, -- This is the item's view id, so this is not unique
            workspace_id BLOB NOT NULL,
            pane_id INTEGER NOT NULL,
            kind TEXT NOT NULL,
            position INTEGER NOT NULL,
            FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id) ON DELETE CASCADE
            FOREIGN KEY(pane_id) REFERENCES panes(pane_id) ON DELETE CASCADE
            PRIMARY KEY(item_id, workspace_id)
        ) STRICT;
    "}],
);

impl Db {
    pub(crate) fn get_items(&self, pane_id: PaneId) -> Result<Vec<SerializedItem>> {
        Ok(self
            .prepare(indoc! {"
                SELECT item_id, kind FROM items
                WHERE pane_id = ?
                ORDER BY position"})?
            .with_bindings(pane_id)?
            .rows::<(ItemId, SerializedItemKind)>()?
            .into_iter()
            .map(|(item_id, kind)| match kind {
                SerializedItemKind::Terminal => SerializedItem::Terminal { item_id },
                _ => unimplemented!(),
            })
            .collect())
    }

    pub(crate) fn save_items(
        &self,
        workspace_id: &WorkspaceId,
        pane_id: PaneId,
        items: &[SerializedItem],
    ) -> Result<()> {
        let mut delete_old = self
            .prepare("DELETE FROM items WHERE workspace_id = ? AND pane_id = ? AND item_id = ?")
            .context("Preparing deletion")?;
        let mut insert_new = self.prepare(
            "INSERT INTO items(item_id, workspace_id, pane_id, kind, position) VALUES (?, ?, ?, ?, ?)",
        ).context("Preparing insertion")?;
        for (position, item) in items.iter().enumerate() {
            delete_old
                .with_bindings((workspace_id, pane_id, item.item_id()))?
                .exec()?;

            insert_new
                .with_bindings((item.item_id(), workspace_id, pane_id, item.kind(), position))?
                .exec()?;
        }

        Ok(())
    }
}
