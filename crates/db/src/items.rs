use std::{ffi::OsStr, fmt::Display, hash::Hash, os::unix::prelude::OsStrExt, path::PathBuf};

use anyhow::Result;
use collections::HashSet;
use rusqlite::{named_params, params};

use super::Db;

/// Current design makes the cut at the item level,
///   - Maybe A little more bottom up, serialize 'Terminals' and 'Editors' directly, and then make a seperate
///   - items table, with a kind, and an integer that acts as a key to one of these other tables
/// This column is a foreign key to ONE OF: editors, terminals, searches
///   -

// (workspace_id, item_id)
// kind -> ::Editor::

// ->
// At the workspace level
// -> (Workspace_ID, item_id)
// -> One shot, big query, load everything up:

// -> SerializedWorkspace::deserialize(tx, itemKey)
//     -> SerializedEditor::deserialize(tx, itemKey)

//         ->
// -> Workspace::new(SerializedWorkspace)
//     -> Editor::new(serialized_workspace[???]serializedEditor)

// //Pros: Keeps sql out of every body elese, makes changing it easier (e.g. for loading from a network or RocksDB)
// //Cons: DB has to know the internals of the entire rest of the app

// Workspace
// Worktree roots
// Pane groups
// Dock
// Items
// Sidebars

pub(crate) const ITEMS_M_1: &str = "
CREATE TABLE items(
    workspace_id INTEGER,
    item_id INTEGER,
    kind TEXT NOT NULL,
    PRIMARY KEY (workspace_id, item_id)
    FOREIGN KEY(workspace_id) REFERENCES workspace_ids(workspace_id)
) STRICT;

CREATE TABLE project_searches(
    workspace_id INTEGER,
    item_id INTEGER,
    query TEXT,
    PRIMARY KEY (workspace_id, item_id)
    FOREIGN KEY(workspace_id) REFERENCES workspace_ids(workspace_id)
) STRICT;

CREATE TABLE editors(
    workspace_id INTEGER,
    item_id INTEGER,
    path BLOB NOT NULL,
    PRIMARY KEY (workspace_id, item_id)
    FOREIGN KEY(workspace_id) REFERENCES workspace_ids(workspace_id)
) STRICT;
";
