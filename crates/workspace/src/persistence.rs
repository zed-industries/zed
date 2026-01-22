pub mod model;

use std::{
    borrow::Cow,
    collections::BTreeMap,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};

use anyhow::{Context as _, Result, bail};
use collections::{HashMap, HashSet, IndexSet};
use db::{
    kvp::KEY_VALUE_STORE,
    query,
    sqlez::{connection::Connection, domain::Domain},
    sqlez_macros::sql,
};
use gpui::{Axis, Bounds, Task, WindowBounds, WindowId, point, size};
use project::{
    debugger::breakpoint_store::{BreakpointState, SourceBreakpoint},
    trusted_worktrees::{DbTrustedPaths, RemoteHostLocation},
};

use language::{LanguageName, Toolchain, ToolchainScope};
use remote::{
    DockerConnectionOptions, RemoteConnectionOptions, SshConnectionOptions, WslConnectionOptions,
};
use serde::{Deserialize, Serialize};
use sqlez::{
    bindable::{Bind, Column, StaticColumnCount},
    statement::Statement,
    thread_safe_connection::ThreadSafeConnection,
};

use ui::{App, SharedString, px};
use util::{ResultExt, maybe, rel_path::RelPath};
use uuid::Uuid;

use crate::{
    WorkspaceId,
    path_list::{PathList, SerializedPathList},
    persistence::model::RemoteConnectionKind,
};

use model::{
    GroupId, ItemId, PaneId, RemoteConnectionId, SerializedItem, SerializedPane,
    SerializedPaneGroup, SerializedWorkspace,
};

use self::model::{DockStructure, SerializedWorkspaceLocation};

// https://www.sqlite.org/limits.html
// > <..> the maximum value of a host parameter number is SQLITE_MAX_VARIABLE_NUMBER,
// > which defaults to <..> 32766 for SQLite versions after 3.32.0.
const MAX_QUERY_PLACEHOLDERS: usize = 32000;

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

const DEFAULT_WINDOW_BOUNDS_KEY: &str = "default_window_bounds";

pub fn read_default_window_bounds() -> Option<(Uuid, WindowBounds)> {
    let json_str = KEY_VALUE_STORE
        .read_kvp(DEFAULT_WINDOW_BOUNDS_KEY)
        .log_err()
        .flatten()?;

    let (display_uuid, persisted) =
        serde_json::from_str::<(Uuid, WindowBoundsJson)>(&json_str).ok()?;
    Some((display_uuid, persisted.into()))
}

pub async fn write_default_window_bounds(
    bounds: WindowBounds,
    display_uuid: Uuid,
) -> anyhow::Result<()> {
    let persisted = WindowBoundsJson::from(bounds);
    let json_str = serde_json::to_string(&(display_uuid, persisted))?;
    KEY_VALUE_STORE
        .write_kvp(DEFAULT_WINDOW_BOUNDS_KEY.to_string(), json_str)
        .await?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub enum WindowBoundsJson {
    Windowed {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    },
    Maximized {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    },
    Fullscreen {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    },
}

impl From<WindowBounds> for WindowBoundsJson {
    fn from(b: WindowBounds) -> Self {
        match b {
            WindowBounds::Windowed(bounds) => {
                let origin = bounds.origin;
                let size = bounds.size;
                WindowBoundsJson::Windowed {
                    x: f32::from(origin.x).round() as i32,
                    y: f32::from(origin.y).round() as i32,
                    width: f32::from(size.width).round() as i32,
                    height: f32::from(size.height).round() as i32,
                }
            }
            WindowBounds::Maximized(bounds) => {
                let origin = bounds.origin;
                let size = bounds.size;
                WindowBoundsJson::Maximized {
                    x: f32::from(origin.x).round() as i32,
                    y: f32::from(origin.y).round() as i32,
                    width: f32::from(size.width).round() as i32,
                    height: f32::from(size.height).round() as i32,
                }
            }
            WindowBounds::Fullscreen(bounds) => {
                let origin = bounds.origin;
                let size = bounds.size;
                WindowBoundsJson::Fullscreen {
                    x: f32::from(origin.x).round() as i32,
                    y: f32::from(origin.y).round() as i32,
                    width: f32::from(size.width).round() as i32,
                    height: f32::from(size.height).round() as i32,
                }
            }
        }
    }
}

impl From<WindowBoundsJson> for WindowBounds {
    fn from(n: WindowBoundsJson) -> Self {
        match n {
            WindowBoundsJson::Windowed {
                x,
                y,
                width,
                height,
            } => WindowBounds::Windowed(Bounds {
                origin: point(px(x as f32), px(y as f32)),
                size: size(px(width as f32), px(height as f32)),
            }),
            WindowBoundsJson::Maximized {
                x,
                y,
                width,
                height,
            } => WindowBounds::Maximized(Bounds {
                origin: point(px(x as f32), px(y as f32)),
                size: size(px(width as f32), px(height as f32)),
            }),
            WindowBoundsJson::Fullscreen {
                x,
                y,
                width,
                height,
            } => WindowBounds::Fullscreen(Bounds {
                origin: point(px(x as f32), px(y as f32)),
                size: size(px(width as f32), px(height as f32)),
            }),
        }
    }
}

const DEFAULT_DOCK_STATE_KEY: &str = "default_dock_state";

pub fn read_default_dock_state() -> Option<DockStructure> {
    let json_str = KEY_VALUE_STORE
        .read_kvp(DEFAULT_DOCK_STATE_KEY)
        .log_err()
        .flatten()?;

    serde_json::from_str::<DockStructure>(&json_str).ok()
}

pub async fn write_default_dock_state(docks: DockStructure) -> anyhow::Result<()> {
    let json_str = serde_json::to_string(&docks)?;
    KEY_VALUE_STORE
        .write_kvp(DEFAULT_DOCK_STATE_KEY.to_string(), json_str)
        .await?;
    Ok(())
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

#[derive(Clone, Debug, PartialEq)]
struct SerializedPixels(gpui::Pixels);
impl sqlez::bindable::StaticColumnCount for SerializedPixels {}

impl sqlez::bindable::Bind for SerializedPixels {
    fn bind(
        &self,
        statement: &sqlez::statement::Statement,
        start_index: i32,
    ) -> anyhow::Result<i32> {
        let this: i32 = u32::from(self.0) as _;
        this.bind(statement, start_index)
    }
}

pub struct WorkspaceDb(ThreadSafeConnection);

impl Domain for WorkspaceDb {
    const NAME: &str = stringify!(WorkspaceDb);

    const MIGRATIONS: &[&str] = &[
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
                                replace(workspaces.local_paths_array, ',', CHAR(10))
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

            DELETE FROM workspaces_2
            WHERE workspace_id NOT IN (
                SELECT MAX(workspace_id)
                FROM workspaces_2
                GROUP BY ssh_connection_id, paths
            );

            DROP TABLE ssh_projects;
            DROP TABLE workspaces;
            ALTER TABLE workspaces_2 RENAME TO workspaces;

            CREATE UNIQUE INDEX ix_workspaces_location ON workspaces(ssh_connection_id, paths);
        ),
        // Fix any data from when workspaces.paths were briefly encoded as JSON arrays
        sql!(
            UPDATE workspaces
            SET paths = CASE
                WHEN substr(paths, 1, 2) = '[' || '"' AND substr(paths, -2, 2) = '"' || ']' THEN
                    replace(
                        substr(paths, 3, length(paths) - 4),
                        '"' || ',' || '"',
                        CHAR(10)
                    )
                ELSE
                    replace(paths, ',', CHAR(10))
            END
            WHERE paths IS NOT NULL
        ),
        sql!(
            CREATE TABLE remote_connections(
                id INTEGER PRIMARY KEY,
                kind TEXT NOT NULL,
                host TEXT,
                port INTEGER,
                user TEXT,
                distro TEXT
            );

            CREATE TABLE workspaces_2(
                workspace_id INTEGER PRIMARY KEY,
                paths TEXT,
                paths_order TEXT,
                remote_connection_id INTEGER REFERENCES remote_connections(id),
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

            INSERT INTO remote_connections
            SELECT
                id,
                "ssh" as kind,
                host,
                port,
                user,
                NULL as distro
            FROM ssh_connections;

            INSERT
            INTO workspaces_2
            SELECT
                workspace_id,
                paths,
                paths_order,
                ssh_connection_id as remote_connection_id,
                timestamp,
                window_state,
                window_x,
                window_y,
                window_width,
                window_height,
                display,
                left_dock_visible,
                left_dock_active_panel,
                right_dock_visible,
                right_dock_active_panel,
                bottom_dock_visible,
                bottom_dock_active_panel,
                left_dock_zoom,
                right_dock_zoom,
                bottom_dock_zoom,
                fullscreen,
                centered_layout,
                session_id,
                window_id
            FROM
                workspaces;

            DROP TABLE workspaces;
            ALTER TABLE workspaces_2 RENAME TO workspaces;

            CREATE UNIQUE INDEX ix_workspaces_location ON workspaces(remote_connection_id, paths);
        ),
        sql!(CREATE TABLE user_toolchains (
            remote_connection_id INTEGER,
            workspace_id INTEGER NOT NULL,
            worktree_id INTEGER NOT NULL,
            relative_worktree_path TEXT NOT NULL,
            language_name TEXT NOT NULL,
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            raw_json TEXT NOT NULL,

            PRIMARY KEY (workspace_id, worktree_id, relative_worktree_path, language_name, name, path, raw_json)
        ) STRICT;),
        sql!(
            DROP TABLE ssh_connections;
        ),
        sql!(
            ALTER TABLE remote_connections ADD COLUMN name TEXT;
            ALTER TABLE remote_connections ADD COLUMN container_id TEXT;
        ),
        sql!(
            CREATE TABLE IF NOT EXISTS trusted_worktrees (
                trust_id INTEGER PRIMARY KEY AUTOINCREMENT,
                absolute_path TEXT,
                user_name TEXT,
                host_name TEXT
            ) STRICT;
        ),
        sql!(CREATE TABLE toolchains2 (
            workspace_id INTEGER,
            worktree_root_path TEXT NOT NULL,
            language_name TEXT NOT NULL,
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            raw_json TEXT NOT NULL,
            relative_worktree_path TEXT NOT NULL,
            PRIMARY KEY (workspace_id, worktree_root_path, language_name, relative_worktree_path)) STRICT;
            INSERT OR REPLACE INTO toolchains2
                // The `instr(paths, '\n') = 0` part allows us to find all
                // workspaces that have a single worktree, as `\n` is used as a
                // separator when serializing the workspace paths, so if no `\n` is
                // found, we know we have a single worktree.
                SELECT toolchains.workspace_id, paths, language_name, name, path, raw_json, relative_worktree_path FROM toolchains INNER JOIN workspaces ON toolchains.workspace_id = workspaces.workspace_id AND instr(paths, '\n') = 0;
            DROP TABLE toolchains;
            ALTER TABLE toolchains2 RENAME TO toolchains;
        ),
        sql!(CREATE TABLE user_toolchains2 (
            remote_connection_id INTEGER,
            workspace_id INTEGER NOT NULL,
            worktree_root_path TEXT NOT NULL,
            relative_worktree_path TEXT NOT NULL,
            language_name TEXT NOT NULL,
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            raw_json TEXT NOT NULL,

            PRIMARY KEY (workspace_id, worktree_root_path, relative_worktree_path, language_name, name, path, raw_json)) STRICT;
            INSERT OR REPLACE INTO user_toolchains2
                // The `instr(paths, '\n') = 0` part allows us to find all
                // workspaces that have a single worktree, as `\n` is used as a
                // separator when serializing the workspace paths, so if no `\n` is
                // found, we know we have a single worktree.
                SELECT user_toolchains.remote_connection_id, user_toolchains.workspace_id, paths, relative_worktree_path, language_name, name, path, raw_json  FROM user_toolchains INNER JOIN workspaces ON user_toolchains.workspace_id = workspaces.workspace_id AND instr(paths, '\n') = 0;
            DROP TABLE user_toolchains;
            ALTER TABLE user_toolchains2 RENAME TO user_toolchains;
        ),
        sql!(
            ALTER TABLE remote_connections ADD COLUMN use_podman BOOLEAN;
        ),
    ];

    // Allow recovering from bad migration that was initially shipped to nightly
    // when introducing the ssh_connections table.
    fn should_allow_migration_change(_index: usize, old: &str, new: &str) -> bool {
        old.starts_with("CREATE TABLE ssh_connections")
            && new.starts_with("CREATE TABLE ssh_connections")
    }
}

db::static_connection!(DB, WorkspaceDb, []);

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

    pub(crate) fn remote_workspace_for_roots<P: AsRef<Path>>(
        &self,
        worktree_roots: &[P],
        remote_project_id: RemoteConnectionId,
    ) -> Option<SerializedWorkspace> {
        self.workspace_for_roots_internal(worktree_roots, Some(remote_project_id))
    }

    pub(crate) fn workspace_for_roots_internal<P: AsRef<Path>>(
        &self,
        worktree_roots: &[P],
        remote_connection_id: Option<RemoteConnectionId>,
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
                    remote_connection_id IS ?
                LIMIT 1
            })
            .and_then(|mut prepared_statement| {
                (prepared_statement)((
                    root_paths.serialize().paths,
                    remote_connection_id.map(|id| id.0 as i32),
                ))
            })
            .context("No workspaces found")
            .warn_on_err()
            .flatten()?;

        let paths = PathList::deserialize(&SerializedPathList {
            paths,
            order: paths_order,
        });

        let remote_connection_options = if let Some(remote_connection_id) = remote_connection_id {
            self.remote_connection(remote_connection_id)
                .context("Get remote connection")
                .log_err()
        } else {
            None
        };

        Some(SerializedWorkspace {
            id: workspace_id,
            location: match remote_connection_options {
                Some(options) => SerializedWorkspaceLocation::Remote(options),
                None => SerializedWorkspaceLocation::Local,
            },
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
            user_toolchains: self.user_toolchains(workspace_id, remote_connection_id),
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

    fn user_toolchains(
        &self,
        workspace_id: WorkspaceId,
        remote_connection_id: Option<RemoteConnectionId>,
    ) -> BTreeMap<ToolchainScope, IndexSet<Toolchain>> {
        type RowKind = (WorkspaceId, String, String, String, String, String, String);

        let toolchains: Vec<RowKind> = self
            .select_bound(sql! {
                SELECT workspace_id, worktree_root_path, relative_worktree_path,
                language_name, name, path, raw_json
                FROM user_toolchains WHERE remote_connection_id IS ?1 AND (
                      workspace_id IN (0, ?2)
                )
            })
            .and_then(|mut statement| {
                (statement)((remote_connection_id.map(|id| id.0), workspace_id))
            })
            .unwrap_or_default();
        let mut ret = BTreeMap::<_, IndexSet<_>>::default();

        for (
            _workspace_id,
            worktree_root_path,
            relative_worktree_path,
            language_name,
            name,
            path,
            raw_json,
        ) in toolchains
        {
            // INTEGER's that are primary keys (like workspace ids, remote connection ids and such) start at 1, so we're safe to
            let scope = if _workspace_id == WorkspaceId(0) {
                debug_assert_eq!(worktree_root_path, String::default());
                debug_assert_eq!(relative_worktree_path, String::default());
                ToolchainScope::Global
            } else {
                debug_assert_eq!(workspace_id, _workspace_id);
                debug_assert_eq!(
                    worktree_root_path == String::default(),
                    relative_worktree_path == String::default()
                );

                let Some(relative_path) = RelPath::unix(&relative_worktree_path).log_err() else {
                    continue;
                };
                if worktree_root_path != String::default()
                    && relative_worktree_path != String::default()
                {
                    ToolchainScope::Subproject(
                        Arc::from(worktree_root_path.as_ref()),
                        relative_path.into(),
                    )
                } else {
                    ToolchainScope::Project
                }
            };
            let Ok(as_json) = serde_json::from_str(&raw_json) else {
                continue;
            };
            let toolchain = Toolchain {
                name: SharedString::from(name),
                path: SharedString::from(path),
                language_name: LanguageName::from_proto(language_name),
                as_json,
            };
            ret.entry(scope).or_default().insert(toolchain);
        }

        ret
    }

    /// Saves a workspace using the worktree roots. Will garbage collect any workspaces
    /// that used this workspace previously
    pub(crate) async fn save_workspace(&self, workspace: SerializedWorkspace) {
        let paths = workspace.paths.serialize();
        log::debug!("Saving workspace at location: {:?}", workspace.location);
        self.write(move |conn| {
            conn.with_savepoint("update_worktrees", || {
                let remote_connection_id = match workspace.location.clone() {
                    SerializedWorkspaceLocation::Local => None,
                    SerializedWorkspaceLocation::Remote(connection_options) => {
                        Some(Self::get_or_create_remote_connection_internal(
                            conn,
                            connection_options
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

                conn.exec_bound(
                    sql!(
                        DELETE FROM user_toolchains WHERE workspace_id = ?1;
                    )
                )?(workspace.id).context("Clearing old user toolchains")?;

                for (scope, toolchains) in workspace.user_toolchains {
                    for toolchain in toolchains {
                        let query = sql!(INSERT OR REPLACE INTO user_toolchains(remote_connection_id, workspace_id, worktree_root_path, relative_worktree_path, language_name, name, path, raw_json) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8));
                        let (workspace_id, worktree_root_path, relative_worktree_path) = match scope {
                            ToolchainScope::Subproject(ref worktree_root_path, ref path) => (Some(workspace.id), Some(worktree_root_path.to_string_lossy().into_owned()), Some(path.as_unix_str().to_owned())),
                            ToolchainScope::Project => (Some(workspace.id), None, None),
                            ToolchainScope::Global => (None, None, None),
                        };
                        let args = (remote_connection_id, workspace_id.unwrap_or(WorkspaceId(0)), worktree_root_path.unwrap_or_default(), relative_worktree_path.unwrap_or_default(),
                        toolchain.language_name.as_ref().to_owned(), toolchain.name.to_string(), toolchain.path.to_string(), toolchain.as_json.to_string());
                        if let Err(err) = conn.exec_bound(query)?(args) {
                            log::error!("{err}");
                            continue;
                        }
                    }
                }

                conn.exec_bound(sql!(
                    DELETE
                    FROM workspaces
                    WHERE
                        workspace_id != ?1 AND
                        paths IS ?2 AND
                        remote_connection_id IS ?3
                ))?((
                    workspace.id,
                    paths.paths.clone(),
                    remote_connection_id,
                ))
                .context("clearing out old locations")?;

                // Upsert
                let query = sql!(
                    INSERT INTO workspaces(
                        workspace_id,
                        paths,
                        paths_order,
                        remote_connection_id,
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
                        remote_connection_id = ?4,
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
                    remote_connection_id,
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

    pub(crate) async fn get_or_create_remote_connection(
        &self,
        options: RemoteConnectionOptions,
    ) -> Result<RemoteConnectionId> {
        self.write(move |conn| Self::get_or_create_remote_connection_internal(conn, options))
            .await
    }

    fn get_or_create_remote_connection_internal(
        this: &Connection,
        options: RemoteConnectionOptions,
    ) -> Result<RemoteConnectionId> {
        let kind;
        let mut user = None;
        let mut host = None;
        let mut port = None;
        let mut distro = None;
        let mut name = None;
        let mut container_id = None;
        let mut use_podman = None;
        match options {
            RemoteConnectionOptions::Ssh(options) => {
                kind = RemoteConnectionKind::Ssh;
                host = Some(options.host.to_string());
                port = options.port;
                user = options.username;
            }
            RemoteConnectionOptions::Wsl(options) => {
                kind = RemoteConnectionKind::Wsl;
                distro = Some(options.distro_name);
                user = options.user;
            }
            RemoteConnectionOptions::Docker(options) => {
                kind = RemoteConnectionKind::Docker;
                container_id = Some(options.container_id);
                name = Some(options.name);
                use_podman = Some(options.use_podman)
            }
            #[cfg(any(test, feature = "test-support"))]
            RemoteConnectionOptions::Mock(options) => {
                kind = RemoteConnectionKind::Ssh;
                host = Some(format!("mock-{}", options.id));
            }
        }
        Self::get_or_create_remote_connection_query(
            this,
            kind,
            host,
            port,
            user,
            distro,
            name,
            container_id,
            use_podman,
        )
    }

    fn get_or_create_remote_connection_query(
        this: &Connection,
        kind: RemoteConnectionKind,
        host: Option<String>,
        port: Option<u16>,
        user: Option<String>,
        distro: Option<String>,
        name: Option<String>,
        container_id: Option<String>,
        use_podman: Option<bool>,
    ) -> Result<RemoteConnectionId> {
        if let Some(id) = this.select_row_bound(sql!(
            SELECT id
            FROM remote_connections
            WHERE
                kind IS ? AND
                host IS ? AND
                port IS ? AND
                user IS ? AND
                distro IS ? AND
                name IS ? AND
                container_id IS ?
            LIMIT 1
        ))?((
            kind.serialize(),
            host.clone(),
            port,
            user.clone(),
            distro.clone(),
            name.clone(),
            container_id.clone(),
        ))? {
            Ok(RemoteConnectionId(id))
        } else {
            let id = this.select_row_bound(sql!(
                INSERT INTO remote_connections (
                    kind,
                    host,
                    port,
                    user,
                    distro,
                    name,
                    container_id,
                    use_podman
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                RETURNING id
            ))?((
                kind.serialize(),
                host,
                port,
                user,
                distro,
                name,
                container_id,
                use_podman,
            ))?
            .context("failed to insert remote project")?;
            Ok(RemoteConnectionId(id))
        }
    }

    query! {
        pub async fn next_id() -> Result<WorkspaceId> {
            INSERT INTO workspaces DEFAULT VALUES RETURNING workspace_id
        }
    }

    fn recent_workspaces(
        &self,
    ) -> Result<Vec<(WorkspaceId, PathList, Option<RemoteConnectionId>)>> {
        Ok(self
            .recent_workspaces_query()?
            .into_iter()
            .map(|(id, paths, order, remote_connection_id)| {
                (
                    id,
                    PathList::deserialize(&SerializedPathList { paths, order }),
                    remote_connection_id.map(RemoteConnectionId),
                )
            })
            .collect())
    }

    query! {
        fn recent_workspaces_query() -> Result<Vec<(WorkspaceId, String, String, Option<u64>)>> {
            SELECT workspace_id, paths, paths_order, remote_connection_id
            FROM workspaces
            WHERE
                paths IS NOT NULL OR
                remote_connection_id IS NOT NULL
            ORDER BY timestamp DESC
        }
    }

    fn session_workspaces(
        &self,
        session_id: String,
    ) -> Result<Vec<(PathList, Option<u64>, Option<RemoteConnectionId>)>> {
        Ok(self
            .session_workspaces_query(session_id)?
            .into_iter()
            .map(|(paths, order, window_id, remote_connection_id)| {
                (
                    PathList::deserialize(&SerializedPathList { paths, order }),
                    window_id,
                    remote_connection_id.map(RemoteConnectionId),
                )
            })
            .collect())
    }

    query! {
        fn session_workspaces_query(session_id: String) -> Result<Vec<(String, String, Option<u64>, Option<u64>)>> {
            SELECT paths, paths_order, window_id, remote_connection_id
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

    fn remote_connections(&self) -> Result<HashMap<RemoteConnectionId, RemoteConnectionOptions>> {
        Ok(self.select(sql!(
            SELECT
                id, kind, host, port, user, distro, container_id, name, use_podman
            FROM
                remote_connections
        ))?()?
        .into_iter()
        .filter_map(
            |(id, kind, host, port, user, distro, container_id, name, use_podman)| {
                Some((
                    RemoteConnectionId(id),
                    Self::remote_connection_from_row(
                        kind,
                        host,
                        port,
                        user,
                        distro,
                        container_id,
                        name,
                        use_podman,
                    )?,
                ))
            },
        )
        .collect())
    }

    pub(crate) fn remote_connection(
        &self,
        id: RemoteConnectionId,
    ) -> Result<RemoteConnectionOptions> {
        let (kind, host, port, user, distro, container_id, name, use_podman) =
            self.select_row_bound(sql!(
                SELECT kind, host, port, user, distro, container_id, name, use_podman
                FROM remote_connections
                WHERE id = ?
            ))?(id.0)?
            .context("no such remote connection")?;
        Self::remote_connection_from_row(
            kind,
            host,
            port,
            user,
            distro,
            container_id,
            name,
            use_podman,
        )
        .context("invalid remote_connection row")
    }

    fn remote_connection_from_row(
        kind: String,
        host: Option<String>,
        port: Option<u16>,
        user: Option<String>,
        distro: Option<String>,
        container_id: Option<String>,
        name: Option<String>,
        use_podman: Option<bool>,
    ) -> Option<RemoteConnectionOptions> {
        match RemoteConnectionKind::deserialize(&kind)? {
            RemoteConnectionKind::Wsl => Some(RemoteConnectionOptions::Wsl(WslConnectionOptions {
                distro_name: distro?,
                user: user,
            })),
            RemoteConnectionKind::Ssh => Some(RemoteConnectionOptions::Ssh(SshConnectionOptions {
                host: host?.into(),
                port,
                username: user,
                ..Default::default()
            })),
            RemoteConnectionKind::Docker => {
                Some(RemoteConnectionOptions::Docker(DockerConnectionOptions {
                    container_id: container_id?,
                    name: name?,
                    upload_binary_over_docker_exec: false,
                    use_podman: use_podman?,
                }))
            }
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
    pub async fn recent_workspaces_on_disk(
        &self,
    ) -> Result<Vec<(WorkspaceId, SerializedWorkspaceLocation, PathList)>> {
        let mut result = Vec::new();
        let mut delete_tasks = Vec::new();
        let remote_connections = self.remote_connections()?;

        for (id, paths, remote_connection_id) in self.recent_workspaces()? {
            if let Some(remote_connection_id) = remote_connection_id {
                if let Some(connection_options) = remote_connections.get(&remote_connection_id) {
                    result.push((
                        id,
                        SerializedWorkspaceLocation::Remote(connection_options.clone()),
                        paths,
                    ));
                } else {
                    delete_tasks.push(self.delete_workspace_by_id(id));
                }
                continue;
            }

            let has_wsl_path = if cfg!(windows) {
                paths
                    .paths()
                    .iter()
                    .any(|path| util::paths::WslPath::from_path(path).is_some())
            } else {
                false
            };

            // Delete the workspace if any of the paths are WSL paths.
            // If a local workspace points to WSL, this check will cause us to wait for the
            // WSL VM and file server to boot up. This can block for many seconds.
            // Supported scenarios use remote workspaces.
            if !has_wsl_path && paths.paths().iter().all(|path| path.exists()) {
                // Only show directories in recent projects
                if paths.paths().iter().any(|path| path.is_dir()) {
                    result.push((id, SerializedWorkspaceLocation::Local, paths));
                }
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

        for (paths, window_id, remote_connection_id) in
            self.session_workspaces(last_session_id.to_owned())?
        {
            if let Some(remote_connection_id) = remote_connection_id {
                workspaces.push((
                    SerializedWorkspaceLocation::Remote(
                        self.remote_connection(remote_connection_id)?,
                    ),
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

    pub(crate) async fn toolchains(
        &self,
        workspace_id: WorkspaceId,
    ) -> Result<Vec<(Toolchain, Arc<Path>, Arc<RelPath>)>> {
        self.write(move |this| {
            let mut select = this
                .select_bound(sql!(
                    SELECT
                        name, path, worktree_root_path, relative_worktree_path, language_name, raw_json
                    FROM toolchains
                    WHERE workspace_id = ?
                ))
                .context("select toolchains")?;

            let toolchain: Vec<(String, String, String, String, String, String)> =
                select(workspace_id)?;

            Ok(toolchain
                .into_iter()
                .filter_map(
                    |(name, path, worktree_root_path, relative_worktree_path, language, json)| {
                        Some((
                            Toolchain {
                                name: name.into(),
                                path: path.into(),
                                language_name: LanguageName::new(&language),
                                as_json: serde_json::Value::from_str(&json).ok()?,
                            },
                           Arc::from(worktree_root_path.as_ref()),
                            RelPath::from_proto(&relative_worktree_path).log_err()?,
                        ))
                    },
                )
                .collect())
        })
        .await
    }

    pub async fn set_toolchain(
        &self,
        workspace_id: WorkspaceId,
        worktree_root_path: Arc<Path>,
        relative_worktree_path: Arc<RelPath>,
        toolchain: Toolchain,
    ) -> Result<()> {
        log::debug!(
            "Setting toolchain for workspace, worktree: {worktree_root_path:?}, relative path: {relative_worktree_path:?}, toolchain: {}",
            toolchain.name
        );
        self.write(move |conn| {
            let mut insert = conn
                .exec_bound(sql!(
                    INSERT INTO toolchains(workspace_id, worktree_root_path, relative_worktree_path, language_name, name, path, raw_json) VALUES (?, ?, ?, ?, ?,  ?, ?)
                    ON CONFLICT DO
                    UPDATE SET
                        name = ?5,
                        path = ?6,
                        raw_json = ?7
                ))
                .context("Preparing insertion")?;

            insert((
                workspace_id,
                worktree_root_path.to_string_lossy().into_owned(),
                relative_worktree_path.as_unix_str(),
                toolchain.language_name.as_ref(),
                toolchain.name.as_ref(),
                toolchain.path.as_ref(),
                toolchain.as_json.to_string(),
            ))?;

            Ok(())
        }).await
    }

    pub(crate) async fn save_trusted_worktrees(
        &self,
        trusted_worktrees: HashMap<Option<RemoteHostLocation>, HashSet<PathBuf>>,
    ) -> anyhow::Result<()> {
        use anyhow::Context as _;
        use db::sqlez::statement::Statement;
        use itertools::Itertools as _;

        DB.clear_trusted_worktrees()
            .await
            .context("clearing previous trust state")?;

        let trusted_worktrees = trusted_worktrees
            .into_iter()
            .flat_map(|(host, abs_paths)| {
                abs_paths
                    .into_iter()
                    .map(move |abs_path| (Some(abs_path), host.clone()))
            })
            .collect::<Vec<_>>();
        let mut first_worktree;
        let mut last_worktree = 0_usize;
        for (count, placeholders) in std::iter::once("(?, ?, ?)")
            .cycle()
            .take(trusted_worktrees.len())
            .chunks(MAX_QUERY_PLACEHOLDERS / 3)
            .into_iter()
            .map(|chunk| {
                let mut count = 0;
                let placeholders = chunk
                    .inspect(|_| {
                        count += 1;
                    })
                    .join(", ");
                (count, placeholders)
            })
            .collect::<Vec<_>>()
        {
            first_worktree = last_worktree;
            last_worktree = last_worktree + count;
            let query = format!(
                r#"INSERT INTO trusted_worktrees(absolute_path, user_name, host_name)
VALUES {placeholders};"#
            );

            let trusted_worktrees = trusted_worktrees[first_worktree..last_worktree].to_vec();
            self.write(move |conn| {
                let mut statement = Statement::prepare(conn, query)?;
                let mut next_index = 1;
                for (abs_path, host) in trusted_worktrees {
                    let abs_path = abs_path.as_ref().map(|abs_path| abs_path.to_string_lossy());
                    next_index = statement.bind(
                        &abs_path.as_ref().map(|abs_path| abs_path.as_ref()),
                        next_index,
                    )?;
                    next_index = statement.bind(
                        &host
                            .as_ref()
                            .and_then(|host| Some(host.user_name.as_ref()?.as_str())),
                        next_index,
                    )?;
                    next_index = statement.bind(
                        &host.as_ref().map(|host| host.host_identifier.as_str()),
                        next_index,
                    )?;
                }
                statement.exec()
            })
            .await
            .context("inserting new trusted state")?;
        }
        Ok(())
    }

    pub fn fetch_trusted_worktrees(&self) -> Result<DbTrustedPaths> {
        let trusted_worktrees = DB.trusted_worktrees()?;
        Ok(trusted_worktrees
            .into_iter()
            .filter_map(|(abs_path, user_name, host_name)| {
                let db_host = match (user_name, host_name) {
                    (None, Some(host_name)) => Some(RemoteHostLocation {
                        user_name: None,
                        host_identifier: SharedString::new(host_name),
                    }),
                    (Some(user_name), Some(host_name)) => Some(RemoteHostLocation {
                        user_name: Some(SharedString::new(user_name)),
                        host_identifier: SharedString::new(host_name),
                    }),
                    _ => None,
                };
                Some((db_host, abs_path?))
            })
            .fold(HashMap::default(), |mut acc, (remote_host, abs_path)| {
                acc.entry(remote_host)
                    .or_insert_with(HashSet::default)
                    .insert(abs_path);
                acc
            }))
    }

    query! {
        fn trusted_worktrees() -> Result<Vec<(Option<PathBuf>, Option<String>, Option<String>)>> {
            SELECT absolute_path, user_name, host_name
            FROM trusted_worktrees
        }
    }

    query! {
        pub async fn clear_trusted_worktrees() -> Result<()> {
            DELETE FROM trusted_worktrees
        }
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
    use remote::SshConnectionOptions;
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
            user_toolchains: Default::default(),
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
            user_toolchains: Default::default(),
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
            user_toolchains: Default::default(),
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
                |_, _, _| false,
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
                |_, _, _| false,
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
            user_toolchains: Default::default(),
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
            user_toolchains: Default::default(),
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
            user_toolchains: Default::default(),
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
            user_toolchains: Default::default(),
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
            user_toolchains: Default::default(),
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
            user_toolchains: Default::default(),
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
            user_toolchains: Default::default(),
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
            user_toolchains: Default::default(),
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
            user_toolchains: Default::default(),
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
            user_toolchains: Default::default(),
        };

        let connection_id = db
            .get_or_create_remote_connection(RemoteConnectionOptions::Ssh(SshConnectionOptions {
                host: "my-host".into(),
                port: Some(1234),
                ..Default::default()
            }))
            .await
            .unwrap();

        let workspace_5 = SerializedWorkspace {
            id: WorkspaceId(5),
            paths: PathList::default(),
            location: SerializedWorkspaceLocation::Remote(
                db.remote_connection(connection_id).unwrap(),
            ),
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            breakpoints: Default::default(),
            session_id: Some("session-id-2".to_owned()),
            window_id: Some(50),
            user_toolchains: Default::default(),
        };

        let workspace_6 = SerializedWorkspace {
            id: WorkspaceId(6),
            paths: PathList::new(&["/tmp6c", "/tmp6b", "/tmp6a"]),
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            breakpoints: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            session_id: Some("session-id-3".to_owned()),
            window_id: Some(60),
            user_toolchains: Default::default(),
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
            PathList::new(&["/tmp6c", "/tmp6b", "/tmp6a"]),
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
            user_toolchains: Default::default(),
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
            user_toolchains: Default::default(),
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
    async fn test_last_session_workspace_locations_remote() {
        let db =
            WorkspaceDb::open_test_db("test_serializing_workspaces_last_session_workspaces_remote")
                .await;

        let remote_connections = [
            ("host-1", "my-user-1"),
            ("host-2", "my-user-2"),
            ("host-3", "my-user-3"),
            ("host-4", "my-user-4"),
        ]
        .into_iter()
        .map(|(host, user)| async {
            let options = RemoteConnectionOptions::Ssh(SshConnectionOptions {
                host: host.into(),
                username: Some(user.to_string()),
                ..Default::default()
            });
            db.get_or_create_remote_connection(options.clone())
                .await
                .unwrap();
            options
        })
        .collect::<Vec<_>>();

        let remote_connections = futures::future::join_all(remote_connections).await;

        let workspaces = [
            (1, remote_connections[0].clone(), 9),
            (2, remote_connections[1].clone(), 5),
            (3, remote_connections[2].clone(), 8),
            (4, remote_connections[3].clone(), 2),
        ]
        .into_iter()
        .map(|(id, remote_connection, window_id)| SerializedWorkspace {
            id: WorkspaceId(id),
            paths: PathList::default(),
            location: SerializedWorkspaceLocation::Remote(remote_connection),
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            session_id: Some("one-session".to_owned()),
            breakpoints: Default::default(),
            window_id: Some(window_id),
            user_toolchains: Default::default(),
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
                SerializedWorkspaceLocation::Remote(remote_connections[3].clone()),
                PathList::default()
            )
        );
        assert_eq!(
            have[1],
            (
                SerializedWorkspaceLocation::Remote(remote_connections[2].clone()),
                PathList::default()
            )
        );
        assert_eq!(
            have[2],
            (
                SerializedWorkspaceLocation::Remote(remote_connections[1].clone()),
                PathList::default()
            )
        );
        assert_eq!(
            have[3],
            (
                SerializedWorkspaceLocation::Remote(remote_connections[0].clone()),
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
            .get_or_create_remote_connection(RemoteConnectionOptions::Ssh(SshConnectionOptions {
                host: host.clone().into(),
                port,
                username: user.clone(),
                ..Default::default()
            }))
            .await
            .unwrap();

        // Test that calling the function again with the same parameters returns the same project
        let same_connection = db
            .get_or_create_remote_connection(RemoteConnectionOptions::Ssh(SshConnectionOptions {
                host: host.clone().into(),
                port,
                username: user.clone(),
                ..Default::default()
            }))
            .await
            .unwrap();

        assert_eq!(connection_id, same_connection);

        // Test with different parameters
        let host2 = "otherexample.com".to_string();
        let port2 = None;
        let user2 = Some("otheruser".to_string());

        let different_connection = db
            .get_or_create_remote_connection(RemoteConnectionOptions::Ssh(SshConnectionOptions {
                host: host2.clone().into(),
                port: port2,
                username: user2.clone(),
                ..Default::default()
            }))
            .await
            .unwrap();

        assert_ne!(connection_id, different_connection);
    }

    #[gpui::test]
    async fn test_get_or_create_ssh_project_with_null_user() {
        let db = WorkspaceDb::open_test_db("test_get_or_create_ssh_project_with_null_user").await;

        let (host, port, user) = ("example.com".to_string(), None, None);

        let connection_id = db
            .get_or_create_remote_connection(RemoteConnectionOptions::Ssh(SshConnectionOptions {
                host: host.clone().into(),
                port,
                username: None,
                ..Default::default()
            }))
            .await
            .unwrap();

        let same_connection_id = db
            .get_or_create_remote_connection(RemoteConnectionOptions::Ssh(SshConnectionOptions {
                host: host.clone().into(),
                port,
                username: user.clone(),
                ..Default::default()
            }))
            .await
            .unwrap();

        assert_eq!(connection_id, same_connection_id);
    }

    #[gpui::test]
    async fn test_get_remote_connections() {
        let db = WorkspaceDb::open_test_db("test_get_remote_connections").await;

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
                db.get_or_create_remote_connection(RemoteConnectionOptions::Ssh(
                    SshConnectionOptions {
                        host: host.clone().into(),
                        port: *port,
                        username: user.clone(),
                        ..Default::default()
                    },
                ))
                .await
                .unwrap(),
            );
        }

        let stored_connections = db.remote_connections().unwrap();
        assert_eq!(
            stored_connections,
            [
                (
                    ids[0],
                    RemoteConnectionOptions::Ssh(SshConnectionOptions {
                        host: "example.com".into(),
                        port: None,
                        username: None,
                        ..Default::default()
                    }),
                ),
                (
                    ids[1],
                    RemoteConnectionOptions::Ssh(SshConnectionOptions {
                        host: "anotherexample.com".into(),
                        port: Some(123),
                        username: Some("user2".into()),
                        ..Default::default()
                    }),
                ),
                (
                    ids[2],
                    RemoteConnectionOptions::Ssh(SshConnectionOptions {
                        host: "yetanother.com".into(),
                        port: Some(345),
                        username: None,
                        ..Default::default()
                    }),
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

    #[gpui::test]
    async fn test_empty_workspace_window_bounds() {
        zlog::init_test();

        let db = WorkspaceDb::open_test_db("test_empty_workspace_window_bounds").await;
        let id = db.next_id().await.unwrap();

        // Create a workspace with empty paths (empty workspace)
        let empty_paths: &[&str] = &[];
        let display_uuid = Uuid::new_v4();
        let window_bounds = SerializedWindowBounds(WindowBounds::Windowed(Bounds {
            origin: point(px(100.0), px(200.0)),
            size: size(px(800.0), px(600.0)),
        }));

        let workspace = SerializedWorkspace {
            id,
            paths: PathList::new(empty_paths),
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: None,
            display: None,
            docks: Default::default(),
            breakpoints: Default::default(),
            centered_layout: false,
            session_id: None,
            window_id: None,
            user_toolchains: Default::default(),
        };

        // Save the workspace (this creates the record with empty paths)
        db.save_workspace(workspace.clone()).await;

        // Save window bounds separately (as the actual code does via set_window_open_status)
        db.set_window_open_status(id, window_bounds, display_uuid)
            .await
            .unwrap();

        // Retrieve it using empty paths
        let retrieved = db.workspace_for_roots(empty_paths).unwrap();

        // Verify window bounds were persisted
        assert_eq!(retrieved.id, id);
        assert!(retrieved.window_bounds.is_some());
        assert_eq!(retrieved.window_bounds.unwrap().0, window_bounds.0);
        assert!(retrieved.display.is_some());
        assert_eq!(retrieved.display.unwrap(), display_uuid);
    }
}
