pub mod model;

use std::path::Path;

use anyhow::{anyhow, bail, Context, Result};
use db::{define_connection, query, sqlez::connection::Connection, sqlez_macros::sql};
use gpui::{point, size, Axis, Bounds, WindowBounds};

use sqlez::{
    bindable::{Bind, Column, StaticColumnCount},
    statement::Statement,
};

use util::{unzip_option, ResultExt};
use uuid::Uuid;

use crate::WorkspaceId;

use model::{
    GroupId, PaneId, SerializedItem, SerializedPane, SerializedPaneGroup, SerializedWorkspace,
    WorkspaceLocation,
};

use self::model::DockStructure;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct SerializedAxis(pub(crate) gpui::Axis);
impl sqlez::bindable::StaticColumnCount for SerializedAxis {}
impl sqlez::bindable::Bind for SerializedAxis {
    fn bind(
        &self,
        statement: &sqlez::statement::Statement,
        start_index: i32,
    ) -> anyhow::Result<i32> {
        match self.0 {
            gpui::Axis::Horizontal => "Horizontal",
            gpui::Axis::Vertical => "Vertical",
        }
        .bind(statement, start_index)
    }
}

impl sqlez::bindable::Column for SerializedAxis {
    fn column(
        statement: &mut sqlez::statement::Statement,
        start_index: i32,
    ) -> anyhow::Result<(Self, i32)> {
        String::column(statement, start_index).and_then(|(axis_text, next_index)| {
            Ok((
                match axis_text.as_str() {
                    "Horizontal" => Self(Axis::Horizontal),
                    "Vertical" => Self(Axis::Vertical),
                    _ => anyhow::bail!("Stored serialized item kind is incorrect"),
                },
                next_index,
            ))
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SerializedWindowsBounds(pub(crate) WindowBounds);

impl StaticColumnCount for SerializedWindowsBounds {
    fn column_count() -> usize {
        5
    }
}

impl Bind for SerializedWindowsBounds {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        let (region, next_index) = match self.0 {
            WindowBounds::Fullscreen => {
                let next_index = statement.bind(&"Fullscreen", start_index)?;
                (None, next_index)
            }
            WindowBounds::Maximized => {
                let next_index = statement.bind(&"Maximized", start_index)?;
                (None, next_index)
            }
            WindowBounds::Fixed(region) => {
                let next_index = statement.bind(&"Fixed", start_index)?;
                (Some(region), next_index)
            }
        };

        statement.bind(
            &region.map(|region| {
                (
                    SerializedGlobalPixels(region.origin.x),
                    SerializedGlobalPixels(region.origin.y),
                    SerializedGlobalPixels(region.size.width),
                    SerializedGlobalPixels(region.size.height),
                )
            }),
            next_index,
        )
    }
}

impl Column for SerializedWindowsBounds {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (window_state, next_index) = String::column(statement, start_index)?;
        let bounds = match window_state.as_str() {
            "Fullscreen" => SerializedWindowsBounds(WindowBounds::Fullscreen),
            "Maximized" => SerializedWindowsBounds(WindowBounds::Maximized),
            "Fixed" => {
                let ((x, y, width, height), _) = Column::column(statement, next_index)?;
                let x: f64 = x;
                let y: f64 = y;
                let width: f64 = width;
                let height: f64 = height;
                SerializedWindowsBounds(WindowBounds::Fixed(Bounds {
                    origin: point(x.into(), y.into()),
                    size: size(width.into(), height.into()),
                }))
            }
            _ => bail!("Window State did not have a valid string"),
        };

        Ok((bounds, next_index + 4))
    }
}

#[derive(Clone, Debug, PartialEq)]
struct SerializedGlobalPixels(gpui::GlobalPixels);
impl sqlez::bindable::StaticColumnCount for SerializedGlobalPixels {}

impl sqlez::bindable::Bind for SerializedGlobalPixels {
    fn bind(
        &self,
        statement: &sqlez::statement::Statement,
        start_index: i32,
    ) -> anyhow::Result<i32> {
        let this: f64 = self.0.into();
        let this: f32 = this as _;
        this.bind(statement, start_index)
    }
}

define_connection! {
    // Current schema shape using pseudo-rust syntax:
    //
    // workspaces(
    //   workspace_id: usize, // Primary key for workspaces
    //   workspace_location: Bincode<Vec<PathBuf>>,
    //   dock_visible: bool, // Deprecated
    //   dock_anchor: DockAnchor, // Deprecated
    //   dock_pane: Option<usize>, // Deprecated
    //   left_sidebar_open: boolean,
    //   timestamp: String, // UTC YYYY-MM-DD HH:MM:SS
    //   window_state: String, // WindowBounds Discriminant
    //   window_x: Option<f32>, // WindowBounds::Fixed RectF x
    //   window_y: Option<f32>, // WindowBounds::Fixed RectF y
    //   window_width: Option<f32>, // WindowBounds::Fixed RectF width
    //   window_height: Option<f32>, // WindowBounds::Fixed RectF height
    //   display: Option<Uuid>, // Display id
    // )
    //
    // pane_groups(
    //   group_id: usize, // Primary key for pane_groups
    //   workspace_id: usize, // References workspaces table
    //   parent_group_id: Option<usize>, // None indicates that this is the root node
    //   position: Optiopn<usize>, // None indicates that this is the root node
    //   axis: Option<Axis>, // 'Vertical', 'Horizontal'
    //   flexes: Option<Vec<f32>>, // A JSON array of floats
    // )
    //
    // panes(
    //     pane_id: usize, // Primary key for panes
    //     workspace_id: usize, // References workspaces table
    //     active: bool,
    // )
    //
    // center_panes(
    //     pane_id: usize, // Primary key for center_panes
    //     parent_group_id: Option<usize>, // References pane_groups. If none, this is the root
    //     position: Option<usize>, // None indicates this is the root
    // )
    //
    // CREATE TABLE items(
    //     item_id: usize, // This is the item's view id, so this is not unique
    //     workspace_id: usize, // References workspaces table
    //     pane_id: usize, // References panes table
    //     kind: String, // Indicates which view this connects to. This is the key in the item_deserializers global
    //     position: usize, // Position of the item in the parent pane. This is equivalent to panes' position column
    //     active: bool, // Indicates if this item is the active one in the pane
    // )
    pub static ref DB: WorkspaceDb<()> =
    &[sql!(
        CREATE TABLE workspaces(
            workspace_id INTEGER PRIMARY KEY,
            workspace_location BLOB UNIQUE,
            dock_visible INTEGER, // Deprecated. Preserving so users can downgrade Zed.
            dock_anchor TEXT, // Deprecated. Preserving so users can downgrade Zed.
            dock_pane INTEGER, // Deprecated.  Preserving so users can downgrade Zed.
            left_sidebar_open INTEGER, // Boolean
            timestamp TEXT DEFAULT CURRENT_TIMESTAMP NOT NULL,
            FOREIGN KEY(dock_pane) REFERENCES panes(pane_id)
        ) STRICT;

        CREATE TABLE pane_groups(
            group_id INTEGER PRIMARY KEY,
            workspace_id INTEGER NOT NULL,
            parent_group_id INTEGER, // NULL indicates that this is a root node
            position INTEGER, // NULL indicates that this is a root node
            axis TEXT NOT NULL, // Enum: 'Vertical' / 'Horizontal'
            FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE,
            FOREIGN KEY(parent_group_id) REFERENCES pane_groups(group_id) ON DELETE CASCADE
        ) STRICT;

        CREATE TABLE panes(
            pane_id INTEGER PRIMARY KEY,
            workspace_id INTEGER NOT NULL,
            active INTEGER NOT NULL, // Boolean
            FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        ) STRICT;

        CREATE TABLE center_panes(
            pane_id INTEGER PRIMARY KEY,
            parent_group_id INTEGER, // NULL means that this is a root pane
            position INTEGER, // NULL means that this is a root pane
            FOREIGN KEY(pane_id) REFERENCES panes(pane_id)
            ON DELETE CASCADE,
            FOREIGN KEY(parent_group_id) REFERENCES pane_groups(group_id) ON DELETE CASCADE
        ) STRICT;

        CREATE TABLE items(
            item_id INTEGER NOT NULL, // This is the item's view id, so this is not unique
            workspace_id INTEGER NOT NULL,
            pane_id INTEGER NOT NULL,
            kind TEXT NOT NULL,
            position INTEGER NOT NULL,
            active INTEGER NOT NULL,
            FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE,
            FOREIGN KEY(pane_id) REFERENCES panes(pane_id)
            ON DELETE CASCADE,
            PRIMARY KEY(item_id, workspace_id)
        ) STRICT;
    ),
    sql!(
        ALTER TABLE workspaces ADD COLUMN window_state TEXT;
        ALTER TABLE workspaces ADD COLUMN window_x REAL;
        ALTER TABLE workspaces ADD COLUMN window_y REAL;
        ALTER TABLE workspaces ADD COLUMN window_width REAL;
        ALTER TABLE workspaces ADD COLUMN window_height REAL;
        ALTER TABLE workspaces ADD COLUMN display BLOB;
    ),
    // Drop foreign key constraint from workspaces.dock_pane to panes table.
    sql!(
        CREATE TABLE workspaces_2(
            workspace_id INTEGER PRIMARY KEY,
            workspace_location BLOB UNIQUE,
            dock_visible INTEGER, // Deprecated. Preserving so users can downgrade Zed.
            dock_anchor TEXT, // Deprecated. Preserving so users can downgrade Zed.
            dock_pane INTEGER, // Deprecated.  Preserving so users can downgrade Zed.
            left_sidebar_open INTEGER, // Boolean
            timestamp TEXT DEFAULT CURRENT_TIMESTAMP NOT NULL,
            window_state TEXT,
            window_x REAL,
            window_y REAL,
            window_width REAL,
            window_height REAL,
            display BLOB
        ) STRICT;
        INSERT INTO workspaces_2 SELECT * FROM workspaces;
        DROP TABLE workspaces;
        ALTER TABLE workspaces_2 RENAME TO workspaces;
    ),
    // Add panels related information
    sql!(
        ALTER TABLE workspaces ADD COLUMN left_dock_visible INTEGER; //bool
        ALTER TABLE workspaces ADD COLUMN left_dock_active_panel TEXT;
        ALTER TABLE workspaces ADD COLUMN right_dock_visible INTEGER; //bool
        ALTER TABLE workspaces ADD COLUMN right_dock_active_panel TEXT;
        ALTER TABLE workspaces ADD COLUMN bottom_dock_visible INTEGER; //bool
        ALTER TABLE workspaces ADD COLUMN bottom_dock_active_panel TEXT;
    ),
    // Add panel zoom persistence
    sql!(
        ALTER TABLE workspaces ADD COLUMN left_dock_zoom INTEGER; //bool
        ALTER TABLE workspaces ADD COLUMN right_dock_zoom INTEGER; //bool
        ALTER TABLE workspaces ADD COLUMN bottom_dock_zoom INTEGER; //bool
    ),
    // Add pane group flex data
    sql!(
        ALTER TABLE pane_groups ADD COLUMN flexes TEXT;
    )
    ];
}

impl WorkspaceDb {
    /// Returns a serialized workspace for the given worktree_roots. If the passed array
    /// is empty, the most recent workspace is returned instead. If no workspace for the
    /// passed roots is stored, returns none.
    pub(crate) fn workspace_for_roots<P: AsRef<Path>>(
        &self,
        worktree_roots: &[P],
    ) -> Option<SerializedWorkspace> {
        let workspace_location: WorkspaceLocation = worktree_roots.into();

        // Note that we re-assign the workspace_id here in case it's empty
        // and we've grabbed the most recent workspace
        let (workspace_id, workspace_location, bounds, display, docks): (
            WorkspaceId,
            WorkspaceLocation,
            Option<SerializedWindowsBounds>,
            Option<Uuid>,
            DockStructure,
        ) = self
            .select_row_bound(sql! {
                SELECT
                    workspace_id,
                    workspace_location,
                    window_state,
                    window_x,
                    window_y,
                    window_width,
                    window_height,
                    display,
                    left_dock_visible,
                    left_dock_active_panel,
                    left_dock_zoom,
                    right_dock_visible,
                    right_dock_active_panel,
                    right_dock_zoom,
                    bottom_dock_visible,
                    bottom_dock_active_panel,
                    bottom_dock_zoom
                FROM workspaces
                WHERE workspace_location = ?
            })
            .and_then(|mut prepared_statement| (prepared_statement)(&workspace_location))
            .context("No workspaces found")
            .warn_on_err()
            .flatten()?;

        Some(SerializedWorkspace {
            id: workspace_id,
            location: workspace_location.clone(),
            center_group: self
                .get_center_pane_group(workspace_id)
                .context("Getting center group")
                .log_err()?,
            bounds: bounds.map(|bounds| bounds.0),
            display,
            docks,
        })
    }

    /// Saves a workspace using the worktree roots. Will garbage collect any workspaces
    /// that used this workspace previously
    pub(crate) async fn save_workspace(&self, workspace: SerializedWorkspace) {
        self.write(move |conn| {
            conn.with_savepoint("update_worktrees", || {
                // Clear out panes and pane_groups
                conn.exec_bound(sql!(
                    DELETE FROM pane_groups WHERE workspace_id = ?1;
                    DELETE FROM panes WHERE workspace_id = ?1;))?(workspace.id)
                .expect("Clearing old panes");

                conn.exec_bound(sql!(
                    DELETE FROM workspaces WHERE workspace_location = ? AND workspace_id != ?
                ))?((&workspace.location, workspace.id))
                .context("clearing out old locations")?;

                // Upsert
                conn.exec_bound(sql!(
                    INSERT INTO workspaces(
                        workspace_id,
                        workspace_location,
                        left_dock_visible,
                        left_dock_active_panel,
                        left_dock_zoom,
                        right_dock_visible,
                        right_dock_active_panel,
                        right_dock_zoom,
                        bottom_dock_visible,
                        bottom_dock_active_panel,
                        bottom_dock_zoom,
                        timestamp
                    )
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, CURRENT_TIMESTAMP)
                    ON CONFLICT DO
                    UPDATE SET
                        workspace_location = ?2,
                        left_dock_visible = ?3,
                        left_dock_active_panel = ?4,
                        left_dock_zoom = ?5,
                        right_dock_visible = ?6,
                        right_dock_active_panel = ?7,
                        right_dock_zoom = ?8,
                        bottom_dock_visible = ?9,
                        bottom_dock_active_panel = ?10,
                        bottom_dock_zoom = ?11,
                        timestamp = CURRENT_TIMESTAMP
                ))?((workspace.id, &workspace.location, workspace.docks))
                .context("Updating workspace")?;

                // Save center pane group
                Self::save_pane_group(conn, workspace.id, &workspace.center_group, None)
                    .context("save pane group in save workspace")?;

                Ok(())
            })
            .log_err();
        })
        .await;
    }

    query! {
        pub async fn next_id() -> Result<WorkspaceId> {
            INSERT INTO workspaces DEFAULT VALUES RETURNING workspace_id
        }
    }

    query! {
        fn recent_workspaces() -> Result<Vec<(WorkspaceId, WorkspaceLocation)>> {
            SELECT workspace_id, workspace_location
            FROM workspaces
            WHERE workspace_location IS NOT NULL
            ORDER BY timestamp DESC
        }
    }

    query! {
        pub async fn delete_workspace_by_id(id: WorkspaceId) -> Result<()> {
            DELETE FROM workspaces
            WHERE workspace_id IS ?
        }
    }

    // Returns the recent locations which are still valid on disk and deletes ones which no longer
    // exist.
    pub async fn recent_workspaces_on_disk(&self) -> Result<Vec<(WorkspaceId, WorkspaceLocation)>> {
        let mut result = Vec::new();
        let mut delete_tasks = Vec::new();
        for (id, location) in self.recent_workspaces()? {
            if location.paths().iter().all(|path| path.exists())
                && location.paths().iter().any(|path| path.is_dir())
            {
                result.push((id, location));
            } else {
                delete_tasks.push(self.delete_workspace_by_id(id));
            }
        }

        futures::future::join_all(delete_tasks).await;
        Ok(result)
    }

    pub async fn last_workspace(&self) -> Result<Option<WorkspaceLocation>> {
        Ok(self
            .recent_workspaces_on_disk()
            .await?
            .into_iter()
            .next()
            .map(|(_, location)| location))
    }

    fn get_center_pane_group(&self, workspace_id: WorkspaceId) -> Result<SerializedPaneGroup> {
        Ok(self
            .get_pane_group(workspace_id, None)?
            .into_iter()
            .next()
            .unwrap_or_else(|| {
                SerializedPaneGroup::Pane(SerializedPane {
                    active: true,
                    children: vec![],
                })
            }))
    }

    fn get_pane_group(
        &self,
        workspace_id: WorkspaceId,
        group_id: Option<GroupId>,
    ) -> Result<Vec<SerializedPaneGroup>> {
        type GroupKey = (Option<GroupId>, WorkspaceId);
        type GroupOrPane = (
            Option<GroupId>,
            Option<SerializedAxis>,
            Option<PaneId>,
            Option<bool>,
            Option<String>,
        );
        self.select_bound::<GroupKey, GroupOrPane>(sql!(
            SELECT group_id, axis, pane_id, active, flexes
                FROM (SELECT
                        group_id,
                        axis,
                        NULL as pane_id,
                        NULL as active,
                        position,
                        parent_group_id,
                        workspace_id,
                        flexes
                      FROM pane_groups
                    UNION
                      SELECT
                        NULL,
                        NULL,
                        center_panes.pane_id,
                        panes.active as active,
                        position,
                        parent_group_id,
                        panes.workspace_id as workspace_id,
                        NULL
                      FROM center_panes
                      JOIN panes ON center_panes.pane_id = panes.pane_id)
                WHERE parent_group_id IS ? AND workspace_id = ?
                ORDER BY position
        ))?((group_id, workspace_id))?
        .into_iter()
        .map(|(group_id, axis, pane_id, active, flexes)| {
            if let Some((group_id, axis)) = group_id.zip(axis) {
                let flexes = flexes
                    .map(|flexes: String| serde_json::from_str::<Vec<f32>>(&flexes))
                    .transpose()?;

                Ok(SerializedPaneGroup::Group {
                    axis,
                    children: self.get_pane_group(workspace_id, Some(group_id))?,
                    flexes,
                })
            } else if let Some((pane_id, active)) = pane_id.zip(active) {
                Ok(SerializedPaneGroup::Pane(SerializedPane::new(
                    self.get_items(pane_id)?,
                    active,
                )))
            } else {
                bail!("Pane Group Child was neither a pane group or a pane");
            }
        })
        // Filter out panes and pane groups which don't have any children or items
        .filter(|pane_group| match pane_group {
            Ok(SerializedPaneGroup::Group { children, .. }) => !children.is_empty(),
            Ok(SerializedPaneGroup::Pane(pane)) => !pane.children.is_empty(),
            _ => true,
        })
        .collect::<Result<_>>()
    }

    fn save_pane_group(
        conn: &Connection,
        workspace_id: WorkspaceId,
        pane_group: &SerializedPaneGroup,
        parent: Option<(GroupId, usize)>,
    ) -> Result<()> {
        match pane_group {
            SerializedPaneGroup::Group {
                axis,
                children,
                flexes,
            } => {
                let (parent_id, position) = unzip_option(parent);

                let flex_string = flexes
                    .as_ref()
                    .map(|flexes| serde_json::json!(flexes).to_string());

                let group_id = conn.select_row_bound::<_, i64>(sql!(
                    INSERT INTO pane_groups(
                        workspace_id,
                        parent_group_id,
                        position,
                        axis,
                        flexes
                    )
                    VALUES (?, ?, ?, ?, ?)
                    RETURNING group_id
                ))?((
                    workspace_id,
                    parent_id,
                    position,
                    *axis,
                    flex_string,
                ))?
                .ok_or_else(|| anyhow!("Couldn't retrieve group_id from inserted pane_group"))?;

                for (position, group) in children.iter().enumerate() {
                    Self::save_pane_group(conn, workspace_id, group, Some((group_id, position)))?
                }

                Ok(())
            }
            SerializedPaneGroup::Pane(pane) => {
                Self::save_pane(conn, workspace_id, pane, parent)?;
                Ok(())
            }
        }
    }

    fn save_pane(
        conn: &Connection,
        workspace_id: WorkspaceId,
        pane: &SerializedPane,
        parent: Option<(GroupId, usize)>,
    ) -> Result<PaneId> {
        let pane_id = conn.select_row_bound::<_, i64>(sql!(
            INSERT INTO panes(workspace_id, active)
            VALUES (?, ?)
            RETURNING pane_id
        ))?((workspace_id, pane.active))?
        .ok_or_else(|| anyhow!("Could not retrieve inserted pane_id"))?;

        let (parent_id, order) = unzip_option(parent);
        conn.exec_bound(sql!(
            INSERT INTO center_panes(pane_id, parent_group_id, position)
            VALUES (?, ?, ?)
        ))?((pane_id, parent_id, order))?;

        Self::save_items(conn, workspace_id, pane_id, &pane.children).context("Saving items")?;

        Ok(pane_id)
    }

    fn get_items(&self, pane_id: PaneId) -> Result<Vec<SerializedItem>> {
        self.select_bound(sql!(
            SELECT kind, item_id, active FROM items
            WHERE pane_id = ?
                ORDER BY position
        ))?(pane_id)
    }

    fn save_items(
        conn: &Connection,
        workspace_id: WorkspaceId,
        pane_id: PaneId,
        items: &[SerializedItem],
    ) -> Result<()> {
        let mut insert = conn.exec_bound(sql!(
            INSERT INTO items(workspace_id, pane_id, position, kind, item_id, active) VALUES (?, ?, ?, ?, ?, ?)
        )).context("Preparing insertion")?;
        for (position, item) in items.iter().enumerate() {
            insert((workspace_id, pane_id, position, item))?;
        }

        Ok(())
    }

    query! {
        pub async fn update_timestamp(workspace_id: WorkspaceId) -> Result<()> {
            UPDATE workspaces
            SET timestamp = CURRENT_TIMESTAMP
            WHERE workspace_id = ?
        }
    }

    query! {
        pub(crate) async fn set_window_bounds(workspace_id: WorkspaceId, bounds: SerializedWindowsBounds, display: Uuid) -> Result<()> {
            UPDATE workspaces
            SET window_state = ?2,
                window_x = ?3,
                window_y = ?4,
                window_width = ?5,
                window_height = ?6,
                display = ?7
            WHERE workspace_id = ?1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use db::open_test_db;
    use gpui;

    #[gpui::test]
    async fn test_next_id_stability() {
        env_logger::try_init().ok();

        let db = WorkspaceDb(open_test_db("test_next_id_stability").await);

        db.write(|conn| {
            conn.migrate(
                "test_table",
                &[sql!(
                    CREATE TABLE test_table(
                        text TEXT,
                        workspace_id INTEGER,
                        FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                        ON DELETE CASCADE
                    ) STRICT;
                )],
            )
            .unwrap();
        })
        .await;

        let id = db.next_id().await.unwrap();
        // Assert the empty row got inserted
        assert_eq!(
            Some(id),
            db.select_row_bound::<WorkspaceId, WorkspaceId>(sql!(
                SELECT workspace_id FROM workspaces WHERE workspace_id = ?
            ))
            .unwrap()(id)
            .unwrap()
        );

        db.write(move |conn| {
            conn.exec_bound(sql!(INSERT INTO test_table(text, workspace_id) VALUES (?, ?)))
                .unwrap()(("test-text-1", id))
            .unwrap()
        })
        .await;

        let test_text_1 = db
            .select_row_bound::<_, String>(sql!(SELECT text FROM test_table WHERE workspace_id = ?))
            .unwrap()(1)
        .unwrap()
        .unwrap();
        assert_eq!(test_text_1, "test-text-1");
    }

    #[gpui::test]
    async fn test_workspace_id_stability() {
        env_logger::try_init().ok();

        let db = WorkspaceDb(open_test_db("test_workspace_id_stability").await);

        db.write(|conn| {
            conn.migrate(
                "test_table",
                &[sql!(
                        CREATE TABLE test_table(
                            text TEXT,
                            workspace_id INTEGER,
                            FOREIGN KEY(workspace_id)
                                REFERENCES workspaces(workspace_id)
                            ON DELETE CASCADE
                        ) STRICT;)],
            )
        })
        .await
        .unwrap();

        let mut workspace_1 = SerializedWorkspace {
            id: 1,
            location: (["/tmp", "/tmp2"]).into(),
            center_group: Default::default(),
            bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
        };

        let workspace_2 = SerializedWorkspace {
            id: 2,
            location: (["/tmp"]).into(),
            center_group: Default::default(),
            bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
        };

        db.save_workspace(workspace_1.clone()).await;

        db.write(|conn| {
            conn.exec_bound(sql!(INSERT INTO test_table(text, workspace_id) VALUES (?, ?)))
                .unwrap()(("test-text-1", 1))
            .unwrap();
        })
        .await;

        db.save_workspace(workspace_2.clone()).await;

        db.write(|conn| {
            conn.exec_bound(sql!(INSERT INTO test_table(text, workspace_id) VALUES (?, ?)))
                .unwrap()(("test-text-2", 2))
            .unwrap();
        })
        .await;

        workspace_1.location = (["/tmp", "/tmp3"]).into();
        db.save_workspace(workspace_1.clone()).await;
        db.save_workspace(workspace_1).await;
        db.save_workspace(workspace_2).await;

        let test_text_2 = db
            .select_row_bound::<_, String>(sql!(SELECT text FROM test_table WHERE workspace_id = ?))
            .unwrap()(2)
        .unwrap()
        .unwrap();
        assert_eq!(test_text_2, "test-text-2");

        let test_text_1 = db
            .select_row_bound::<_, String>(sql!(SELECT text FROM test_table WHERE workspace_id = ?))
            .unwrap()(1)
        .unwrap()
        .unwrap();
        assert_eq!(test_text_1, "test-text-1");
    }

    fn group(axis: Axis, children: Vec<SerializedPaneGroup>) -> SerializedPaneGroup {
        SerializedPaneGroup::Group {
            axis: SerializedAxis(axis),
            flexes: None,
            children,
        }
    }

    #[gpui::test]
    async fn test_full_workspace_serialization() {
        env_logger::try_init().ok();

        let db = WorkspaceDb(open_test_db("test_full_workspace_serialization").await);

        //  -----------------
        //  | 1,2   | 5,6   |
        //  | - - - |       |
        //  | 3,4   |       |
        //  -----------------
        let center_group = group(
            Axis::Horizontal,
            vec![
                group(
                    Axis::Vertical,
                    vec![
                        SerializedPaneGroup::Pane(SerializedPane::new(
                            vec![
                                SerializedItem::new("Terminal", 5, false),
                                SerializedItem::new("Terminal", 6, true),
                            ],
                            false,
                        )),
                        SerializedPaneGroup::Pane(SerializedPane::new(
                            vec![
                                SerializedItem::new("Terminal", 7, true),
                                SerializedItem::new("Terminal", 8, false),
                            ],
                            false,
                        )),
                    ],
                ),
                SerializedPaneGroup::Pane(SerializedPane::new(
                    vec![
                        SerializedItem::new("Terminal", 9, false),
                        SerializedItem::new("Terminal", 10, true),
                    ],
                    false,
                )),
            ],
        );

        let workspace = SerializedWorkspace {
            id: 5,
            location: (["/tmp", "/tmp2"]).into(),
            center_group,
            bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
        };

        db.save_workspace(workspace.clone()).await;
        let round_trip_workspace = db.workspace_for_roots(&["/tmp2", "/tmp"]);

        assert_eq!(workspace, round_trip_workspace.unwrap());

        // Test guaranteed duplicate IDs
        db.save_workspace(workspace.clone()).await;
        db.save_workspace(workspace.clone()).await;

        let round_trip_workspace = db.workspace_for_roots(&["/tmp", "/tmp2"]);
        assert_eq!(workspace, round_trip_workspace.unwrap());
    }

    #[gpui::test]
    async fn test_workspace_assignment() {
        env_logger::try_init().ok();

        let db = WorkspaceDb(open_test_db("test_basic_functionality").await);

        let workspace_1 = SerializedWorkspace {
            id: 1,
            location: (["/tmp", "/tmp2"]).into(),
            center_group: Default::default(),
            bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
        };

        let mut workspace_2 = SerializedWorkspace {
            id: 2,
            location: (["/tmp"]).into(),
            center_group: Default::default(),
            bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
        };

        db.save_workspace(workspace_1.clone()).await;
        db.save_workspace(workspace_2.clone()).await;

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
        workspace_2.location = (["/tmp", "/tmp2"]).into();

        db.save_workspace(workspace_2.clone()).await;
        assert_eq!(
            db.workspace_for_roots(&["/tmp", "/tmp2"]).unwrap(),
            workspace_2
        );

        // Test other mechanism for mutating
        let mut workspace_3 = SerializedWorkspace {
            id: 3,
            location: (&["/tmp", "/tmp2"]).into(),
            center_group: Default::default(),
            bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
        };

        db.save_workspace(workspace_3.clone()).await;
        assert_eq!(
            db.workspace_for_roots(&["/tmp", "/tmp2"]).unwrap(),
            workspace_3
        );

        // Make sure that updating paths differently also works
        workspace_3.location = (["/tmp3", "/tmp4", "/tmp2"]).into();
        db.save_workspace(workspace_3.clone()).await;
        assert_eq!(db.workspace_for_roots(&["/tmp2", "tmp"]), None);
        assert_eq!(
            db.workspace_for_roots(&["/tmp2", "/tmp3", "/tmp4"])
                .unwrap(),
            workspace_3
        );
    }

    use crate::persistence::model::SerializedWorkspace;
    use crate::persistence::model::{SerializedItem, SerializedPane, SerializedPaneGroup};

    fn default_workspace<P: AsRef<Path>>(
        workspace_id: &[P],
        center_group: &SerializedPaneGroup,
    ) -> SerializedWorkspace {
        SerializedWorkspace {
            id: 4,
            location: workspace_id.into(),
            center_group: center_group.clone(),
            bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
        }
    }

    #[gpui::test]
    async fn test_simple_split() {
        env_logger::try_init().ok();

        let db = WorkspaceDb(open_test_db("simple_split").await);

        //  -----------------
        //  | 1,2   | 5,6   |
        //  | - - - |       |
        //  | 3,4   |       |
        //  -----------------
        let center_pane = group(
            Axis::Horizontal,
            vec![
                group(
                    Axis::Vertical,
                    vec![
                        SerializedPaneGroup::Pane(SerializedPane::new(
                            vec![
                                SerializedItem::new("Terminal", 1, false),
                                SerializedItem::new("Terminal", 2, true),
                            ],
                            false,
                        )),
                        SerializedPaneGroup::Pane(SerializedPane::new(
                            vec![
                                SerializedItem::new("Terminal", 4, false),
                                SerializedItem::new("Terminal", 3, true),
                            ],
                            true,
                        )),
                    ],
                ),
                SerializedPaneGroup::Pane(SerializedPane::new(
                    vec![
                        SerializedItem::new("Terminal", 5, true),
                        SerializedItem::new("Terminal", 6, false),
                    ],
                    false,
                )),
            ],
        );

        let workspace = default_workspace(&["/tmp"], &center_pane);

        db.save_workspace(workspace.clone()).await;

        let new_workspace = db.workspace_for_roots(&["/tmp"]).unwrap();

        assert_eq!(workspace.center_group, new_workspace.center_group);
    }

    #[gpui::test]
    async fn test_cleanup_panes() {
        env_logger::try_init().ok();

        let db = WorkspaceDb(open_test_db("test_cleanup_panes").await);

        let center_pane = group(
            Axis::Horizontal,
            vec![
                group(
                    Axis::Vertical,
                    vec![
                        SerializedPaneGroup::Pane(SerializedPane::new(
                            vec![
                                SerializedItem::new("Terminal", 1, false),
                                SerializedItem::new("Terminal", 2, true),
                            ],
                            false,
                        )),
                        SerializedPaneGroup::Pane(SerializedPane::new(
                            vec![
                                SerializedItem::new("Terminal", 4, false),
                                SerializedItem::new("Terminal", 3, true),
                            ],
                            true,
                        )),
                    ],
                ),
                SerializedPaneGroup::Pane(SerializedPane::new(
                    vec![
                        SerializedItem::new("Terminal", 5, false),
                        SerializedItem::new("Terminal", 6, true),
                    ],
                    false,
                )),
            ],
        );

        let id = &["/tmp"];

        let mut workspace = default_workspace(id, &center_pane);

        db.save_workspace(workspace.clone()).await;

        workspace.center_group = group(
            Axis::Vertical,
            vec![
                SerializedPaneGroup::Pane(SerializedPane::new(
                    vec![
                        SerializedItem::new("Terminal", 1, false),
                        SerializedItem::new("Terminal", 2, true),
                    ],
                    false,
                )),
                SerializedPaneGroup::Pane(SerializedPane::new(
                    vec![
                        SerializedItem::new("Terminal", 4, true),
                        SerializedItem::new("Terminal", 3, false),
                    ],
                    true,
                )),
            ],
        );

        db.save_workspace(workspace.clone()).await;

        let new_workspace = db.workspace_for_roots(id).unwrap();

        assert_eq!(workspace.center_group, new_workspace.center_group);
    }
}
