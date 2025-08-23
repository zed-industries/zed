pub mod model;

use std::{
    borrow::Cow,
    collections::BTreeMap,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};

use anyhow::{Context as _, Result, bail};
use collections::HashMap;
use db::{define_connection, query, sqlez::connection::Connection, sqlez_macros::sql};
use gpui::{Axis, Bounds, Task, WindowBounds, WindowId, point, size};
use project::debugger::breakpoint_store::{BreakpointState, SourceBreakpoint};

use language::{LanguageName, Toolchain};
use project::WorktreeId;
use sqlez::{
    bindable::{Bind, Column, StaticColumnCount},
    statement::{SqlType, Statement},
    thread_safe_connection::ThreadSafeConnection,
};

use ui::{App, px};
use util::{ResultExt, maybe};
use uuid::Uuid;

use crate::{
    WorkspaceId,
    path_list::{PathList, SerializedPathList},
};

use model::{
    GroupId, ItemId, PaneId, SerializedItem, SerializedPane, SerializedPaneGroup,
    SerializedSshConnection, SerializedWorkspace, SshConnectionId,
};

use self::model::{DockStructure, SerializedWorkspaceLocation};

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

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub(crate) struct SerializedWindowBounds(pub(crate) WindowBounds);

impl StaticColumnCount for SerializedWindowBounds {
    fn column_count() -> usize {
        5
    }
}

impl Bind for SerializedWindowBounds {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        match self.0 {
            WindowBounds::Windowed(bounds) => {
                let next_index = statement.bind(&"Windowed", start_index)?;
                statement.bind(
                    &(
                        SerializedPixels(bounds.origin.x),
                        SerializedPixels(bounds.origin.y),
                        SerializedPixels(bounds.size.width),
                        SerializedPixels(bounds.size.height),
                    ),
                    next_index,
                )
            }
            WindowBounds::Maximized(bounds) => {
                let next_index = statement.bind(&"Maximized", start_index)?;
                statement.bind(
                    &(
                        SerializedPixels(bounds.origin.x),
                        SerializedPixels(bounds.origin.y),
                        SerializedPixels(bounds.size.width),
                        SerializedPixels(bounds.size.height),
                    ),
                    next_index,
                )
            }
            WindowBounds::Fullscreen(bounds) => {
                let next_index = statement.bind(&"FullScreen", start_index)?;
                statement.bind(
                    &(
                        SerializedPixels(bounds.origin.x),
                        SerializedPixels(bounds.origin.y),
                        SerializedPixels(bounds.size.width),
                        SerializedPixels(bounds.size.height),
                    ),
                    next_index,
                )
            }
        }
    }
}

impl Column for SerializedWindowBounds {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (window_state, next_index) = String::column(statement, start_index)?;
        let ((x, y, width, height), _): ((i32, i32, i32, i32), _) =
            Column::column(statement, next_index)?;
        let bounds = Bounds {
            origin: point(px(x as f32), px(y as f32)),
            size: size(px(width as f32), px(height as f32)),
        };

        let status = match window_state.as_str() {
            "Windowed" | "Fixed" => SerializedWindowBounds(WindowBounds::Windowed(bounds)),
            "Maximized" => SerializedWindowBounds(WindowBounds::Maximized(bounds)),
            "FullScreen" => SerializedWindowBounds(WindowBounds::Fullscreen(bounds)),
            _ => bail!("Window State did not have a valid string"),
        };

        Ok((status, next_index + 4))
    }
}

#[derive(Debug)]
pub struct Breakpoint {
    pub position: u32,
    pub message: Option<Arc<str>>,
    pub condition: Option<Arc<str>>,
    pub hit_condition: Option<Arc<str>>,
    pub state: BreakpointState,
}

/// Wrapper for DB type of a breakpoint
struct BreakpointStateWrapper<'a>(Cow<'a, BreakpointState>);

impl From<BreakpointState> for BreakpointStateWrapper<'static> {
    fn from(kind: BreakpointState) -> Self {
        BreakpointStateWrapper(Cow::Owned(kind))
    }
}
impl StaticColumnCount for BreakpointStateWrapper<'_> {
    fn column_count() -> usize {
        1
    }
}

impl Bind for BreakpointStateWrapper<'_> {
    fn bind(&self, statement: &Statement, start_index: i32) -> anyhow::Result<i32> {
        statement.bind(&self.0.to_int(), start_index)
    }
}

impl Column for BreakpointStateWrapper<'_> {
    fn column(statement: &mut Statement, start_index: i32) -> anyhow::Result<(Self, i32)> {
        let state = statement.column_int(start_index)?;

        match state {
            0 => Ok((BreakpointState::Enabled.into(), start_index + 1)),
            1 => Ok((BreakpointState::Disabled.into(), start_index + 1)),
            _ => anyhow::bail!("Invalid BreakpointState discriminant {state}"),
        }
    }
}

/// This struct is used to implement traits on Vec<breakpoint>
#[derive(Debug)]
#[allow(dead_code)]
struct Breakpoints(Vec<Breakpoint>);

impl sqlez::bindable::StaticColumnCount for Breakpoint {
    fn column_count() -> usize {
        // Position, log message, condition message, and hit condition message
        4 + BreakpointStateWrapper::column_count()
    }
}

impl sqlez::bindable::Bind for Breakpoint {
    fn bind(
        &self,
        statement: &sqlez::statement::Statement,
        start_index: i32,
    ) -> anyhow::Result<i32> {
        let next_index = statement.bind(&self.position, start_index)?;
        let next_index = statement.bind(&self.message, next_index)?;
        let next_index = statement.bind(&self.condition, next_index)?;
        let next_index = statement.bind(&self.hit_condition, next_index)?;
        statement.bind(
            &BreakpointStateWrapper(Cow::Borrowed(&self.state)),
            next_index,
        )
    }
}

impl Column for Breakpoint {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let position = statement
            .column_int(start_index)
            .with_context(|| format!("Failed to read BreakPoint at index {start_index}"))?
            as u32;
        let (message, next_index) = Option::<String>::column(statement, start_index + 1)?;
        let (condition, next_index) = Option::<String>::column(statement, next_index)?;
        let (hit_condition, next_index) = Option::<String>::column(statement, next_index)?;
        let (state, next_index) = BreakpointStateWrapper::column(statement, next_index)?;

        Ok((
            Breakpoint {
                position,
                message: message.map(Arc::from),
                condition: condition.map(Arc::from),
                hit_condition: hit_condition.map(Arc::from),
                state: state.0.into_owned(),
            },
            next_index,
        ))
    }
}

impl Column for Breakpoints {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let mut breakpoints = Vec::new();
        let mut index = start_index;

        loop {
            match statement.column_type(index) {
                Ok(SqlType::Null) => break,
                _ => {
                    let (breakpoint, next_index) = Breakpoint::column(statement, index)?;

                    breakpoints.push(breakpoint);
                    index = next_index;
                }
            }
        }
        Ok((Breakpoints(breakpoints), index))
    }
}

#[derive(Clone, Debug, PartialEq)]
struct SerializedPixels(gpui::Pixels);
impl sqlez::bindable::StaticColumnCount for SerializedPixels {}

impl sqlez::bindable::Bind for SerializedPixels {
    fn bind(
        &self,
        statement: &sqlez::statement::Statement,
        start_index: i32,
    ) -> anyhow::Result<i32> {
        let this: i32 = self.0.0 as i32;
        this.bind(statement, start_index)
    }
}

define_connection! {
    pub static ref DB: WorkspaceDb<()> =
    &[
    sql!(
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
    ),
    // Add fullscreen field to workspace
    // Deprecated, `WindowBounds` holds the fullscreen state now.
    // Preserving so users can downgrade Zed.
    sql!(
        ALTER TABLE workspaces ADD COLUMN fullscreen INTEGER; //bool
    ),
    // Add preview field to items
    sql!(
        ALTER TABLE items ADD COLUMN preview INTEGER; //bool
    ),
    // Add centered_layout field to workspace
    sql!(
        ALTER TABLE workspaces ADD COLUMN centered_layout INTEGER; //bool
    ),
    sql!(
        CREATE TABLE remote_projects (
            remote_project_id INTEGER NOT NULL UNIQUE,
            path TEXT,
            dev_server_name TEXT
        );
        ALTER TABLE workspaces ADD COLUMN remote_project_id INTEGER;
        ALTER TABLE workspaces RENAME COLUMN workspace_location TO local_paths;
    ),
    sql!(
        DROP TABLE remote_projects;
        CREATE TABLE dev_server_projects (
            id INTEGER NOT NULL UNIQUE,
            path TEXT,
            dev_server_name TEXT
        );
        ALTER TABLE workspaces DROP COLUMN remote_project_id;
        ALTER TABLE workspaces ADD COLUMN dev_server_project_id INTEGER;
    ),
    sql!(
        ALTER TABLE workspaces ADD COLUMN local_paths_order BLOB;
    ),
    sql!(
        ALTER TABLE workspaces ADD COLUMN session_id TEXT DEFAULT NULL;
    ),
    sql!(
        ALTER TABLE workspaces ADD COLUMN window_id INTEGER DEFAULT NULL;
    ),
    sql!(
        ALTER TABLE panes ADD COLUMN pinned_count INTEGER DEFAULT 0;
    ),
    sql!(
        CREATE TABLE ssh_projects (
            id INTEGER PRIMARY KEY,
            host TEXT NOT NULL,
            port INTEGER,
            path TEXT NOT NULL,
            user TEXT
        );
        ALTER TABLE workspaces ADD COLUMN ssh_project_id INTEGER REFERENCES ssh_projects(id) ON DELETE CASCADE;
    ),
    sql!(
        ALTER TABLE ssh_projects RENAME COLUMN path TO paths;
    ),
    sql!(
        CREATE TABLE toolchains (
            workspace_id INTEGER,
            worktree_id INTEGER,
            language_name TEXT NOT NULL,
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            PRIMARY KEY (workspace_id, worktree_id, language_name)
        );
    ),
    sql!(
        ALTER TABLE toolchains ADD COLUMN raw_json TEXT DEFAULT "{}";
    ),
    sql!(
            CREATE TABLE breakpoints (
                workspace_id INTEGER NOT NULL,
                path TEXT NOT NULL,
                breakpoint_location INTEGER NOT NULL,
                kind INTEGER NOT NULL,
                log_message TEXT,
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
                ON UPDATE CASCADE
            );
        ),
    sql!(
        ALTER TABLE workspaces ADD COLUMN local_paths_array TEXT;
        CREATE UNIQUE INDEX local_paths_array_uq ON workspaces(local_paths_array);
        ALTER TABLE workspaces ADD COLUMN local_paths_order_array TEXT;
    ),
    sql!(
        ALTER TABLE breakpoints ADD COLUMN state INTEGER DEFAULT(0) NOT NULL
    ),
    sql!(
        ALTER TABLE breakpoints DROP COLUMN kind
    ),
    sql!(ALTER TABLE toolchains ADD COLUMN relative_worktree_path TEXT DEFAULT "" NOT NULL),
    sql!(
        ALTER TABLE breakpoints ADD COLUMN condition TEXT;
        ALTER TABLE breakpoints ADD COLUMN hit_condition TEXT;
    ),
    sql!(CREATE TABLE toolchains2 (
        workspace_id INTEGER,
        worktree_id INTEGER,
        language_name TEXT NOT NULL,
        name TEXT NOT NULL,
        path TEXT NOT NULL,
        raw_json TEXT NOT NULL,
        relative_worktree_path TEXT NOT NULL,
        PRIMARY KEY (workspace_id, worktree_id, language_name, relative_worktree_path)) STRICT;
        INSERT INTO toolchains2
            SELECT * FROM toolchains;
        DROP TABLE toolchains;
        ALTER TABLE toolchains2 RENAME TO toolchains;
    ),
    sql!(
        CREATE TABLE ssh_connections (
            id INTEGER PRIMARY KEY,
            host TEXT NOT NULL,
            port INTEGER,
            user TEXT
        );

        INSERT INTO ssh_connections (host, port, user)
        SELECT DISTINCT host, port, user
        FROM ssh_projects;

        CREATE TABLE workspaces_2(
            workspace_id INTEGER PRIMARY KEY,
            paths TEXT,
            paths_order TEXT,
            ssh_connection_id INTEGER REFERENCES ssh_connections(id),
            timestamp TEXT DEFAULT CURRENT_TIMESTAMP NOT NULL,
            window_state TEXT,
            window_x REAL,
            window_y REAL,
            window_width REAL,
            window_height REAL,
            display BLOB,
            left_dock_visible INTEGER,
            left_dock_active_panel TEXT,
            right_dock_visible INTEGER,
            right_dock_active_panel TEXT,
            bottom_dock_visible INTEGER,
            bottom_dock_active_panel TEXT,
            left_dock_zoom INTEGER,
            right_dock_zoom INTEGER,
            bottom_dock_zoom INTEGER,
            fullscreen INTEGER,
            centered_layout INTEGER,
            session_id TEXT,
            window_id INTEGER
        ) STRICT;

        INSERT
        INTO workspaces_2
        SELECT
            workspaces.workspace_id,
            CASE
                WHEN ssh_projects.id IS NOT NULL THEN ssh_projects.paths
                ELSE
                    CASE
                        WHEN workspaces.local_paths_array IS NULL OR workspaces.local_paths_array = "" THEN
                            NULL
                        ELSE
                            json('[' || '"' || replace(workspaces.local_paths_array, ',', '"' || "," || '"') || '"' || ']')
                    END
            END as paths,

            CASE
                WHEN ssh_projects.id IS NOT NULL THEN ""
                ELSE workspaces.local_paths_order_array
            END as paths_order,

            CASE
                WHEN ssh_projects.id IS NOT NULL THEN (
                    SELECT ssh_connections.id
                    FROM ssh_connections
                    WHERE
                        ssh_connections.host IS ssh_projects.host AND
                        ssh_connections.port IS ssh_projects.port AND
                        ssh_connections.user IS ssh_projects.user
                )
                ELSE NULL
            END as ssh_connection_id,

            workspaces.timestamp,
            workspaces.window_state,
            workspaces.window_x,
            workspaces.window_y,
            workspaces.window_width,
            workspaces.window_height,
            workspaces.display,
            workspaces.left_dock_visible,
            workspaces.left_dock_active_panel,
            workspaces.right_dock_visible,
            workspaces.right_dock_active_panel,
            workspaces.bottom_dock_visible,
            workspaces.bottom_dock_active_panel,
            workspaces.left_dock_zoom,
            workspaces.right_dock_zoom,
            workspaces.bottom_dock_zoom,
            workspaces.fullscreen,
            workspaces.centered_layout,
            workspaces.session_id,
            workspaces.window_id
        FROM
            workspaces LEFT JOIN
            ssh_projects ON
            workspaces.ssh_project_id = ssh_projects.id;

        DROP TABLE ssh_projects;
        DROP TABLE workspaces;
        ALTER TABLE workspaces_2 RENAME TO workspaces;

        CREATE UNIQUE INDEX ix_workspaces_location ON workspaces(ssh_connection_id, paths);
    ),
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
        self.workspace_for_roots_internal(worktree_roots, None)
    }

    pub(crate) fn ssh_workspace_for_roots<P: AsRef<Path>>(
        &self,
        worktree_roots: &[P],
        ssh_project_id: SshConnectionId,
    ) -> Option<SerializedWorkspace> {
        self.workspace_for_roots_internal(worktree_roots, Some(ssh_project_id))
    }

    pub(crate) fn workspace_for_roots_internal<P: AsRef<Path>>(
        &self,
        worktree_roots: &[P],
        ssh_connection_id: Option<SshConnectionId>,
    ) -> Option<SerializedWorkspace> {
        // paths are sorted before db interactions to ensure that the order of the paths
        // doesn't affect the workspace selection for existing workspaces
        let root_paths = PathList::new(worktree_roots);

        // Note that we re-assign the workspace_id here in case it's empty
        // and we've grabbed the most recent workspace
        let (
            workspace_id,
            paths,
            paths_order,
            window_bounds,
            display,
            centered_layout,
            docks,
            window_id,
        ): (
            WorkspaceId,
            String,
            String,
            Option<SerializedWindowBounds>,
            Option<Uuid>,
            Option<bool>,
            DockStructure,
            Option<u64>,
        ) = self
            .select_row_bound(sql! {
                SELECT
                    workspace_id,
                    paths,
                    paths_order,
                    window_state,
                    window_x,
                    window_y,
                    window_width,
                    window_height,
                    display,
                    centered_layout,
                    left_dock_visible,
                    left_dock_active_panel,
                    left_dock_zoom,
                    right_dock_visible,
                    right_dock_active_panel,
                    right_dock_zoom,
                    bottom_dock_visible,
                    bottom_dock_active_panel,
                    bottom_dock_zoom,
                    window_id
                FROM workspaces
                WHERE
                    paths IS ? AND
                    ssh_connection_id IS ?
                LIMIT 1
            })
            .map(|mut prepared_statement| {
                (prepared_statement)((
                    root_paths.serialize().paths,
                    ssh_connection_id.map(|id| id.0 as i32),
                ))
                .unwrap()
            })
            .context("No workspaces found")
            .warn_on_err()
            .flatten()?;

        let paths = PathList::deserialize(&SerializedPathList {
            paths,
            order: paths_order,
        });

        Some(SerializedWorkspace {
            id: workspace_id,
            location: SerializedWorkspaceLocation::Local,
            paths,
            center_group: self
                .get_center_pane_group(workspace_id)
                .context("Getting center group")
                .log_err()?,
            window_bounds,
            centered_layout: centered_layout.unwrap_or(false),
            display,
            docks,
            session_id: None,
            breakpoints: self.breakpoints(workspace_id),
            window_id,
        })
    }

    fn breakpoints(&self, workspace_id: WorkspaceId) -> BTreeMap<Arc<Path>, Vec<SourceBreakpoint>> {
        let breakpoints: Result<Vec<(PathBuf, Breakpoint)>> = self
            .select_bound(sql! {
                SELECT path, breakpoint_location, log_message, condition, hit_condition, state
                FROM breakpoints
                WHERE workspace_id = ?
            })
            .and_then(|mut prepared_statement| (prepared_statement)(workspace_id));

        match breakpoints {
            Ok(bp) => {
                if bp.is_empty() {
                    log::debug!("Breakpoints are empty after querying database for them");
                }

                let mut map: BTreeMap<Arc<Path>, Vec<SourceBreakpoint>> = Default::default();

                for (path, breakpoint) in bp {
                    let path: Arc<Path> = path.into();
                    map.entry(path.clone()).or_default().push(SourceBreakpoint {
                        row: breakpoint.position,
                        path,
                        message: breakpoint.message,
                        condition: breakpoint.condition,
                        hit_condition: breakpoint.hit_condition,
                        state: breakpoint.state,
                    });
                }

                for (path, bps) in map.iter() {
                    log::info!(
                        "Got {} breakpoints from database at path: {}",
                        bps.len(),
                        path.to_string_lossy()
                    );
                }

                map
            }
            Err(msg) => {
                log::error!("Breakpoints query failed with msg: {msg}");
                Default::default()
            }
        }
    }

    /// Saves a workspace using the worktree roots. Will garbage collect any workspaces
    /// that used this workspace previously
    pub(crate) async fn save_workspace(&self, workspace: SerializedWorkspace) {
        let paths = workspace.paths.serialize();
        log::debug!("Saving workspace at location: {:?}", workspace.location);
        self.write(move |conn| {
            conn.with_savepoint("update_worktrees", || {
                let ssh_connection_id = match &workspace.location {
                    SerializedWorkspaceLocation::Local => None,
                    SerializedWorkspaceLocation::Ssh(connection) => {
                        Some(Self::get_or_create_ssh_connection_query(
                            conn,
                            connection.host.clone(),
                            connection.port,
                            connection.user.clone(),
                        )?.0)
                    }
                };

                // Clear out panes and pane_groups
                conn.exec_bound(sql!(
                    DELETE FROM pane_groups WHERE workspace_id = ?1;
                    DELETE FROM panes WHERE workspace_id = ?1;))?(workspace.id)
                    .context("Clearing old panes")?;

                conn.exec_bound(
                    sql!(
                        DELETE FROM breakpoints WHERE workspace_id = ?1;
                        DELETE FROM toolchains WHERE workspace_id = ?1;
                    )
                )?(workspace.id).context("Clearing old breakpoints")?;

                for (path, breakpoints) in workspace.breakpoints {
                    for bp in breakpoints {
                        let state = BreakpointStateWrapper::from(bp.state);
                        match conn.exec_bound(sql!(
                            INSERT INTO breakpoints (workspace_id, path, breakpoint_location,  log_message, condition, hit_condition, state)
                            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);))?

                        ((
                            workspace.id,
                            path.as_ref(),
                            bp.row,
                            bp.message,
                            bp.condition,
                            bp.hit_condition,
                            state,
                        )) {
                            Ok(_) => {
                                log::debug!("Stored breakpoint at row: {} in path: {}", bp.row, path.to_string_lossy())
                            }
                            Err(err) => {
                                log::error!("{err}");
                                continue;
                            }
                        }
                    }
                }

                conn.exec_bound(sql!(
                    DELETE
                    FROM workspaces
                    WHERE
                        workspace_id != ?1 AND
                        paths IS ?2 AND
                        ssh_connection_id IS ?3
                ))?((
                    workspace.id,
                    paths.paths.clone(),
                    ssh_connection_id,
                ))
                .context("clearing out old locations")?;

                // Upsert
                let query = sql!(
                    INSERT INTO workspaces(
                        workspace_id,
                        paths,
                        paths_order,
                        ssh_connection_id,
                        left_dock_visible,
                        left_dock_active_panel,
                        left_dock_zoom,
                        right_dock_visible,
                        right_dock_active_panel,
                        right_dock_zoom,
                        bottom_dock_visible,
                        bottom_dock_active_panel,
                        bottom_dock_zoom,
                        session_id,
                        window_id,
                        timestamp
                    )
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, CURRENT_TIMESTAMP)
                    ON CONFLICT DO
                    UPDATE SET
                        paths = ?2,
                        paths_order = ?3,
                        ssh_connection_id = ?4,
                        left_dock_visible = ?5,
                        left_dock_active_panel = ?6,
                        left_dock_zoom = ?7,
                        right_dock_visible = ?8,
                        right_dock_active_panel = ?9,
                        right_dock_zoom = ?10,
                        bottom_dock_visible = ?11,
                        bottom_dock_active_panel = ?12,
                        bottom_dock_zoom = ?13,
                        session_id = ?14,
                        window_id = ?15,
                        timestamp = CURRENT_TIMESTAMP
                );
                let mut prepared_query = conn.exec_bound(query)?;
                let args = (
                    workspace.id,
                    paths.paths.clone(),
                    paths.order.clone(),
                    ssh_connection_id,
                    workspace.docks,
                    workspace.session_id,
                    workspace.window_id,
                );

                prepared_query(args).context("Updating workspace")?;

                // Save center pane group
                Self::save_pane_group(conn, workspace.id, &workspace.center_group, None)
                    .context("save pane group in save workspace")?;

                Ok(())
            })
            .log_err();
        })
        .await;
    }

    pub(crate) async fn get_or_create_ssh_connection(
        &self,
        host: String,
        port: Option<u16>,
        user: Option<String>,
    ) -> Result<SshConnectionId> {
        self.write(move |conn| Self::get_or_create_ssh_connection_query(conn, host, port, user))
            .await
    }

    fn get_or_create_ssh_connection_query(
        this: &Connection,
        host: String,
        port: Option<u16>,
        user: Option<String>,
    ) -> Result<SshConnectionId> {
        if let Some(id) = this.select_row_bound(sql!(
            SELECT id FROM ssh_connections WHERE host IS ? AND port IS ? AND user IS ? LIMIT 1
        ))?((host.clone(), port, user.clone()))?
        {
            Ok(SshConnectionId(id))
        } else {
            log::debug!("Inserting SSH project at host {host}");
            let id = this.select_row_bound(sql!(
                INSERT INTO ssh_connections (
                    host,
                    port,
                    user
                ) VALUES (?1, ?2, ?3)
                RETURNING id
            ))?((host, port, user))?
            .context("failed to insert ssh project")?;
            Ok(SshConnectionId(id))
        }
    }

    query! {
        pub async fn next_id() -> Result<WorkspaceId> {
            INSERT INTO workspaces DEFAULT VALUES RETURNING workspace_id
        }
    }

    fn recent_workspaces(&self) -> Result<Vec<(WorkspaceId, PathList, Option<u64>)>> {
        Ok(self
            .recent_workspaces_query()?
            .into_iter()
            .map(|(id, paths, order, ssh_connection_id)| {
                (
                    id,
                    PathList::deserialize(&SerializedPathList { paths, order }),
                    ssh_connection_id,
                )
            })
            .collect())
    }

    query! {
        fn recent_workspaces_query() -> Result<Vec<(WorkspaceId, String, String, Option<u64>)>> {
            SELECT workspace_id, paths, paths_order, ssh_connection_id
            FROM workspaces
            WHERE
                paths IS NOT NULL OR
                ssh_connection_id IS NOT NULL
            ORDER BY timestamp DESC
        }
    }

    fn session_workspaces(
        &self,
        session_id: String,
    ) -> Result<Vec<(PathList, Option<u64>, Option<SshConnectionId>)>> {
        Ok(self
            .session_workspaces_query(session_id)?
            .into_iter()
            .map(|(paths, order, window_id, ssh_connection_id)| {
                (
                    PathList::deserialize(&SerializedPathList { paths, order }),
                    window_id,
                    ssh_connection_id.map(SshConnectionId),
                )
            })
            .collect())
    }

    query! {
        fn session_workspaces_query(session_id: String) -> Result<Vec<(String, String, Option<u64>, Option<u64>)>> {
            SELECT paths, paths_order, window_id, ssh_connection_id
            FROM workspaces
            WHERE session_id = ?1
            ORDER BY timestamp DESC
        }
    }

    query! {
        pub fn breakpoints_for_file(workspace_id: WorkspaceId, file_path: &Path) -> Result<Vec<Breakpoint>> {
            SELECT breakpoint_location
            FROM breakpoints
            WHERE  workspace_id= ?1 AND path = ?2
        }
    }

    query! {
        pub fn clear_breakpoints(file_path: &Path) -> Result<()> {
            DELETE FROM breakpoints
            WHERE file_path = ?2
        }
    }

    fn ssh_connections(&self) -> Result<HashMap<SshConnectionId, SerializedSshConnection>> {
        Ok(self
            .ssh_connections_query()?
            .into_iter()
            .map(|(id, host, port, user)| {
                (
                    SshConnectionId(id),
                    SerializedSshConnection { host, port, user },
                )
            })
            .collect())
    }

    query! {
        pub fn ssh_connections_query() -> Result<Vec<(u64, String, Option<u16>, Option<String>)>> {
            SELECT id, host, port, user
            FROM ssh_connections
        }
    }

    pub(crate) fn ssh_connection(&self, id: SshConnectionId) -> Result<SerializedSshConnection> {
        let row = self.ssh_connection_query(id.0)?;
        Ok(SerializedSshConnection {
            host: row.0,
            port: row.1,
            user: row.2,
        })
    }

    query! {
        fn ssh_connection_query(id: u64) -> Result<(String, Option<u16>, Option<String>)> {
            SELECT host, port, user
            FROM ssh_connections
            WHERE id = ?
        }
    }

    pub(crate) fn last_window(
        &self,
    ) -> anyhow::Result<(Option<Uuid>, Option<SerializedWindowBounds>)> {
        let mut prepared_query =
            self.select::<(Option<Uuid>, Option<SerializedWindowBounds>)>(sql!(
                SELECT
                display,
                window_state, window_x, window_y, window_width, window_height
                FROM workspaces
                WHERE paths
                IS NOT NULL
                ORDER BY timestamp DESC
                LIMIT 1
            ))?;
        let result = prepared_query()?;
        Ok(result.into_iter().next().unwrap_or((None, None)))
    }

    query! {
        pub async fn delete_workspace_by_id(id: WorkspaceId) -> Result<()> {
            DELETE FROM toolchains WHERE workspace_id = ?1;
            DELETE FROM workspaces
            WHERE workspace_id IS ?
        }
    }

    // Returns the recent locations which are still valid on disk and deletes ones which no longer
    // exist.
    pub async fn recent_workspaces_on_disk(
        &self,
    ) -> Result<Vec<(WorkspaceId, SerializedWorkspaceLocation, PathList)>> {
        let mut result = Vec::new();
        let mut delete_tasks = Vec::new();
        let ssh_connections = self.ssh_connections()?;

        for (id, paths, ssh_connection_id) in self.recent_workspaces()? {
            if let Some(ssh_connection_id) = ssh_connection_id.map(SshConnectionId) {
                if let Some(ssh_connection) = ssh_connections.get(&ssh_connection_id) {
                    result.push((
                        id,
                        SerializedWorkspaceLocation::Ssh(ssh_connection.clone()),
                        paths,
                    ));
                } else {
                    delete_tasks.push(self.delete_workspace_by_id(id));
                }
                continue;
            }

            if paths.paths().iter().all(|path| path.exists())
                && paths.paths().iter().any(|path| path.is_dir())
            {
                result.push((id, SerializedWorkspaceLocation::Local, paths));
            } else {
                delete_tasks.push(self.delete_workspace_by_id(id));
            }
        }

        futures::future::join_all(delete_tasks).await;
        Ok(result)
    }

    pub async fn last_workspace(&self) -> Result<Option<(SerializedWorkspaceLocation, PathList)>> {
        Ok(self
            .recent_workspaces_on_disk()
            .await?
            .into_iter()
            .next()
            .map(|(_, location, paths)| (location, paths)))
    }

    // Returns the locations of the workspaces that were still opened when the last
    // session was closed (i.e. when Zed was quit).
    // If `last_session_window_order` is provided, the returned locations are ordered
    // according to that.
    pub fn last_session_workspace_locations(
        &self,
        last_session_id: &str,
        last_session_window_stack: Option<Vec<WindowId>>,
    ) -> Result<Vec<(SerializedWorkspaceLocation, PathList)>> {
        let mut workspaces = Vec::new();

        for (paths, window_id, ssh_connection_id) in
            self.session_workspaces(last_session_id.to_owned())?
        {
            if let Some(ssh_connection_id) = ssh_connection_id {
                workspaces.push((
                    SerializedWorkspaceLocation::Ssh(self.ssh_connection(ssh_connection_id)?),
                    paths,
                    window_id.map(WindowId::from),
                ));
            } else if paths.paths().iter().all(|path| path.exists())
                && paths.paths().iter().any(|path| path.is_dir())
            {
                workspaces.push((
                    SerializedWorkspaceLocation::Local,
                    paths,
                    window_id.map(WindowId::from),
                ));
            }
        }

        if let Some(stack) = last_session_window_stack {
            workspaces.sort_by_key(|(_, _, window_id)| {
                window_id
                    .and_then(|id| stack.iter().position(|&order_id| order_id == id))
                    .unwrap_or(usize::MAX)
            });
        }

        Ok(workspaces
            .into_iter()
            .map(|(location, paths, _)| (location, paths))
            .collect::<Vec<_>>())
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
                    pinned_count: 0,
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
            Option<usize>,
            Option<String>,
        );
        self.select_bound::<GroupKey, GroupOrPane>(sql!(
            SELECT group_id, axis, pane_id, active, pinned_count, flexes
                FROM (SELECT
                        group_id,
                        axis,
                        NULL as pane_id,
                        NULL as active,
                        NULL as pinned_count,
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
                        pinned_count,
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
        .map(|(group_id, axis, pane_id, active, pinned_count, flexes)| {
            let maybe_pane = maybe!({ Some((pane_id?, active?, pinned_count?)) });
            if let Some((group_id, axis)) = group_id.zip(axis) {
                let flexes = flexes
                    .map(|flexes: String| serde_json::from_str::<Vec<f32>>(&flexes))
                    .transpose()?;

                Ok(SerializedPaneGroup::Group {
                    axis,
                    children: self.get_pane_group(workspace_id, Some(group_id))?,
                    flexes,
                })
            } else if let Some((pane_id, active, pinned_count)) = maybe_pane {
                Ok(SerializedPaneGroup::Pane(SerializedPane::new(
                    self.get_items(pane_id)?,
                    active,
                    pinned_count,
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
        if parent.is_none() {
            log::debug!("Saving a pane group for workspace {workspace_id:?}");
        }
        match pane_group {
            SerializedPaneGroup::Group {
                axis,
                children,
                flexes,
            } => {
                let (parent_id, position) = parent.unzip();

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
                .context("Couldn't retrieve group_id from inserted pane_group")?;

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
            INSERT INTO panes(workspace_id, active, pinned_count)
            VALUES (?, ?, ?)
            RETURNING pane_id
        ))?((workspace_id, pane.active, pane.pinned_count))?
        .context("Could not retrieve inserted pane_id")?;

        let (parent_id, order) = parent.unzip();
        conn.exec_bound(sql!(
            INSERT INTO center_panes(pane_id, parent_group_id, position)
            VALUES (?, ?, ?)
        ))?((pane_id, parent_id, order))?;

        Self::save_items(conn, workspace_id, pane_id, &pane.children).context("Saving items")?;

        Ok(pane_id)
    }

    fn get_items(&self, pane_id: PaneId) -> Result<Vec<SerializedItem>> {
        self.select_bound(sql!(
            SELECT kind, item_id, active, preview FROM items
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
            INSERT INTO items(workspace_id, pane_id, position, kind, item_id, active, preview) VALUES (?, ?, ?, ?, ?, ?, ?)
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
        pub(crate) async fn set_window_open_status(workspace_id: WorkspaceId, bounds: SerializedWindowBounds, display: Uuid) -> Result<()> {
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

    query! {
        pub(crate) async fn set_centered_layout(workspace_id: WorkspaceId, centered_layout: bool) -> Result<()> {
            UPDATE workspaces
            SET centered_layout = ?2
            WHERE workspace_id = ?1
        }
    }

    query! {
        pub(crate) async fn set_session_id(workspace_id: WorkspaceId, session_id: Option<String>) -> Result<()> {
            UPDATE workspaces
            SET session_id = ?2
            WHERE workspace_id = ?1
        }
    }

    pub async fn toolchain(
        &self,
        workspace_id: WorkspaceId,
        worktree_id: WorktreeId,
        relative_path: String,
        language_name: LanguageName,
    ) -> Result<Option<Toolchain>> {
        self.write(move |this| {
            let mut select = this
                .select_bound(sql!(
                    SELECT name, path, raw_json FROM toolchains WHERE workspace_id = ? AND language_name = ? AND worktree_id = ? AND relative_path = ?
                ))
                .context("Preparing insertion")?;

            let toolchain: Vec<(String, String, String)> =
                select((workspace_id, language_name.as_ref().to_string(), worktree_id.to_usize(), relative_path))?;

            Ok(toolchain.into_iter().next().and_then(|(name, path, raw_json)| Some(Toolchain {
                name: name.into(),
                path: path.into(),
                language_name,
                as_json: serde_json::Value::from_str(&raw_json).ok()?
            })))
        })
        .await
    }

    pub(crate) async fn toolchains(
        &self,
        workspace_id: WorkspaceId,
    ) -> Result<Vec<(Toolchain, WorktreeId, Arc<Path>)>> {
        self.write(move |this| {
            let mut select = this
                .select_bound(sql!(
                    SELECT name, path, worktree_id, relative_worktree_path, language_name, raw_json FROM toolchains WHERE workspace_id = ?
                ))
                .context("Preparing insertion")?;

            let toolchain: Vec<(String, String, u64, String, String, String)> =
                select(workspace_id)?;

            Ok(toolchain.into_iter().filter_map(|(name, path, worktree_id, relative_worktree_path, language_name, raw_json)| Some((Toolchain {
                name: name.into(),
                path: path.into(),
                language_name: LanguageName::new(&language_name),
                as_json: serde_json::Value::from_str(&raw_json).ok()?
            }, WorktreeId::from_proto(worktree_id), Arc::from(relative_worktree_path.as_ref())))).collect())
        })
        .await
    }
    pub async fn set_toolchain(
        &self,
        workspace_id: WorkspaceId,
        worktree_id: WorktreeId,
        relative_worktree_path: String,
        toolchain: Toolchain,
    ) -> Result<()> {
        log::debug!(
            "Setting toolchain for workspace, worktree: {worktree_id:?}, relative path: {relative_worktree_path:?}, toolchain: {}",
            toolchain.name
        );
        self.write(move |conn| {
            let mut insert = conn
                .exec_bound(sql!(
                    INSERT INTO toolchains(workspace_id, worktree_id, relative_worktree_path, language_name, name, path, raw_json) VALUES (?, ?, ?, ?, ?,  ?, ?)
                    ON CONFLICT DO
                    UPDATE SET
                        name = ?5,
                        path = ?6,
                        raw_json = ?7
                ))
                .context("Preparing insertion")?;

            insert((
                workspace_id,
                worktree_id.to_usize(),
                relative_worktree_path,
                toolchain.language_name.as_ref(),
                toolchain.name.as_ref(),
                toolchain.path.as_ref(),
                toolchain.as_json.to_string(),
            ))?;

            Ok(())
        }).await
    }
}

pub fn delete_unloaded_items(
    alive_items: Vec<ItemId>,
    workspace_id: WorkspaceId,
    table: &'static str,
    db: &ThreadSafeConnection,
    cx: &mut App,
) -> Task<Result<()>> {
    let db = db.clone();
    cx.spawn(async move |_| {
        let placeholders = alive_items
            .iter()
            .map(|_| "?")
            .collect::<Vec<&str>>()
            .join(", ");

        let query = format!(
            "DELETE FROM {table} WHERE workspace_id = ? AND item_id NOT IN ({placeholders})"
        );

        db.write(move |conn| {
            let mut statement = Statement::prepare(conn, query)?;
            let mut next_index = statement.bind(&workspace_id, 1)?;
            for id in alive_items {
                next_index = statement.bind(&id, next_index)?;
            }
            statement.exec()
        })
        .await
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::model::{
        SerializedItem, SerializedPane, SerializedPaneGroup, SerializedWorkspace,
    };
    use gpui;
    use pretty_assertions::assert_eq;
    use std::{thread, time::Duration};

    #[gpui::test]
    async fn test_breakpoints() {
        zlog::init_test();

        let db = WorkspaceDb::open_test_db("test_breakpoints").await;
        let id = db.next_id().await.unwrap();

        let path = Path::new("/tmp/test.rs");

        let breakpoint = Breakpoint {
            position: 123,
            message: None,
            state: BreakpointState::Enabled,
            condition: None,
            hit_condition: None,
        };

        let log_breakpoint = Breakpoint {
            position: 456,
            message: Some("Test log message".into()),
            state: BreakpointState::Enabled,
            condition: None,
            hit_condition: None,
        };

        let disable_breakpoint = Breakpoint {
            position: 578,
            message: None,
            state: BreakpointState::Disabled,
            condition: None,
            hit_condition: None,
        };

        let condition_breakpoint = Breakpoint {
            position: 789,
            message: None,
            state: BreakpointState::Enabled,
            condition: Some("x > 5".into()),
            hit_condition: None,
        };

        let hit_condition_breakpoint = Breakpoint {
            position: 999,
            message: None,
            state: BreakpointState::Enabled,
            condition: None,
            hit_condition: Some(">= 3".into()),
        };

        let workspace = SerializedWorkspace {
            id,
            paths: PathList::new(&["/tmp"]),
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            breakpoints: {
                let mut map = collections::BTreeMap::default();
                map.insert(
                    Arc::from(path),
                    vec![
                        SourceBreakpoint {
                            row: breakpoint.position,
                            path: Arc::from(path),
                            message: breakpoint.message.clone(),
                            state: breakpoint.state,
                            condition: breakpoint.condition.clone(),
                            hit_condition: breakpoint.hit_condition.clone(),
                        },
                        SourceBreakpoint {
                            row: log_breakpoint.position,
                            path: Arc::from(path),
                            message: log_breakpoint.message.clone(),
                            state: log_breakpoint.state,
                            condition: log_breakpoint.condition.clone(),
                            hit_condition: log_breakpoint.hit_condition.clone(),
                        },
                        SourceBreakpoint {
                            row: disable_breakpoint.position,
                            path: Arc::from(path),
                            message: disable_breakpoint.message.clone(),
                            state: disable_breakpoint.state,
                            condition: disable_breakpoint.condition.clone(),
                            hit_condition: disable_breakpoint.hit_condition.clone(),
                        },
                        SourceBreakpoint {
                            row: condition_breakpoint.position,
                            path: Arc::from(path),
                            message: condition_breakpoint.message.clone(),
                            state: condition_breakpoint.state,
                            condition: condition_breakpoint.condition.clone(),
                            hit_condition: condition_breakpoint.hit_condition.clone(),
                        },
                        SourceBreakpoint {
                            row: hit_condition_breakpoint.position,
                            path: Arc::from(path),
                            message: hit_condition_breakpoint.message.clone(),
                            state: hit_condition_breakpoint.state,
                            condition: hit_condition_breakpoint.condition.clone(),
                            hit_condition: hit_condition_breakpoint.hit_condition.clone(),
                        },
                    ],
                );
                map
            },
            session_id: None,
            window_id: None,
        };

        db.save_workspace(workspace.clone()).await;

        let loaded = db.workspace_for_roots(&["/tmp"]).unwrap();
        let loaded_breakpoints = loaded.breakpoints.get(&Arc::from(path)).unwrap();

        assert_eq!(loaded_breakpoints.len(), 5);

        // normal breakpoint
        assert_eq!(loaded_breakpoints[0].row, breakpoint.position);
        assert_eq!(loaded_breakpoints[0].message, breakpoint.message);
        assert_eq!(loaded_breakpoints[0].condition, breakpoint.condition);
        assert_eq!(
            loaded_breakpoints[0].hit_condition,
            breakpoint.hit_condition
        );
        assert_eq!(loaded_breakpoints[0].state, breakpoint.state);
        assert_eq!(loaded_breakpoints[0].path, Arc::from(path));

        // enabled breakpoint
        assert_eq!(loaded_breakpoints[1].row, log_breakpoint.position);
        assert_eq!(loaded_breakpoints[1].message, log_breakpoint.message);
        assert_eq!(loaded_breakpoints[1].condition, log_breakpoint.condition);
        assert_eq!(
            loaded_breakpoints[1].hit_condition,
            log_breakpoint.hit_condition
        );
        assert_eq!(loaded_breakpoints[1].state, log_breakpoint.state);
        assert_eq!(loaded_breakpoints[1].path, Arc::from(path));

        // disable breakpoint
        assert_eq!(loaded_breakpoints[2].row, disable_breakpoint.position);
        assert_eq!(loaded_breakpoints[2].message, disable_breakpoint.message);
        assert_eq!(
            loaded_breakpoints[2].condition,
            disable_breakpoint.condition
        );
        assert_eq!(
            loaded_breakpoints[2].hit_condition,
            disable_breakpoint.hit_condition
        );
        assert_eq!(loaded_breakpoints[2].state, disable_breakpoint.state);
        assert_eq!(loaded_breakpoints[2].path, Arc::from(path));

        // condition breakpoint
        assert_eq!(loaded_breakpoints[3].row, condition_breakpoint.position);
        assert_eq!(loaded_breakpoints[3].message, condition_breakpoint.message);
        assert_eq!(
            loaded_breakpoints[3].condition,
            condition_breakpoint.condition
        );
        assert_eq!(
            loaded_breakpoints[3].hit_condition,
            condition_breakpoint.hit_condition
        );
        assert_eq!(loaded_breakpoints[3].state, condition_breakpoint.state);
        assert_eq!(loaded_breakpoints[3].path, Arc::from(path));

        // hit condition breakpoint
        assert_eq!(loaded_breakpoints[4].row, hit_condition_breakpoint.position);
        assert_eq!(
            loaded_breakpoints[4].message,
            hit_condition_breakpoint.message
        );
        assert_eq!(
            loaded_breakpoints[4].condition,
            hit_condition_breakpoint.condition
        );
        assert_eq!(
            loaded_breakpoints[4].hit_condition,
            hit_condition_breakpoint.hit_condition
        );
        assert_eq!(loaded_breakpoints[4].state, hit_condition_breakpoint.state);
        assert_eq!(loaded_breakpoints[4].path, Arc::from(path));
    }

    #[gpui::test]
    async fn test_remove_last_breakpoint() {
        zlog::init_test();

        let db = WorkspaceDb::open_test_db("test_remove_last_breakpoint").await;
        let id = db.next_id().await.unwrap();

        let singular_path = Path::new("/tmp/test_remove_last_breakpoint.rs");

        let breakpoint_to_remove = Breakpoint {
            position: 100,
            message: None,
            state: BreakpointState::Enabled,
            condition: None,
            hit_condition: None,
        };

        let workspace = SerializedWorkspace {
            id,
            paths: PathList::new(&["/tmp"]),
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            breakpoints: {
                let mut map = collections::BTreeMap::default();
                map.insert(
                    Arc::from(singular_path),
                    vec![SourceBreakpoint {
                        row: breakpoint_to_remove.position,
                        path: Arc::from(singular_path),
                        message: None,
                        state: BreakpointState::Enabled,
                        condition: None,
                        hit_condition: None,
                    }],
                );
                map
            },
            session_id: None,
            window_id: None,
        };

        db.save_workspace(workspace.clone()).await;

        let loaded = db.workspace_for_roots(&["/tmp"]).unwrap();
        let loaded_breakpoints = loaded.breakpoints.get(&Arc::from(singular_path)).unwrap();

        assert_eq!(loaded_breakpoints.len(), 1);
        assert_eq!(loaded_breakpoints[0].row, breakpoint_to_remove.position);
        assert_eq!(loaded_breakpoints[0].message, breakpoint_to_remove.message);
        assert_eq!(
            loaded_breakpoints[0].condition,
            breakpoint_to_remove.condition
        );
        assert_eq!(
            loaded_breakpoints[0].hit_condition,
            breakpoint_to_remove.hit_condition
        );
        assert_eq!(loaded_breakpoints[0].state, breakpoint_to_remove.state);
        assert_eq!(loaded_breakpoints[0].path, Arc::from(singular_path));

        let workspace_without_breakpoint = SerializedWorkspace {
            id,
            paths: PathList::new(&["/tmp"]),
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            breakpoints: collections::BTreeMap::default(),
            session_id: None,
            window_id: None,
        };

        db.save_workspace(workspace_without_breakpoint.clone())
            .await;

        let loaded_after_remove = db.workspace_for_roots(&["/tmp"]).unwrap();
        let empty_breakpoints = loaded_after_remove
            .breakpoints
            .get(&Arc::from(singular_path));

        assert!(empty_breakpoints.is_none());
    }

    #[gpui::test]
    async fn test_next_id_stability() {
        zlog::init_test();

        let db = WorkspaceDb::open_test_db("test_next_id_stability").await;

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
        zlog::init_test();

        let db = WorkspaceDb::open_test_db("test_workspace_id_stability").await;

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
            id: WorkspaceId(1),
            paths: PathList::new(&["/tmp", "/tmp2"]),
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            breakpoints: Default::default(),
            session_id: None,
            window_id: None,
        };

        let workspace_2 = SerializedWorkspace {
            id: WorkspaceId(2),
            paths: PathList::new(&["/tmp"]),
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            breakpoints: Default::default(),
            session_id: None,
            window_id: None,
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

        workspace_1.paths = PathList::new(&["/tmp", "/tmp3"]);
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
        zlog::init_test();

        let db = WorkspaceDb::open_test_db("test_full_workspace_serialization").await;

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
                                SerializedItem::new("Terminal", 5, false, false),
                                SerializedItem::new("Terminal", 6, true, false),
                            ],
                            false,
                            0,
                        )),
                        SerializedPaneGroup::Pane(SerializedPane::new(
                            vec![
                                SerializedItem::new("Terminal", 7, true, false),
                                SerializedItem::new("Terminal", 8, false, false),
                            ],
                            false,
                            0,
                        )),
                    ],
                ),
                SerializedPaneGroup::Pane(SerializedPane::new(
                    vec![
                        SerializedItem::new("Terminal", 9, false, false),
                        SerializedItem::new("Terminal", 10, true, false),
                    ],
                    false,
                    0,
                )),
            ],
        );

        let workspace = SerializedWorkspace {
            id: WorkspaceId(5),
            paths: PathList::new(&["/tmp", "/tmp2"]),
            location: SerializedWorkspaceLocation::Local,
            center_group,
            window_bounds: Default::default(),
            breakpoints: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            session_id: None,
            window_id: Some(999),
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
        zlog::init_test();

        let db = WorkspaceDb::open_test_db("test_basic_functionality").await;

        let workspace_1 = SerializedWorkspace {
            id: WorkspaceId(1),
            paths: PathList::new(&["/tmp", "/tmp2"]),
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            breakpoints: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            session_id: None,
            window_id: Some(1),
        };

        let mut workspace_2 = SerializedWorkspace {
            id: WorkspaceId(2),
            paths: PathList::new(&["/tmp"]),
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            breakpoints: Default::default(),
            session_id: None,
            window_id: Some(2),
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
        workspace_2.paths = PathList::new(&["/tmp", "/tmp2"]);

        db.save_workspace(workspace_2.clone()).await;
        assert_eq!(
            db.workspace_for_roots(&["/tmp", "/tmp2"]).unwrap(),
            workspace_2
        );

        // Test other mechanism for mutating
        let mut workspace_3 = SerializedWorkspace {
            id: WorkspaceId(3),
            paths: PathList::new(&["/tmp2", "/tmp"]),
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            breakpoints: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            session_id: None,
            window_id: Some(3),
        };

        db.save_workspace(workspace_3.clone()).await;
        assert_eq!(
            db.workspace_for_roots(&["/tmp", "/tmp2"]).unwrap(),
            workspace_3
        );

        // Make sure that updating paths differently also works
        workspace_3.paths = PathList::new(&["/tmp3", "/tmp4", "/tmp2"]);
        db.save_workspace(workspace_3.clone()).await;
        assert_eq!(db.workspace_for_roots(&["/tmp2", "tmp"]), None);
        assert_eq!(
            db.workspace_for_roots(&["/tmp2", "/tmp3", "/tmp4"])
                .unwrap(),
            workspace_3
        );
    }

    #[gpui::test]
    async fn test_session_workspaces() {
        zlog::init_test();

        let db = WorkspaceDb::open_test_db("test_serializing_workspaces_session_id").await;

        let workspace_1 = SerializedWorkspace {
            id: WorkspaceId(1),
            paths: PathList::new(&["/tmp1"]),
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            breakpoints: Default::default(),
            session_id: Some("session-id-1".to_owned()),
            window_id: Some(10),
        };

        let workspace_2 = SerializedWorkspace {
            id: WorkspaceId(2),
            paths: PathList::new(&["/tmp2"]),
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            breakpoints: Default::default(),
            session_id: Some("session-id-1".to_owned()),
            window_id: Some(20),
        };

        let workspace_3 = SerializedWorkspace {
            id: WorkspaceId(3),
            paths: PathList::new(&["/tmp3"]),
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            breakpoints: Default::default(),
            session_id: Some("session-id-2".to_owned()),
            window_id: Some(30),
        };

        let workspace_4 = SerializedWorkspace {
            id: WorkspaceId(4),
            paths: PathList::new(&["/tmp4"]),
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            breakpoints: Default::default(),
            session_id: None,
            window_id: None,
        };

        let connection_id = db
            .get_or_create_ssh_connection("my-host".to_string(), Some(1234), None)
            .await
            .unwrap();

        let workspace_5 = SerializedWorkspace {
            id: WorkspaceId(5),
            paths: PathList::default(),
            location: SerializedWorkspaceLocation::Ssh(db.ssh_connection(connection_id).unwrap()),
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            breakpoints: Default::default(),
            session_id: Some("session-id-2".to_owned()),
            window_id: Some(50),
        };

        let workspace_6 = SerializedWorkspace {
            id: WorkspaceId(6),
            paths: PathList::new(&["/tmp6a", "/tmp6b", "/tmp6c"]),
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            breakpoints: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            session_id: Some("session-id-3".to_owned()),
            window_id: Some(60),
        };

        db.save_workspace(workspace_1.clone()).await;
        thread::sleep(Duration::from_millis(1000)); // Force timestamps to increment
        db.save_workspace(workspace_2.clone()).await;
        db.save_workspace(workspace_3.clone()).await;
        thread::sleep(Duration::from_millis(1000)); // Force timestamps to increment
        db.save_workspace(workspace_4.clone()).await;
        db.save_workspace(workspace_5.clone()).await;
        db.save_workspace(workspace_6.clone()).await;

        let locations = db.session_workspaces("session-id-1".to_owned()).unwrap();
        assert_eq!(locations.len(), 2);
        assert_eq!(locations[0].0, PathList::new(&["/tmp2"]));
        assert_eq!(locations[0].1, Some(20));
        assert_eq!(locations[1].0, PathList::new(&["/tmp1"]));
        assert_eq!(locations[1].1, Some(10));

        let locations = db.session_workspaces("session-id-2".to_owned()).unwrap();
        assert_eq!(locations.len(), 2);
        assert_eq!(locations[0].0, PathList::default());
        assert_eq!(locations[0].1, Some(50));
        assert_eq!(locations[0].2, Some(connection_id));
        assert_eq!(locations[1].0, PathList::new(&["/tmp3"]));
        assert_eq!(locations[1].1, Some(30));

        let locations = db.session_workspaces("session-id-3".to_owned()).unwrap();
        assert_eq!(locations.len(), 1);
        assert_eq!(
            locations[0].0,
            PathList::new(&["/tmp6a", "/tmp6b", "/tmp6c"]),
        );
        assert_eq!(locations[0].1, Some(60));
    }

    fn default_workspace<P: AsRef<Path>>(
        paths: &[P],
        center_group: &SerializedPaneGroup,
    ) -> SerializedWorkspace {
        SerializedWorkspace {
            id: WorkspaceId(4),
            paths: PathList::new(paths),
            location: SerializedWorkspaceLocation::Local,
            center_group: center_group.clone(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            breakpoints: Default::default(),
            centered_layout: false,
            session_id: None,
            window_id: None,
        }
    }

    #[gpui::test]
    async fn test_last_session_workspace_locations() {
        let dir1 = tempfile::TempDir::with_prefix("dir1").unwrap();
        let dir2 = tempfile::TempDir::with_prefix("dir2").unwrap();
        let dir3 = tempfile::TempDir::with_prefix("dir3").unwrap();
        let dir4 = tempfile::TempDir::with_prefix("dir4").unwrap();

        let db =
            WorkspaceDb::open_test_db("test_serializing_workspaces_last_session_workspaces").await;

        let workspaces = [
            (1, vec![dir1.path()], 9),
            (2, vec![dir2.path()], 5),
            (3, vec![dir3.path()], 8),
            (4, vec![dir4.path()], 2),
            (5, vec![dir1.path(), dir2.path(), dir3.path()], 3),
            (6, vec![dir4.path(), dir3.path(), dir2.path()], 4),
        ]
        .into_iter()
        .map(|(id, paths, window_id)| SerializedWorkspace {
            id: WorkspaceId(id),
            paths: PathList::new(paths.as_slice()),
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            session_id: Some("one-session".to_owned()),
            breakpoints: Default::default(),
            window_id: Some(window_id),
        })
        .collect::<Vec<_>>();

        for workspace in workspaces.iter() {
            db.save_workspace(workspace.clone()).await;
        }

        let stack = Some(Vec::from([
            WindowId::from(2), // Top
            WindowId::from(8),
            WindowId::from(5),
            WindowId::from(9),
            WindowId::from(3),
            WindowId::from(4), // Bottom
        ]));

        let locations = db
            .last_session_workspace_locations("one-session", stack)
            .unwrap();
        assert_eq!(
            locations,
            [
                (
                    SerializedWorkspaceLocation::Local,
                    PathList::new(&[dir4.path()])
                ),
                (
                    SerializedWorkspaceLocation::Local,
                    PathList::new(&[dir3.path()])
                ),
                (
                    SerializedWorkspaceLocation::Local,
                    PathList::new(&[dir2.path()])
                ),
                (
                    SerializedWorkspaceLocation::Local,
                    PathList::new(&[dir1.path()])
                ),
                (
                    SerializedWorkspaceLocation::Local,
                    PathList::new(&[dir1.path(), dir2.path(), dir3.path()])
                ),
                (
                    SerializedWorkspaceLocation::Local,
                    PathList::new(&[dir4.path(), dir3.path(), dir2.path()])
                ),
            ]
        );
    }

    #[gpui::test]
    async fn test_last_session_workspace_locations_ssh_projects() {
        let db = WorkspaceDb::open_test_db(
            "test_serializing_workspaces_last_session_workspaces_ssh_projects",
        )
        .await;

        let ssh_connections = [
            ("host-1", "my-user-1"),
            ("host-2", "my-user-2"),
            ("host-3", "my-user-3"),
            ("host-4", "my-user-4"),
        ]
        .into_iter()
        .map(|(host, user)| async {
            db.get_or_create_ssh_connection(host.to_string(), None, Some(user.to_string()))
                .await
                .unwrap();
            SerializedSshConnection {
                host: host.into(),
                port: None,
                user: Some(user.into()),
            }
        })
        .collect::<Vec<_>>();

        let ssh_connections = futures::future::join_all(ssh_connections).await;

        let workspaces = [
            (1, ssh_connections[0].clone(), 9),
            (2, ssh_connections[1].clone(), 5),
            (3, ssh_connections[2].clone(), 8),
            (4, ssh_connections[3].clone(), 2),
        ]
        .into_iter()
        .map(|(id, ssh_connection, window_id)| SerializedWorkspace {
            id: WorkspaceId(id),
            paths: PathList::default(),
            location: SerializedWorkspaceLocation::Ssh(ssh_connection),
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            session_id: Some("one-session".to_owned()),
            breakpoints: Default::default(),
            window_id: Some(window_id),
        })
        .collect::<Vec<_>>();

        for workspace in workspaces.iter() {
            db.save_workspace(workspace.clone()).await;
        }

        let stack = Some(Vec::from([
            WindowId::from(2), // Top
            WindowId::from(8),
            WindowId::from(5),
            WindowId::from(9), // Bottom
        ]));

        let have = db
            .last_session_workspace_locations("one-session", stack)
            .unwrap();
        assert_eq!(have.len(), 4);
        assert_eq!(
            have[0],
            (
                SerializedWorkspaceLocation::Ssh(ssh_connections[3].clone()),
                PathList::default()
            )
        );
        assert_eq!(
            have[1],
            (
                SerializedWorkspaceLocation::Ssh(ssh_connections[2].clone()),
                PathList::default()
            )
        );
        assert_eq!(
            have[2],
            (
                SerializedWorkspaceLocation::Ssh(ssh_connections[1].clone()),
                PathList::default()
            )
        );
        assert_eq!(
            have[3],
            (
                SerializedWorkspaceLocation::Ssh(ssh_connections[0].clone()),
                PathList::default()
            )
        );
    }

    #[gpui::test]
    async fn test_get_or_create_ssh_project() {
        let db = WorkspaceDb::open_test_db("test_get_or_create_ssh_project").await;

        let host = "example.com".to_string();
        let port = Some(22_u16);
        let user = Some("user".to_string());

        let connection_id = db
            .get_or_create_ssh_connection(host.clone(), port, user.clone())
            .await
            .unwrap();

        // Test that calling the function again with the same parameters returns the same project
        let same_connection = db
            .get_or_create_ssh_connection(host.clone(), port, user.clone())
            .await
            .unwrap();

        assert_eq!(connection_id, same_connection);

        // Test with different parameters
        let host2 = "otherexample.com".to_string();
        let port2 = None;
        let user2 = Some("otheruser".to_string());

        let different_connection = db
            .get_or_create_ssh_connection(host2.clone(), port2, user2.clone())
            .await
            .unwrap();

        assert_ne!(connection_id, different_connection);
    }

    #[gpui::test]
    async fn test_get_or_create_ssh_project_with_null_user() {
        let db = WorkspaceDb::open_test_db("test_get_or_create_ssh_project_with_null_user").await;

        let (host, port, user) = ("example.com".to_string(), None, None);

        let connection_id = db
            .get_or_create_ssh_connection(host.clone(), port, None)
            .await
            .unwrap();

        let same_connection_id = db
            .get_or_create_ssh_connection(host.clone(), port, user.clone())
            .await
            .unwrap();

        assert_eq!(connection_id, same_connection_id);
    }

    #[gpui::test]
    async fn test_get_ssh_connections() {
        let db = WorkspaceDb::open_test_db("test_get_ssh_connections").await;

        let connections = [
            ("example.com".to_string(), None, None),
            (
                "anotherexample.com".to_string(),
                Some(123_u16),
                Some("user2".to_string()),
            ),
            ("yetanother.com".to_string(), Some(345_u16), None),
        ];

        let mut ids = Vec::new();
        for (host, port, user) in connections.iter() {
            ids.push(
                db.get_or_create_ssh_connection(host.clone(), *port, user.clone())
                    .await
                    .unwrap(),
            );
        }

        let stored_projects = db.ssh_connections().unwrap();
        assert_eq!(
            stored_projects,
            [
                (
                    ids[0],
                    SerializedSshConnection {
                        host: "example.com".into(),
                        port: None,
                        user: None,
                    }
                ),
                (
                    ids[1],
                    SerializedSshConnection {
                        host: "anotherexample.com".into(),
                        port: Some(123),
                        user: Some("user2".into()),
                    }
                ),
                (
                    ids[2],
                    SerializedSshConnection {
                        host: "yetanother.com".into(),
                        port: Some(345),
                        user: None,
                    }
                ),
            ]
            .into_iter()
            .collect::<HashMap<_, _>>(),
        );
    }

    #[gpui::test]
    async fn test_simple_split() {
        zlog::init_test();

        let db = WorkspaceDb::open_test_db("simple_split").await;

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
                                SerializedItem::new("Terminal", 1, false, false),
                                SerializedItem::new("Terminal", 2, true, false),
                            ],
                            false,
                            0,
                        )),
                        SerializedPaneGroup::Pane(SerializedPane::new(
                            vec![
                                SerializedItem::new("Terminal", 4, false, false),
                                SerializedItem::new("Terminal", 3, true, false),
                            ],
                            true,
                            0,
                        )),
                    ],
                ),
                SerializedPaneGroup::Pane(SerializedPane::new(
                    vec![
                        SerializedItem::new("Terminal", 5, true, false),
                        SerializedItem::new("Terminal", 6, false, false),
                    ],
                    false,
                    0,
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
        zlog::init_test();

        let db = WorkspaceDb::open_test_db("test_cleanup_panes").await;

        let center_pane = group(
            Axis::Horizontal,
            vec![
                group(
                    Axis::Vertical,
                    vec![
                        SerializedPaneGroup::Pane(SerializedPane::new(
                            vec![
                                SerializedItem::new("Terminal", 1, false, false),
                                SerializedItem::new("Terminal", 2, true, false),
                            ],
                            false,
                            0,
                        )),
                        SerializedPaneGroup::Pane(SerializedPane::new(
                            vec![
                                SerializedItem::new("Terminal", 4, false, false),
                                SerializedItem::new("Terminal", 3, true, false),
                            ],
                            true,
                            0,
                        )),
                    ],
                ),
                SerializedPaneGroup::Pane(SerializedPane::new(
                    vec![
                        SerializedItem::new("Terminal", 5, false, false),
                        SerializedItem::new("Terminal", 6, true, false),
                    ],
                    false,
                    0,
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
                        SerializedItem::new("Terminal", 1, false, false),
                        SerializedItem::new("Terminal", 2, true, false),
                    ],
                    false,
                    0,
                )),
                SerializedPaneGroup::Pane(SerializedPane::new(
                    vec![
                        SerializedItem::new("Terminal", 4, true, false),
                        SerializedItem::new("Terminal", 3, false, false),
                    ],
                    true,
                    0,
                )),
            ],
        );

        db.save_workspace(workspace.clone()).await;

        let new_workspace = db.workspace_for_roots(id).unwrap();

        assert_eq!(workspace.center_group, new_workspace.center_group);
    }
}
