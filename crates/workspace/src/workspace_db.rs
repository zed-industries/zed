use anyhow::{bail, Context, Result};

use db::Db;
use util::{iife, unzip_option, ResultExt};

use std::path::{Path, PathBuf};

use indoc::indoc;
use sqlez::{connection::Connection, domain::Domain, migrations::Migration};

use super::Workspace;

use self::model::{
    Axis, GroupId, PaneId, SerializedItem, SerializedItemKind, SerializedPane, SerializedPaneGroup,
    SerializedWorkspace, WorkspaceId,
};

// 1) Move all of this into Workspace crate
// 2) Deserialize items fully
// 3) Typed prepares (including how you expect to pull data out)
// 4) Investigate Tree column impls

pub(crate) const WORKSPACES_MIGRATION: Migration = Migration::new(
    "workspace",
    &[indoc! {"
        CREATE TABLE workspaces(
            workspace_id BLOB PRIMARY KEY,
            dock_anchor TEXT, -- Enum: 'Bottom' / 'Right' / 'Expanded'
            dock_visible INTEGER, -- Boolean
            timestamp TEXT DEFAULT CURRENT_TIMESTAMP NOT NULL
        ) STRICT;
    "}],
);

pub(crate) const PANE_MIGRATIONS: Migration = Migration::new(
    "pane",
    &[indoc! {"
        CREATE TABLE pane_groups(
            group_id INTEGER PRIMARY KEY,
            workspace_id BLOB NOT NULL,
            parent_group_id INTEGER, -- NULL indicates that this is a root node
            position INTEGER, -- NULL indicates that this is a root node
            axis TEXT NOT NULL, -- Enum:  'Vertical' / 'Horizontal'
            FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id) ON DELETE CASCADE,
            FOREIGN KEY(parent_group_id) REFERENCES pane_groups(group_id) ON DELETE CASCADE
        ) STRICT;
        
        CREATE TABLE panes(
            pane_id INTEGER PRIMARY KEY,
            workspace_id BLOB NOT NULL,
            parent_group_id INTEGER, -- NULL, this is a dock pane
            position INTEGER, -- NULL, this is a dock pane
            FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id) ON DELETE CASCADE,
            FOREIGN KEY(parent_group_id) REFERENCES pane_groups(group_id) ON DELETE CASCADE
        ) STRICT;
    "}],
);

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

impl Domain for Workspace {
    fn migrate(conn: &Connection) -> anyhow::Result<()> {
        WORKSPACES_MIGRATION.run(&conn)?;
        PANE_MIGRATIONS.run(&conn)?;
        ITEM_MIGRATIONS.run(&conn)
    }
}

impl Workspace {
    /// Returns a serialized workspace for the given worktree_roots. If the passed array
    /// is empty, the most recent workspace is returned instead. If no workspace for the
    /// passed roots is stored, returns none.
    pub fn workspace_for_roots<P: AsRef<Path>>(
        db: &Db<Workspace>,
        worktree_roots: &[P],
    ) -> Option<SerializedWorkspace> {
        let workspace_id: WorkspaceId = worktree_roots.into();

        // Note that we re-assign the workspace_id here in case it's empty
        // and we've grabbed the most recent workspace
        let (workspace_id, dock_anchor, dock_visible) = iife!({
            if worktree_roots.len() == 0 {
                db.select_row(indoc! {"
                        SELECT workspace_id, dock_anchor, dock_visible 
                        FROM workspaces 
                        ORDER BY timestamp DESC LIMIT 1"})?()?
            } else {
                db.select_row_bound(indoc! {"
                        SELECT workspace_id, dock_anchor, dock_visible 
                        FROM workspaces 
                        WHERE workspace_id = ?"})?(&workspace_id)?
            }
            .context("No workspaces found")
        })
        .warn_on_err()
        .flatten()?;

        Some(SerializedWorkspace {
            dock_pane: Workspace::get_dock_pane(&db, &workspace_id)
                .context("Getting dock pane")
                .log_err()?,
            center_group: Workspace::get_center_pane_group(&db, &workspace_id)
                .context("Getting center group")
                .log_err()?,
            dock_anchor,
            dock_visible,
        })
    }

    /// Saves a workspace using the worktree roots. Will garbage collect any workspaces
    /// that used this workspace previously
    pub fn save_workspace<P: AsRef<Path>>(
        db: &Db<Workspace>,
        worktree_roots: &[P],
        old_roots: Option<&[P]>,
        workspace: &SerializedWorkspace,
    ) {
        let workspace_id: WorkspaceId = worktree_roots.into();

        db.with_savepoint("update_worktrees", || {
            if let Some(old_roots) = old_roots {
                let old_id: WorkspaceId = old_roots.into();

                db.exec_bound("DELETE FROM WORKSPACES WHERE workspace_id = ?")?(&old_id)?;
            }

            // Delete any previous workspaces with the same roots. This cascades to all
            // other tables that are based on the same roots set.
            // Insert new workspace into workspaces table if none were found
            db.exec_bound("DELETE FROM workspaces WHERE workspace_id = ?;")?(&workspace_id)?;

            db.exec_bound(
                "INSERT INTO workspaces(workspace_id, dock_anchor, dock_visible) VALUES (?, ?, ?)",
            )?((&workspace_id, workspace.dock_anchor, workspace.dock_visible))?;

            // Save center pane group and dock pane
            Workspace::save_pane_group(db, &workspace_id, &workspace.center_group, None)?;
            Workspace::save_pane(db, &workspace_id, &workspace.dock_pane, None)?;

            Ok(())
        })
        .with_context(|| {
            format!(
                "Update workspace with roots {:?}",
                worktree_roots
                    .iter()
                    .map(|p| p.as_ref())
                    .collect::<Vec<_>>()
            )
        })
        .log_err();
    }

    /// Returns the previous workspace ids sorted by last modified along with their opened worktree roots
    pub fn recent_workspaces(conn: &Connection, limit: usize) -> Vec<Vec<PathBuf>> {
        iife!({
            // TODO, upgrade anyhow: https://docs.rs/anyhow/1.0.66/anyhow/fn.Ok.html
            Ok::<_, anyhow::Error>(
                conn.select_bound::<usize, WorkspaceId>(
                    "SELECT workspace_id FROM workspaces ORDER BY timestamp DESC LIMIT ?",
                )?(limit)?
                .into_iter()
                .map(|id| id.paths())
                .collect::<Vec<Vec<PathBuf>>>(),
            )
        })
        .log_err()
        .unwrap_or_default()
    }

    pub(crate) fn get_center_pane_group(
        db: &Db<Workspace>,
        workspace_id: &WorkspaceId,
    ) -> Result<SerializedPaneGroup> {
        Workspace::get_pane_group_children(&db, workspace_id, None)?
            .into_iter()
            .next()
            .context("No center pane group")
    }

    fn get_pane_group_children<'a>(
        db: &Db<Workspace>,
        workspace_id: &WorkspaceId,
        group_id: Option<GroupId>,
    ) -> Result<Vec<SerializedPaneGroup>> {
        db.select_bound::<(Option<GroupId>, &WorkspaceId), (Option<GroupId>, Option<Axis>, Option<PaneId>)>(indoc! {"
            SELECT group_id, axis, pane_id
                FROM (SELECT group_id, axis, NULL as pane_id, position,  parent_group_id, workspace_id
                FROM pane_groups
                  UNION
                SELECT NULL, NULL,  pane_id,  position,  parent_group_id, workspace_id
                FROM panes
                -- Remove the dock panes from the union
                WHERE parent_group_id IS NOT NULL and position IS NOT NULL) 
            WHERE parent_group_id IS ? AND workspace_id = ?
            ORDER BY position
            "})?((group_id, workspace_id))?
            .into_iter()
            .map(|(group_id, axis, pane_id)| {
                if let Some((group_id, axis)) = group_id.zip(axis) {
                    Ok(SerializedPaneGroup::Group {
                        axis,
                        children: Workspace::get_pane_group_children(
                            db,
                            workspace_id,
                            Some(group_id),
                        )?,
                    })
                } else if let Some(pane_id) = pane_id {
                    Ok(SerializedPaneGroup::Pane(SerializedPane {
                        children: Workspace::get_items(db, pane_id)?,
                    }))
                } else {
                    bail!("Pane Group Child was neither a pane group or a pane");
                }
            })
            .collect::<Result<_>>()
    }

    pub(crate) fn save_pane_group(
        db: &Db<Workspace>,
        workspace_id: &WorkspaceId,
        pane_group: &SerializedPaneGroup,
        parent: Option<(GroupId, usize)>,
    ) -> Result<()> {
        if parent.is_none() && !matches!(pane_group, SerializedPaneGroup::Group { .. }) {
            bail!("Pane groups must have a SerializedPaneGroup::Group at the root")
        }

        let (parent_id, position) = unzip_option(parent);

        match pane_group {
            SerializedPaneGroup::Group { axis, children } => {
                let parent_id = db.insert_bound("INSERT INTO pane_groups(workspace_id, parent_group_id, position, axis) VALUES (?, ?, ?, ?)")?
                    ((workspace_id, parent_id, position, *axis))?;

                for (position, group) in children.iter().enumerate() {
                    Workspace::save_pane_group(
                        db,
                        workspace_id,
                        group,
                        Some((parent_id, position)),
                    )?
                }
                Ok(())
            }
            SerializedPaneGroup::Pane(pane) => Workspace::save_pane(db, workspace_id, pane, parent),
        }
    }

    pub(crate) fn get_dock_pane(
        db: &Db<Workspace>,
        workspace_id: &WorkspaceId,
    ) -> Result<SerializedPane> {
        let pane_id = db.select_row_bound(indoc! {"
                SELECT pane_id FROM panes 
                WHERE workspace_id = ? AND parent_group_id IS NULL AND position IS NULL"})?(
            workspace_id,
        )?
        .context("No dock pane for workspace")?;

        Ok(SerializedPane::new(
            Workspace::get_items(db, pane_id).context("Reading items")?,
        ))
    }

    pub(crate) fn save_pane(
        db: &Db<Workspace>,
        workspace_id: &WorkspaceId,
        pane: &SerializedPane,
        parent: Option<(GroupId, usize)>,
    ) -> Result<()> {
        let (parent_id, order) = unzip_option(parent);

        let pane_id = db.insert_bound(
            "INSERT INTO panes(workspace_id, parent_group_id, position) VALUES (?, ?, ?)",
        )?((workspace_id, parent_id, order))?;

        Workspace::save_items(db, workspace_id, pane_id, &pane.children).context("Saving items")
    }

    pub(crate) fn get_items(db: &Db<Workspace>, pane_id: PaneId) -> Result<Vec<SerializedItem>> {
        Ok(db.select_bound(indoc! {"
                SELECT item_id, kind FROM items
                WHERE pane_id = ?
                ORDER BY position"})?(pane_id)?
        .into_iter()
        .map(|(item_id, kind)| match kind {
            SerializedItemKind::Terminal => SerializedItem::Terminal { item_id },
            _ => unimplemented!(),
        })
        .collect())
    }

    pub(crate) fn save_items(
        db: &Db<Workspace>,
        workspace_id: &WorkspaceId,
        pane_id: PaneId,
        items: &[SerializedItem],
    ) -> Result<()> {
        let mut delete_old = db
            .exec_bound("DELETE FROM items WHERE workspace_id = ? AND pane_id = ? AND item_id = ?")
            .context("Preparing deletion")?;
        let mut insert_new = db.exec_bound(
            "INSERT INTO items(item_id, workspace_id, pane_id, kind, position) VALUES (?, ?, ?, ?, ?)",
        ).context("Preparing insertion")?;
        for (position, item) in items.iter().enumerate() {
            delete_old((workspace_id, pane_id, item.item_id()))?;
            insert_new((item.item_id(), workspace_id, pane_id, item.kind(), position))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::workspace_db::model::DockAnchor::{Bottom, Expanded, Right};
    use crate::{Db, Workspace};

    #[test]
    fn test_workspace_assignment() {
        // env_logger::try_init().ok();

        let db = Db::open_in_memory("test_basic_functionality");

        let workspace_1 = SerializedWorkspace {
            dock_anchor: Bottom,
            dock_visible: true,
            center_group: Default::default(),
            dock_pane: Default::default(),
        };

        let workspace_2 = SerializedWorkspace {
            dock_anchor: Expanded,
            dock_visible: false,
            center_group: Default::default(),
            dock_pane: Default::default(),
        };

        let workspace_3 = SerializedWorkspace {
            dock_anchor: Right,
            dock_visible: true,
            center_group: Default::default(),
            dock_pane: Default::default(),
        };

        Workspace::save_workspace(&db, &["/tmp", "/tmp2"], None, &workspace_1);
        Workspace::save_workspace(&db, &["/tmp"], None, &workspace_2);

        db.write_to("test.db").unwrap();

        // Test that paths are treated as a set
        assert_eq!(
            Workspace::workspace_for_roots(&db, &["/tmp", "/tmp2"]).unwrap(),
            workspace_1
        );
        assert_eq!(
            Workspace::workspace_for_roots(&db, &["/tmp2", "/tmp"]).unwrap(),
            workspace_1
        );

        // Make sure that other keys work
        assert_eq!(
            Workspace::workspace_for_roots(&db, &["/tmp"]).unwrap(),
            workspace_2
        );
        assert_eq!(
            Workspace::workspace_for_roots(&db, &["/tmp3", "/tmp2", "/tmp4"]),
            None
        );

        // Test 'mutate' case of updating a pre-existing id
        Workspace::save_workspace(
            &db,
            &["/tmp", "/tmp2"],
            Some(&["/tmp", "/tmp2"]),
            &workspace_2,
        );
        assert_eq!(
            Workspace::workspace_for_roots(&db, &["/tmp", "/tmp2"]).unwrap(),
            workspace_2
        );

        // Test other mechanism for mutating
        Workspace::save_workspace(&db, &["/tmp", "/tmp2"], None, &workspace_3);
        assert_eq!(
            Workspace::workspace_for_roots(&db, &["/tmp", "/tmp2"]).unwrap(),
            workspace_3
        );

        // Make sure that updating paths differently also works
        Workspace::save_workspace(
            &db,
            &["/tmp3", "/tmp4", "/tmp2"],
            Some(&["/tmp", "/tmp2"]),
            &workspace_3,
        );
        assert_eq!(Workspace::workspace_for_roots(&db, &["/tmp2", "tmp"]), None);
        assert_eq!(
            Workspace::workspace_for_roots(&db, &["/tmp2", "/tmp3", "/tmp4"]).unwrap(),
            workspace_3
        );
    }

    use crate::workspace_db::model::SerializedWorkspace;
    use crate::workspace_db::model::{SerializedItem, SerializedPane, SerializedPaneGroup};

    fn default_workspace(
        dock_pane: SerializedPane,
        center_group: &SerializedPaneGroup,
    ) -> SerializedWorkspace {
        SerializedWorkspace {
            dock_anchor: crate::workspace_db::model::DockAnchor::Right,
            dock_visible: false,
            center_group: center_group.clone(),
            dock_pane,
        }
    }

    #[test]
    fn test_basic_dock_pane() {
        // env_logger::try_init().ok();

        let db = Db::open_in_memory("basic_dock_pane");

        let dock_pane = crate::workspace_db::model::SerializedPane {
            children: vec![
                SerializedItem::Terminal { item_id: 1 },
                SerializedItem::Terminal { item_id: 4 },
                SerializedItem::Terminal { item_id: 2 },
                SerializedItem::Terminal { item_id: 3 },
            ],
        };

        let workspace = default_workspace(dock_pane, &Default::default());

        Workspace::save_workspace(&db, &["/tmp"], None, &workspace);

        let new_workspace = Workspace::workspace_for_roots(&db, &["/tmp"]).unwrap();

        assert_eq!(workspace.dock_pane, new_workspace.dock_pane);
    }

    #[test]
    fn test_simple_split() {
        // env_logger::try_init().ok();

        let db = Db::open_in_memory("simple_split");

        //  -----------------
        //  | 1,2   | 5,6   |
        //  | - - - |       |
        //  | 3,4   |       |
        //  -----------------
        let center_pane = SerializedPaneGroup::Group {
            axis: crate::workspace_db::model::Axis::Horizontal,
            children: vec![
                SerializedPaneGroup::Group {
                    axis: crate::workspace_db::model::Axis::Vertical,
                    children: vec![
                        SerializedPaneGroup::Pane(SerializedPane {
                            children: vec![
                                SerializedItem::Terminal { item_id: 1 },
                                SerializedItem::Terminal { item_id: 2 },
                            ],
                        }),
                        SerializedPaneGroup::Pane(SerializedPane {
                            children: vec![
                                SerializedItem::Terminal { item_id: 4 },
                                SerializedItem::Terminal { item_id: 3 },
                            ],
                        }),
                    ],
                },
                SerializedPaneGroup::Pane(SerializedPane {
                    children: vec![
                        SerializedItem::Terminal { item_id: 5 },
                        SerializedItem::Terminal { item_id: 6 },
                    ],
                }),
            ],
        };

        let workspace = default_workspace(Default::default(), &center_pane);

        Workspace::save_workspace(&db, &["/tmp"], None, &workspace);

        assert_eq!(workspace.center_group, center_pane);
    }
}

pub mod model {
    use std::{
        path::{Path, PathBuf},
        sync::Arc,
    };

    use anyhow::{bail, Result};

    use sqlez::{
        bindable::{Bind, Column},
        statement::Statement,
    };

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct WorkspaceId(Vec<PathBuf>);

    impl WorkspaceId {
        pub fn paths(self) -> Vec<PathBuf> {
            self.0
        }
    }

    impl<P: AsRef<Path>, T: IntoIterator<Item = P>> From<T> for WorkspaceId {
        fn from(iterator: T) -> Self {
            let mut roots = iterator
                .into_iter()
                .map(|p| p.as_ref().to_path_buf())
                .collect::<Vec<_>>();
            roots.sort();
            Self(roots)
        }
    }

    impl Bind for &WorkspaceId {
        fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
            bincode::serialize(&self.0)
                .expect("Bincode serialization of paths should not fail")
                .bind(statement, start_index)
        }
    }

    impl Column for WorkspaceId {
        fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
            let blob = statement.column_blob(start_index)?;
            Ok((WorkspaceId(bincode::deserialize(blob)?), start_index + 1))
        }
    }

    #[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
    pub enum DockAnchor {
        #[default]
        Bottom,
        Right,
        Expanded,
    }

    impl Bind for DockAnchor {
        fn bind(&self, statement: &Statement, start_index: i32) -> anyhow::Result<i32> {
            match self {
                DockAnchor::Bottom => "Bottom",
                DockAnchor::Right => "Right",
                DockAnchor::Expanded => "Expanded",
            }
            .bind(statement, start_index)
        }
    }

    impl Column for DockAnchor {
        fn column(statement: &mut Statement, start_index: i32) -> anyhow::Result<(Self, i32)> {
            String::column(statement, start_index).and_then(|(anchor_text, next_index)| {
                Ok((
                    match anchor_text.as_ref() {
                        "Bottom" => DockAnchor::Bottom,
                        "Right" => DockAnchor::Right,
                        "Expanded" => DockAnchor::Expanded,
                        _ => bail!("Stored dock anchor is incorrect"),
                    },
                    next_index,
                ))
            })
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct SerializedWorkspace {
        pub dock_anchor: DockAnchor,
        pub dock_visible: bool,
        pub center_group: SerializedPaneGroup,
        pub dock_pane: SerializedPane,
    }

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub enum Axis {
        #[default]
        Horizontal,
        Vertical,
    }

    impl Bind for Axis {
        fn bind(&self, statement: &Statement, start_index: i32) -> anyhow::Result<i32> {
            match self {
                Axis::Horizontal => "Horizontal",
                Axis::Vertical => "Vertical",
            }
            .bind(statement, start_index)
        }
    }

    impl Column for Axis {
        fn column(statement: &mut Statement, start_index: i32) -> anyhow::Result<(Self, i32)> {
            String::column(statement, start_index).and_then(|(axis_text, next_index)| {
                Ok((
                    match axis_text.as_str() {
                        "Horizontal" => Axis::Horizontal,
                        "Vertical" => Axis::Vertical,
                        _ => bail!("Stored serialized item kind is incorrect"),
                    },
                    next_index,
                ))
            })
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum SerializedPaneGroup {
        Group {
            axis: Axis,
            children: Vec<SerializedPaneGroup>,
        },
        Pane(SerializedPane),
    }

    // Dock panes, and grouped panes combined?
    // AND we're collapsing PaneGroup::Pane
    // In the case where

    impl Default for SerializedPaneGroup {
        fn default() -> Self {
            Self::Group {
                axis: Axis::Horizontal,
                children: vec![Self::Pane(Default::default())],
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Default, Clone)]
    pub struct SerializedPane {
        pub(crate) children: Vec<SerializedItem>,
    }

    impl SerializedPane {
        pub fn new(children: Vec<SerializedItem>) -> Self {
            SerializedPane { children }
        }
    }

    pub type GroupId = i64;
    pub type PaneId = i64;
    pub type ItemId = usize;

    pub(crate) enum SerializedItemKind {
        Editor,
        Diagnostics,
        ProjectSearch,
        Terminal,
    }

    impl Bind for SerializedItemKind {
        fn bind(&self, statement: &Statement, start_index: i32) -> anyhow::Result<i32> {
            match self {
                SerializedItemKind::Editor => "Editor",
                SerializedItemKind::Diagnostics => "Diagnostics",
                SerializedItemKind::ProjectSearch => "ProjectSearch",
                SerializedItemKind::Terminal => "Terminal",
            }
            .bind(statement, start_index)
        }
    }

    impl Column for SerializedItemKind {
        fn column(statement: &mut Statement, start_index: i32) -> anyhow::Result<(Self, i32)> {
            String::column(statement, start_index).and_then(|(kind_text, next_index)| {
                Ok((
                    match kind_text.as_ref() {
                        "Editor" => SerializedItemKind::Editor,
                        "Diagnostics" => SerializedItemKind::Diagnostics,
                        "ProjectSearch" => SerializedItemKind::ProjectSearch,
                        "Terminal" => SerializedItemKind::Terminal,
                        _ => bail!("Stored serialized item kind is incorrect"),
                    },
                    next_index,
                ))
            })
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum SerializedItem {
        Editor { item_id: usize, path: Arc<Path> },
        Diagnostics { item_id: usize },
        ProjectSearch { item_id: usize, query: String },
        Terminal { item_id: usize },
    }

    impl SerializedItem {
        pub fn item_id(&self) -> usize {
            match self {
                SerializedItem::Editor { item_id, .. } => *item_id,
                SerializedItem::Diagnostics { item_id } => *item_id,
                SerializedItem::ProjectSearch { item_id, .. } => *item_id,
                SerializedItem::Terminal { item_id } => *item_id,
            }
        }

        pub(crate) fn kind(&self) -> SerializedItemKind {
            match self {
                SerializedItem::Editor { .. } => SerializedItemKind::Editor,
                SerializedItem::Diagnostics { .. } => SerializedItemKind::Diagnostics,
                SerializedItem::ProjectSearch { .. } => SerializedItemKind::ProjectSearch,
                SerializedItem::Terminal { .. } => SerializedItemKind::Terminal,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use sqlez::connection::Connection;

        use crate::workspace_db::model::DockAnchor;

        use super::WorkspaceId;

        #[test]
        fn test_workspace_round_trips() {
            let db = Connection::open_memory("workspace_id_round_trips");

            db.exec(indoc::indoc! {"
                CREATE TABLE workspace_id_test(
                    workspace_id BLOB,
                    dock_anchor TEXT
                );"})
                .unwrap()()
            .unwrap();

            let workspace_id: WorkspaceId = WorkspaceId::from(&["\test2", "\test1"]);

            db.exec_bound("INSERT INTO workspace_id_test(workspace_id, dock_anchor) VALUES (?,?)")
                .unwrap()((&workspace_id, DockAnchor::Bottom))
            .unwrap();

            assert_eq!(
                db.select_row("SELECT workspace_id, dock_anchor FROM workspace_id_test LIMIT 1")
                    .unwrap()()
                .unwrap(),
                Some((WorkspaceId::from(&["\test1", "\test2"]), DockAnchor::Bottom))
            );
        }
    }
}
