// use std::{
//     ffi::OsStr,
//     fmt::Display,
//     hash::Hash,
//     os::unix::prelude::OsStrExt,
//     path::{Path, PathBuf},
//     sync::Arc,
// };

// use anyhow::Result;
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

#[derive(Debug, PartialEq, Eq)]
pub struct ItemId {
    pub item_id: usize,
}

// enum SerializedItemKind {
//     Editor,
//     Diagnostics,
//     ProjectSearch,
//     Terminal,
// }

// struct SerializedItemRow {
//     kind: SerializedItemKind,
//     item_id: usize,
//     path: Option<Arc<Path>>,
//     query: Option<String>,
// }

// #[derive(Debug, PartialEq, Eq)]
// pub enum SerializedItem {
//     Editor { item_id: usize, path: Arc<Path> },
//     Diagnostics { item_id: usize },
//     ProjectSearch { item_id: usize, query: String },
//     Terminal { item_id: usize },
// }

// impl SerializedItem {
//     pub fn item_id(&self) -> usize {
//         match self {
//             SerializedItem::Editor { item_id, .. } => *item_id,
//             SerializedItem::Diagnostics { item_id } => *item_id,
//             SerializedItem::ProjectSearch { item_id, .. } => *item_id,
//             SerializedItem::Terminal { item_id } => *item_id,
//         }
//     }
// }

// impl Db {
//     pub fn get_item(&self, item_id: ItemId) -> SerializedItem {
//         unimplemented!()
//     }

//     pub fn save_item(&self, workspace_id: WorkspaceId, item: &SerializedItem) {}

//     pub fn close_item(&self, item_id: ItemId) {}
// }
