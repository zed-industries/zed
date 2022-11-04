pub(crate) mod items;
pub mod model;
pub(crate) mod pane;

use anyhow::Context;
use util::{iife, ResultExt};

use std::path::{Path, PathBuf};

use indoc::indoc;
use sqlez::migrations::Migration;

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

use self::model::{SerializedWorkspace, WorkspaceId, WorkspaceRow};

use super::Db;

impl Db {
    /// Returns a serialized workspace for the given worktree_roots. If the passed array
    /// is empty, the most recent workspace is returned instead. If no workspace for the
    /// passed roots is stored, returns none.
    pub fn workspace_for_roots<P: AsRef<Path>>(
        &self,
        worktree_roots: &[P],
    ) -> Option<SerializedWorkspace> {
        let workspace_id: WorkspaceId = worktree_roots.into();

        // Note that we re-assign the workspace_id here in case it's empty
        // and we've grabbed the most recent workspace
        let (workspace_id, dock_anchor, dock_visible) = iife!({
            if worktree_roots.len() == 0 {
                self.prepare(indoc! {"
                        SELECT workspace_id, dock_anchor, dock_visible 
                        FROM workspaces 
                        ORDER BY timestamp DESC LIMIT 1"})?
                    .maybe_row::<WorkspaceRow>()
            } else {
                self.prepare(indoc! {"
                        SELECT workspace_id, dock_anchor, dock_visible 
                        FROM workspaces 
                        WHERE workspace_id = ?"})?
                    .with_bindings(&workspace_id)?
                    .maybe_row::<WorkspaceRow>()
            }
        })
        .log_err()
        .flatten()?;

        Some(SerializedWorkspace {
            dock_pane: self
                .get_dock_pane(&workspace_id)
                .context("Getting dock pane")
                .log_err()?,
            center_group: self
                .get_center_group(&workspace_id)
                .context("Getting center group")
                .log_err()?,
            dock_anchor,
            dock_visible,
        })
    }

    /// Saves a workspace using the worktree roots. Will garbage collect any workspaces
    /// that used this workspace previously
    pub fn save_workspace<P: AsRef<Path>>(
        &self,
        worktree_roots: &[P],
        old_roots: Option<&[P]>,
        workspace: &SerializedWorkspace,
    ) {
        let workspace_id: WorkspaceId = worktree_roots.into();

        self.with_savepoint("update_worktrees", || {
            if let Some(old_roots) = old_roots {
                let old_id: WorkspaceId = old_roots.into();

                self.prepare("DELETE FROM WORKSPACES WHERE workspace_id = ?")?
                    .with_bindings(&old_id)?
                    .exec()?;
            }

            // Delete any previous workspaces with the same roots. This cascades to all
            // other tables that are based on the same roots set.
            // Insert new workspace into workspaces table if none were found
            self.prepare("DELETE FROM workspaces WHERE workspace_id = ?;")?
                .with_bindings(&workspace_id)?
                .exec()?;

            self.prepare(
                "INSERT INTO workspaces(workspace_id, dock_anchor, dock_visible) VALUES (?, ?, ?)",
            )?
            .with_bindings((&workspace_id, workspace.dock_anchor, workspace.dock_visible))?
            .exec()?;

            // Save center pane group and dock pane
            self.save_center_group(&workspace_id, &workspace.center_group)?;
            self.save_dock_pane(&workspace_id, &workspace.dock_pane)?;

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
    pub fn recent_workspaces(&self, limit: usize) -> Vec<Vec<PathBuf>> {
        iife!({
            // TODO, upgrade anyhow: https://docs.rs/anyhow/1.0.66/anyhow/fn.Ok.html
            Ok::<_, anyhow::Error>(
                self.prepare(
                    "SELECT workspace_id FROM workspaces ORDER BY timestamp DESC LIMIT ?",
                )?
                .with_bindings(limit)?
                .rows::<WorkspaceId>()?
                .into_iter()
                .map(|id| id.paths())
                .collect::<Vec<Vec<PathBuf>>>(),
            )
        })
        .log_err()
        .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        model::{
            DockAnchor::{Bottom, Expanded, Right},
            SerializedWorkspace,
        },
        Db,
    };

    #[test]
    fn test_basic_functionality() {
        env_logger::init();

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

        db.save_workspace(&["/tmp", "/tmp2"], None, &workspace_1);
        db.save_workspace(&["/tmp"], None, &workspace_2);

        db.write_to("test.db").unwrap();

        // Test that paths are treated as a set
        assert_eq!(
            db.workspace_for_roots(&["/tmp", "/tmp2"]).unwrap(),
            workspace_1
        );
        assert_eq!(
            db.workspace_for_roots(&["/tmp2", "/tmp"]).unwrap(),
            workspace_1
        );

        // Make sure that other keys work
        assert_eq!(db.workspace_for_roots(&["/tmp"]).unwrap(), workspace_2);
        assert_eq!(db.workspace_for_roots(&["/tmp3", "/tmp2", "/tmp4"]), None);

        // Test 'mutate' case of updating a pre-existing id
        db.save_workspace(&["/tmp", "/tmp2"], Some(&["/tmp", "/tmp2"]), &workspace_2);
        assert_eq!(
            db.workspace_for_roots(&["/tmp", "/tmp2"]).unwrap(),
            workspace_2
        );

        // Test other mechanism for mutating
        db.save_workspace(&["/tmp", "/tmp2"], None, &workspace_3);
        assert_eq!(
            db.workspace_for_roots(&["/tmp", "/tmp2"]).unwrap(),
            workspace_3
        );

        // Make sure that updating paths differently also works
        db.save_workspace(
            &["/tmp3", "/tmp4", "/tmp2"],
            Some(&["/tmp", "/tmp2"]),
            &workspace_3,
        );
        assert_eq!(db.workspace_for_roots(&["/tmp2", "tmp"]), None);
        assert_eq!(
            db.workspace_for_roots(&["/tmp2", "/tmp3", "/tmp4"])
                .unwrap(),
            workspace_3
        );
    }
}
