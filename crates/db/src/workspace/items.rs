use anyhow::{Context, Result};
use indoc::indoc;
use sqlez::migrations::Migration;

use crate::{
    model::{ItemId, PaneId, SerializedItem, SerializedItemKind, WorkspaceId},
    Db,
};
// use collections::HashSet;
// use rusqlite::{named_params, params, types::FromSql};

// use crate::workspace::WorkspaceId;

// use super::Db;

// /// Current design makes the cut at the item level,
// ///   - Maybe A little more bottom up, serialize 'Terminals' and 'Editors' directly, and then make a seperate
// ///   - items table, with a kind, and an integer that acts as a key to one of these other tables
// /// This column is a foreign key to ONE OF: editors, terminals, searches
// ///   -

// // (workspace_id, item_id)
// // kind -> ::Editor::

// // ->
// // At the workspace level
// // -> (Workspace_ID, item_id)
// // -> One shot, big query, load everything up:

// // -> SerializedWorkspace::deserialize(tx, itemKey)
// //     -> SerializedEditor::deserialize(tx, itemKey)

// //         ->
// // -> Workspace::new(SerializedWorkspace)
// //     -> Editor::new(serialized_workspace[???]serializedEditor)

// // //Pros: Keeps sql out of every body elese, makes changing it easier (e.g. for loading from a network or RocksDB)
// // //Cons: DB has to know the internals of the entire rest of the app

// // Workspace
// // Worktree roots
// // Pane groups
// // Dock
// // Items
// // Sidebars

// // Things I'm doing: finding about nullability for foreign keys
// pub(crate) const ITEMS_M_1: &str = "
// CREATE TABLE project_searches(
//     workspace_id INTEGER,
//     item_id INTEGER,
//     query TEXT,
//     PRIMARY KEY (workspace_id, item_id)
//     FOREIGN KEY(workspace_id) REFERENCES workspace_ids(workspace_id)
// ) STRICT;

// CREATE TABLE editors(
//     workspace_id INTEGER,
//     item_id INTEGER,
//     path BLOB NOT NULL,
//     PRIMARY KEY (workspace_id, item_id)
//     FOREIGN KEY(workspace_id) REFERENCES workspace_ids(workspace_id)
// ) STRICT;
// ";

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
