pub mod model;

use std::{
    borrow::Cow,
    collections::BTreeMap,
    path::{Path, PathBuf},
    str::FromStr,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use chrono::{DateTime, NaiveDateTime, Utc};
use fs::Fs;

use anyhow::{Context as _, Result, bail};
use collections::{HashMap, HashSet, IndexSet};
use db::{
    kvp::KeyValueStore,
    query,
    sqlez::{connection::Connection, domain::Domain},
    sqlez_macros::sql,
};
use gpui::{Axis, Bounds, WindowBounds, WindowId, point, size};
use project::{
    ProjectGroupKey,
    bookmark_store::SerializedBookmark,
    debugger::breakpoint_store::{BreakpointState, SourceBreakpoint},
    trusted_worktrees::{DbTrustedPaths, RemoteHostLocation},
};

use language::{LanguageName, Toolchain, ToolchainScope};
#[cfg(any(test, feature = "test-support"))]
use remote::MockConnectionOptions;
use remote::{
    DockerConnectionOptions, RemoteConnectionIdentity, RemoteConnectionOptions,
    SshConnectionOptions, WslConnectionOptions, remote_connection_identity,
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

use self::model::{DockStructure, SerializedWorkspaceLocation, SessionWorkspace};

// https://www.sqlite.org/limits.html
// > <..> the maximum value of a host parameter number is SQLITE_MAX_VARIABLE_NUMBER,
// > which defaults to <..> 32766 for SQLite versions after 3.32.0.
const MAX_QUERY_PLACEHOLDERS: usize = 32000;

fn parse_timestamp(text: &str) -> DateTime<Utc> {
    NaiveDateTime::parse_from_str(text, "%Y-%m-%d %H:%M:%S")
        .map(|naive| naive.and_utc())
        .unwrap_or_else(|_| Utc::now())
}

fn contains_wsl_path(paths: &PathList) -> bool {
    cfg!(windows)
        && paths
            .paths()
            .iter()
            .any(|path| util::paths::WslPath::from_path(path).is_some())
}

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

pub fn read_default_window_bounds(kvp: &KeyValueStore) -> Option<(Uuid, WindowBounds)> {
    let json_str = kvp
        .read_kvp(DEFAULT_WINDOW_BOUNDS_KEY)
        .log_err()
        .flatten()?;

    let (display_uuid, persisted) =
        serde_json::from_str::<(Uuid, WindowBoundsJson)>(&json_str).ok()?;
    Some((display_uuid, persisted.into()))
}

pub async fn write_default_window_bounds(
    kvp: &KeyValueStore,
    bounds: WindowBounds,
    display_uuid: Uuid,
) -> anyhow::Result<()> {
    let persisted = WindowBoundsJson::from(bounds);
    let json_str = serde_json::to_string(&(display_uuid, persisted))?;
    kvp.write_kvp(DEFAULT_WINDOW_BOUNDS_KEY.to_string(), json_str)
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

fn read_multi_workspace_state(window_id: WindowId, cx: &App) -> model::MultiWorkspaceState {
    let kvp = KeyValueStore::global(cx);
    kvp.scoped("multi_workspace_state")
        .read(&window_id.as_u64().to_string())
        .log_err()
        .flatten()
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default()
}

pub fn read_serialized_multi_workspaces(
    session_workspaces: Vec<model::SessionWorkspace>,
    cx: &App,
) -> Vec<model::SerializedMultiWorkspace> {
    let mut window_groups: Vec<Vec<model::SessionWorkspace>> = Vec::new();
    let mut window_id_to_group: HashMap<WindowId, usize> = HashMap::default();

    for session_workspace in session_workspaces {
        match session_workspace.window_id {
            Some(window_id) => {
                let group_index = *window_id_to_group.entry(window_id).or_insert_with(|| {
                    window_groups.push(Vec::new());
                    window_groups.len() - 1
                });
                window_groups[group_index].push(session_workspace);
            }
            None => {
                window_groups.push(vec![session_workspace]);
            }
        }
    }

    window_groups
        .into_iter()
        .filter_map(|mut group| {
            let window_id = group.first().and_then(|sw| sw.window_id);
            let state = window_id
                .map(|wid| read_multi_workspace_state(wid, cx))
                .unwrap_or_default();
            let active_workspace = state
                .active_workspace_id
                .and_then(|id| group.iter().position(|ws| ws.workspace_id == id))
                // If the persisted active workspace can't be matched (e.g. its
                // pointer was lost or its row was pruned), fall back to the
                // first workspace that actually has paths rather than blindly
                // taking index 0, so a stray scratch/empty workspace isn't
                // restored as the focused window. Only if none have paths do we
                // fall back to the first entry.
                .or_else(|| group.iter().position(|ws| !ws.paths.is_empty()))
                .or(Some(0))
                .filter(|index| *index < group.len())
                .map(|index| group.remove(index))?;
            let workspace_ids = std::iter::once(active_workspace.workspace_id)
                .chain(group.iter().map(|workspace| workspace.workspace_id))
                .collect();
            let scratch_workspaces = group
                .into_iter()
                .filter(|workspace| {
                    workspace.location == SerializedWorkspaceLocation::Local
                        && workspace.paths.is_empty()
                })
                .map(|workspace| workspace.workspace_id)
                .collect::<Vec<_>>();
            Some(model::SerializedMultiWorkspace {
                active_workspace,
                workspace_ids,
                scratch_workspaces,
                state,
            })
        })
        .collect()
}

const DEFAULT_DOCK_STATE_KEY: &str = "default_dock_state";

pub fn read_default_dock_state(kvp: &KeyValueStore) -> Option<DockStructure> {
    let json_str = kvp.read_kvp(DEFAULT_DOCK_STATE_KEY).log_err().flatten()?;

    serde_json::from_str::<DockStructure>(&json_str).ok()
}

pub async fn write_default_dock_state(
    kvp: &KeyValueStore,
    docks: DockStructure,
) -> anyhow::Result<()> {
    let json_str = serde_json::to_string(&docks)?;
    kvp.write_kvp(DEFAULT_DOCK_STATE_KEY.to_string(), json_str)
        .await?;
    Ok(())
}

#[derive(Debug)]
pub struct Bookmark {
    pub row: u32,
    pub label: String,
}

impl sqlez::bindable::StaticColumnCount for Bookmark {
    fn column_count() -> usize {
        // row, label
        2
    }
}

impl sqlez::bindable::Bind for Bookmark {
    fn bind(
        &self,
        statement: &sqlez::statement::Statement,
        start_index: i32,
    ) -> anyhow::Result<i32> {
        let next_index = statement.bind(&self.row, start_index)?;
        statement.bind(&self.label, next_index)
    }
}

impl Column for Bookmark {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let row = statement
            .column_int(start_index)
            .with_context(|| format!("Failed to read bookmark at index {start_index}"))?
            as u32;

        let (label, next_index) = String::column(statement, start_index + 1)?;

        Ok((Bookmark { row, label }, next_index))
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

pub(crate) struct WorkspaceCloseSnapshot {
    pub(crate) workspace_ids: Vec<WorkspaceId>,
    bindings: Vec<(WorkspaceId, Option<String>, Option<u64>)>,
    checkpoints: Vec<(String, String)>,
    project_groups: Vec<ProjectGroupKey>,
}

pub(crate) struct WorkspaceSerializationGuard {
    pub(crate) restoration_cancelled: Option<Arc<AtomicBool>>,
    pub(crate) serialization_fence: Arc<AtomicBool>,
}

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
        sql!(
            ALTER TABLE remote_connections ADD COLUMN remote_env TEXT;
        ),
        sql!(
            CREATE TABLE bookmarks (
                workspace_id INTEGER NOT NULL,
                path TEXT NOT NULL,
                row INTEGER NOT NULL,
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
                ON UPDATE CASCADE
            );
        ),
        sql!(
            ALTER TABLE workspaces ADD COLUMN identity_paths TEXT;
            ALTER TABLE workspaces ADD COLUMN identity_paths_order TEXT;
        ),
        sql!(
            ALTER TABLE bookmarks ADD COLUMN label TEXT NOT NULL DEFAULT "";
        ),
        sql!(
            CREATE TABLE recent_navigation_history (
                workspace_id INTEGER NOT NULL,
                path BLOB NOT NULL,
                position INTEGER NOT NULL,
                PRIMARY KEY (workspace_id, path),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
                ON UPDATE CASCADE
            ) STRICT;
        ),
        sql!(
            ALTER TABLE workspaces ADD COLUMN native_window_state BLOB;
        ),
        sql!(
            DROP INDEX ix_workspaces_location;
            CREATE INDEX ix_workspaces_location ON workspaces(remote_connection_id, paths);
        ),
        sql!(
            CREATE TABLE item_persistence_id_sequence (
                singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
                last_id INTEGER NOT NULL CHECK (last_id >= 0)
            ) STRICT;
        ),
    ];

    // Allow recovering from bad migration that was initially shipped to nightly
    // when introducing the ssh_connections table.
    fn should_allow_migration_change(_index: usize, old: &str, new: &str) -> bool {
        old.starts_with("CREATE TABLE ssh_connections")
            && new.starts_with("CREATE TABLE ssh_connections")
    }
}

db::static_connection!(WorkspaceDb, [KeyValueStore]);

const WORKSPACE_RESTORE_NAMESPACE: &str = "unfinished_workspace_restores";

impl WorkspaceDb {
    pub async fn next_item_id(&self) -> Result<ItemId> {
        self.write(|connection| {
            connection.with_savepoint("allocate_item_persistence_id", || {
                let last_id = connection.select_row::<i64>(
                    "SELECT last_id FROM item_persistence_id_sequence WHERE singleton = 1",
                )?()?;
                let last_id = if let Some(last_id) = last_id {
                    last_id
                } else {
                    let tables = connection.select::<String>(
                        "SELECT schema.name FROM sqlite_schema AS schema
                         WHERE schema.type = 'table' AND EXISTS (
                             SELECT 1 FROM pragma_table_info(schema.name) AS column
                             WHERE column.name = 'item_id'
                         )",
                    )?()?;
                    let mut last_id = 0;
                    for table in tables {
                        let table = table.replace('"', "\"\"");
                        let maximum = connection.select_row::<i64>(&format!(
                            "SELECT COALESCE(MAX(item_id), 0) FROM \"{table}\""
                        ))?()?
                        .context("Missing maximum persisted item ID")?;
                        last_id = last_id.max(maximum);
                    }
                    last_id
                };
                let item_id = last_id
                    .checked_add(1)
                    .context("Persisted item IDs exhausted")?;
                connection.exec_bound(
                    "INSERT INTO item_persistence_id_sequence (singleton, last_id) VALUES (1, ?)
                     ON CONFLICT (singleton) DO UPDATE SET last_id = excluded.last_id",
                )?(item_id)?;
                Ok(item_id as ItemId)
            })
        })
        .await
    }

    pub fn unfinished_workspace_restores(&self) -> Result<Vec<model::SerializedMultiWorkspace>> {
        let values = self
            .select_bound::<_, String>("SELECT value FROM scoped_kv_store WHERE namespace = ?")?(
            WORKSPACE_RESTORE_NAMESPACE,
        )?;
        let mut checkpoints = values
            .into_iter()
            .map(|value| Self::read_workspace_restore_checkpoint(self, &value))
            .collect::<Result<Vec<_>>>()?;
        checkpoints.sort_by_key(|checkpoint| checkpoint.order);
        let mut workspaces = Vec::new();
        for checkpoint in checkpoints {
            let Some(&first_existing) = checkpoint.workspace_ids.first() else {
                continue;
            };
            let mut workspace = checkpoint.into_workspace();
            let active_id = workspace
                .state
                .active_workspace_id
                .filter(|id| workspace.workspace_ids.contains(id))
                .or_else(|| {
                    workspace
                        .workspace_ids
                        .contains(&workspace.active_workspace.workspace_id)
                        .then_some(workspace.active_workspace.workspace_id)
                })
                .unwrap_or(first_existing);
            if active_id != workspace.active_workspace.workspace_id {
                workspace.active_workspace = self.session_workspace_for_id(active_id)?;
                workspace.state.active_workspace_id = Some(active_id);
            }
            workspaces.push(workspace);
        }
        Ok(workspaces)
    }

    pub(crate) async fn checkpoint_workspace_restores(
        &self,
        workspaces: &[model::SerializedMultiWorkspace],
    ) -> Result<()> {
        let checkpoints = workspaces
            .iter()
            .enumerate()
            .map(|(order, workspace)| {
                let checkpoint = model::WorkspaceRestoreCheckpoint::new(workspace, order);
                Ok((
                    workspace.active_workspace.workspace_id.0.to_string(),
                    serde_json::to_string(&checkpoint)?,
                ))
            })
            .collect::<Result<Vec<_>>>()?;
        self.write(move |connection| {
            connection.with_savepoint("checkpoint_workspace_restores", || {
                connection.exec_bound("DELETE FROM scoped_kv_store WHERE namespace = ?")?(
                    WORKSPACE_RESTORE_NAMESPACE,
                )?;
                for (key, value) in checkpoints {
                    connection.exec_bound(
                        "INSERT INTO scoped_kv_store(namespace, key, value) VALUES (?, ?, ?)",
                    )?((WORKSPACE_RESTORE_NAMESPACE, key, value))?;
                }
                Ok(())
            })
        })
        .await
    }

    pub(crate) fn workspace_ids_for_close(
        &self,
        restore_key: Option<WorkspaceId>,
        workspace_ids: &[WorkspaceId],
    ) -> Result<Vec<WorkspaceId>> {
        let mut ids = workspace_ids.iter().copied().collect::<HashSet<_>>();
        if let Some(restore_key) = restore_key
            && let Some(value) =
                self.select_row_bound::<_, String>(
                    "SELECT value FROM scoped_kv_store WHERE namespace = ? AND key = ?",
                )?((WORKSPACE_RESTORE_NAMESPACE, restore_key.0.to_string()))?
        {
            ids.extend(Self::read_workspace_restore_checkpoint(self, &value)?.workspace_ids);
        }
        let mut ids = ids.into_iter().collect::<Vec<_>>();
        ids.sort_unstable();
        Ok(ids)
    }

    pub(crate) fn workspace_item_ids(&self, id: WorkspaceId) -> Result<Vec<ItemId>> {
        self.session_workspace_for_id(id)?;
        self.select_bound(sql! {
            SELECT item_id FROM items WHERE workspace_id = ? ORDER BY item_id
        })?(id)
    }

    pub(crate) async fn close_workspaces(
        &self,
        restore_key: Option<WorkspaceId>,
        workspace_ids: Vec<WorkspaceId>,
        project_groups: Vec<ProjectGroupKey>,
        serialization_fence: Arc<AtomicBool>,
        unchanged: impl Fn() -> Result<()> + Send + 'static,
    ) -> Result<WorkspaceCloseSnapshot> {
        self.write(move |connection| {
            let snapshot = connection.with_savepoint("close_workspaces", || {
                unchanged()?;
                let affected_ids = workspace_ids.iter().copied().collect::<HashSet<_>>();
                let checkpoints = if let Some(restore_key) = restore_key {
                    let key = restore_key.0.to_string();
                    if let Some(value) = connection.select_row_bound::<_, String>(
                        "SELECT value FROM scoped_kv_store WHERE namespace = ? AND key = ?",
                    )?((WORKSPACE_RESTORE_NAMESPACE, key.clone()))? {
                        let checkpoint = Self::read_workspace_restore_checkpoint(connection, &value)?;
                        anyhow::ensure!(
                            checkpoint.workspace_ids.iter().all(|id| affected_ids.contains(id)),
                            "Workspace membership changed while preparing to close"
                        );
                        connection.exec_bound(
                            "DELETE FROM scoped_kv_store WHERE namespace = ? AND key = ?",
                        )?((WORKSPACE_RESTORE_NAMESPACE, key.clone()))?;
                        vec![(key, value)]
                    } else {
                        Vec::new()
                    }
                } else {
                    Self::remove_workspace_restore_members_on_connection(
                        connection,
                        &workspace_ids,
                        &project_groups,
                    )?
                };
                let mut bindings = Vec::new();
                for id in &affected_ids {
                    if let Some((session_id, window_id)) = connection
                        .select_row_bound::<_, (Option<String>, Option<u64>)>(
                            "SELECT session_id, window_id FROM workspaces WHERE workspace_id = ?",
                        )?(*id)?
                    {
                        bindings.push((*id, session_id, window_id));
                    }
                    connection.exec_bound(
                        "UPDATE workspaces SET session_id = NULL, window_id = NULL WHERE workspace_id = ?",
                    )?(*id)?;
                }
                unchanged()?;
                Ok(WorkspaceCloseSnapshot {
                    workspace_ids: affected_ids.into_iter().collect(),
                    bindings,
                    checkpoints,
                    project_groups,
                })
            })?;
            serialization_fence.store(true, Ordering::Relaxed);
            Ok(snapshot)
        })
        .await
    }

    pub(crate) async fn restore_workspace_close(
        &self,
        snapshot: WorkspaceCloseSnapshot,
        retained_ids: Vec<WorkspaceId>,
    ) -> Result<()> {
        self.write(move |connection| {
            connection.with_savepoint("restore_workspace_close", || {
                let mut restored_ids = HashSet::default();
                for (id, session_id, window_id) in snapshot.bindings {
                    if !retained_ids.contains(&id) {
                        continue;
                    }
                    let current = connection.select_row_bound::<_, (Option<String>, Option<u64>)>(
                        "SELECT session_id, window_id FROM workspaces WHERE workspace_id = ?",
                    )?(id)?;
                    if current == Some((None, None))
                        || current == Some((session_id.clone(), window_id))
                    {
                        connection.exec_bound(
                            "UPDATE workspaces SET session_id = ?2, window_id = ?3 WHERE workspace_id = ?1",
                        )?((id, session_id, window_id))?;
                        restored_ids.insert(id);
                    }
                }
                for (key, value) in snapshot.checkpoints {
                    let mut previous = Self::read_workspace_restore_checkpoint(connection, &value)?;
                    let removed = previous.workspace_ids.iter().copied()
                        .filter(|id| !restored_ids.contains(id))
                        .collect::<Vec<_>>();
                    previous.remove_members(&removed, &[]);
                    if previous.workspace_ids.is_empty() {
                        continue;
                    }
                    let order = previous.order;
                    let previous = previous.into_workspace();
                    let restored = if let Some(current) = connection.select_row_bound::<_, String>(
                        "SELECT value FROM scoped_kv_store WHERE namespace = ? AND key = ?",
                    )?((WORKSPACE_RESTORE_NAMESPACE, key.clone()))? {
                        let mut current = Self::read_workspace_restore_checkpoint(connection, &current)?
                            .into_workspace();
                        for id in previous.workspace_ids {
                            if !current.workspace_ids.contains(&id) {
                                current.workspace_ids.push(id);
                            }
                        }
                        for id in previous.scratch_workspaces {
                            if !current.scratch_workspaces.contains(&id) {
                                current.scratch_workspaces.push(id);
                            }
                        }
                        current.state.restore_project_groups(&previous.state, &snapshot.project_groups);
                        if current.state.active_workspace_id.is_none() {
                            current.state.active_workspace_id = previous.state.active_workspace_id;
                        }
                        current
                    } else {
                        previous
                    };
                    let checkpoint = model::WorkspaceRestoreCheckpoint::new(&restored, order);
                    connection.exec_bound(
                        "INSERT OR REPLACE INTO scoped_kv_store(namespace, key, value) VALUES (?, ?, ?)",
                    )?((WORKSPACE_RESTORE_NAMESPACE, key, serde_json::to_string(&checkpoint)?))?;
                }
                Ok(())
            })
        })
        .await
    }

    pub(crate) async fn save_multi_workspace(
        &self,
        window_id: WindowId,
        state: model::MultiWorkspaceState,
        workspace_ids: Vec<WorkspaceId>,
        session_id: Option<String>,
        cancelled: Arc<AtomicBool>,
        serialization_fence: Arc<AtomicBool>,
        restore_key: Option<WorkspaceId>,
        completing_restore: bool,
    ) -> Result<bool> {
        let serialized_state = serde_json::to_string(&state)?;
        self.write(move |connection| {
            connection.with_savepoint_rollback("save_multi_workspace", || {
                if cancelled.load(Ordering::Relaxed)
                    || serialization_fence.load(Ordering::Relaxed)
                {
                    return Ok(None);
                }
                for id in &workspace_ids {
                    connection.exec_bound("UPDATE workspaces SET session_id = ?2, window_id = ?3 WHERE workspace_id = ?1")?((*id, session_id.clone(), Some(window_id.as_u64())))?;
                }
                connection.exec_bound("INSERT OR REPLACE INTO scoped_kv_store(namespace, key, value) VALUES (?, ?, ?)")?(("multi_workspace_state", window_id.as_u64().to_string(), serialized_state))?;
                if let Some(restore_key) = restore_key {
                    if let Some(value) = connection.select_row_bound::<_, String>(
                        "SELECT value FROM scoped_kv_store WHERE namespace = ? AND key = ?",
                    )?((WORKSPACE_RESTORE_NAMESPACE, restore_key.0.to_string()))? {
                        let checkpoint = Self::read_workspace_restore_checkpoint(connection, &value)?;
                        if completing_restore {
                            anyhow::ensure!(checkpoint.workspace_ids.iter().all(|id| workspace_ids.contains(id)),
                                "Workspace restoration has unflushed members");
                            connection.exec_bound("DELETE FROM scoped_kv_store WHERE namespace = ? AND key = ?")?((WORKSPACE_RESTORE_NAMESPACE, restore_key.0.to_string()))?;
                        } else {
                            let order = checkpoint.order;
                            let mut saved = checkpoint.into_workspace();
                            for id in workspace_ids {
                                if !saved.workspace_ids.contains(&id) {
                                    saved.workspace_ids.push(id);
                                }
                            }
                            if let Some(active_id) = state.active_workspace_id {
                                anyhow::ensure!(saved.workspace_ids.contains(&active_id),
                                    "Active workspace is missing from window membership");
                                saved.state.active_workspace_id = Some(active_id);
                            }
                            let previous_groups = saved.state.project_groups.iter().cloned()
                                .map(ProjectGroupKey::from).collect::<Vec<_>>();
                            let mut merged_state = state.clone();
                            merged_state.restore_project_groups(&saved.state, &previous_groups);
                            saved.state.project_groups = merged_state.project_groups;
                            saved.scratch_workspaces = connection.select_bound::<_, WorkspaceId>(
                                "SELECT workspace_id FROM workspaces
                                 WHERE workspace_id IN (SELECT value FROM json_each(?1))
                                 AND remote_connection_id IS NULL AND COALESCE(paths, '') = ''
                                 AND workspace_id IS NOT ?2 ORDER BY workspace_id",
                            )?((serde_json::to_string(&saved.workspace_ids)?, saved.state.active_workspace_id))?;
                            let checkpoint = model::WorkspaceRestoreCheckpoint::new(&saved, order);
                            connection.exec_bound(
                                "UPDATE scoped_kv_store SET value = ?3 WHERE namespace = ?1 AND key = ?2",
                            )?((WORKSPACE_RESTORE_NAMESPACE, restore_key.0.to_string(), serde_json::to_string(&checkpoint)?))?;
                        }
                    }
                }
                Ok((!cancelled.load(Ordering::Relaxed)
                    && !serialization_fence.load(Ordering::Relaxed)).then_some(()))
            }).map(|result| result.is_some())
        }).await
    }

    #[cfg(test)]
    pub(crate) fn workspace_restore_pending(&self, restore_key: WorkspaceId) -> Result<bool> {
        Ok(self.select_row_bound::<_, String>(
            "SELECT key FROM scoped_kv_store WHERE namespace = ? AND key = ?",
        )?((WORKSPACE_RESTORE_NAMESPACE, restore_key.0.to_string()))?
        .is_some())
    }

    pub(crate) fn workspace_exists(&self, workspace_id: WorkspaceId) -> Result<bool> {
        Ok(self.select_row_bound::<_, WorkspaceId>(
            "SELECT workspace_id FROM workspaces WHERE workspace_id = ?",
        )?(workspace_id)?
        .is_some())
    }

    pub fn session_workspace_for_id(&self, workspace_id: WorkspaceId) -> Result<SessionWorkspace> {
        let (paths, order, window_id, remote_connection_id) = self.select_row_bound::<_, (Option<String>, Option<String>, Option<u64>, Option<u64>)>(
            "SELECT paths, paths_order, window_id, remote_connection_id FROM workspaces WHERE workspace_id = ?",
        )?(workspace_id)?.context("Workspace row is missing")?;
        Ok(SessionWorkspace {
            workspace_id,
            location: match remote_connection_id {
                Some(id) => SerializedWorkspaceLocation::Remote(
                    self.remote_connection(RemoteConnectionId(id))?,
                ),
                None => SerializedWorkspaceLocation::Local,
            },
            paths: PathList::deserialize(&SerializedPathList {
                paths: paths.unwrap_or_default(),
                order: order.unwrap_or_default(),
            }),
            window_id: window_id.map(WindowId::from),
        })
    }

    fn read_workspace_restore_checkpoint(
        connection: &Connection,
        value: &str,
    ) -> Result<model::WorkspaceRestoreCheckpoint> {
        let mut checkpoint = serde_json::from_str::<model::WorkspaceRestoreCheckpoint>(value)?;
        let missing_ids = connection.select_bound::<_, WorkspaceId>(
            "SELECT member.value FROM json_each(?, '$.workspace_ids') AS member
             WHERE NOT EXISTS (SELECT 1 FROM workspaces WHERE workspace_id = member.value)",
        )?(value)?;
        checkpoint.remove_members(&missing_ids, &[]);
        Ok(checkpoint)
    }

    fn remove_workspace_restore_members_on_connection(
        connection: &Connection,
        workspace_ids: &[WorkspaceId],
        groups: &[ProjectGroupKey],
    ) -> Result<Vec<(String, String)>> {
        let values = connection.select_bound::<_, (String, String)>(
            "SELECT key, value FROM scoped_kv_store WHERE namespace = ?",
        )?(WORKSPACE_RESTORE_NAMESPACE)?;
        let mut delete_checkpoint =
            connection.exec_bound("DELETE FROM scoped_kv_store WHERE namespace = ? AND key = ?")?;
        let mut update_checkpoint = connection.exec_bound(
            "UPDATE scoped_kv_store SET value = ?3 WHERE namespace = ?1 AND key = ?2",
        )?;
        let mut checkpoints = Vec::new();
        for (key, value) in values {
            let mut checkpoint = Self::read_workspace_restore_checkpoint(connection, &value)?;
            if !checkpoint.workspace_ids.is_empty() {
                if !checkpoint
                    .workspace_ids
                    .iter()
                    .any(|id| workspace_ids.contains(id))
                {
                    continue;
                }
                checkpoints.push((key.clone(), value));
            }
            checkpoint.remove_members(workspace_ids, groups);
            if checkpoint.workspace_ids.is_empty() {
                delete_checkpoint((WORKSPACE_RESTORE_NAMESPACE, key))?;
            } else {
                update_checkpoint((
                    WORKSPACE_RESTORE_NAMESPACE,
                    key,
                    serde_json::to_string(&checkpoint)?,
                ))?;
            }
        }
        Ok(checkpoints)
    }

    #[cfg(test)]
    fn workspace_for_roots<P: AsRef<Path>>(
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
        let id = self
            .workspace_ids_for_roots(worktree_roots, remote_connection_id)
            .log_err()?
            .into_iter()
            .next()?;
        self.workspace_for_id(id)
    }

    pub(crate) fn workspace_ids_for_roots<P: AsRef<Path>>(
        &self,
        worktree_roots: &[P],
        remote_connection_id: Option<RemoteConnectionId>,
    ) -> Result<Vec<WorkspaceId>> {
        let paths = PathList::new(worktree_roots);
        if paths.is_empty() && remote_connection_id.is_none() {
            return Ok(Vec::new());
        }
        self.select_bound(sql! {
            SELECT workspace_id FROM workspaces
            WHERE paths IS ? AND remote_connection_id IS ?
            ORDER BY timestamp DESC, workspace_id DESC
        })?((
            paths.serialize().paths,
            remote_connection_id.map(|id| id.0 as i32),
        ))
    }

    /// Returns the workspace with the given ID, loading all associated data.
    pub(crate) fn workspace_for_id(
        &self,
        workspace_id: WorkspaceId,
    ) -> Option<SerializedWorkspace> {
        let (
            paths,
            paths_order,
            identity_paths,
            identity_paths_order,
            window_bounds,
            display,
            centered_layout,
            docks,
            window_id,
            remote_connection_id,
        ): (
            String,
            String,
            Option<String>,
            Option<String>,
            Option<SerializedWindowBounds>,
            Option<Uuid>,
            Option<bool>,
            DockStructure,
            Option<u64>,
            Option<i32>,
        ) = self
            .select_row_bound(sql! {
                SELECT
                    paths,
                    paths_order,
                    identity_paths,
                    identity_paths_order,
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
                    window_id,
                    remote_connection_id
                FROM workspaces
                WHERE workspace_id = ?
            })
            .and_then(|mut prepared_statement| (prepared_statement)(workspace_id))
            .context("No workspace found for id")
            .warn_on_err()
            .flatten()?;

        let paths = PathList::deserialize(&SerializedPathList {
            paths,
            order: paths_order,
        });
        let identity_paths = identity_paths.map(|paths| {
            PathList::deserialize(&SerializedPathList {
                paths,
                order: identity_paths_order.unwrap_or_default(),
            })
        });

        let remote_connection_id = remote_connection_id.map(|id| RemoteConnectionId(id as u64));
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
            identity_paths,
            center_group: self
                .get_center_pane_group(workspace_id)
                .context("Getting center group")
                .log_err()?,
            window_bounds,
            centered_layout: centered_layout.unwrap_or(false),
            display,
            docks,
            session_id: None,
            bookmarks: self.bookmarks(workspace_id),
            breakpoints: self.breakpoints(workspace_id),
            window_id,
            user_toolchains: self.user_toolchains(workspace_id, remote_connection_id),
            recent_navigation_history: self.recent_navigation_history(workspace_id),
        })
    }

    fn recent_navigation_history(&self, workspace_id: WorkspaceId) -> Vec<PathBuf> {
        self.select_bound(sql!(
            SELECT path
            FROM recent_navigation_history
            WHERE workspace_id = ?
            ORDER BY position
        ))
        .and_then(|mut statement| statement(workspace_id))
        .context("Loading recent navigation history")
        .log_err()
        .unwrap_or_default()
    }

    fn bookmarks(&self, workspace_id: WorkspaceId) -> BTreeMap<Arc<Path>, Vec<SerializedBookmark>> {
        let bookmarks: Result<Vec<(PathBuf, Bookmark)>> = self
            .select_bound(sql! {
                SELECT path, row, label
                FROM bookmarks
                WHERE workspace_id = ?
                ORDER BY path, row
            })
            .and_then(|mut prepared_statement| (prepared_statement)(workspace_id));

        match bookmarks {
            Ok(bookmarks) => {
                if bookmarks.is_empty() {
                    log::debug!("Bookmarks are empty after querying database for them");
                }

                let mut map: BTreeMap<_, Vec<_>> = BTreeMap::default();

                for (path, bookmark) in bookmarks {
                    let path: Arc<Path> = path.into();
                    map.entry(path.clone())
                        .or_default()
                        .push(SerializedBookmark {
                            row: bookmark.row,
                            label: bookmark.label,
                        })
                }

                map
            }
            Err(e) => {
                log::error!("Failed to load bookmarks: {}", e);
                BTreeMap::default()
            }
        }
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

                let Some(relative_path) = RelPath::from_unix_str(&relative_worktree_path).log_err()
                else {
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

    #[cfg(test)]
    pub(crate) async fn save_workspace(&self, workspace: SerializedWorkspace) {
        self.save_workspace_checked(workspace, None).await.log_err();
    }

    pub(crate) async fn save_workspace_checked(
        &self,
        workspace: SerializedWorkspace,
        guard: Option<WorkspaceSerializationGuard>,
    ) -> Result<()> {
        let paths = workspace.paths.serialize();
        let identity_paths = workspace.identity_paths.map(|paths| paths.serialize());
        log::debug!("Saving workspace at location: {:?}", workspace.location);
        self.write(move |conn| {
            conn.with_savepoint("update_worktrees", || {
                let (session_id, window_id) = if let Some(guard) = guard {
                    anyhow::ensure!(!guard.restoration_cancelled.is_some_and(|cancelled| cancelled.load(Ordering::Relaxed)),
                        "Workspace closed during restoration");
                    if guard.serialization_fence.load(Ordering::Relaxed) {
                        return Ok(());
                    }
                    conn.select_row_bound::<_, (Option<String>, Option<u64>)>(
                        "SELECT session_id, window_id FROM workspaces WHERE workspace_id = ?",
                    )?(workspace.id)?.unwrap_or_default()
                } else {
                    (workspace.session_id, workspace.window_id)
                };
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
                        DELETE FROM bookmarks WHERE workspace_id = ?1;
                    )
                )?(workspace.id).context("Clearing old bookmarks")?;

                for (path, bookmarks) in workspace.bookmarks {
                    for bookmark in bookmarks {
                        conn.exec_bound(sql!(
                            INSERT INTO bookmarks (workspace_id, path, row, label)
                            VALUES (?1, ?2, ?3, ?4);
                        ))?((workspace.id, path.as_ref(), bookmark.row, bookmark.label)).context("Inserting bookmark")?;
                    }
                }

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


                // Upsert
                let query = sql!(
                    INSERT INTO workspaces(
                        workspace_id,
                        paths,
                        paths_order,
                        identity_paths,
                        identity_paths_order,
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
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, CURRENT_TIMESTAMP)
                    ON CONFLICT(workspace_id) DO
                    UPDATE SET
                        paths = ?2,
                        paths_order = ?3,
                        identity_paths = ?4,
                        identity_paths_order = ?5,
                        remote_connection_id = ?6,
                        left_dock_visible = ?7,
                        left_dock_active_panel = ?8,
                        left_dock_zoom = ?9,
                        right_dock_visible = ?10,
                        right_dock_active_panel = ?11,
                        right_dock_zoom = ?12,
                        bottom_dock_visible = ?13,
                        bottom_dock_active_panel = ?14,
                        bottom_dock_zoom = ?15,
                        session_id = ?16,
                        window_id = ?17,
                        timestamp = CURRENT_TIMESTAMP
                );
                let mut prepared_query = conn.exec_bound(query)?;
                let args = (
                    workspace.id,
                    paths.paths.clone(),
                    paths.order.clone(),
                    identity_paths.as_ref().map(|paths| paths.paths.clone()),
                    identity_paths.as_ref().map(|paths| paths.order.clone()),
                    remote_connection_id,
                    workspace.docks,
                    session_id,
                    window_id,
                );

                prepared_query(args).context("Updating workspace")?;

                conn.exec_bound(sql!(
                    DELETE FROM recent_navigation_history WHERE workspace_id = ?;
                ))?(workspace.id)
                .context("Clearing recent navigation history")?;

                let mut insert_recent_path = conn.exec_bound(sql!(
                    INSERT INTO recent_navigation_history(workspace_id, path, position)
                    VALUES (?, ?, ?);
                ))?;
                for (position, path) in workspace.recent_navigation_history.iter().enumerate() {
                    insert_recent_path((workspace.id, path.as_path(), position))
                        .context("Inserting recent navigation history")?;
                }

                // Save center pane group
                Self::save_pane_group(conn, workspace.id, &workspace.center_group, None)
                    .context("save pane group in save workspace")?;

                Ok(())
            })
        })
        .await
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
        let identity = remote_connection_identity(&options);
        let kind;
        let user: Option<String>;
        let mut host = None;
        let mut port = None;
        let mut distro = None;
        let mut name = None;
        let mut container_id = None;
        let mut use_podman = None;
        let mut remote_env = None;

        match identity {
            RemoteConnectionIdentity::Ssh {
                host: identity_host,
                username,
                port: identity_port,
            } => {
                kind = RemoteConnectionKind::Ssh;
                host = Some(identity_host);
                port = identity_port;
                user = username;
            }
            RemoteConnectionIdentity::Wsl {
                distro_name,
                user: identity_user,
            } => {
                kind = RemoteConnectionKind::Wsl;
                distro = Some(distro_name);
                user = identity_user;
            }
            RemoteConnectionIdentity::Docker {
                container_id: identity_container_id,
                name: identity_name,
                remote_user,
            } => {
                kind = RemoteConnectionKind::Docker;
                container_id = Some(identity_container_id);
                name = Some(identity_name);
                user = Some(remote_user);
            }
            #[cfg(any(test, feature = "test-support"))]
            RemoteConnectionIdentity::Mock { id } => {
                kind = RemoteConnectionKind::Mock;
                host = Some(id.to_string());
                user = None;
            }
        }

        if let RemoteConnectionOptions::Docker(options) = options {
            use_podman = Some(options.use_podman);
            remote_env = serde_json::to_string(&options.remote_env).ok();
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
            remote_env,
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
        remote_env: Option<String>,
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
                    use_podman,
                    remote_env
                    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
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
                remote_env,
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
    ) -> Result<
        Vec<(
            WorkspaceId,
            PathList,
            Option<PathList>,
            Option<RemoteConnectionId>,
            Option<String>,
            DateTime<Utc>,
        )>,
    > {
        Ok(self
            .recent_workspaces_query()?
            .into_iter()
            .map(
                |(
                    id,
                    paths,
                    order,
                    identity_paths,
                    identity_paths_order,
                    remote_connection_id,
                    session_id,
                    timestamp,
                )| {
                    (
                        id,
                        PathList::deserialize(&SerializedPathList { paths, order }),
                        identity_paths.map(|paths| {
                            PathList::deserialize(&SerializedPathList {
                                paths,
                                order: identity_paths_order.unwrap_or_default(),
                            })
                        }),
                        remote_connection_id.map(RemoteConnectionId),
                        session_id,
                        parse_timestamp(&timestamp),
                    )
                },
            )
            .collect())
    }

    query! {
        fn recent_workspaces_query() -> Result<Vec<(WorkspaceId, String, String, Option<String>, Option<String>, Option<u64>, Option<String>, String)>> {
            SELECT workspace_id, paths, paths_order, identity_paths, identity_paths_order, remote_connection_id, session_id, timestamp
            FROM workspaces
            WHERE
                paths IS NOT NULL OR
                remote_connection_id IS NOT NULL
            ORDER BY timestamp DESC, workspace_id DESC
        }
    }

    fn session_workspaces(
        &self,
        session_id: String,
    ) -> Result<
        Vec<(
            WorkspaceId,
            PathList,
            Option<u64>,
            Option<RemoteConnectionId>,
        )>,
    > {
        Ok(self
            .session_workspaces_query(session_id)?
            .into_iter()
            .map(
                |(workspace_id, paths, order, window_id, remote_connection_id)| {
                    (
                        WorkspaceId(workspace_id),
                        PathList::deserialize(&SerializedPathList { paths, order }),
                        window_id,
                        remote_connection_id.map(RemoteConnectionId),
                    )
                },
            )
            .collect())
    }

    query! {
        fn session_workspaces_query(session_id: String) -> Result<Vec<(i64, String, String, Option<u64>, Option<u64>)>> {
            SELECT workspace_id, paths, paths_order, window_id, remote_connection_id
            FROM workspaces
            WHERE session_id = ?1
            ORDER BY timestamp DESC, workspace_id DESC
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
                id, kind, host, port, user, distro, container_id, name, use_podman, remote_env
            FROM
                remote_connections
        ))?()?
        .into_iter()
        .filter_map(
            |(id, kind, host, port, user, distro, container_id, name, use_podman, remote_env)| {
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
                        remote_env,
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
        let (kind, host, port, user, distro, container_id, name, use_podman, remote_env) =
            self.select_row_bound(sql!(
                SELECT kind, host, port, user, distro, container_id, name, use_podman, remote_env
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
            remote_env,
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
        remote_env: Option<String>,
    ) -> Option<RemoteConnectionOptions> {
        match RemoteConnectionKind::deserialize(&kind)? {
            #[cfg(any(test, feature = "test-support"))]
            RemoteConnectionKind::Mock => {
                Some(RemoteConnectionOptions::Mock(MockConnectionOptions {
                    id: host?.parse::<u64>().ok()?,
                }))
            }
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
                let remote_env: BTreeMap<String, String> =
                    serde_json::from_str(&remote_env?).ok()?;
                Some(RemoteConnectionOptions::Docker(DockerConnectionOptions {
                    container_id: container_id?,
                    name: name?,
                    remote_user: user?,
                    upload_binary_over_docker_exec: false,
                    use_podman: use_podman?,
                    remote_env,
                }))
            }
        }
    }

    pub async fn delete_workspace_by_id(&self, id: WorkspaceId) -> Result<()> {
        self.write(move |connection| {
            connection.with_savepoint("delete_workspace_by_id", || {
                Self::remove_workspace_restore_members_on_connection(connection, &[id], &[])?;
                connection.exec_bound("DELETE FROM workspaces WHERE workspace_id = ?")?(id)
            })
        })
        .await
    }

    async fn all_paths_exist(paths: &[PathBuf], fs: &dyn Fs, require_directory: bool) -> bool {
        let mut any_dir = false;
        for path in paths {
            match fs.metadata(path).await.ok().flatten() {
                None => return false,
                Some(meta) => {
                    if meta.is_dir {
                        any_dir = true;
                    }
                }
            }
        }
        !require_directory || any_dir
    }

    // Returns the raw recent workspace history. Scratch workspaces (no paths) are filtered
    // out because they are restored separately by `last_session_workspace_locations`.
    pub async fn recent_project_workspaces_ungrouped(
        &self,
        fs: &dyn Fs,
    ) -> Result<Vec<RecentWorkspace>> {
        let remote_connections = self.remote_connections()?;
        let mut result = Vec::new();
        let mut seen = HashSet::default();
        for (id, paths, identity_paths_hint, remote_connection_id, _session_id, timestamp) in
            self.recent_workspaces()?
        {
            if !seen.insert((remote_connection_id, paths.paths().to_vec())) {
                continue;
            }
            if let Some(remote_connection_id) = remote_connection_id {
                if let Some(connection_options) = remote_connections.get(&remote_connection_id) {
                    result.push(RecentWorkspace {
                        workspace_id: id,
                        location: SerializedWorkspaceLocation::Remote(connection_options.clone()),
                        paths: paths.clone(),
                        identity_paths: identity_paths_hint.unwrap_or(paths),
                        timestamp,
                    });
                }
                continue;
            }

            if paths.paths().is_empty() || contains_wsl_path(&paths) {
                continue;
            }

            if Self::all_paths_exist(paths.paths(), fs, true).await {
                let identity_paths = resolve_local_workspace_identity(fs, &paths)
                    .await
                    .or(identity_paths_hint)
                    .unwrap_or_else(|| paths.clone());
                result.push(RecentWorkspace {
                    workspace_id: id,
                    location: SerializedWorkspaceLocation::Local,
                    paths,
                    identity_paths,
                    timestamp,
                });
            }
        }

        Ok(result)
    }

    // Returns the recent project workspaces suitable for recent-project UIs.
    // Entries are deduplicated by git worktree identity, but preserve the original
    // serialized paths for reopening.
    pub async fn recent_project_workspaces(&self, fs: &dyn Fs) -> Result<Vec<RecentWorkspace>> {
        Ok(dedupe_recent_workspaces(
            self.recent_project_workspaces_ungrouped(fs).await?,
        ))
    }

    pub async fn delete_recent_workspace_group(
        &self,
        target: &RecentWorkspace,
    ) -> Result<Vec<WorkspaceId>> {
        let target_paths = &target.identity_paths;
        let target_remote_connection = match &target.location {
            SerializedWorkspaceLocation::Local => None,
            SerializedWorkspaceLocation::Remote(connection) => {
                Some(remote_connection_identity(connection))
            }
        };

        let remote_connections = self.remote_connections()?;

        let mut workspace_ids = Vec::new();
        for (workspace_id, paths, identity_paths, remote_connection_id, _, _) in
            self.recent_workspaces()?
        {
            let remote_connection = if let Some(id) = remote_connection_id {
                let Some(connection_options) = remote_connections.get(&id) else {
                    continue;
                };
                Some(remote_connection_identity(connection_options))
            } else {
                None
            };
            if remote_connection == target_remote_connection
                && &identity_paths.unwrap_or(paths) == target_paths
            {
                workspace_ids.push(workspace_id);
            }
        }

        futures::future::try_join_all(
            workspace_ids
                .iter()
                .copied()
                .map(|workspace_id| self.delete_workspace_by_id(workspace_id)),
        )
        .await?;

        Ok(workspace_ids)
    }

    // Deletes workspace rows that can no longer be restored from. Remote workspaces whose
    // connection was removed, and (on Windows) workspaces pointing at WSL paths, are cleaned
    // up immediately. Local workspaces with no valid paths on disk are kept for seven days
    // after going stale. Workspaces belonging to the current session or the last session are
    // always preserved so that an in-progress restore can rehydrate them.
    pub async fn garbage_collect_workspaces(
        &self,
        fs: &dyn Fs,
        current_session_id: &str,
        last_session_id: Option<&str>,
    ) -> Result<()> {
        let checkpoint_ids = self
            .unfinished_workspace_restores()?
            .into_iter()
            .flat_map(|workspace| workspace.workspace_ids)
            .collect::<HashSet<_>>();
        let remote_connections = self.remote_connections()?;
        let now = Utc::now();
        let mut workspaces_to_delete = Vec::new();
        let candidates = self.select::<(
            WorkspaceId,
            Option<String>,
            Option<String>,
            Option<u64>,
            Option<String>,
            Option<u64>,
            String,
        )>(
            "SELECT workspace_id, paths, paths_order, remote_connection_id, session_id, window_id, timestamp
             FROM workspaces WHERE paths IS NOT NULL OR remote_connection_id IS NOT NULL",
        )?()?;
        for candidate in candidates {
            let (id, paths, order, remote_connection_id, session_id, _, timestamp) = &candidate;
            if checkpoint_ids.contains(id) {
                continue;
            }
            if let Some(session_id) = session_id.as_deref() {
                if session_id == current_session_id || Some(session_id) == last_session_id {
                    continue;
                }
            }

            if let Some(remote_connection_id) = remote_connection_id {
                if !remote_connections.contains_key(&RemoteConnectionId(*remote_connection_id)) {
                    workspaces_to_delete.push(candidate);
                }
                continue;
            }
            let paths = PathList::deserialize(&SerializedPathList {
                paths: paths.clone().unwrap_or_default(),
                order: order.clone().unwrap_or_default(),
            });

            // Delete the workspace if any of the paths are WSL paths. If a
            // local workspace points to WSL, attempting to read its metadata
            // will wait for the WSL VM and file server to boot up. This can
            // block for many seconds. Supported scenarios use remote
            // workspaces.
            if contains_wsl_path(&paths) {
                workspaces_to_delete.push(candidate);
                continue;
            }

            if (paths.is_empty() || !Self::all_paths_exist(paths.paths(), fs, false).await)
                && now - parse_timestamp(timestamp) >= chrono::Duration::days(7)
            {
                workspaces_to_delete.push(candidate);
            }
        }

        self.write(move |connection| {
            for (id, paths, order, remote_connection_id, session_id, window_id, timestamp) in
                workspaces_to_delete
            {
                connection.exec_bound(
                    "DELETE FROM workspaces WHERE workspace_id = ?1
                     AND session_id IS ?3 AND window_id IS ?4
                     AND paths IS ?5 AND paths_order IS ?6
                     AND remote_connection_id IS ?7 AND timestamp = ?8
                     AND (remote_connection_id IS NULL OR NOT EXISTS (
                         SELECT 1 FROM remote_connections WHERE id = workspaces.remote_connection_id
                     ))
                     AND NOT EXISTS (
                        SELECT 1 FROM scoped_kv_store AS checkpoint,
                            json_each(checkpoint.value, '$.workspace_ids') AS member
                        WHERE checkpoint.namespace = ?2 AND member.value = ?1
                    )",
                )?((
                    id,
                    WORKSPACE_RESTORE_NAMESPACE,
                    session_id,
                    window_id,
                    paths,
                    order,
                    remote_connection_id,
                    timestamp,
                ))?;
            }
            Ok(())
        })
        .await
    }

    pub async fn last_workspace(&self, fs: &dyn Fs) -> Result<Option<RecentWorkspace>> {
        Ok(self.recent_project_workspaces(fs).await?.into_iter().next())
    }

    // Returns the locations of the workspaces that were still opened when the last
    // session was closed (i.e. when Zed was quit).
    // If `last_session_window_order` is provided, the returned locations are ordered
    // according to that.
    pub async fn last_session_workspace_locations(
        &self,
        last_session_id: &str,
        last_session_window_stack: Option<Vec<WindowId>>,
        _fs: &dyn Fs,
    ) -> Result<Vec<SessionWorkspace>> {
        let mut workspaces = Vec::new();

        for (workspace_id, paths, window_id, remote_connection_id) in
            self.session_workspaces(last_session_id.to_owned())?
        {
            workspaces.push(SessionWorkspace {
                workspace_id,
                location: match remote_connection_id {
                    Some(id) => SerializedWorkspaceLocation::Remote(self.remote_connection(id)?),
                    None => SerializedWorkspaceLocation::Local,
                },
                paths,
                window_id: window_id.map(WindowId::from),
            });
        }

        if let Some(stack) = last_session_window_stack {
            workspaces.sort_by_key(|workspace| {
                workspace
                    .window_id
                    .and_then(|id| stack.iter().position(|&order_id| order_id == id))
                    .unwrap_or(usize::MAX)
            });
        }

        Ok(workspaces)
    }

    fn get_center_pane_group(&self, workspace_id: WorkspaceId) -> Result<SerializedPaneGroup> {
        Ok(self
            .get_pane_group(workspace_id, None)?
            .into_iter()
            .find_map(SerializedPaneGroup::normalize)
            .unwrap_or_else(|| SerializedPaneGroup::Pane(SerializedPane::new(Vec::new(), true, 0))))
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

    #[cfg(test)]
    query! {
        pub(crate) async fn set_timestamp_for_tests(workspace_id: WorkspaceId, timestamp: String) -> Result<()> {
            UPDATE workspaces
            SET timestamp = ?2
            WHERE workspace_id = ?1
        }
    }

    query! {
        pub(crate) async fn set_window_open_status(workspace_id: WorkspaceId, bounds: SerializedWindowBounds, display: Uuid, native_window_state: Option<Vec<u8>>) -> Result<()> {
            UPDATE workspaces
            SET window_state = ?2,
                window_x = ?3,
                window_y = ?4,
                window_width = ?5,
                window_height = ?6,
                display = ?7,
                native_window_state = ?8
            WHERE workspace_id = ?1
        }
    }

    query! {
        pub(crate) fn native_window_state(workspace_id: WorkspaceId) -> Result<Option<(Option<Uuid>, Option<Vec<u8>>)>> {
            SELECT display, native_window_state
            FROM workspaces
            WHERE workspace_id = ?
        }
    }

    query! {
        pub(crate) async fn set_centered_layout(workspace_id: WorkspaceId, centered_layout: bool) -> Result<()> {
            UPDATE workspaces
            SET centered_layout = ?2
            WHERE workspace_id = ?1
        }
    }

    pub(crate) async fn set_session_binding(
        &self,
        workspace_id: WorkspaceId,
        session_id: Option<String>,
        window_id: Option<u64>,
        cancelled: Option<Arc<AtomicBool>>,
    ) -> Result<()> {
        self.write(move |connection| {
            if cancelled.is_some_and(|cancelled| cancelled.load(Ordering::Relaxed)) {
                return Ok(());
            }
            connection.exec_bound(
                "UPDATE workspaces SET session_id = ?2, window_id = ?3 WHERE workspace_id = ?1",
            )?((workspace_id, session_id, window_id))
        })
        .await
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
                            RelPath::from_unix_str(&relative_worktree_path).log_err()?.into(),
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

        self.clear_trusted_worktrees()
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
        let trusted_worktrees = self.trusted_worktrees()?;
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

#[derive(Clone, Debug, PartialEq)]
pub struct RecentWorkspace {
    pub workspace_id: WorkspaceId,
    pub location: SerializedWorkspaceLocation,
    pub paths: PathList,
    pub identity_paths: PathList,
    pub timestamp: DateTime<Utc>,
}

impl RecentWorkspace {
    pub fn project_group_key(&self) -> ProjectGroupKey {
        let host = match &self.location {
            SerializedWorkspaceLocation::Local => None,
            SerializedWorkspaceLocation::Remote(options) => Some(options.clone()),
        };
        ProjectGroupKey::new(host, self.identity_paths.clone())
    }
}

async fn resolve_local_workspace_identity(fs: &dyn Fs, paths: &PathList) -> Option<PathList> {
    let raw_paths = paths.paths();
    let resolved_paths = futures::future::join_all(
        raw_paths
            .iter()
            .map(|path| project::git_store::resolve_git_worktree_to_main_repo(fs, path)),
    )
    .await;

    if resolved_paths.iter().all(|resolved| resolved.is_none()) {
        return None;
    }

    let resolved_paths: Vec<PathBuf> = raw_paths
        .iter()
        .zip(resolved_paths.iter())
        .map(|(original, resolved)| {
            resolved
                .as_ref()
                .cloned()
                .unwrap_or_else(|| original.clone())
        })
        .collect();
    let resolved_path_refs: Vec<&Path> = resolved_paths.iter().map(PathBuf::as_path).collect();
    Some(PathList::new(&resolved_path_refs))
}

fn dedupe_recent_workspaces(
    workspaces: impl IntoIterator<Item = RecentWorkspace>,
) -> Vec<RecentWorkspace> {
    let mut indices_by_key: HashMap<(Option<RemoteConnectionIdentity>, Vec<PathBuf>), usize> =
        HashMap::default();
    let mut result: Vec<RecentWorkspace> = Vec::new();
    for workspace in workspaces {
        let location_identity = match &workspace.location {
            SerializedWorkspaceLocation::Local => None,
            SerializedWorkspaceLocation::Remote(connection) => {
                Some(remote_connection_identity(connection))
            }
        };
        let key = (location_identity, workspace.identity_paths.paths().to_vec());
        if let Some(&existing_index) = indices_by_key.get(&key) {
            if workspace.timestamp > result[existing_index].timestamp {
                result[existing_index] = workspace;
            }
        } else {
            indices_by_key.insert(key, result.len());
            result.push(workspace);
        }
    }

    result
}

pub fn delete_unloaded_items(
    alive_items: Vec<ItemId>,
    workspace_id: WorkspaceId,
    table: &'static str,
    db: &ThreadSafeConnection,
) -> crate::item::ItemCleanup {
    let candidates = db
        .select_bound::<_, ItemId>(&format!(
            "SELECT item_id FROM {table} WHERE workspace_id = ?"
        ))
        .and_then(|mut select| select(workspace_id));
    let db = db.clone();
    Box::pin(async move {
        let mut candidates = candidates?;
        let alive_items = alive_items.into_iter().collect::<HashSet<_>>();
        candidates.retain(|id| !alive_items.contains(id));

        db.write(move |connection| {
            connection.with_savepoint("delete_unloaded_items", || {
                for candidates in candidates.chunks(MAX_QUERY_PLACEHOLDERS - 1) {
                    let placeholders = candidates.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
                    let mut statement = Statement::prepare(
                        connection,
                        format!(
                            "DELETE FROM {table} WHERE workspace_id = ? AND item_id IN ({placeholders})
                             AND NOT EXISTS (
                                 SELECT 1 FROM items
                                 WHERE items.workspace_id = {table}.workspace_id
                                 AND items.item_id = {table}.item_id
                             )"
                        ),
                    )?;
                    let mut next_index = statement.bind(&workspace_id, 1)?;
                    for id in candidates {
                        next_index = statement.bind(id, next_index)?;
                    }
                    statement.exec()?;
                }
                Ok(())
            })
        })
        .await
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PathList;
    use crate::ProjectGroupKey;
    use crate::RemovalIntent;
    use crate::invalid_item_view::RestoringItem;
    use crate::item::{ItemHandle, bind_item_persistence_id, item_persistence_id, test::TestItem};
    use crate::{
        SerializableItemRegistry, Workspace,
        multi_workspace::MultiWorkspace,
        persistence::{
            model::{
                SerializedItem, SerializedPane, SerializedPaneGroup, SerializedWorkspace,
                SessionWorkspace,
            },
            read_multi_workspace_state,
        },
    };
    use agent_settings::AgentSettings;
    use client::proto;
    use futures::{FutureExt, StreamExt, channel::oneshot};
    use gpui::{AppContext as _, Task, TaskExt, WeakEntity};

    use parking_lot::Mutex;
    use pretty_assertions::assert_eq;
    use project::Project;
    use remote::{RemoteClient, SshConnectionOptions};
    use serde_json::json;
    use settings::Settings as _;
    use std::{collections::VecDeque, sync::atomic::AtomicUsize, thread, time::Duration};

    /// Creates a unique directory in a FakeFs, returning the path.
    /// Uses a UUID suffix to avoid collisions with other tests sharing the global DB.
    async fn unique_test_dir(fs: &fs::FakeFs, prefix: &str) -> PathBuf {
        let dir = PathBuf::from(format!("/test-dirs/{}-{}", prefix, uuid::Uuid::new_v4()));
        fs.insert_tree(&dir, json!({})).await;
        dir
    }

    #[gpui::test]
    async fn test_same_path_history_preserves_independent_workspaces(
        cx: &mut gpui::TestAppContext,
    ) {
        let db = WorkspaceDb(
            db::open_test_db::<(KeyValueStore, WorkspaceDb)>(
                "same_path_history_preserves_independent_workspaces",
            )
            .await,
        );
        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({"file.txt": "saved"}))
            .await;
        let closed = workspace_with(1, &[Path::new("/project")], pane_with_items(&[11]), None);
        let previous = workspace_with(
            2,
            &[Path::new("/project")],
            pane_with_items(&[22]),
            Some("older-session"),
        );
        let current = workspace_with(
            3,
            &[Path::new("/project")],
            pane_with_items(&[33]),
            Some("current-session"),
        );
        for workspace in [&closed, &previous, &current] {
            db.save_workspace(workspace.clone()).await;
        }
        db.set_timestamp_for_tests(closed.id, "2000-01-01 00:00:00".to_owned())
            .await
            .unwrap();
        db.set_timestamp_for_tests(previous.id, "2000-01-01 00:00:00".to_owned())
            .await
            .unwrap();
        db.garbage_collect_workspaces(fs.as_ref(), "current-session", Some("last-session"))
            .await
            .unwrap();
        for workspace in [closed, previous, current] {
            assert_eq!(
                db.workspace_for_id(workspace.id).unwrap().center_group,
                workspace.center_group
            );
        }
        assert_eq!(
            db.workspace_for_roots(&["/project"]).unwrap().id,
            WorkspaceId(3)
        );
        assert_eq!(
            db.recent_project_workspaces(fs.as_ref())
                .await
                .unwrap()
                .iter()
                .map(|workspace| workspace.workspace_id)
                .collect::<Vec<_>>(),
            vec![WorkspaceId(3)]
        );
        for id in [4, 5] {
            let mut workspace = remote_workspace_with(id, "host", &[Path::new("/project")]);
            workspace.center_group = pane_with_items(&[id]);
            db.save_workspace(workspace).await;
        }
        db.save_workspace(workspace_with(
            6,
            &[Path::new("/project/file.txt")],
            pane_with_items(&[6]),
            None,
        ))
        .await;
        db.set_timestamp_for_tests(WorkspaceId(6), "2000-01-01 00:00:00".to_owned())
            .await
            .unwrap();
        db.garbage_collect_workspaces(fs.as_ref(), "current-session", Some("last-session"))
            .await
            .unwrap();
        for id in [4, 5, 6] {
            assert_eq!(
                db.workspace_for_id(WorkspaceId(id)).unwrap().center_group,
                pane_with_items(&[id as u64])
            );
        }
    }

    #[gpui::test]
    async fn test_unloaded_workspace_ownership_survives_serialization(
        cx: &mut gpui::TestAppContext,
    ) {
        crate::tests::init_test(cx);
        let fs = fs::FakeFs::new(cx.executor());
        let directory = unique_test_dir(&fs, "unloaded-ownership").await;
        let project = Project::test(fs.clone(), [], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let db = cx.update(|_, cx| WorkspaceDb::global(cx));
        let active_id = db.next_id().await.unwrap();
        let unloaded_id = db.next_id().await.unwrap();
        let retained_id = db.next_id().await.unwrap();
        let other_id = db.next_id().await.unwrap();
        let key = ProjectGroupKey::new(None, PathList::new(std::slice::from_ref(&directory)));
        for (id, session) in [
            (unloaded_id, "old-session"),
            (retained_id, "old-session"),
            (other_id, "other-session"),
        ] {
            db.save_workspace(workspace_with(
                i64::from(id) as u64,
                &[directory.as_path()],
                pane_with_items(&[i64::from(id) as u64]),
                Some(session),
            ))
            .await;
        }
        let window = multi_workspace.update_in(cx, |multi_workspace, window, cx| {
            multi_workspace.workspace().update(cx, |workspace, _| {
                workspace.set_database_id(active_id);
                workspace.session_id = Some("restored-session".to_owned());
            });
            multi_workspace.restore_workspace_ids(&[active_id, unloaded_id, retained_id], cx);
            multi_workspace.restore_project_groups(
                vec![crate::SerializedProjectGroupState {
                    key: key.clone(),
                    expanded: true,
                }],
                cx,
            );
            window.window_handle().downcast::<MultiWorkspace>().unwrap()
        });
        cx.update(|_, cx| {
            assert!(crate::WorkspaceIdReservation::new(unloaded_id, None, cx).is_none());
            assert!(crate::WorkspaceIdReservation::new(unloaded_id, Some(window), cx).is_none());
        });
        cx.cx.update(|cx| {
            assert!(crate::WorkspaceIdReservation::new(unloaded_id, None, cx).is_none());
            assert!(crate::WorkspaceIdReservation::new(active_id, Some(window), cx).is_none());
            let reservation =
                crate::WorkspaceIdReservation::new(unloaded_id, Some(window), cx).unwrap();
            assert!(crate::WorkspaceIdReservation::new(unloaded_id, Some(window), cx).is_none());
            drop(reservation);
            assert!(crate::WorkspaceIdReservation::new(unloaded_id, Some(window), cx).is_some());
        });
        let flush = multi_workspace.update_in(cx, |multi_workspace, window, cx| {
            multi_workspace.flush_pending_serialization(window, cx)
        });
        flush.await.unwrap();
        let session_workspaces = db
            .last_session_workspace_locations("restored-session", None, fs.as_ref())
            .await
            .unwrap();
        let restored = cx.update(|_, cx| read_serialized_multi_workspaces(session_workspaces, cx));
        assert_eq!(restored.len(), 1);
        let mut workspace_ids = restored[0].workspace_ids.clone();
        workspace_ids.sort();
        assert_eq!(workspace_ids, vec![active_id, unloaded_id, retained_id]);
        assert_eq!(
            db.workspace_for_id(unloaded_id).unwrap().window_id,
            Some(window.window_id().as_u64())
        );
        assert_eq!(
            db.workspace_for_id(unloaded_id).unwrap().center_group,
            pane_with_items(&[i64::from(unloaded_id) as u64])
        );
        multi_workspace.read_with(cx, |multi_workspace, cx| {
            assert_eq!(multi_workspace.workspaces().count(), 1);
            assert_eq!(
                multi_workspace
                    .unloaded_workspace_for_paths(
                        &PathList::new(&[directory]),
                        &SerializedWorkspaceLocation::Local,
                        cx
                    )
                    .unwrap()
                    .id,
                unloaded_id
            );
        });
        let source_fence = multi_workspace.read_with(cx, |multi_workspace, _| {
            multi_workspace.serialization_fence.clone()
        });
        multi_workspace
            .update_in(cx, |multi_workspace, window, cx| {
                multi_workspace.open_project_group_in_new_window(&key, window, cx)
            })
            .await
            .unwrap();
        let moved_windows = cx.cx.update(|cx| {
            cx.windows()
                .into_iter()
                .filter_map(|window| window.downcast::<MultiWorkspace>())
                .filter(|window| {
                    window.read(cx).is_ok_and(|multi_workspace| {
                        multi_workspace.workspace().read(cx).database_id() == Some(unloaded_id)
                    })
                })
                .collect::<Vec<_>>()
        });
        assert_eq!(moved_windows.len(), 1);
        let moved_window = moved_windows[0];
        assert!(source_fence.load(Ordering::Relaxed));
        moved_window
            .read_with(cx, |multi_workspace, cx| {
                assert_eq!(multi_workspace.workspaces().count(), 1);
                assert!(multi_workspace.owns_workspace_id(retained_id, cx));
                assert!(!Arc::ptr_eq(
                    &source_fence,
                    &multi_workspace.serialization_fence
                ));
                assert!(!multi_workspace.serialization_fence.load(Ordering::Relaxed));
                assert!(Arc::ptr_eq(
                    &multi_workspace.workspace().read(cx).serialization_fence,
                    &multi_workspace.serialization_fence,
                ));
            })
            .unwrap();
        multi_workspace.read_with(cx, |multi_workspace, cx| {
            assert!(!multi_workspace.owns_workspace_id(unloaded_id, cx));
            assert!(!multi_workspace.owns_workspace_id(retained_id, cx));
        });
        for id in [unloaded_id, retained_id] {
            assert_eq!(
                db.workspace_for_id(id).unwrap().window_id,
                Some(moved_window.window_id().as_u64())
            );
        }
        for id in [retained_id, other_id] {
            assert_eq!(
                db.workspace_for_id(id).unwrap().center_group,
                pane_with_items(&[i64::from(id) as u64])
            );
        }
    }

    #[gpui::test]
    async fn test_removing_unloaded_project_during_resolution_prevents_publication(
        cx: &mut gpui::TestAppContext,
        server_cx: &mut gpui::TestAppContext,
    ) {
        crate::tests::init_test(cx);
        cx.update(|cx| release_channel::init("0.0.0".parse().unwrap(), cx));
        server_cx.update(|cx| release_channel::init("0.0.0".parse().unwrap(), cx));
        let fs = fs::FakeFs::new(cx.executor());
        let directory = unique_test_dir(&fs, "removed-during-resolution").await;
        let remote_path = directory.join("remote");
        let (options, server_session, connect_guard) = RemoteClient::fake_server(cx, server_cx);
        let (entered, reached) = oneshot::channel();
        let (resume, suspended) = oneshot::channel();
        cx.update(|cx| {
            crate::register_serializable_item::<TestItem>(cx);
            cx.set_global(SuspendedItemRestore {
                barrier: Some((entered, suspended)),
                writes: Arc::new(AtomicUsize::new(0)),
                cleanups: 0,
            });
            let descriptor = cx
                .global_mut::<SerializableItemRegistry>()
                .descriptors_by_kind
                .get_mut("TestItem")
                .unwrap();
            descriptor.deserialize = |_, workspace, workspace_id, _, _, cx| {
                let gate = cx.global_mut::<SuspendedItemRestore>();
                let (entered, suspended) = gate.barrier.take().unwrap();
                let writes = gate.writes.clone();
                let item = cx.new(|cx| {
                    TestItem::new_deserialized(workspace_id, cx).with_serialize(move || {
                        writes.fetch_add(1, Ordering::Relaxed);
                        Some(Task::ready(Ok(())))
                    })
                });
                entered.send(workspace).unwrap();
                cx.foreground_executor().spawn(async move {
                    suspended.await?;
                    Ok(Box::new(item) as Box<dyn ItemHandle>)
                })
            };
            descriptor.cleanup = |_, _, _, cx| {
                cx.global_mut::<SuspendedItemRestore>().cleanups += 1;
                futures::future::ready(Ok(())).boxed()
            };
        });
        let handler = server_cx.new(|_| ());
        server_session
            .add_request_handler::<proto::Ping, _, _, _>(handler.downgrade(), |_, _, _| async {
                Ok(proto::Ack {})
            });
        server_session.add_request_handler::<proto::AddWorktree, _, _, _>(handler.downgrade(), {
            let remote_path = remote_path.clone();
            move |_, _, _| {
                let remote_path = remote_path.clone();
                async move {
                    Ok(proto::AddWorktreeResponse {
                        worktree_id: 1,
                        canonicalized_path: remote_path.to_string_lossy().into_owned(),
                        root_repo_common_dir: None,
                        root_repo_is_linked_worktree: false,
                    })
                }
            }
        });
        drop(connect_guard);
        let remote_client = RemoteClient::connect_mock(options.clone(), cx).await;
        let project = Project::test(fs, [directory.as_path()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let db = cx.update(|_, cx| WorkspaceDb::global(cx));
        let active_id = db.next_id().await.unwrap();
        let unloaded_id = db.next_id().await.unwrap();
        let mut saved = workspace_with(
            unloaded_id.0 as u64,
            &[remote_path.as_path()],
            SerializedPaneGroup::Pane(SerializedPane::new(
                vec![SerializedItem::new(
                    "TestItem",
                    unloaded_id.0 as u64,
                    true,
                    false,
                )],
                true,
                0,
            )),
            Some("previous-session"),
        );
        saved.location = SerializedWorkspaceLocation::Remote(options.clone());
        db.save_workspace_checked(saved.clone(), None)
            .await
            .unwrap();
        let SerializedWorkspaceLocation::Remote(saved_options) =
            db.workspace_for_id(unloaded_id).unwrap().location
        else {
            unreachable!();
        };
        let key = ProjectGroupKey::new(
            Some(saved_options),
            PathList::new(std::slice::from_ref(&remote_path)),
        );
        let (window, active_workspace, app_state) =
            multi_workspace.update_in(cx, |multi_workspace, window, cx| {
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, _| workspace.set_database_id(active_id));
                multi_workspace.retain_active_workspace(cx);
                multi_workspace.restore_workspace_ids(&[unloaded_id], cx);
                multi_workspace.restore_project_groups(
                    vec![crate::SerializedProjectGroupState {
                        key: key.clone(),
                        expanded: true,
                    }],
                    cx,
                );
                (
                    window.window_handle().downcast::<MultiWorkspace>().unwrap(),
                    workspace.clone(),
                    workspace.read(cx).app_state().clone(),
                )
            });
        let project = cx.update(|_, cx| {
            Project::remote(
                remote_client,
                app_state.client.clone(),
                app_state.node_runtime.clone(),
                app_state.user_store.clone(),
                app_state.languages.clone(),
                app_state.fs.clone(),
                false,
                cx,
            )
        });
        let loading = crate::open_remote_project_with_existing_connection(
            options,
            project,
            vec![remote_path],
            app_state,
            window,
            None,
            None,
            &mut cx.cx.to_async(),
        );
        let staged_workspace = reached.await.unwrap();
        cx.update(|_, cx| {
            let workspace = staged_workspace.upgrade().unwrap();
            assert_eq!(workspace.read(cx).database_id(), Some(unloaded_id));
            assert!(workspace.read(cx).is_restoring());
        });
        assert!(!loading.is_ready());
        multi_workspace.read_with(cx, |multi_workspace, _| {
            assert_eq!(multi_workspace.unloaded_workspace_ids(), &[unloaded_id]);
            assert_eq!(
                multi_workspace.workspaces().cloned().collect::<Vec<_>>(),
                vec![active_workspace.clone()]
            );
        });
        let session_binding = || {
            db.select_row_bound::<_, (Option<String>, Option<u64>)>(
                "SELECT session_id, window_id FROM workspaces WHERE workspace_id = ?",
            )
            .unwrap()(unloaded_id)
            .unwrap()
        };
        for discard in [false, true] {
            let binding = session_binding();
            let groups = multi_workspace.read_with(cx, |multi_workspace, _| {
                multi_workspace.project_group_keys()
            });
            let removal = multi_workspace.update_in(cx, |multi_workspace, window, cx| {
                multi_workspace.remove_project_group(&key, window, cx)
            });
            cx.run_until_parked();
            assert!(cx.has_pending_prompt());
            cx.simulate_prompt_answer(if discard {
                "Close Without Saving"
            } else {
                "Cancel"
            });
            assert_eq!(removal.await.unwrap(), discard);
            if !discard {
                assert_eq!(session_binding(), binding);
                multi_workspace.read_with(cx, |multi_workspace, _| {
                    assert_eq!(multi_workspace.project_group_keys(), groups);
                    assert_eq!(multi_workspace.unloaded_workspace_ids(), &[unloaded_id]);
                });
                assert!(!loading.is_ready());
            }
        }
        assert_eq!(session_binding(), Some((None, None)));
        assert!(!loading.is_ready());
        resume.send(()).unwrap();
        assert!(loading.await.is_err());
        cx.run_until_parked();
        assert_eq!(session_binding(), Some((None, None)));
        assert_eq!(
            db.workspace_for_id(unloaded_id).unwrap().center_group,
            saved.center_group
        );
        cx.update(|_, cx| {
            let gate = cx.global::<SuspendedItemRestore>();
            assert_eq!(gate.writes.load(Ordering::Relaxed), 0);
            assert_eq!(gate.cleanups, 0);
        });
        assert!(staged_workspace.upgrade().is_none());
        multi_workspace.read_with(cx, |multi_workspace, cx| {
            assert_eq!(multi_workspace.workspace(), &active_workspace);
            assert_eq!(
                multi_workspace.workspaces().cloned().collect::<Vec<_>>(),
                vec![active_workspace.clone()]
            );
            assert_eq!(multi_workspace.workspace_ids(cx), vec![active_id]);
            assert_eq!(
                active_workspace
                    .read(cx)
                    .worktrees(cx)
                    .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
                    .collect::<Vec<_>>(),
                vec![directory.clone()]
            );
            assert_eq!(
                multi_workspace.project_group_keys(),
                vec![ProjectGroupKey::new(None, PathList::new(&[directory]))]
            );
        });
    }

    #[gpui::test]
    async fn test_history_waits_for_restore_plan(cx: &mut gpui::TestAppContext) {
        crate::tests::init_test(cx);
        let fs = fs::FakeFs::new(cx.executor());
        let project = Project::test(fs.clone(), [], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let app_state = multi_workspace.read_with(cx, |multi_workspace, cx| {
            multi_workspace.workspace().read(cx).app_state().clone()
        });
        let db = cx.update(|_, cx| WorkspaceDb::global(cx));
        for planned in [Some(true), Some(false), None] {
            let directory = unique_test_dir(&fs, "restore-plan").await;
            let missing_id = db.next_id().await.unwrap();
            let missing_scratch_id = db.next_id().await.unwrap();
            let saved_id = db.next_id().await.unwrap();
            db.delete_workspace_by_id(missing_id).await.unwrap();
            db.delete_workspace_by_id(missing_scratch_id).await.unwrap();
            db.save_workspace(workspace_with(
                i64::from(saved_id) as u64,
                &[directory.as_path()],
                empty_pane_group(),
                Some("previous-session"),
            ))
            .await;
            let record = model::SerializedMultiWorkspace {
                active_workspace: SessionWorkspace {
                    workspace_id: missing_id,
                    location: SerializedWorkspaceLocation::Local,
                    paths: PathList::default(),
                    window_id: None,
                },
                workspace_ids: vec![missing_id, missing_scratch_id, saved_id],
                scratch_workspaces: vec![missing_scratch_id],
                state: model::MultiWorkspaceState {
                    project_groups: vec![model::SerializedProjectGroup::from_group(
                        &ProjectGroupKey::new(
                            None,
                            PathList::new(std::slice::from_ref(&directory)),
                        ),
                        true,
                    )],
                    ..model::MultiWorkspaceState::default()
                },
            };
            let mut plan = Some(
                cx.update(|_, cx| crate::WorkspaceRestorePlan::new(cx))
                    .unwrap(),
            );
            let request = cx.update(|_, cx| {
                crate::open_paths(
                    std::slice::from_ref(&directory),
                    app_state.clone(),
                    crate::OpenOptions {
                        open_mode: crate::OpenMode::NewWindow,
                        workspace_matching: crate::WorkspaceMatching::None,
                        ..crate::OpenOptions::default()
                    },
                    cx,
                )
            });
            cx.run_until_parked();
            assert!(!request.is_ready());
            match planned {
                Some(true) => plan
                    .as_mut()
                    .unwrap()
                    .set_workspaces(
                        std::slice::from_ref(&record),
                        app_state.session.clone(),
                        &mut cx.cx.to_async(),
                    )
                    .await
                    .unwrap(),
                Some(false) => plan
                    .as_mut()
                    .unwrap()
                    .set_workspaces(&[], app_state.session.clone(), &mut cx.cx.to_async())
                    .await
                    .unwrap(),
                None => {
                    drop(plan.take());
                    assert!(request.await.is_err());
                    continue;
                }
            }
            let result = request.await.unwrap();
            let opened_id = result
                .workspace
                .read_with(cx, |workspace, _| workspace.database_id());
            if planned == Some(true) {
                assert_ne!(opened_id, Some(saved_id));
                let restored = cx
                    .update(|_, cx| {
                        let record = record.clone();
                        let app_state = app_state.clone();
                        cx.spawn(async move |cx| {
                            crate::restore_multiworkspace(record, app_state, true, cx).await
                        })
                    })
                    .await
                    .unwrap();
                assert_eq!(
                    restored
                        .read_with(cx, |multi_workspace, cx| multi_workspace
                            .workspace()
                            .read(cx)
                            .database_id())
                        .unwrap(),
                    Some(saved_id)
                );
                cx.cx.update(|cx| {
                    assert!(
                        crate::WorkspaceIdReservation::for_history(missing_id, None, cx).is_none()
                    );
                });
                restored
                    .update(cx, |multi_workspace, window, cx| {
                        multi_workspace.flush_restoration(window, cx)
                    })
                    .unwrap()
                    .await
                    .unwrap();
                plan.as_mut()
                    .unwrap()
                    .finish_window(record.active_workspace.workspace_id);
                cx.cx.update(|cx| {
                    assert!(
                        crate::WorkspaceIdReservation::for_history(missing_id, None, cx).is_some()
                    );
                    assert!(
                        crate::WorkspaceIdReservation::for_history(saved_id, None, cx).is_none()
                    );
                });
                assert!(
                    cx.update(|_, cx| crate::open_paths(
                        &[directory],
                        app_state.clone(),
                        crate::OpenOptions {
                            workspace_id: Some(saved_id),
                            ..crate::OpenOptions::default()
                        },
                        cx
                    ))
                    .await
                    .is_err()
                );
            } else {
                assert_eq!(opened_id, Some(saved_id));
            }
            drop(plan);
        }
    }

    #[gpui::test]
    async fn test_keyless_close_preserves_unowned_restore_members(cx: &mut gpui::TestAppContext) {
        crate::tests::init_test(cx);
        let db = cx.update(|cx| WorkspaceDb::global(cx));
        let recovered_id = db.next_id().await.unwrap();
        let retained_id = db.next_id().await.unwrap();
        for id in [recovered_id, retained_id] {
            db.save_workspace(workspace_with(
                i64::from(id) as u64,
                &[],
                pane_with_items(&[i64::from(id) as u64]),
                Some("old-session"),
            ))
            .await;
        }
        let recovered_group = ProjectGroupKey::new(None, PathList::new(&[Path::new("/recovered")]));
        let retained_group = ProjectGroupKey::new(None, PathList::new(&[Path::new("/retained")]));
        db.checkpoint_workspace_restores(&[model::SerializedMultiWorkspace {
            active_workspace: db.session_workspace_for_id(recovered_id).unwrap(),
            workspace_ids: vec![recovered_id, retained_id],
            scratch_workspaces: vec![retained_id],
            state: model::MultiWorkspaceState {
                active_workspace_id: Some(recovered_id),
                project_groups: vec![
                    model::SerializedProjectGroup::from_group(&recovered_group, false),
                    model::SerializedProjectGroup::from_group(&retained_group, true),
                ],
                ..model::MultiWorkspaceState::default()
            },
        }])
        .await
        .unwrap();
        db.set_session_binding(
            recovered_id,
            Some("current-session".to_owned()),
            Some(100),
            None,
        )
        .await
        .unwrap();

        let closed = db
            .close_workspaces(
                None,
                vec![recovered_id],
                vec![recovered_group.clone()],
                Arc::default(),
                || Ok(()),
            )
            .await
            .unwrap();
        let recovered = db.unfinished_workspace_restores().unwrap();
        assert_eq!(recovered.len(), 1);
        assert_eq!(recovered[0].active_workspace.workspace_id, retained_id);
        assert_eq!(recovered[0].workspace_ids, vec![retained_id]);
        assert_eq!(recovered[0].scratch_workspaces, vec![retained_id]);
        assert_eq!(recovered[0].state.active_workspace_id, Some(retained_id));
        assert_eq!(
            db.workspace_for_id(retained_id).unwrap().center_group,
            pane_with_items(&[i64::from(retained_id) as u64])
        );
        for (id, expected) in [
            (recovered_id, (None, None)),
            (
                retained_id,
                (
                    Some("old-session".to_owned()),
                    Some(i64::from(retained_id) as u64),
                ),
            ),
        ] {
            assert_eq!(
                db.select_row_bound::<_, (Option<String>, Option<u64>)>(
                    "SELECT session_id, window_id FROM workspaces WHERE workspace_id = ?",
                )
                .unwrap()(id)
                .unwrap(),
                Some(expected)
            );
        }
        assert_eq!(
            recovered[0]
                .state
                .project_groups
                .iter()
                .cloned()
                .map(ProjectGroupKey::from)
                .collect::<Vec<_>>(),
            vec![retained_group.clone()]
        );
        db.write(move |connection| {
            connection.exec_bound(
                "UPDATE scoped_kv_store SET value = json_set(value, '$.state.project_groups[0].expanded', json('false')) WHERE namespace = ? AND key = ?",
            )?((WORKSPACE_RESTORE_NAMESPACE, recovered_id.0.to_string()))
        }).await.unwrap();
        db.restore_workspace_close(closed, vec![recovered_id])
            .await
            .unwrap();
        let recovered = db.unfinished_workspace_restores().unwrap();
        assert_eq!(recovered.len(), 1);
        assert_eq!(recovered[0].workspace_ids, vec![retained_id, recovered_id]);
        assert_eq!(recovered[0].scratch_workspaces, vec![retained_id]);
        assert_eq!(recovered[0].state.active_workspace_id, Some(recovered_id));
        assert_eq!(
            recovered[0]
                .state
                .project_groups
                .iter()
                .map(|group| (ProjectGroupKey::from(group.clone()), group.expanded))
                .collect::<Vec<_>>(),
            vec![(recovered_group, false), (retained_group, false)]
        );
        assert_eq!(
            db.select_row_bound::<_, (Option<String>, Option<u64>)>(
                "SELECT session_id, window_id FROM workspaces WHERE workspace_id = ?",
            )
            .unwrap()(recovered_id)
            .unwrap(),
            Some((Some("current-session".to_owned()), Some(100)))
        );
        let closed = db
            .close_workspaces(None, vec![recovered_id], Vec::new(), Arc::default(), || {
                Ok(())
            })
            .await
            .unwrap();
        db.set_session_binding(
            recovered_id,
            Some("other-session".to_owned()),
            Some(200),
            None,
        )
        .await
        .unwrap();
        db.restore_workspace_close(closed, vec![recovered_id])
            .await
            .unwrap();
        assert_eq!(
            db.select_row_bound::<_, (Option<String>, Option<u64>)>(
                "SELECT session_id, window_id FROM workspaces WHERE workspace_id = ?",
            )
            .unwrap()(recovered_id)
            .unwrap(),
            Some((Some("other-session".to_owned()), Some(200)))
        );
        let recovered = db.unfinished_workspace_restores().unwrap();
        assert_eq!(recovered.len(), 1);
        assert_eq!(recovered[0].workspace_ids, vec![retained_id]);
        assert_eq!(recovered[0].scratch_workspaces, vec![retained_id]);
        assert_eq!(recovered[0].state.active_workspace_id, Some(retained_id));

        let members = db.workspace_ids_for_close(Some(recovered_id), &[]).unwrap();
        assert_eq!(members, vec![retained_id]);
        db.write(|connection| {
            connection
                .exec_bound("INSERT INTO scoped_kv_store(namespace, key, value) VALUES (?, ?, ?)")?(
                (
                    WORKSPACE_RESTORE_NAMESPACE,
                    "unrelated",
                    "invalid checkpoint",
                ),
            )
        })
        .await
        .unwrap();
        assert_eq!(
            db.workspace_ids_for_close(Some(retained_id), &[recovered_id])
                .unwrap(),
            vec![recovered_id]
        );
        for (closing_members, expected_error) in [
            (
                Vec::new(),
                "Workspace membership changed while preparing to close",
            ),
            (members.clone(), "Close changed"),
        ] {
            let checks = AtomicUsize::new(0);
            let fence = Arc::new(AtomicBool::new(false));
            let result = db
                .close_workspaces(
                    Some(recovered_id),
                    closing_members,
                    Vec::new(),
                    fence.clone(),
                    move || {
                        anyhow::ensure!(
                            checks.fetch_add(1, Ordering::Relaxed) == 0,
                            "Close changed"
                        );
                        Ok(())
                    },
                )
                .await;
            assert_eq!(result.err().unwrap().to_string(), expected_error);
            assert!(!fence.load(Ordering::Relaxed));
        }
        let closed = db
            .close_workspaces(
                Some(recovered_id),
                members,
                Vec::new(),
                Arc::default(),
                || Ok(()),
            )
            .await
            .unwrap();
        assert!(!db.workspace_restore_pending(recovered_id).unwrap());
        assert_eq!(
            db.select_row_bound::<_, Option<String>>(
                "SELECT session_id FROM workspaces WHERE workspace_id = ?"
            )
            .unwrap()(retained_id)
            .unwrap(),
            Some(None)
        );
        db.restore_workspace_close(closed, vec![retained_id])
            .await
            .unwrap();
        assert_eq!(
            db.workspace_ids_for_close(Some(recovered_id), &[]).unwrap(),
            vec![retained_id]
        );
        assert_eq!(
            db.select_row_bound::<_, Option<String>>(
                "SELECT session_id FROM workspaces WHERE workspace_id = ?"
            )
            .unwrap()(retained_id)
            .unwrap(),
            Some(Some("old-session".to_owned()))
        );
        assert_eq!(
            db.select_row_bound::<_, String>(
                "SELECT value FROM scoped_kv_store WHERE namespace = ? AND key = ?"
            )
            .unwrap()((WORKSPACE_RESTORE_NAMESPACE, "unrelated"))
            .unwrap(),
            Some("invalid checkpoint".to_owned())
        );
        db.write(|connection| {
            connection.exec_bound("DELETE FROM scoped_kv_store WHERE namespace = ? AND key = ?")?((
                WORKSPACE_RESTORE_NAMESPACE,
                "unrelated",
            ))
        })
        .await
        .unwrap();
    }

    #[gpui::test]
    async fn test_unfinished_restore_preserves_saved_payload_on_quit(
        cx: &mut gpui::TestAppContext,
    ) {
        crate::tests::init_test(cx);
        cx.update(crate::register_serializable_item::<TestItem>);
        let fs = fs::FakeFs::new(cx.executor());
        let project = Project::test(fs.clone(), [], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let db = cx.cx.update(|cx| WorkspaceDb::global(cx));
        let active_id = db.next_id().await.unwrap();
        let lazy_id = db.next_id().await.unwrap();
        let pending_id = db.next_id().await.unwrap();
        for id in [active_id, lazy_id, pending_id] {
            db.save_workspace(workspace_with(
                id.0 as u64,
                &[],
                pane_with_items(&[id.0 as u64]),
                Some("old-session"),
            ))
            .await;
            db.set_timestamp_for_tests(id, "2000-01-01 00:00:00".to_owned())
                .await
                .unwrap();
        }
        let records = [
            model::SerializedMultiWorkspace {
                active_workspace: SessionWorkspace {
                    workspace_id: active_id,
                    location: SerializedWorkspaceLocation::Local,
                    paths: PathList::default(),
                    window_id: Some(WindowId::from(91u64)),
                },
                workspace_ids: vec![active_id, lazy_id],
                scratch_workspaces: vec![lazy_id],
                state: model::MultiWorkspaceState {
                    active_workspace_id: Some(active_id),
                    sidebar_open: true,
                    sidebar_state: Some("captured-sidebar".to_owned()),
                    ..model::MultiWorkspaceState::default()
                },
            },
            model::SerializedMultiWorkspace {
                active_workspace: SessionWorkspace {
                    workspace_id: pending_id,
                    location: SerializedWorkspaceLocation::Local,
                    paths: PathList::default(),
                    window_id: Some(WindowId::from(92u64)),
                },
                workspace_ids: vec![pending_id],
                scratch_workspaces: Vec::new(),
                state: model::MultiWorkspaceState::default(),
            },
        ];
        let mut plan = cx.cx.update(crate::WorkspaceRestorePlan::new).unwrap();
        let session = multi_workspace.read_with(cx, |multi_workspace, cx| {
            multi_workspace
                .workspace()
                .read(cx)
                .app_state()
                .session
                .clone()
        });
        plan.set_workspaces(&records, session.clone(), &mut cx.cx.to_async())
            .await
            .unwrap();
        let written = Arc::new(AtomicBool::new(false));
        let (window, live_item) = multi_workspace.update_in(cx, |multi_workspace, window, cx| {
            multi_workspace.set_restore_key(active_id);
            let live_item = multi_workspace.workspace().update(cx, |workspace, cx| {
                workspace.set_database_id(active_id);
                workspace.session_id = Some("current-session".to_owned());
                let restore = workspace.load_workspace(
                    db.workspace_for_id(active_id).unwrap(),
                    Vec::new(),
                    window,
                    cx,
                );
                assert!(workspace.is_restoring());
                drop(restore);
                let item = cx.new(|cx| {
                    let written = written.clone();
                    TestItem::new(cx).with_dirty(true).with_serialize(move || {
                        written.store(true, Ordering::Relaxed);
                        Some(gpui::Task::ready(Ok(())))
                    })
                });
                workspace.add_item_to_active_pane(Box::new(item.clone()), None, true, window, cx);
                item
            });
            (
                window.window_handle().downcast::<MultiWorkspace>().unwrap(),
                live_item,
            )
        });
        let mut async_cx = cx.to_async();
        assert!(crate::prepare_windows_to_quit(&[window], &mut async_cx).await);
        assert!(written.load(Ordering::Relaxed));
        let live_item_id = cx
            .update(|_, cx| item_persistence_id(&live_item, cx))
            .await
            .unwrap();
        let active_manifest = SerializedPaneGroup::Pane(SerializedPane::new(
            vec![
                SerializedItem::new("Terminal", active_id.0 as u64, false, false),
                SerializedItem::new("TestItem", live_item_id, true, false),
            ],
            true,
            0,
        ));
        assert!(!plan.has_closed_windows());
        assert!(!plan.was_window_closed(active_id));
        assert!(!plan.was_window_closed(pending_id));
        drop(plan);
        let recovered = db.unfinished_workspace_restores().unwrap();
        assert_eq!(recovered.len(), 2);
        for (recovered, expected) in recovered.iter().zip(&records) {
            assert_eq!(recovered.active_workspace, expected.active_workspace);
            assert_eq!(recovered.workspace_ids, expected.workspace_ids);
            assert_eq!(recovered.scratch_workspaces, expected.scratch_workspaces);
            assert_eq!(
                serde_json::to_value(&recovered.state).unwrap(),
                serde_json::to_value(&expected.state).unwrap()
            );
        }
        db.garbage_collect_workspaces(fs.as_ref(), "current-session", None)
            .await
            .unwrap();
        assert_eq!(
            db.workspace_for_id(active_id).unwrap().center_group,
            active_manifest
        );
        for id in [lazy_id, pending_id] {
            assert_eq!(
                db.workspace_for_id(id).unwrap().center_group,
                pane_with_items(&[id.0 as u64])
            );
        }
        let mut plan = cx.cx.update(crate::WorkspaceRestorePlan::new).unwrap();
        plan.set_workspaces(&recovered, session, &mut cx.cx.to_async())
            .await
            .unwrap();
        assert!(!plan.has_closed_windows());
        let pending_save = multi_workspace.update_in(cx, |multi_workspace, window, cx| {
            multi_workspace.workspace().update(cx, |workspace, cx| {
                workspace.serialize_workspace_snapshot(window, cx)
            })
        });
        let app_state = multi_workspace.read_with(cx, |multi_workspace, cx| {
            multi_workspace.workspace().read(cx).app_state().clone()
        });
        let placeholder = cx.cx.add_window(|window, cx| {
            let workspace = cx.new(|cx| {
                crate::Workspace::new(None, project.clone(), app_state.clone(), window, cx)
            });
            let mut multi_workspace = MultiWorkspace::new(workspace, window, cx);
            multi_workspace.set_restore_key(pending_id);
            multi_workspace
        });
        assert_ne!(
            placeholder
                .read_with(cx, |multi_workspace, cx| multi_workspace.database_id(cx))
                .unwrap(),
            Some(pending_id)
        );
        let close = cx.update(|_, cx| {
            cx.spawn(async move |cx| {
                crate::prepare_window_to_close(window, crate::CloseIntent::CloseWindow, cx).await
            })
        });
        cx.run_until_parked();
        assert!(cx.has_pending_prompt());
        cx.simulate_prompt_answer("Close Without Saving");
        cx.run_until_parked();
        assert!(cx.has_pending_prompt());
        cx.simulate_prompt_answer("Don't Save");
        assert!(close.await.unwrap());
        assert!(plan.has_closed_windows());
        assert!(plan.was_window_closed(active_id));
        assert!(!plan.was_window_closed(lazy_id));
        assert!(!plan.was_window_closed(pending_id));
        assert!(pending_save.await.is_err());
        assert_eq!(
            db.unfinished_workspace_restores()
                .unwrap()
                .iter()
                .map(|record| record.active_workspace.workspace_id)
                .collect::<Vec<_>>(),
            vec![pending_id]
        );
        for id in [active_id, lazy_id] {
            let saved = db.workspace_for_id(id).unwrap();
            assert_eq!(
                db.select_row_bound::<_, Option<String>>(
                    "SELECT session_id FROM workspaces WHERE workspace_id = ?"
                )
                .unwrap()(id)
                .unwrap(),
                Some(None)
            );
            assert_eq!(saved.center_group, pane_with_items(&[id.0 as u64]));
        }
        cx.cx.add_window(|window, cx| {
            let workspace =
                cx.new(|cx| crate::Workspace::new(None, project, app_state, window, cx));
            MultiWorkspace::new(workspace, window, cx)
        });
        for discard in [false, true] {
            let close = cx.update(|_, cx| {
                cx.spawn(async move |cx| {
                    crate::prepare_window_to_close(placeholder, crate::CloseIntent::CloseWindow, cx)
                        .await
                })
            });
            cx.run_until_parked();
            assert!(cx.has_pending_prompt());
            cx.simulate_prompt_answer(if discard {
                "Close Without Saving"
            } else {
                "Cancel"
            });
            assert_eq!(close.await.unwrap(), discard);
            assert_eq!(plan.was_window_closed(pending_id), discard);
            assert_eq!(
                db.unfinished_workspace_restores().unwrap().len(),
                usize::from(!discard)
            );
            assert_eq!(
                db.workspace_for_id(pending_id).unwrap().center_group,
                pane_with_items(&[pending_id.0 as u64])
            );
        }
        assert!(plan.has_closed_windows());
        assert!(plan.was_window_closed(active_id));
        assert!(!plan.was_window_closed(lazy_id));
        assert!(plan.was_window_closed(pending_id));
        assert_eq!(db.unfinished_workspace_restores().unwrap().len(), 0);
        assert_eq!(
            db.select_row_bound::<_, Option<String>>(
                "SELECT session_id FROM workspaces WHERE workspace_id = ?"
            )
            .unwrap()(pending_id)
            .unwrap(),
            Some(None)
        );
        db.checkpoint_workspace_restores(&[]).await.unwrap();
    }

    #[gpui::test]
    async fn test_flush_serialization_allows_restored_item_replacement(
        cx: &mut gpui::TestAppContext,
    ) {
        crate::tests::init_test(cx);
        cx.update(crate::register_serializable_item::<TestItem>);
        let db = cx.update(|cx| WorkspaceDb::global(cx));
        db.write(|connection| {
            connection.exec(
                "CREATE TABLE restoration_payloads (item_id INTEGER PRIMARY KEY, contents INTEGER NOT NULL) STRICT",
            )?()
        })
        .await
        .unwrap();
        for edit_after_replacement in [false, true] {
            let project = Project::test(fs::FakeFs::new(cx.executor()), [], cx).await;
            let (multi_workspace, cx) =
                cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
            let workspace_id = db.next_id().await.unwrap();
            let restored_id = db.next_item_id().await.unwrap();
            db.write(move |connection| {
                connection.exec_bound("INSERT INTO restoration_payloads VALUES (?, 0)")?(
                    restored_id,
                )
            })
            .await
            .unwrap();
            let payload = Arc::new(AtomicUsize::new(0));
            let (resume, suspended) = oneshot::channel::<()>();
            let suspended = suspended.shared();
            let (blocker, placeholder) =
                multi_workspace.update_in(cx, |multi_workspace, window, cx| {
                    multi_workspace.workspace().update(cx, |workspace, cx| {
                        workspace.set_database_id(workspace_id);
                        workspace.set_restoring_workspace(true);
                        let executor = cx.background_executor().clone();
                        let blocker = cx.new(|cx| {
                            TestItem::new(cx).with_serialize(move || {
                                let suspended = suspended.clone();
                                Some(executor.spawn(async move {
                                    suspended.await?;
                                    Ok(())
                                }))
                            })
                        });
                        let placeholder = cx.new(|cx| {
                            RestoringItem::new(
                                SerializedItem::new("TestItem", restored_id, false, false),
                                cx,
                            )
                        });
                        workspace.add_item_to_active_pane(
                            Box::new(blocker.clone()),
                            None,
                            true,
                            window,
                            cx,
                        );
                        workspace.add_item_to_active_pane(
                            Box::new(placeholder.clone()),
                            None,
                            true,
                            window,
                            cx,
                        );
                        (blocker, placeholder)
                    })
                });
            let blocker_id = cx
                .update(|_, cx| item_persistence_id(&blocker, cx))
                .await
                .unwrap();
            cx.run_until_parked();
            let flush = multi_workspace.update_in(cx, |multi_workspace, window, cx| {
                multi_workspace.flush_restoration(window, cx)
            });
            cx.run_until_parked();
            assert!(!flush.is_ready());
            let replacement = multi_workspace.update_in(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    let executor = cx.background_executor().clone();
                    let replacement = cx.new(|cx| {
                        bind_item_persistence_id(&cx.entity(), restored_id, cx).unwrap();
                        let persistence_id = item_persistence_id(&cx.entity(), cx);
                        let payload = payload.clone();
                        let db = db.clone();
                        TestItem::new(cx).with_dirty(true).with_serialize(move || {
                            let contents = payload.load(Ordering::Relaxed) as u64;
                            let persistence_id = persistence_id.clone();
                            let db = db.clone();
                            Some(executor.spawn(async move {
                                let item_id = persistence_id
                                    .await
                                    .map_err(|error| anyhow::anyhow!(error))?;
                                db.write(move |connection| {
                                    connection.exec_bound(
                                        "INSERT OR REPLACE INTO restoration_payloads VALUES (?, ?)",
                                    )?((item_id, contents))
                                })
                                .await
                            }))
                        })
                    });
                    workspace.active_pane().update(cx, |pane, cx| {
                        assert!(pane.replace_item(
                            placeholder.entity_id(),
                            Box::new(replacement.clone()),
                            window,
                            cx
                        ));
                    });
                    workspace.set_restoring_workspace(false);
                    replacement
                })
            });
            let replacement_id = cx
                .update(|_, cx| item_persistence_id(&replacement, cx))
                .await
                .unwrap();
            assert_ne!(replacement_id, restored_id);
            cx.run_until_parked();
            if edit_after_replacement {
                payload.store(1, Ordering::Relaxed);
                multi_workspace.update(cx, |multi_workspace, cx| {
                    multi_workspace.workspace().update(cx, |workspace, cx| {
                        workspace
                            .enqueue_item_serialization(
                                replacement.to_serializable_item_handle(cx).unwrap(),
                            )
                            .unwrap();
                    });
                });
                cx.run_until_parked();
            }
            resume.send(()).unwrap();
            if edit_after_replacement {
                assert_eq!(
                    flush.await.unwrap_err().to_string(),
                    "Workspace changed during serialization flush"
                );
            } else {
                flush.await.unwrap();
            }
            let mut expected = vec![blocker_id, restored_id];
            expected.sort_unstable();
            assert_eq!(
                db.select_bound::<_, ItemId>(
                    "SELECT item_id FROM items WHERE workspace_id = ? ORDER BY item_id"
                )
                .unwrap()(workspace_id)
                .unwrap(),
                expected
            );
            multi_workspace
                .update_in(cx, |multi_workspace, window, cx| {
                    multi_workspace.flush_pending_serialization(window, cx)
                })
                .await
                .unwrap();
            let mut expected = vec![blocker_id, replacement_id];
            expected.sort_unstable();
            assert_eq!(
                db.select_bound::<_, ItemId>(
                    "SELECT item_id FROM items WHERE workspace_id = ? ORDER BY item_id"
                )
                .unwrap()(workspace_id)
                .unwrap(),
                expected
            );
            let mut expected_payloads = vec![
                (restored_id, 0),
                (replacement_id, u64::from(edit_after_replacement)),
            ];
            expected_payloads.sort_unstable();
            assert_eq!(
                db.select_bound::<_, (ItemId, u64)>(
                    "SELECT item_id, contents FROM restoration_payloads WHERE item_id IN (?, ?) ORDER BY item_id"
                )
                .unwrap()((restored_id, replacement_id))
                .unwrap(),
                expected_payloads
            );
        }
    }

    #[gpui::test]
    async fn test_pending_checkpoint_membership_publishes_with_window_bindings() {
        let db = WorkspaceDb(
            db::open_test_db::<(KeyValueStore, WorkspaceDb)>(
                "pending_checkpoint_membership_publication",
            )
            .await,
        );
        let active_id = WorkspaceId(1);
        let unloaded_id = WorkspaceId(2);
        let added_id = WorkspaceId(3);
        for id in [active_id, unloaded_id, added_id] {
            db.save_workspace_checked(
                workspace_with(
                    id.0 as u64,
                    &[],
                    pane_with_items(&[id.0 as u64]),
                    Some("previous"),
                ),
                None,
            )
            .await
            .unwrap();
        }
        db.checkpoint_workspace_restores(&[model::SerializedMultiWorkspace {
            active_workspace: db.session_workspace_for_id(active_id).unwrap(),
            workspace_ids: vec![active_id, unloaded_id],
            scratch_workspaces: vec![unloaded_id],
            state: model::MultiWorkspaceState::default(),
        }])
        .await
        .unwrap();
        let mut added = workspace_with(
            added_id.0 as u64,
            &[Path::new("/added")],
            pane_with_items(&[added_id.0 as u64]),
            Some("current"),
        );
        added.window_id = Some(99);
        db.save_workspace_checked(
            added,
            Some(WorkspaceSerializationGuard {
                restoration_cancelled: Some(Arc::default()),
                serialization_fence: Arc::default(),
            }),
        )
        .await
        .unwrap();
        assert_eq!(
            db.select_row_bound::<_, (Option<String>, Option<u64>)>(
                "SELECT session_id, window_id FROM workspaces WHERE workspace_id = ?",
            )
            .unwrap()(added_id)
            .unwrap(),
            Some((Some("previous".to_owned()), Some(3)))
        );
        let state = model::MultiWorkspaceState {
            active_workspace_id: Some(added_id),
            ..model::MultiWorkspaceState::default()
        };
        assert!(
            db.save_multi_workspace(
                WindowId::from(99),
                state.clone(),
                vec![active_id, added_id],
                Some("current".to_owned()),
                Arc::default(),
                Arc::default(),
                Some(active_id),
                false,
            )
            .await
            .unwrap()
        );
        let checkpoints = db.unfinished_workspace_restores().unwrap();
        assert_eq!(checkpoints.len(), 1);
        let checkpoint = checkpoints.first().unwrap();
        assert_eq!(checkpoint.workspace_ids, [active_id, unloaded_id, added_id]);
        assert_eq!(checkpoint.active_workspace.workspace_id, added_id);
        assert_eq!(
            checkpoint.active_workspace.paths,
            PathList::new(&[Path::new("/added")])
        );
        assert_eq!(checkpoint.scratch_workspaces, [active_id, unloaded_id]);
        assert_eq!(
            db.session_workspaces("current".to_owned())
                .unwrap()
                .into_iter()
                .map(|(id, _, window_id, _)| (id, window_id))
                .collect::<Vec<_>>(),
            [
                (added_id, Some(WindowId::from(99).as_u64())),
                (active_id, Some(WindowId::from(99).as_u64()))
            ]
        );
        assert!(
            db.save_multi_workspace(
                WindowId::from(100),
                model::MultiWorkspaceState {
                    active_workspace_id: Some(active_id),
                    ..state.clone()
                },
                vec![active_id, added_id],
                Some("uncommitted".to_owned()),
                Arc::default(),
                Arc::default(),
                Some(active_id),
                true,
            )
            .await
            .is_err()
        );
        assert!(db.workspace_restore_pending(active_id).unwrap());
        assert_eq!(
            db.session_workspaces("uncommitted".to_owned()).unwrap(),
            Vec::new()
        );
        assert_eq!(
            db.unfinished_workspace_restores()
                .unwrap()
                .first()
                .unwrap()
                .active_workspace
                .workspace_id,
            added_id
        );
        assert!(
            db.save_multi_workspace(
                WindowId::from(99),
                state,
                vec![active_id, unloaded_id, added_id],
                Some("current".to_owned()),
                Arc::default(),
                Arc::default(),
                Some(active_id),
                true,
            )
            .await
            .unwrap()
        );
        assert!(!db.workspace_restore_pending(active_id).unwrap());
    }

    #[gpui::test]
    async fn test_restore_checkpoint_membership_during_flush(cx: &mut gpui::TestAppContext) {
        crate::tests::init_test(cx);
        cx.update(crate::register_serializable_item::<TestItem>);
        let fs = fs::FakeFs::new(cx.executor());
        let directory = unique_test_dir(&fs, "restore-member").await;
        let project = Project::test(fs, [], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let db = cx.cx.update(|cx| WorkspaceDb::global(cx));
        let active_id = db.next_id().await.unwrap();
        let lazy_id = db.next_id().await.unwrap();
        let pending_id = db.next_id().await.unwrap();
        let survivor_id = db.next_id().await.unwrap();
        for id in [active_id, lazy_id, pending_id, survivor_id] {
            db.save_workspace(workspace_with(
                id.0 as u64,
                &[directory.as_path()],
                empty_pane_group(),
                Some("old-session"),
            ))
            .await;
        }
        let key = ProjectGroupKey::new(None, PathList::new(&[directory]));
        let mut records =
            [vec![active_id, lazy_id], vec![pending_id, survivor_id]].map(|workspace_ids| {
                model::SerializedMultiWorkspace {
                    active_workspace: db
                        .session_workspace_for_id(*workspace_ids.first().unwrap())
                        .unwrap(),
                    workspace_ids,
                    scratch_workspaces: Vec::new(),
                    state: model::MultiWorkspaceState::default(),
                }
            });
        records[0]
            .state
            .project_groups
            .push(model::SerializedProjectGroup::from_group(&key, true));
        records[1]
            .state
            .project_groups
            .push(model::SerializedProjectGroup::from_group(&key, true));
        records[1].scratch_workspaces.push(survivor_id);
        let mut plan = cx.cx.update(crate::WorkspaceRestorePlan::new).unwrap();
        let session = multi_workspace.read_with(cx, |multi_workspace, cx| {
            multi_workspace
                .workspace()
                .read(cx)
                .app_state()
                .session
                .clone()
        });
        plan.set_workspaces(&records, session, &mut cx.cx.to_async())
            .await
            .unwrap();
        multi_workspace.update(cx, |multi_workspace, cx| {
            multi_workspace.set_restore_key(active_id);
            multi_workspace.workspace().update(cx, |workspace, _| {
                workspace.set_database_id(active_id);
            });
            multi_workspace.restore_workspace_ids(&[lazy_id], cx);
            multi_workspace.restore_project_groups(
                vec![crate::SerializedProjectGroupState {
                    key: key.clone(),
                    expanded: true,
                }],
                cx,
            );
        });
        for remove_group in [false, true] {
            let (resume, suspended) = oneshot::channel::<()>();
            let suspended = suspended.shared();
            multi_workspace.update_in(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    let executor = cx.background_executor().clone();
                    let item = cx.new(|cx| {
                        TestItem::new(cx).with_serialize(move || {
                            let suspended = suspended.clone();
                            Some(executor.spawn(async move {
                                suspended.await?;
                                Ok(())
                            }))
                        })
                    });
                    workspace.add_item_to_active_pane(Box::new(item), None, true, window, cx);
                });
            });
            cx.run_until_parked();
            let flush = multi_workspace.update_in(cx, |multi_workspace, window, cx| {
                multi_workspace.flush_restoration(window, cx)
            });
            cx.run_until_parked();
            assert!(!flush.is_ready());
            if remove_group {
                assert!(
                    multi_workspace
                        .update_in(cx, |multi_workspace, window, cx| multi_workspace
                            .remove_project_group(&key, window, cx))
                        .await
                        .unwrap()
                );
            } else {
                multi_workspace.update_in(cx, |multi_workspace, window, cx| {
                    multi_workspace.workspace().update(cx, |workspace, cx| {
                        let item = cx.new(TestItem::new);
                        workspace.add_item_to_active_pane(Box::new(item), None, true, window, cx);
                    });
                });
                cx.run_until_parked();
            }
            let recovered = db.unfinished_workspace_restores().unwrap();
            assert_eq!(
                recovered
                    .iter()
                    .map(|record| record.workspace_ids.clone())
                    .collect::<Vec<_>>(),
                vec![
                    if remove_group {
                        vec![active_id]
                    } else {
                        vec![active_id, lazy_id]
                    },
                    vec![pending_id, survivor_id],
                ]
            );
            assert_eq!(
                recovered[0].state.project_groups.len(),
                usize::from(!remove_group)
            );
            assert_eq!(recovered[1].state.project_groups.len(), 1);
            resume.send(()).unwrap();
            assert_eq!(
                flush.await.unwrap_err().to_string(),
                if remove_group {
                    "Workspace membership changed during serialization flush"
                } else {
                    "Workspace changed during serialization flush"
                }
            );
            assert!(db.workspace_restore_pending(active_id).unwrap());
        }
        assert_eq!(
            db.select_row_bound::<_, Option<String>>(
                "SELECT session_id FROM workspaces WHERE workspace_id = ?"
            )
            .unwrap()(lazy_id)
            .unwrap(),
            Some(None)
        );
        for restoring in [true, false] {
            multi_workspace.update_in(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    workspace.set_restoring_workspace(restoring);
                    let item = cx.new(TestItem::new);
                    workspace.add_item_to_active_pane(Box::new(item), None, true, window, cx);
                });
            });
            let flush = multi_workspace.update_in(cx, |multi_workspace, window, cx| {
                multi_workspace.flush_restoration(window, cx)
            });
            flush.await.unwrap();
            assert_eq!(
                db.select_row_bound::<_, usize>(
                    "SELECT COUNT(*) FROM items WHERE workspace_id = ?"
                )
                .unwrap()(active_id)
                .unwrap(),
                Some(if restoring { 4 } else { 5 })
            );
            assert_eq!(
                db.unfinished_workspace_restores()
                    .unwrap()
                    .iter()
                    .map(|workspace| workspace.active_workspace.workspace_id)
                    .collect::<Vec<_>>(),
                if restoring {
                    vec![active_id, pending_id]
                } else {
                    vec![pending_id]
                }
            );
        }
        plan.finish_window(active_id);
        db.close_workspaces(
            None,
            vec![pending_id],
            Vec::new(),
            Arc::default(),
            || Ok(()),
        )
        .await
        .unwrap();
        let recovered = db.unfinished_workspace_restores().unwrap();
        assert_eq!(recovered[0].active_workspace.workspace_id, survivor_id);
        assert_eq!(recovered[0].workspace_ids, vec![survivor_id]);
        assert_eq!(recovered[0].scratch_workspaces, vec![survivor_id]);
        assert_eq!(
            db.select_row_bound::<_, Option<String>>(
                "SELECT session_id FROM workspaces WHERE workspace_id = ?"
            )
            .unwrap()(survivor_id)
            .unwrap(),
            Some(Some("old-session".to_owned()))
        );
        db.write(move |connection| {
            connection.exec_bound("INSERT INTO pane_groups(workspace_id, axis) VALUES (?, ?)")?((
                survivor_id,
                "invalid-axis",
            ))
        })
        .await
        .unwrap();
        assert!(db.workspace_for_id(survivor_id).is_none());
        assert_eq!(
            db.unfinished_workspace_restores().unwrap()[0].workspace_ids,
            vec![survivor_id]
        );
        db.delete_workspace_by_id(survivor_id).await.unwrap();
        assert_eq!(db.unfinished_workspace_restores().unwrap().len(), 0);
        assert!(!db.workspace_restore_pending(pending_id).unwrap());

        for delete_directly in [true, false] {
            for (deleted_id, remaining_id, scratch_ids) in [
                (pending_id, survivor_id, vec![survivor_id]),
                (survivor_id, pending_id, Vec::new()),
            ] {
                db.save_workspace(remote_workspace_with(
                    pending_id.0 as u64,
                    "checkpoint-host",
                    &[Path::new("/remote")],
                ))
                .await;
                let scratch = workspace_with(
                    survivor_id.0 as u64,
                    &[],
                    pane_with_items(&[survivor_id.0 as u64]),
                    Some("old-session"),
                );
                db.save_workspace(scratch.clone()).await;
                let mut record = records[1].clone();
                record.active_workspace = db.session_workspace_for_id(pending_id).unwrap();
                record.state.active_workspace_id = Some(pending_id);
                db.checkpoint_workspace_restores(std::slice::from_ref(&record))
                    .await
                    .unwrap();
                if delete_directly {
                    db.write(move |connection| {
                        connection.exec_bound("DELETE FROM workspaces WHERE workspace_id = ?")?(
                            deleted_id,
                        )
                    })
                    .await
                    .unwrap();
                } else {
                    db.delete_workspace_by_id(deleted_id).await.unwrap();
                }
                let recovered = db.unfinished_workspace_restores().unwrap();
                assert_eq!(recovered.len(), 1);
                assert_eq!(
                    recovered[0].active_workspace,
                    db.session_workspace_for_id(remaining_id).unwrap()
                );
                assert_eq!(recovered[0].workspace_ids, vec![remaining_id]);
                assert_eq!(recovered[0].scratch_workspaces, scratch_ids);
                record.state.active_workspace_id = Some(remaining_id);
                assert_eq!(
                    serde_json::to_value(&recovered[0].state).unwrap(),
                    serde_json::to_value(&record.state).unwrap()
                );
                assert!(db.workspace_restore_pending(pending_id).unwrap());
                assert_eq!(
                    db.workspace_ids_for_close(Some(pending_id), &[]).unwrap(),
                    vec![remaining_id]
                );
                db.checkpoint_workspace_restores(&recovered).await.unwrap();
                assert_eq!(
                    db.select_bound::<_, String>(
                        "SELECT key FROM scoped_kv_store WHERE namespace = ?"
                    )
                    .unwrap()(WORKSPACE_RESTORE_NAMESPACE)
                    .unwrap(),
                    vec![remaining_id.0.to_string()]
                );
                if remaining_id == survivor_id {
                    assert_eq!(
                        db.workspace_for_id(survivor_id).unwrap().center_group,
                        scratch.center_group
                    );
                }
                db.write(move |connection| {
                    connection.exec_bound("DELETE FROM workspaces WHERE workspace_id = ?")?(
                        remaining_id,
                    )
                })
                .await
                .unwrap();
                assert_eq!(db.unfinished_workspace_restores().unwrap().len(), 0);
                assert_eq!(
                    db.workspace_ids_for_close(Some(remaining_id), &[]).unwrap(),
                    Vec::new()
                );
                db.delete_workspace_by_id(remaining_id).await.unwrap();
                assert!(!db.workspace_restore_pending(remaining_id).unwrap());
            }
        }
        db.checkpoint_workspace_restores(&[]).await.unwrap();
    }

    #[gpui::test]
    async fn test_delete_workspace_rolls_back_checkpoint_on_failure() {
        let db = WorkspaceDb(
            db::open_test_db::<(KeyValueStore, WorkspaceDb)>(
                "delete_workspace_checkpoint_rollback",
            )
            .await,
        );
        let active_id = db.next_id().await.unwrap();
        let survivor_id = db.next_id().await.unwrap();
        for id in [active_id, survivor_id] {
            db.save_workspace_checked(
                workspace_with(id.0 as u64, &[], pane_with_items(&[id.0 as u64]), None),
                None,
            )
            .await
            .unwrap();
        }
        let saved_workspace = db.workspace_for_id(active_id).unwrap();
        let read_checkpoints = || {
            db.select_bound::<_, (String, String)>(
                "SELECT key, value FROM scoped_kv_store WHERE namespace = ? ORDER BY key",
            )
            .unwrap()(WORKSPACE_RESTORE_NAMESPACE)
            .unwrap()
        };
        db.write(|connection| {
            connection.exec(
                "CREATE TRIGGER reject_workspace_deletion BEFORE DELETE ON workspaces
                 BEGIN SELECT RAISE(ABORT, 'reject workspace deletion'); END",
            )?()
        })
        .await
        .unwrap();
        for workspace_ids in [vec![active_id], vec![active_id, survivor_id]] {
            db.checkpoint_workspace_restores(&[model::SerializedMultiWorkspace {
                active_workspace: db.session_workspace_for_id(active_id).unwrap(),
                scratch_workspaces: workspace_ids.iter().skip(1).copied().collect(),
                workspace_ids,
                state: model::MultiWorkspaceState {
                    active_workspace_id: Some(active_id),
                    ..model::MultiWorkspaceState::default()
                },
            }])
            .await
            .unwrap();
            let checkpoints = read_checkpoints();
            assert!(db.delete_workspace_by_id(active_id).await.is_err());
            assert_eq!(read_checkpoints(), checkpoints);
            assert_eq!(db.workspace_for_id(active_id).unwrap(), saved_workspace);
        }
        db.write(|connection| connection.exec("DROP TRIGGER reject_workspace_deletion")?())
            .await
            .unwrap();
        db.delete_workspace_by_id(active_id).await.unwrap();
        assert!(!db.workspace_exists(active_id).unwrap());
        assert_eq!(
            db.unfinished_workspace_restores().unwrap()[0].workspace_ids,
            vec![survivor_id]
        );
    }

    #[gpui::test]
    async fn test_remote_restore_selects_saved_identity(cx: &mut gpui::TestAppContext) {
        crate::tests::init_test(cx);
        let fs = fs::FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let db = cx.update(|_, cx| WorkspaceDb::global(cx));
        let saved_id = db.next_id().await.unwrap();
        let newer_id = db.next_id().await.unwrap();
        for id in [saved_id, newer_id] {
            db.save_workspace(remote_workspace_with(
                i64::from(id) as u64,
                "restore-host",
                &[Path::new("/project")],
            ))
            .await;
        }
        db.set_centered_layout(saved_id, true).await.unwrap();
        let options = RemoteConnectionOptions::Ssh(SshConnectionOptions {
            host: "restore-host".into(),
            ..SshConnectionOptions::default()
        });
        let position = cx
            .update(|_, cx| {
                crate::remote_workspace_position_from_db(
                    options.clone(),
                    &[PathBuf::from("/project")],
                    Some(saved_id),
                    cx,
                )
            })
            .await
            .unwrap();
        assert!(position.centered_layout);
        let window = multi_workspace.update_in(cx, |_, window, _| {
            window.window_handle().downcast::<MultiWorkspace>().unwrap()
        });
        let (id, serialized, reservation, _) = cx
            .update(|_, cx| {
                let options = options.clone();
                cx.spawn(async move |cx| {
                    crate::deserialize_remote_project(
                        options,
                        vec![PathBuf::from("/project")],
                        Some(saved_id),
                        None,
                        window,
                        cx,
                    )
                    .await
                })
            })
            .await
            .unwrap();
        assert_eq!(id, saved_id);
        assert_eq!(serialized.unwrap().id, saved_id);
        assert!(
            cx.update(|_, cx| {
                let options = options.clone();
                cx.spawn(async move |cx| {
                    crate::deserialize_remote_project(
                        options,
                        vec![PathBuf::from("/project")],
                        Some(saved_id),
                        None,
                        window,
                        cx,
                    )
                    .await
                })
            })
            .await
            .is_err()
        );
        drop(reservation);
        assert!(
            crate::load_remote_workspace_by_id(
                saved_id,
                &RemoteConnectionOptions::Ssh(SshConnectionOptions {
                    host: "other-host".into(),
                    ..SshConnectionOptions::default()
                }),
                &db
            )
            .is_err()
        );
    }

    #[gpui::test]
    async fn test_new_window_reuses_closed_history(cx: &mut gpui::TestAppContext) {
        crate::tests::init_test(cx);
        let fs = fs::FakeFs::new(cx.executor());
        let directory = unique_test_dir(&fs, "closed-history").await;
        let project = Project::test(fs, [], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let app_state = multi_workspace.read_with(cx, |multi_workspace, cx| {
            multi_workspace.workspace().read(cx).app_state().clone()
        });
        let db = cx.update(|_, cx| WorkspaceDb::global(cx));
        let saved_id = db.next_id().await.unwrap();
        let mut saved = workspace_with(
            i64::from(saved_id) as u64,
            &[directory.as_path()],
            empty_pane_group(),
            None,
        );
        saved.recent_navigation_history = vec![directory.join("remembered.rs")];
        db.save_workspace(saved.clone()).await;
        let options = || crate::OpenOptions {
            open_mode: crate::OpenMode::NewWindow,
            workspace_matching: crate::WorkspaceMatching::None,
            ..crate::OpenOptions::default()
        };
        let restored = cx
            .update(|_, cx| {
                crate::open_paths(
                    std::slice::from_ref(&directory),
                    app_state.clone(),
                    options(),
                    cx,
                )
            })
            .await
            .unwrap();
        assert_eq!(
            restored
                .workspace
                .read_with(cx, |workspace, _| workspace.database_id()),
            Some(saved_id)
        );
        assert_eq!(
            restored.workspace.read_with(cx, |workspace, _| workspace
                .persisted_recent_navigation_history
                .clone()),
            saved.recent_navigation_history
        );
        let closed_id = db.next_id().await.unwrap();
        let mut closed = saved.clone();
        closed.id = closed_id;
        closed.recent_navigation_history = vec![directory.join("closed.rs")];
        db.save_workspace(closed.clone()).await;
        db.set_timestamp_for_tests(closed_id, "2000-01-01 00:00:00".to_owned())
            .await
            .unwrap();
        let independent = cx
            .update(|_, cx| {
                crate::open_paths(
                    std::slice::from_ref(&directory),
                    app_state.clone(),
                    options(),
                    cx,
                )
            })
            .await
            .unwrap();
        assert_eq!(
            independent
                .workspace
                .read_with(cx, |workspace, _| workspace.database_id()),
            Some(closed_id)
        );
        assert_eq!(
            independent.workspace.read_with(cx, |workspace, _| workspace
                .persisted_recent_navigation_history
                .clone()),
            closed.recent_navigation_history
        );
        let fresh = cx
            .update(|_, cx| crate::open_paths(&[directory], app_state, options(), cx))
            .await
            .unwrap();
        let fresh_id = fresh
            .workspace
            .read_with(cx, |workspace, _| workspace.database_id());
        assert_ne!(fresh_id, Some(saved_id));
        assert_ne!(fresh_id, Some(closed_id));
        assert_eq!(
            fresh.workspace.read_with(cx, |workspace, _| workspace
                .persisted_recent_navigation_history
                .clone()),
            Vec::<PathBuf>::new()
        );
    }

    #[gpui::test]
    async fn test_multi_workspace_serializes_on_add_and_remove(cx: &mut gpui::TestAppContext) {
        crate::tests::init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        let project1 = Project::test(fs.clone(), [], cx).await;
        let project2 = Project::test(fs.clone(), [], cx).await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project1.clone(), window, cx));

        multi_workspace.update(cx, |mw, cx| {
            mw.open_sidebar(cx);
        });

        multi_workspace.update_in(cx, |mw, _, cx| {
            mw.set_random_database_id(cx);
        });

        let window_id =
            multi_workspace.update_in(cx, |_, window, _cx| window.window_handle().window_id());

        // --- Add a second workspace ---
        let workspace2 = multi_workspace.update_in(cx, |mw, window, cx| {
            let workspace = cx.new(|cx| crate::Workspace::test_new(project2.clone(), window, cx));
            workspace.update(cx, |ws, _cx| ws.set_random_database_id());
            mw.activate(workspace.clone(), None, window, cx);
            workspace
        });

        // Run background tasks so serialize has a chance to flush.
        cx.run_until_parked();

        // Read back the persisted state and check that the active workspace ID was written.
        let state_after_add = cx.update(|_, cx| read_multi_workspace_state(window_id, cx));
        let active_workspace2_db_id = workspace2.read_with(cx, |ws, _| ws.database_id());
        assert_eq!(
            state_after_add.active_workspace_id, active_workspace2_db_id,
            "After adding a second workspace, the serialized active_workspace_id should match \
             the newly activated workspace's database id"
        );

        // --- Remove the non-active workspace ---
        multi_workspace.update_in(cx, |mw, _window, cx| {
            let active = mw.workspace().clone();
            let ws = mw
                .workspaces()
                .find(|ws| *ws != &active)
                .expect("should have a non-active workspace");
            mw.remove([ws.clone()], RemovalIntent::CloseProject, _window, cx)
                .detach_and_log_err(cx);
        });

        cx.run_until_parked();

        let state_after_remove = cx.update(|_, cx| read_multi_workspace_state(window_id, cx));
        let remaining_db_id =
            multi_workspace.read_with(cx, |mw, cx| mw.workspace().read(cx).database_id());
        assert_eq!(
            state_after_remove.active_workspace_id, remaining_db_id,
            "After removing a workspace, the serialized active_workspace_id should match \
             the remaining active workspace's database id"
        );
    }

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
            identity_paths: None,
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            bookmarks: Default::default(),
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
            recent_navigation_history: Default::default(),
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
            identity_paths: None,
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            bookmarks: Default::default(),
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
            recent_navigation_history: Default::default(),
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
            identity_paths: None,
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            bookmarks: Default::default(),
            breakpoints: collections::BTreeMap::default(),
            session_id: None,
            window_id: None,
            user_toolchains: Default::default(),
            recent_navigation_history: Default::default(),
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
    async fn test_item_persistence_id_allocation() {
        let db = WorkspaceDb::open_test_db("test_item_persistence_id_allocation").await;
        db.write(|connection| {
            connection.exec(
                "CREATE TABLE editors (workspace_id INTEGER, item_id INTEGER);
                 CREATE TABLE terminals (workspace_id INTEGER, item_id INTEGER);
                 CREATE TABLE \"orphan\"\"items\" (item_id INTEGER);",
            )?()?;
            connection.exec(
                "INSERT INTO editors VALUES (1, 100);
                 INSERT INTO terminals VALUES (2, 200);
                 INSERT INTO \"orphan\"\"items\" VALUES (300);",
            )?()
        })
        .await
        .unwrap();

        let mut allocated = futures::future::try_join_all((0..8).map(|_| db.next_item_id()))
            .await
            .unwrap();
        allocated.sort_unstable();
        assert_eq!(allocated, (301..309).collect::<Vec<ItemId>>());

        db.write(|connection| {
            connection.exec(
                "DELETE FROM editors; DELETE FROM terminals; DELETE FROM \"orphan\"\"items\";",
            )?()
        })
        .await
        .unwrap();
        assert_eq!(db.next_item_id().await.unwrap(), 309);

        db.write(|connection| {
            connection.exec_bound(
                "UPDATE item_persistence_id_sequence SET last_id = ? WHERE singleton = 1",
            )?(i64::MAX - 1)
        })
        .await
        .unwrap();
        assert_eq!(db.next_item_id().await.unwrap(), i64::MAX as ItemId);
        assert_eq!(
            db.next_item_id().await.unwrap_err().to_string(),
            "Persisted item IDs exhausted"
        );
        assert_eq!(
            db.select_row::<i64>("SELECT last_id FROM item_persistence_id_sequence")
                .unwrap()()
            .unwrap(),
            Some(i64::MAX)
        );
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
                &mut |_, _, _| false,
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
    async fn test_native_window_state_round_trip() {
        zlog::init_test();

        let db = WorkspaceDb::open_test_db("test_native_window_state_round_trip").await;
        let id = db.next_id().await.unwrap();
        let paths = &["/tmp/native-window-state"];
        let display_uuid = Uuid::new_v4();
        let window_bounds = SerializedWindowBounds(WindowBounds::Windowed(Bounds {
            origin: point(px(100.0), px(200.0)),
            size: size(px(800.0), px(600.0)),
        }));

        db.save_workspace(SerializedWorkspace {
            id,
            paths: PathList::new(paths),
            identity_paths: None,
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: None,
            display: None,
            docks: Default::default(),
            bookmarks: Default::default(),
            breakpoints: Default::default(),
            centered_layout: false,
            session_id: None,
            window_id: None,
            user_toolchains: Default::default(),
            recent_navigation_history: Default::default(),
        })
        .await;
        assert_eq!(db.native_window_state(id).unwrap(), Some((None, None)));
        assert_eq!(
            db.native_window_state(WorkspaceId::from_i64(i64::MAX))
                .unwrap(),
            None
        );

        let state = vec![0u8, 1, 2, 3, 250, 251, 252];
        db.set_window_open_status(id, window_bounds, display_uuid, Some(state.clone()))
            .await
            .unwrap();
        assert_eq!(
            db.native_window_state(id).unwrap(),
            Some((Some(display_uuid), Some(state)))
        );
        let restored = db.workspace_for_roots(paths).unwrap();
        assert_eq!(restored.window_bounds, Some(window_bounds));
        assert_eq!(restored.display, Some(display_uuid));

        let new_state = vec![9u8, 8, 7];
        db.set_window_open_status(id, window_bounds, display_uuid, Some(new_state.clone()))
            .await
            .unwrap();
        assert_eq!(
            db.native_window_state(id).unwrap(),
            Some((Some(display_uuid), Some(new_state)))
        );

        db.set_window_open_status(id, window_bounds, display_uuid, None)
            .await
            .unwrap();
        assert_eq!(
            db.native_window_state(id).unwrap(),
            Some((Some(display_uuid), None))
        );
        assert_eq!(
            db.workspace_for_roots(paths).unwrap().window_bounds,
            Some(window_bounds)
        );
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
                &mut |_, _, _| false,
            )
        })
        .await
        .unwrap();

        let mut workspace_1 = SerializedWorkspace {
            id: WorkspaceId(1),
            paths: PathList::new(&["/tmp", "/tmp2"]),
            identity_paths: None,
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            bookmarks: Default::default(),
            breakpoints: Default::default(),
            session_id: None,
            window_id: None,
            user_toolchains: Default::default(),
            recent_navigation_history: Default::default(),
        };

        let workspace_2 = SerializedWorkspace {
            id: WorkspaceId(2),
            paths: PathList::new(&["/tmp"]),
            identity_paths: None,
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            bookmarks: Default::default(),
            breakpoints: Default::default(),
            session_id: None,
            window_id: None,
            user_toolchains: Default::default(),
            recent_navigation_history: Default::default(),
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

        let db = WorkspaceDb(
            db::open_test_db::<(KeyValueStore, WorkspaceDb)>("test_full_workspace_serialization")
                .await,
        );

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
            identity_paths: None,
            location: SerializedWorkspaceLocation::Local,
            center_group,
            window_bounds: Default::default(),
            bookmarks: Default::default(),
            breakpoints: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            session_id: None,
            window_id: Some(999),
            user_toolchains: Default::default(),
            recent_navigation_history: (0..20)
                .map(|index| PathBuf::from(format!("/tmp2/src/{index}.rs")))
                .collect(),
        };

        db.save_workspace(workspace.clone()).await;

        let round_trip_workspace = db.workspace_for_roots(&["/tmp2", "/tmp"]);
        assert_eq!(workspace, round_trip_workspace.unwrap());

        // Test guaranteed duplicate IDs
        db.save_workspace(workspace.clone()).await;
        db.save_workspace(workspace.clone()).await;

        let round_trip_workspace = db.workspace_for_roots(&["/tmp", "/tmp2"]);
        assert_eq!(workspace, round_trip_workspace.unwrap());

        assert!(db.delete_workspace_by_id(workspace.id).await.is_ok());
        assert!(db.recent_navigation_history(workspace.id).is_empty());
    }

    #[gpui::test]
    async fn test_workspace_assignment() {
        zlog::init_test();

        let db = WorkspaceDb::open_test_db("test_basic_functionality").await;

        let workspace_1 = SerializedWorkspace {
            id: WorkspaceId(1),
            paths: PathList::new(&["/tmp", "/tmp2"]),
            identity_paths: None,
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            bookmarks: Default::default(),
            breakpoints: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            session_id: None,
            window_id: Some(1),
            user_toolchains: Default::default(),
            recent_navigation_history: Default::default(),
        };

        let mut workspace_2 = SerializedWorkspace {
            id: WorkspaceId(2),
            paths: PathList::new(&["/tmp"]),
            identity_paths: None,
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            bookmarks: Default::default(),
            breakpoints: Default::default(),
            session_id: None,
            window_id: Some(2),
            user_toolchains: Default::default(),
            recent_navigation_history: Default::default(),
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
            identity_paths: None,
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            bookmarks: Default::default(),
            breakpoints: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            session_id: None,
            window_id: Some(3),
            user_toolchains: Default::default(),
            recent_navigation_history: Default::default(),
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
            identity_paths: None,
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            bookmarks: Default::default(),
            breakpoints: Default::default(),
            session_id: Some("session-id-1".to_owned()),
            window_id: Some(10),
            user_toolchains: Default::default(),
            recent_navigation_history: Default::default(),
        };

        let workspace_2 = SerializedWorkspace {
            id: WorkspaceId(2),
            paths: PathList::new(&["/tmp2"]),
            identity_paths: None,
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            bookmarks: Default::default(),
            breakpoints: Default::default(),
            session_id: Some("session-id-1".to_owned()),
            window_id: Some(20),
            user_toolchains: Default::default(),
            recent_navigation_history: Default::default(),
        };

        let workspace_3 = SerializedWorkspace {
            id: WorkspaceId(3),
            paths: PathList::new(&["/tmp3"]),
            identity_paths: None,
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            bookmarks: Default::default(),
            breakpoints: Default::default(),
            session_id: Some("session-id-2".to_owned()),
            window_id: Some(30),
            user_toolchains: Default::default(),
            recent_navigation_history: Default::default(),
        };

        let workspace_4 = SerializedWorkspace {
            id: WorkspaceId(4),
            paths: PathList::new(&["/tmp4"]),
            identity_paths: None,
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            bookmarks: Default::default(),
            breakpoints: Default::default(),
            session_id: None,
            window_id: None,
            user_toolchains: Default::default(),
            recent_navigation_history: Default::default(),
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
            identity_paths: None,
            location: SerializedWorkspaceLocation::Remote(
                db.remote_connection(connection_id).unwrap(),
            ),
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            bookmarks: Default::default(),
            breakpoints: Default::default(),
            session_id: Some("session-id-2".to_owned()),
            window_id: Some(50),
            user_toolchains: Default::default(),
            recent_navigation_history: Default::default(),
        };

        let workspace_6 = SerializedWorkspace {
            id: WorkspaceId(6),
            paths: PathList::new(&["/tmp6c", "/tmp6b", "/tmp6a"]),
            identity_paths: None,
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            bookmarks: Default::default(),
            breakpoints: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            session_id: Some("session-id-3".to_owned()),
            window_id: Some(60),
            user_toolchains: Default::default(),
            recent_navigation_history: Default::default(),
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
        assert_eq!(locations[0].0, WorkspaceId(2));
        assert_eq!(locations[0].1, PathList::new(&["/tmp2"]));
        assert_eq!(locations[0].2, Some(20));
        assert_eq!(locations[1].0, WorkspaceId(1));
        assert_eq!(locations[1].1, PathList::new(&["/tmp1"]));
        assert_eq!(locations[1].2, Some(10));

        let locations = db.session_workspaces("session-id-2".to_owned()).unwrap();
        assert_eq!(locations.len(), 2);
        assert_eq!(locations[0].0, WorkspaceId(5));
        assert_eq!(locations[0].1, PathList::default());
        assert_eq!(locations[0].2, Some(50));
        assert_eq!(locations[0].3, Some(connection_id));
        assert_eq!(locations[1].0, WorkspaceId(3));
        assert_eq!(locations[1].1, PathList::new(&["/tmp3"]));
        assert_eq!(locations[1].2, Some(30));

        let locations = db.session_workspaces("session-id-3".to_owned()).unwrap();
        assert_eq!(locations.len(), 1);
        assert_eq!(locations[0].0, WorkspaceId(6));
        assert_eq!(
            locations[0].1,
            PathList::new(&["/tmp6c", "/tmp6b", "/tmp6a"]),
        );
        assert_eq!(locations[0].2, Some(60));
    }

    fn default_workspace<P: AsRef<Path>>(
        paths: &[P],
        center_group: &SerializedPaneGroup,
    ) -> SerializedWorkspace {
        SerializedWorkspace {
            id: WorkspaceId(4),
            paths: PathList::new(paths),
            identity_paths: None,
            location: SerializedWorkspaceLocation::Local,
            center_group: center_group.clone(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            bookmarks: Default::default(),
            breakpoints: Default::default(),
            centered_layout: false,
            session_id: None,
            window_id: None,
            user_toolchains: Default::default(),
            recent_navigation_history: Default::default(),
        }
    }

    #[gpui::test]
    async fn test_last_session_workspace_locations(cx: &mut gpui::TestAppContext) {
        let dir1 = tempfile::TempDir::with_prefix("dir1").unwrap();
        let dir2 = tempfile::TempDir::with_prefix("dir2").unwrap();
        let dir3 = tempfile::TempDir::with_prefix("dir3").unwrap();
        let dir4 = tempfile::TempDir::with_prefix("dir4").unwrap();

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(dir1.path(), json!({})).await;
        fs.insert_tree(dir2.path(), json!({})).await;
        fs.insert_tree(dir3.path(), json!({})).await;
        fs.insert_tree(dir4.path(), json!({})).await;

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
            identity_paths: None,
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            session_id: Some("one-session".to_owned()),
            bookmarks: Default::default(),
            breakpoints: Default::default(),
            window_id: Some(window_id),
            user_toolchains: Default::default(),
            recent_navigation_history: Default::default(),
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
            .last_session_workspace_locations("one-session", stack, fs.as_ref())
            .await
            .unwrap();
        assert_eq!(
            locations,
            [
                SessionWorkspace {
                    workspace_id: WorkspaceId(4),
                    location: SerializedWorkspaceLocation::Local,
                    paths: PathList::new(&[dir4.path()]),
                    window_id: Some(WindowId::from(2u64)),
                },
                SessionWorkspace {
                    workspace_id: WorkspaceId(3),
                    location: SerializedWorkspaceLocation::Local,
                    paths: PathList::new(&[dir3.path()]),
                    window_id: Some(WindowId::from(8u64)),
                },
                SessionWorkspace {
                    workspace_id: WorkspaceId(2),
                    location: SerializedWorkspaceLocation::Local,
                    paths: PathList::new(&[dir2.path()]),
                    window_id: Some(WindowId::from(5u64)),
                },
                SessionWorkspace {
                    workspace_id: WorkspaceId(1),
                    location: SerializedWorkspaceLocation::Local,
                    paths: PathList::new(&[dir1.path()]),
                    window_id: Some(WindowId::from(9u64)),
                },
                SessionWorkspace {
                    workspace_id: WorkspaceId(5),
                    location: SerializedWorkspaceLocation::Local,
                    paths: PathList::new(&[dir1.path(), dir2.path(), dir3.path()]),
                    window_id: Some(WindowId::from(3u64)),
                },
                SessionWorkspace {
                    workspace_id: WorkspaceId(6),
                    location: SerializedWorkspaceLocation::Local,
                    paths: PathList::new(&[dir4.path(), dir3.path(), dir2.path()]),
                    window_id: Some(WindowId::from(4u64)),
                },
            ]
        );
    }

    fn pane_with_items(item_ids: &[ItemId]) -> SerializedPaneGroup {
        SerializedPaneGroup::Pane(SerializedPane::new(
            item_ids
                .iter()
                .map(|id| SerializedItem::new("Terminal", *id, true, false))
                .collect(),
            true,
            0,
        ))
    }

    fn empty_pane_group() -> SerializedPaneGroup {
        SerializedPaneGroup::Pane(SerializedPane::default())
    }

    fn workspace_with(
        id: u64,
        paths: &[&Path],
        center_group: SerializedPaneGroup,
        session_id: Option<&str>,
    ) -> SerializedWorkspace {
        SerializedWorkspace {
            id: WorkspaceId(id as i64),
            paths: PathList::new(paths),
            identity_paths: None,
            location: SerializedWorkspaceLocation::Local,
            center_group,
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            bookmarks: Default::default(),
            breakpoints: Default::default(),
            centered_layout: false,
            session_id: session_id.map(|s| s.to_owned()),
            window_id: Some(id),
            user_toolchains: Default::default(),
            recent_navigation_history: Default::default(),
        }
    }

    fn remote_workspace_with(id: u64, host: &str, paths: &[&Path]) -> SerializedWorkspace {
        SerializedWorkspace {
            id: WorkspaceId(id as i64),
            paths: PathList::new(paths),
            identity_paths: None,
            location: SerializedWorkspaceLocation::Remote(RemoteConnectionOptions::Ssh(
                SshConnectionOptions {
                    host: host.into(),
                    ..Default::default()
                },
            )),
            center_group: empty_pane_group(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            bookmarks: Default::default(),
            breakpoints: Default::default(),
            centered_layout: false,
            session_id: None,
            window_id: Some(id),
            user_toolchains: Default::default(),
            recent_navigation_history: Default::default(),
        }
    }

    async fn local_recent_workspace(
        workspace_id: WorkspaceId,
        paths: PathList,
        timestamp: DateTime<Utc>,
        fs: &dyn Fs,
    ) -> RecentWorkspace {
        let identity_paths = resolve_local_workspace_identity(fs, &paths)
            .await
            .unwrap_or_else(|| paths.clone());
        RecentWorkspace {
            workspace_id,
            location: SerializedWorkspaceLocation::Local,
            paths,
            identity_paths,
            timestamp,
        }
    }

    #[gpui::test]
    async fn test_scratch_only_workspace_restores_from_last_session(cx: &mut gpui::TestAppContext) {
        let fs = fs::FakeFs::new(cx.executor());
        let db =
            WorkspaceDb::open_test_db("test_scratch_only_workspace_restores_from_last_session")
                .await;

        db.save_workspace(workspace_with(1, &[], pane_with_items(&[100]), Some("s1")))
            .await;

        let sessions = db
            .last_session_workspace_locations("s1", None, fs.as_ref())
            .await
            .unwrap();
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].workspace_id, WorkspaceId(1));
        assert!(sessions[0].paths.is_empty());

        let recents = db.recent_project_workspaces(fs.as_ref()).await.unwrap();
        assert!(
            recents
                .iter()
                .all(|workspace| workspace.workspace_id != WorkspaceId(1)),
            "scratch-only workspace must not appear in the recent-projects UI"
        );
    }

    #[gpui::test]
    async fn test_gc_preserves_scratch_inside_window(cx: &mut gpui::TestAppContext) {
        let fs = fs::FakeFs::new(cx.executor());
        let db = WorkspaceDb(
            db::open_test_db::<(KeyValueStore, WorkspaceDb)>(
                "test_gc_preserves_scratch_inside_window",
            )
            .await,
        );

        db.save_workspace(workspace_with(1, &[], empty_pane_group(), None))
            .await;

        db.garbage_collect_workspaces(fs.as_ref(), "current", None)
            .await
            .unwrap();
        assert!(
            db.workspace_for_id(WorkspaceId(1)).is_some(),
            "fresh stale workspace must not be deleted before the 7-day window"
        );
    }

    #[gpui::test]
    async fn test_gc_deletes_stale_outside_window(cx: &mut gpui::TestAppContext) {
        let fs = fs::FakeFs::new(cx.executor());
        let db = WorkspaceDb(
            db::open_test_db::<(KeyValueStore, WorkspaceDb)>(
                "test_gc_deletes_stale_outside_window",
            )
            .await,
        );

        db.save_workspace(workspace_with(1, &[], empty_pane_group(), None))
            .await;
        db.set_timestamp_for_tests(WorkspaceId(1), "2000-01-01 00:00:00".to_owned())
            .await
            .unwrap();

        db.garbage_collect_workspaces(fs.as_ref(), "current", None)
            .await
            .unwrap();
        assert!(
            db.workspace_for_id(WorkspaceId(1)).is_none(),
            "stale empty workspace older than the retention window must be deleted"
        );
    }

    #[gpui::test]
    async fn test_gc_preserves_directory_workspace_with_missing_path(
        cx: &mut gpui::TestAppContext,
    ) {
        let fs = fs::FakeFs::new(cx.executor());
        let db = WorkspaceDb(
            db::open_test_db::<(KeyValueStore, WorkspaceDb)>(
                "test_gc_preserves_directory_workspace_with_missing_path",
            )
            .await,
        );

        let missing_dir = PathBuf::from("/missing-project-dir");
        db.save_workspace(workspace_with(
            1,
            &[missing_dir.as_path()],
            empty_pane_group(),
            None,
        ))
        .await;

        db.garbage_collect_workspaces(fs.as_ref(), "current", None)
            .await
            .unwrap();
        assert!(
            db.workspace_for_id(WorkspaceId(1)).is_some(),
            "a stale workspace within the retention window must be kept"
        );

        db.set_timestamp_for_tests(WorkspaceId(1), "2000-01-01 00:00:00".to_owned())
            .await
            .unwrap();
        db.garbage_collect_workspaces(fs.as_ref(), "current", None)
            .await
            .unwrap();
        assert!(
            db.workspace_for_id(WorkspaceId(1)).is_none(),
            "a stale workspace past the retention window must be deleted"
        );
    }

    #[gpui::test]
    async fn test_gc_preserves_current_and_last_sessions(cx: &mut gpui::TestAppContext) {
        let fs = fs::FakeFs::new(cx.executor());
        let db = WorkspaceDb(
            db::open_test_db::<(KeyValueStore, WorkspaceDb)>(
                "test_gc_preserves_current_and_last_sessions",
            )
            .await,
        );

        db.save_workspace(workspace_with(1, &[], empty_pane_group(), Some("current")))
            .await;
        db.save_workspace(workspace_with(2, &[], empty_pane_group(), Some("last")))
            .await;
        db.save_workspace(workspace_with(3, &[], empty_pane_group(), Some("stale")))
            .await;

        for id in [1, 2, 3] {
            db.set_timestamp_for_tests(WorkspaceId(id), "2000-01-01 00:00:00".to_owned())
                .await
                .unwrap();
        }

        db.garbage_collect_workspaces(fs.as_ref(), "current", Some("last"))
            .await
            .unwrap();

        assert!(
            db.workspace_for_id(WorkspaceId(1)).is_some(),
            "GC must not delete workspaces belonging to the current session"
        );
        assert!(
            db.workspace_for_id(WorkspaceId(2)).is_some(),
            "GC must not delete workspaces belonging to the last session"
        );
        assert!(
            db.workspace_for_id(WorkspaceId(3)).is_none(),
            "GC should still delete stale workspaces from other sessions"
        );
    }

    #[gpui::test]
    async fn test_gc_rechecks_workspace_eligibility_before_delete(cx: &mut gpui::TestAppContext) {
        let fs = fs::FakeFs::new(cx.executor());
        let queued = Arc::new(Mutex::new(None::<VecDeque<Box<dyn FnOnce() + Send>>>));
        let connection = ThreadSafeConnection::builder::<db::AppMigrator>(
            &format!("gc-eligibility-{}", Uuid::new_v4()),
            false,
        )
        .with_connection_initialize_query("PRAGMA foreign_keys=TRUE")
        .with_write_queue_constructor(Box::new({
            let queued = queued.clone();
            move || {
                let queued = queued.clone();
                Box::new(move |write| {
                    let mut queued = queued.lock();
                    if let Some(queue) = queued.as_mut() {
                        queue.push_back(write);
                    } else {
                        drop(queued);
                        write();
                    }
                })
            }
        }))
        .build()
        .await
        .unwrap();
        let db = WorkspaceDb(connection);
        for id in 1..=5 {
            db.save_workspace_checked(workspace_with(id, &[], pane_with_items(&[id]), None), None)
                .await
                .unwrap();
            db.set_timestamp_for_tests(WorkspaceId(id as i64), "2000-01-01 00:00:00".to_owned())
                .await
                .unwrap();
        }
        *queued.lock() = Some(VecDeque::new());
        let mut reopen = Box::pin(db.write(|connection| {
            connection.exec(
                "UPDATE workspaces SET session_id = 'current' WHERE workspace_id = 1;
                 UPDATE workspaces SET timestamp = CURRENT_TIMESTAMP WHERE workspace_id = 2;
                 UPDATE workspaces SET paths = '/reopened', paths_order = '0' WHERE workspace_id = 3;
                 UPDATE workspaces SET window_id = 99 WHERE workspace_id = 4;",
            )?()
        }));
        assert!(futures::poll!(reopen.as_mut()).is_pending());
        let mut collect = Box::pin(db.garbage_collect_workspaces(fs.as_ref(), "current", None));
        assert!(futures::poll!(collect.as_mut()).is_pending());
        assert_eq!(queued.lock().as_ref().unwrap().len(), 2);
        for _ in 0..2 {
            let write = queued.lock().as_mut().unwrap().pop_front().unwrap();
            write();
        }
        reopen.await.unwrap();
        collect.await.unwrap();
        assert_eq!(
            db.select::<WorkspaceId>("SELECT workspace_id FROM workspaces ORDER BY workspace_id")
                .unwrap()()
            .unwrap(),
            [
                WorkspaceId(1),
                WorkspaceId(2),
                WorkspaceId(3),
                WorkspaceId(4)
            ]
        );
        for id in 1..=4 {
            assert_eq!(db.workspace_item_ids(WorkspaceId(id)).unwrap(), [id as u64]);
        }
        *queued.lock() = None;
    }

    #[gpui::test]
    async fn test_gc_deletes_empty_workspace_with_items(cx: &mut gpui::TestAppContext) {
        let fs = fs::FakeFs::new(cx.executor());
        let db = WorkspaceDb(
            db::open_test_db::<(KeyValueStore, WorkspaceDb)>(
                "test_gc_deletes_empty_workspace_with_items",
            )
            .await,
        );

        db.save_workspace(workspace_with(1, &[], pane_with_items(&[100]), None))
            .await;
        db.set_timestamp_for_tests(WorkspaceId(1), "2000-01-01 00:00:00".to_owned())
            .await
            .unwrap();

        db.garbage_collect_workspaces(fs.as_ref(), "current", None)
            .await
            .unwrap();
        assert!(
            db.workspace_for_id(WorkspaceId(1)).is_none(),
            "a stale empty-path workspace must be deleted regardless of its items"
        );
    }

    #[gpui::test]
    async fn test_last_session_restores_workspace_with_missing_paths(
        cx: &mut gpui::TestAppContext,
    ) {
        let fs = fs::FakeFs::new(cx.executor());
        let db =
            WorkspaceDb::open_test_db("test_last_session_restores_workspace_with_missing_paths")
                .await;

        fs.insert_tree("/present", json!({"file.rs": "saved"}))
            .await;
        for paths in [
            vec![Path::new("/gone/file.rs")],
            vec![Path::new("/present"), Path::new("/gone")],
            vec![Path::new("/present/file.rs")],
        ] {
            let workspace = workspace_with(1, &paths, pane_with_items(&[100]), Some("s"));
            db.save_workspace(workspace.clone()).await;

            let sessions = db
                .last_session_workspace_locations("s", None, fs.as_ref())
                .await
                .unwrap();
            assert_eq!(
                sessions,
                vec![SessionWorkspace {
                    workspace_id: workspace.id,
                    location: SerializedWorkspaceLocation::Local,
                    paths: workspace.paths.clone(),
                    window_id: workspace.window_id.map(WindowId::from),
                }]
            );
            assert_eq!(
                db.recent_project_workspaces(fs.as_ref())
                    .await
                    .unwrap()
                    .iter()
                    .map(|workspace| workspace.paths.clone())
                    .collect::<Vec<_>>(),
                Vec::<PathList>::new()
            );
        }
    }

    #[gpui::test]
    async fn test_last_session_workspace_locations_remote(cx: &mut gpui::TestAppContext) {
        let fs = fs::FakeFs::new(cx.executor());
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
            identity_paths: None,
            location: SerializedWorkspaceLocation::Remote(remote_connection),
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            session_id: Some("one-session".to_owned()),
            bookmarks: Default::default(),
            breakpoints: Default::default(),
            window_id: Some(window_id),
            user_toolchains: Default::default(),
            recent_navigation_history: Default::default(),
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
            .last_session_workspace_locations("one-session", stack, fs.as_ref())
            .await
            .unwrap();
        assert_eq!(have.len(), 4);
        assert_eq!(
            have[0],
            SessionWorkspace {
                workspace_id: WorkspaceId(4),
                location: SerializedWorkspaceLocation::Remote(remote_connections[3].clone()),
                paths: PathList::default(),
                window_id: Some(WindowId::from(2u64)),
            }
        );
        assert_eq!(
            have[1],
            SessionWorkspace {
                workspace_id: WorkspaceId(3),
                location: SerializedWorkspaceLocation::Remote(remote_connections[2].clone()),
                paths: PathList::default(),
                window_id: Some(WindowId::from(8u64)),
            }
        );
        assert_eq!(
            have[2],
            SessionWorkspace {
                workspace_id: WorkspaceId(2),
                location: SerializedWorkspaceLocation::Remote(remote_connections[1].clone()),
                paths: PathList::default(),
                window_id: Some(WindowId::from(5u64)),
            }
        );
        assert_eq!(
            have[3],
            SessionWorkspace {
                workspace_id: WorkspaceId(1),
                location: SerializedWorkspaceLocation::Remote(remote_connections[0].clone()),
                paths: PathList::default(),
                window_id: Some(WindowId::from(9u64)),
            }
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

        let mut stored_connections = db.remote_connections().unwrap();
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
        for id in [0, u64::MAX] {
            let options = RemoteConnectionOptions::Mock(MockConnectionOptions { id });
            let connection_id = db
                .get_or_create_remote_connection(options.clone())
                .await
                .unwrap();
            assert_eq!(db.remote_connection(connection_id).unwrap(), options);
            assert_eq!(
                db.get_or_create_remote_connection(options.clone())
                    .await
                    .unwrap(),
                connection_id
            );
            let workspace_id = db.next_id().await.unwrap();
            let mut workspace = workspace_with(
                workspace_id.0 as u64,
                &[Path::new("/mock-project")],
                empty_pane_group(),
                None,
            );
            workspace.location = SerializedWorkspaceLocation::Remote(options.clone());
            db.save_workspace_checked(workspace, None).await.unwrap();
            assert_eq!(
                db.session_workspace_for_id(workspace_id).unwrap().location,
                SerializedWorkspaceLocation::Remote(options.clone())
            );
            stored_connections.insert(connection_id, options);
            let ssh = RemoteConnectionOptions::Ssh(SshConnectionOptions {
                host: id.to_string().into(),
                ..SshConnectionOptions::default()
            });
            let ssh_id = db
                .get_or_create_remote_connection(ssh.clone())
                .await
                .unwrap();
            assert_ne!(ssh_id, connection_id);
            stored_connections.insert(ssh_id, ssh);
        }
        assert_eq!(db.remote_connections().unwrap(), stored_connections);
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
    async fn test_pane_group_normalization() {
        let db = WorkspaceDb::open_test_db("test_pane_group_normalization").await;
        let split = |axis, children, flexes| SerializedPaneGroup::Group {
            axis: SerializedAxis(axis),
            children,
            flexes: Some(flexes),
        };
        let empty = empty_pane_group();
        let empty_split = split(Axis::Vertical, Vec::new(), Vec::new());
        let fallback = SerializedPaneGroup::Pane(SerializedPane::new(Vec::new(), true, 0));
        let first = SerializedPaneGroup::Pane(SerializedPane::new(
            vec![SerializedItem::new("TestItem", 1, true, false)],
            false,
            0,
        ));
        let active = SerializedPaneGroup::Pane(SerializedPane::new(
            vec![SerializedItem::new("TestItem", 2, true, true)],
            true,
            1,
        ));
        let last = SerializedPaneGroup::Pane(SerializedPane::new(
            vec![SerializedItem::new("TestItem", 3, true, false)],
            false,
            0,
        ));
        let multiple = split(
            Axis::Horizontal,
            vec![first.clone(), active.clone(), last.clone()],
            vec![0.5, 1.0, 1.5],
        );
        let uniform = group(Axis::Horizontal, vec![first.clone(), active.clone()]);
        for (name, input, expected) in [
            ("empty root", empty_split.clone(), fallback.clone()),
            (
                "empty nested groups",
                split(
                    Axis::Horizontal,
                    vec![empty.clone(), empty_split.clone()],
                    vec![0.5, 1.5],
                ),
                fallback,
            ),
            (
                "single root child",
                split(
                    Axis::Horizontal,
                    vec![empty.clone(), active.clone()],
                    vec![0.5, 1.5],
                ),
                active.clone(),
            ),
            (
                "single nested child",
                split(
                    Axis::Horizontal,
                    vec![
                        empty_split.clone(),
                        split(
                            Axis::Vertical,
                            vec![empty.clone(), active.clone()],
                            vec![0.5, 1.5],
                        ),
                        empty.clone(),
                    ],
                    vec![0.5, 2.0, 0.5],
                ),
                active.clone(),
            ),
            ("multiple unchanged children", multiple.clone(), multiple),
            (
                "multiple nested children",
                split(
                    Axis::Horizontal,
                    vec![
                        split(
                            Axis::Vertical,
                            vec![empty.clone(), first.clone(), active.clone()],
                            vec![0.5, 1.0, 1.5],
                        ),
                        empty_split,
                        last.clone(),
                    ],
                    vec![1.5, 0.5, 1.0],
                ),
                split(
                    Axis::Horizontal,
                    vec![
                        split(
                            Axis::Vertical,
                            vec![first.clone(), active.clone()],
                            vec![0.8, 1.2],
                        ),
                        last,
                    ],
                    vec![1.2, 0.8],
                ),
            ),
            (
                "mismatched flex count",
                split(
                    Axis::Horizontal,
                    vec![empty.clone(), first.clone(), active.clone()],
                    vec![0.5, 1.5],
                ),
                uniform.clone(),
            ),
            (
                "zero surviving flexes",
                split(
                    Axis::Horizontal,
                    vec![empty, first, active],
                    vec![3.0, 0.0, 0.0],
                ),
                uniform,
            ),
        ] {
            let workspace = default_workspace(&["/pane-normalization"], &input);
            db.save_workspace_checked(workspace.clone(), None)
                .await
                .unwrap();
            assert_eq!(
                db.workspace_for_id(workspace.id).unwrap().center_group,
                expected,
                "{name}"
            );
        }
    }

    #[gpui::test]
    async fn test_delete_unloaded_items_preserves_rows_created_after_snapshot(
        cx: &mut gpui::TestAppContext,
    ) {
        let db = WorkspaceDb::open_test_db(
            "test_delete_unloaded_items_preserves_rows_created_after_snapshot",
        )
        .await;
        let mut workspace = default_workspace(
            &["/cleanup"],
            &SerializedPaneGroup::Pane(SerializedPane::new(Vec::new(), true, 0)),
        );
        workspace.id = WorkspaceId(1);
        db.save_workspace_checked(workspace, None).await.unwrap();
        let last_candidate = MAX_QUERY_PLACEHOLDERS as ItemId + 2;
        db.write(move |connection| {
            connection.exec(
                "CREATE TABLE cleanup_items (
                    workspace_id INTEGER NOT NULL,
                    item_id INTEGER NOT NULL,
                    PRIMARY KEY (workspace_id, item_id)
                ) STRICT",
            )?()?;
            connection.exec_bound(
                "WITH RECURSIVE ids(item_id) AS (
                    VALUES (1) UNION ALL SELECT item_id + 1 FROM ids WHERE item_id < ?
                ) INSERT INTO cleanup_items SELECT 1, item_id FROM ids",
            )?(last_candidate)?;
            connection.exec("INSERT INTO cleanup_items VALUES (2, 3)")?()
        })
        .await
        .unwrap();

        let cleanup = delete_unloaded_items(vec![1, 2], WorkspaceId(1), "cleanup_items", &db);
        cx.run_until_parked();
        assert_eq!(
            db.select_row::<ItemId>("SELECT COUNT(*) FROM cleanup_items WHERE workspace_id = 1")
                .unwrap()()
            .unwrap(),
            Some(last_candidate)
        );

        let new_item_id = last_candidate + 1;
        db.write(move |connection| {
            connection.exec_bound("INSERT INTO cleanup_items VALUES (1, ?)")?(new_item_id)?;
            connection.exec_bound(
                "INSERT INTO items (workspace_id, pane_id, position, kind, item_id, active, preview)
                 SELECT workspace_id, pane_id, 0, 'Cleanup', ?, 1, 0 FROM panes
                 WHERE workspace_id = 1",
            )?(last_candidate)
        })
        .await
        .unwrap();
        cleanup.await.unwrap();

        assert_eq!(
            db.select::<(WorkspaceId, ItemId)>(
                "SELECT workspace_id, item_id FROM cleanup_items ORDER BY workspace_id, item_id"
            )
            .unwrap()()
            .unwrap(),
            vec![
                (WorkspaceId(1), 1),
                (WorkspaceId(1), 2),
                (WorkspaceId(1), last_candidate),
                (WorkspaceId(1), new_item_id),
                (WorkspaceId(2), 3),
            ]
        );
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
            identity_paths: None,
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: None,
            display: None,
            docks: Default::default(),
            bookmarks: Default::default(),
            breakpoints: Default::default(),
            centered_layout: false,
            session_id: None,
            window_id: None,
            user_toolchains: Default::default(),
            recent_navigation_history: Default::default(),
        };

        // Save the workspace (this creates the record with empty paths)
        db.save_workspace(workspace.clone()).await;

        // Save window bounds separately (as the actual code does via set_window_open_status)
        db.set_window_open_status(id, window_bounds, display_uuid, None)
            .await
            .unwrap();

        // Empty workspaces cannot be retrieved by paths (they'd all match).
        // They must be retrieved by workspace_id.
        assert!(db.workspace_for_roots(empty_paths).is_none());

        // Retrieve using workspace_for_id instead
        let retrieved = db.workspace_for_id(id).unwrap();

        // Verify window bounds were persisted
        assert_eq!(retrieved.id, id);
        assert!(retrieved.window_bounds.is_some());
        assert_eq!(retrieved.window_bounds.unwrap().0, window_bounds.0);
        assert!(retrieved.display.is_some());
        assert_eq!(retrieved.display.unwrap(), display_uuid);
    }

    #[gpui::test]
    async fn test_last_session_workspace_locations_groups_by_window_id(
        cx: &mut gpui::TestAppContext,
    ) {
        let dir1 = tempfile::TempDir::with_prefix("dir1").unwrap();
        let dir2 = tempfile::TempDir::with_prefix("dir2").unwrap();
        let dir3 = tempfile::TempDir::with_prefix("dir3").unwrap();
        let dir4 = tempfile::TempDir::with_prefix("dir4").unwrap();
        let dir5 = tempfile::TempDir::with_prefix("dir5").unwrap();

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(dir1.path(), json!({})).await;
        fs.insert_tree(dir2.path(), json!({})).await;
        fs.insert_tree(dir3.path(), json!({})).await;
        fs.insert_tree(dir4.path(), json!({})).await;
        fs.insert_tree(dir5.path(), json!({})).await;

        let db =
            WorkspaceDb::open_test_db("test_last_session_workspace_locations_groups_by_window_id")
                .await;

        // Simulate two MultiWorkspace windows each containing two workspaces,
        // plus one single-workspace window:
        //   Window 10: workspace 1, workspace 2
        //   Window 20: workspace 3, workspace 4
        //   Window 30: workspace 5 (only one)
        //
        // On session restore, the caller should be able to group these by
        // window_id to reconstruct the MultiWorkspace windows.
        let workspaces_data: Vec<(i64, &Path, u64)> = vec![
            (1, dir1.path(), 10),
            (2, dir2.path(), 10),
            (3, dir3.path(), 20),
            (4, dir4.path(), 20),
            (5, dir5.path(), 30),
        ];

        for (id, dir, window_id) in &workspaces_data {
            db.save_workspace(SerializedWorkspace {
                id: WorkspaceId(*id),
                paths: PathList::new(&[*dir]),
                identity_paths: None,
                location: SerializedWorkspaceLocation::Local,
                center_group: Default::default(),
                window_bounds: Default::default(),
                display: Default::default(),
                docks: Default::default(),
                centered_layout: false,
                session_id: Some("test-session".to_owned()),
                bookmarks: Default::default(),
                breakpoints: Default::default(),
                window_id: Some(*window_id),
                user_toolchains: Default::default(),
                recent_navigation_history: Default::default(),
            })
            .await;
        }

        let locations = db
            .last_session_workspace_locations("test-session", None, fs.as_ref())
            .await
            .unwrap();

        // All 5 workspaces should be returned with their window_ids.
        assert_eq!(locations.len(), 5);

        // Every entry should have a window_id so the caller can group them.
        for session_workspace in &locations {
            assert!(
                session_workspace.window_id.is_some(),
                "workspace {:?} missing window_id",
                session_workspace.workspace_id
            );
        }

        // Group by window_id, simulating what the restoration code should do.
        let mut by_window: HashMap<WindowId, Vec<WorkspaceId>> = HashMap::default();
        for session_workspace in &locations {
            if let Some(window_id) = session_workspace.window_id {
                by_window
                    .entry(window_id)
                    .or_default()
                    .push(session_workspace.workspace_id);
            }
        }

        // Should produce 3 windows, not 5.
        assert_eq!(
            by_window.len(),
            3,
            "Expected 3 window groups, got {}: {:?}",
            by_window.len(),
            by_window
        );

        // Window 10 should contain workspaces 1 and 2.
        let window_10 = by_window.get(&WindowId::from(10u64)).unwrap();
        assert_eq!(window_10.len(), 2);
        assert!(window_10.contains(&WorkspaceId(1)));
        assert!(window_10.contains(&WorkspaceId(2)));

        // Window 20 should contain workspaces 3 and 4.
        let window_20 = by_window.get(&WindowId::from(20u64)).unwrap();
        assert_eq!(window_20.len(), 2);
        assert!(window_20.contains(&WorkspaceId(3)));
        assert!(window_20.contains(&WorkspaceId(4)));

        // Window 30 should contain only workspace 5.
        let window_30 = by_window.get(&WindowId::from(30u64)).unwrap();
        assert_eq!(window_30.len(), 1);
        assert!(window_30.contains(&WorkspaceId(5)));
    }

    #[gpui::test]
    async fn test_read_serialized_multi_workspaces_with_state(cx: &mut gpui::TestAppContext) {
        use crate::persistence::model::MultiWorkspaceState;

        // Write multi-workspace state for two windows via the scoped KVP.
        let window_10 = WindowId::from(10u64);
        let window_20 = WindowId::from(20u64);

        let kvp = cx.update(|cx| KeyValueStore::global(cx));

        write_multi_workspace_state(
            &kvp,
            window_10,
            MultiWorkspaceState {
                active_workspace_id: Some(WorkspaceId(2)),
                project_groups: vec![],
                sidebar_open: true,
                sidebar_state: None,
            },
        )
        .await
        .unwrap();

        write_multi_workspace_state(
            &kvp,
            window_20,
            MultiWorkspaceState {
                active_workspace_id: Some(WorkspaceId(3)),
                project_groups: vec![],
                sidebar_open: false,
                sidebar_state: None,
            },
        )
        .await
        .unwrap();

        // Build session workspaces: two in window 10, one in window 20, one with no window.
        let session_workspaces = vec![
            SessionWorkspace {
                workspace_id: WorkspaceId(1),
                location: SerializedWorkspaceLocation::Local,
                paths: PathList::new(&["/a"]),
                window_id: Some(window_10),
            },
            SessionWorkspace {
                workspace_id: WorkspaceId(2),
                location: SerializedWorkspaceLocation::Local,
                paths: PathList::new(&["/b"]),
                window_id: Some(window_10),
            },
            SessionWorkspace {
                workspace_id: WorkspaceId(3),
                location: SerializedWorkspaceLocation::Local,
                paths: PathList::new(&["/c"]),
                window_id: Some(window_20),
            },
            SessionWorkspace {
                workspace_id: WorkspaceId(4),
                location: SerializedWorkspaceLocation::Local,
                paths: PathList::new(&["/d"]),
                window_id: None,
            },
        ];

        let results = cx.update(|cx| read_serialized_multi_workspaces(session_workspaces, cx));

        // Should produce 3 results: window 10, window 20, and the orphan.
        assert_eq!(results.len(), 3);

        // Window 10: active_workspace_id = 2 picks workspace 2 (paths /b), sidebar open.
        let group_10 = &results[0];
        assert_eq!(group_10.active_workspace.workspace_id, WorkspaceId(2));
        assert_eq!(group_10.state.active_workspace_id, Some(WorkspaceId(2)));
        assert_eq!(group_10.workspace_ids, vec![WorkspaceId(2), WorkspaceId(1)]);
        assert_eq!(group_10.state.sidebar_open, true);

        // Window 20: active_workspace_id = 3 picks workspace 3 (paths /c), sidebar closed.
        let group_20 = &results[1];
        assert_eq!(group_20.active_workspace.workspace_id, WorkspaceId(3));
        assert_eq!(group_20.state.active_workspace_id, Some(WorkspaceId(3)));
        assert_eq!(group_20.workspace_ids, vec![WorkspaceId(3)]);
        assert_eq!(group_20.state.sidebar_open, false);

        // Orphan: no active_workspace_id, falls back to first workspace (id 4).
        let group_none = &results[2];
        assert_eq!(group_none.active_workspace.workspace_id, WorkspaceId(4));
        assert_eq!(group_none.state.active_workspace_id, None);
        assert_eq!(group_none.workspace_ids, vec![WorkspaceId(4)]);
        assert_eq!(group_none.state.sidebar_open, false);
    }

    #[gpui::test]
    async fn test_retirement_fences_queued_snapshots_before_foreground_completion(
        cx: &mut gpui::TestAppContext,
    ) {
        crate::tests::init_test(cx);
        let queued = Arc::new(Mutex::new(None::<VecDeque<Box<dyn FnOnce() + Send>>>));
        let connection = ThreadSafeConnection::builder::<db::AppMigrator>(
            &format!("retirement-snapshot-fence-{}", Uuid::new_v4()),
            false,
        )
        .with_connection_initialize_query("PRAGMA foreign_keys=TRUE")
        .with_write_queue_constructor(Box::new({
            let queued = queued.clone();
            move || {
                let queued = queued.clone();
                Box::new(move |write| {
                    let mut queued = queued.lock();
                    if let Some(queue) = queued.as_mut() {
                        queue.push_back(write);
                    } else {
                        drop(queued);
                        write();
                    }
                })
            }
        }))
        .build()
        .await
        .unwrap();
        cx.update(|cx| cx.set_global(db::AppDatabase(connection)));
        let db = cx.update(|cx| WorkspaceDb::global(cx));
        for rollback in [false, true] {
            let fs = fs::FakeFs::new(cx.executor());
            let path = unique_test_dir(&fs, "retirement-fence").await;
            let project = Project::test(fs, [path.as_path()], cx).await;
            let active_id = db.next_id().await.unwrap();
            let unloaded_id = db.next_id().await.unwrap();
            db.save_workspace(workspace_with(
                unloaded_id.0 as u64,
                &[],
                empty_pane_group(),
                Some("previous-session"),
            ))
            .await;
            let (multi_workspace, cx) =
                cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
            let workspace = multi_workspace
                .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
            workspace.update(cx, |workspace, _| workspace.set_database_id(active_id));
            multi_workspace
                .update_in(cx, |multi_workspace, window, cx| {
                    multi_workspace.restore_workspace_ids(&[unloaded_id], cx);
                    multi_workspace.flush_pending_serialization(window, cx)
                })
                .await
                .unwrap();
            cx.run_until_parked();
            workspace.update(cx, |workspace, _| workspace.set_restoring_workspace(true));
            let window =
                cx.update(|window, _| window.window_handle().downcast::<MultiWorkspace>().unwrap());
            let state = cx.update(|_, cx| read_multi_workspace_state(window.window_id(), cx));
            let session_id = workspace.read_with(cx, |workspace, _| workspace.session_id());
            let fence = multi_workspace.read_with(cx, |multi_workspace, _| {
                multi_workspace.serialization_fence.clone()
            });
            let bindings = || {
                db.select_bound::<_, (WorkspaceId, Option<String>, Option<u64>)>(
                    "SELECT workspace_id, session_id, window_id FROM workspaces WHERE workspace_id IN (?, ?) ORDER BY workspace_id",
                )
                .unwrap()((active_id, unloaded_id))
                .unwrap()
            };
            let original_bindings = bindings();
            let retired_bindings = vec![(active_id, None, None), (unloaded_id, None, None)];
            let mut late_workspace = workspace.update_in(cx, |workspace, window, cx| {
                workspace.serialize_workspace_snapshot(window, cx)
            });
            let delayed_workspace = workspace.update_in(cx, |workspace, window, cx| {
                workspace.serialize_workspace_snapshot(window, cx)
            });
            let mut late_membership = Box::pin(db.save_multi_workspace(
                window.window_id(),
                state.clone(),
                vec![active_id, unloaded_id],
                session_id.clone(),
                Arc::default(),
                fence.clone(),
                None,
                false,
            ));
            let delayed_membership = db.save_multi_workspace(
                window.window_id(),
                state,
                vec![active_id, unloaded_id],
                session_id.clone(),
                Arc::default(),
                fence.clone(),
                None,
                false,
            );
            *queued.lock() = Some(VecDeque::new());
            let mut close = Box::pin(db.close_workspaces(
                None,
                vec![active_id, unloaded_id],
                Vec::new(),
                fence.clone(),
                || Ok(()),
            ));
            assert!(futures::poll!(close.as_mut()).is_pending());
            assert!(futures::poll!(late_workspace.as_mut()).is_pending());
            assert!(futures::poll!(late_membership.as_mut()).is_pending());
            assert_eq!(queued.lock().as_ref().unwrap().len(), 3);
            for _ in 0..3 {
                let write = queued.lock().as_mut().unwrap().pop_front().unwrap();
                write();
                assert_eq!(bindings(), retired_bindings, "rollback={rollback}");
                assert_eq!(
                    workspace.read_with(cx, |workspace, _| workspace.session_id()),
                    session_id,
                );
            }
            let snapshot = close.await.unwrap();
            late_workspace.await.unwrap();
            assert!(!late_membership.await.unwrap());
            assert!(fence.load(Ordering::Relaxed));
            workspace.read_with(cx, |workspace, _| {
                assert!(workspace.is_restoring());
                assert!(!workspace.restoration_cancelled.load(Ordering::Relaxed));
            });

            let mut after_retirement = workspace.update_in(cx, |workspace, window, cx| {
                workspace.serialize_workspace_snapshot(window, cx)
            });
            assert!(futures::poll!(after_retirement.as_mut()).is_pending());
            let write = queued.lock().as_mut().unwrap().pop_front().unwrap();
            write();
            after_retirement.await.unwrap();
            multi_workspace.update(cx, |multi_workspace, cx| multi_workspace.serialize(cx));
            cx.run_until_parked();
            assert_eq!(queued.lock().as_ref().unwrap().len(), 1);
            let write = queued.lock().as_mut().unwrap().pop_front().unwrap();
            write();
            assert_eq!(bindings(), retired_bindings);
            assert_eq!(queued.lock().as_ref().unwrap().len(), 0);
            *queued.lock() = None;
            cx.run_until_parked();

            let expected_history = if rollback {
                vec![path.join("after-rollback")]
            } else {
                Vec::new()
            };
            if rollback {
                db.restore_workspace_close(snapshot, vec![active_id, unloaded_id])
                    .await
                    .unwrap();
                workspace.update(cx, |workspace, _| {
                    workspace.persisted_recent_navigation_history = expected_history.clone();
                });
                multi_workspace
                    .update_in(cx, |multi_workspace, window, cx| {
                        multi_workspace.resume_serialization(window, cx);
                        multi_workspace.flush_pending_serialization(window, cx)
                    })
                    .await
                    .unwrap();
                let resumed = multi_workspace.read_with(cx, |multi_workspace, _| {
                    multi_workspace.serialization_fence.clone()
                });
                assert!(!Arc::ptr_eq(&fence, &resumed));
                assert!(!resumed.load(Ordering::Relaxed));
            }
            assert_eq!(
                db.workspace_for_id(active_id)
                    .unwrap()
                    .recent_navigation_history,
                expected_history
            );
            delayed_workspace.await.unwrap();
            assert!(!delayed_membership.await.unwrap());
            assert!(fence.load(Ordering::Relaxed));
            assert_eq!(
                bindings(),
                if rollback {
                    original_bindings
                } else {
                    retired_bindings
                },
            );
            assert_eq!(
                db.workspace_for_id(active_id)
                    .unwrap()
                    .recent_navigation_history,
                expected_history
            );
            workspace.read_with(cx, |workspace, _| {
                assert!(workspace.is_restoring());
                assert!(!workspace.restoration_cancelled.load(Ordering::Relaxed));
            });
            cx.update(|window, _| window.remove_window());
            cx.run_until_parked();
        }
    }

    #[gpui::test]
    async fn test_close_window_preserves_session_when_final_save_changes(
        cx: &mut gpui::TestAppContext,
    ) {
        crate::tests::init_test(cx);
        cx.update(crate::register_serializable_item::<TestItem>);
        let paused = Arc::new(AtomicBool::new(false));
        let queued = Arc::new(Mutex::new(VecDeque::<Box<dyn FnOnce() + Send>>::new()));
        let (writes, mut pending_writes) =
            futures::channel::mpsc::unbounded::<Box<dyn FnOnce() + Send>>();
        let writer = cx.background_executor.spawn(async move {
            while let Some(write) = pending_writes.next().await {
                write();
            }
        });
        let connection = ThreadSafeConnection::builder::<db::AppMigrator>(
            &format!("close-window-{}", Uuid::new_v4()),
            false,
        )
        .with_connection_initialize_query("PRAGMA foreign_keys=TRUE")
        .with_write_queue_constructor(Box::new({
            let paused = paused.clone();
            let queued = queued.clone();
            let writes = writes.clone();
            move || {
                let paused = paused.clone();
                let queued = queued.clone();
                let writes = writes.clone();
                Box::new(move |write| {
                    if paused.load(Ordering::Relaxed) {
                        queued.lock().push_back(write);
                    } else {
                        writes.unbounded_send(write).unwrap();
                    }
                })
            }
        }))
        .build()
        .await
        .unwrap();
        cx.update(|cx| cx.set_global(db::AppDatabase(connection)));
        let db = cx.update(|cx| WorkspaceDb::global(cx));
        db.write(|connection| {
            connection.exec(
                "CREATE TABLE close_payloads (workspace_id INTEGER PRIMARY KEY, contents INTEGER NOT NULL) STRICT",
            )?()
        })
        .await
        .unwrap();
        for (failure, finish_plan_entry) in [
            ("edit", false),
            ("payload", false),
            ("retirement", false),
            ("post-retirement", false),
            ("post-retirement", true),
        ] {
            let project = Project::test(fs::FakeFs::new(cx.executor()), [], cx).await;
            let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
                MultiWorkspace::test_new(project.clone(), window, cx)
            });
            let workspace = multi_workspace
                .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
            let active_id = db.next_id().await.unwrap();
            let lazy_id = db.next_id().await.unwrap();
            let pending_id = db.next_id().await.unwrap();
            let ids = vec![active_id, lazy_id, pending_id];
            for id in &ids {
                db.save_workspace(workspace_with(
                    id.0 as u64,
                    &[],
                    empty_pane_group(),
                    Some("previous-session"),
                ))
                .await;
            }
            let record = model::SerializedMultiWorkspace {
                active_workspace: db.session_workspace_for_id(active_id).unwrap(),
                workspace_ids: ids.clone(),
                scratch_workspaces: vec![lazy_id, pending_id],
                state: model::MultiWorkspaceState::default(),
            };
            let mut records = vec![record];
            if finish_plan_entry {
                let remote_id = db.next_id().await.unwrap();
                db.save_workspace(remote_workspace_with(
                    remote_id.0 as u64,
                    "pending-host",
                    &[Path::new("/pending-project")],
                ))
                .await;
                records.push(model::SerializedMultiWorkspace {
                    active_workspace: db.session_workspace_for_id(remote_id).unwrap(),
                    workspace_ids: vec![remote_id],
                    scratch_workspaces: Vec::new(),
                    state: model::MultiWorkspaceState::default(),
                });
            }
            let app_state = workspace.read_with(cx, |workspace, _| workspace.app_state().clone());
            let mut plan = cx.cx.update(crate::WorkspaceRestorePlan::new).unwrap();
            plan.set_workspaces(&records, app_state.session.clone(), &mut cx.cx.to_async())
                .await
                .unwrap();
            if finish_plan_entry {
                plan.finish_window(active_id);
            }
            cx.cx.add_window(|window, cx| {
                let workspace = cx.new(|cx| Workspace::new(None, project, app_state, window, cx));
                MultiWorkspace::new(workspace, window, cx)
            });
            let armed = Arc::new(AtomicBool::new(false));
            let entered = Arc::new(AtomicBool::new(false));
            let payload = Arc::new(AtomicUsize::new(0));
            let (resume, suspended) = oneshot::channel::<()>();
            let suspended = suspended.shared();
            let item = multi_workspace.update_in(cx, |multi_workspace, window, cx| {
                multi_workspace.set_restore_key(active_id);
                multi_workspace.restore_workspace_ids(&[lazy_id], cx);
                workspace.update(cx, |workspace, cx| {
                    workspace.set_database_id(active_id);
                    workspace.set_restoring_workspace(true);
                    let executor = cx.background_executor().clone();
                    let item = cx.new(|cx| {
                        let armed = armed.clone();
                        let entered = entered.clone();
                        let payload = payload.clone();
                        let db = db.clone();
                        TestItem::new(cx).with_serialize(move || {
                            let wait = armed.load(Ordering::Relaxed);
                            if wait {
                                entered.store(true, Ordering::Relaxed);
                            }
                            let contents = payload.load(Ordering::Relaxed) as u64;
                            let suspended = suspended.clone();
                            let db = db.clone();
                            Some(executor.spawn(async move {
                                if wait {
                                    suspended.await?;
                                    anyhow::ensure!(failure != "payload", "close payload failed");
                                }
                                db.write(move |connection| {
                                    connection.exec_bound(
                                        "INSERT OR REPLACE INTO close_payloads VALUES (?, ?)",
                                    )?((active_id, contents))
                                })
                                .await
                            }))
                        })
                    });
                    workspace.add_item_to_active_pane(
                        Box::new(item.clone()),
                        None,
                        true,
                        window,
                        cx,
                    );
                    item
                })
            });
            multi_workspace
                .update_in(cx, |multi_workspace, window, cx| {
                    multi_workspace.flush_pending_serialization(window, cx)
                })
                .await
                .unwrap();
            cx.run_until_parked();
            let window =
                cx.update(|window, _| window.window_handle().downcast::<MultiWorkspace>().unwrap());
            let session_id = workspace.read_with(cx, |workspace, _| workspace.session_id.clone());
            let expected_bindings = vec![
                (
                    active_id,
                    session_id.clone(),
                    Some(window.window_id().as_u64()),
                ),
                (
                    lazy_id,
                    session_id.clone(),
                    Some(window.window_id().as_u64()),
                ),
                (
                    pending_id,
                    Some("previous-session".to_owned()),
                    Some(pending_id.0 as u64),
                ),
            ];
            let checkpoints = db
                .select_bound::<_, (String, String)>(
                    "SELECT key, value FROM scoped_kv_store WHERE namespace = ? ORDER BY key",
                )
                .unwrap()(WORKSPACE_RESTORE_NAMESPACE)
            .unwrap();
            if failure == "retirement" {
                db.write(|connection| {
                    connection.exec(
                        "CREATE TRIGGER reject_close BEFORE DELETE ON scoped_kv_store
                         WHEN OLD.namespace = 'unfinished_workspace_restores'
                         BEGIN SELECT RAISE(ABORT, 'close retirement failed'); END",
                    )?()
                })
                .await
                .unwrap();
            }
            let retirement_fence = multi_workspace.read_with(cx, |multi_workspace, _| {
                multi_workspace.serialization_fence.clone()
            });
            armed.store(true, Ordering::Relaxed);
            let mut async_cx = cx.cx.to_async();
            let mut close = Box::pin(crate::prepare_window_to_close(
                window,
                crate::CloseIntent::CloseWindow,
                &mut async_cx,
            ));
            while !entered.load(Ordering::Relaxed) {
                assert!(futures::poll!(close.as_mut()).is_pending());
                cx.run_until_parked();
            }
            assert!(futures::poll!(close.as_mut()).is_pending());
            if failure == "edit" {
                payload.store(1, Ordering::Relaxed);
                workspace.update(cx, |workspace, cx| {
                    workspace
                        .enqueue_item_serialization(item.to_serializable_item_handle(cx).unwrap())
                        .unwrap();
                });
            }
            if failure == "post-retirement" {
                paused.store(true, Ordering::Relaxed);
            }
            resume.send(()).unwrap();
            if failure == "post-retirement" {
                while db.workspace_restore_pending(active_id).unwrap() {
                    assert!(futures::poll!(close.as_mut()).is_pending());
                    cx.run_until_parked();
                    let write = queued.lock().pop_front();
                    if let Some(write) = write {
                        write();
                    }
                }
                assert_eq!(
                    workspace.read_with(cx, |workspace, _| workspace.session_id()),
                    session_id
                );
                assert!(retirement_fence.load(Ordering::Relaxed));
                assert!(!workspace.read_with(cx, |workspace, _| {
                    workspace.restoration_cancelled.load(Ordering::Relaxed)
                }));
                payload.store(1, Ordering::Relaxed);
                workspace.update(cx, |workspace, cx| {
                    workspace
                        .enqueue_item_serialization(item.to_serializable_item_handle(cx).unwrap())
                        .unwrap();
                });
                cx.run_until_parked();
                multi_workspace.read_with(cx, |multi_workspace, _| {
                    assert!(Arc::ptr_eq(
                        &retirement_fence,
                        &multi_workspace.serialization_fence,
                    ));
                    assert!(multi_workspace.serialization_fence.load(Ordering::Relaxed));
                });
                paused.store(false, Ordering::Relaxed);
                for write in queued.lock().drain(..) {
                    writes.unbounded_send(write).unwrap();
                }
            }
            let error = close.await.expect_err(failure);
            assert_eq!(
                error.root_cause().to_string(),
                match failure {
                    "payload" => "close payload failed",
                    "retirement" => {
                        "Sqlite call failed with code 1811 and message: Some(\"close retirement failed\")"
                    }
                    _ => "Workspace changed during serialization flush",
                }
            );
            assert!(window.read_with(cx, |_, _| ()).is_ok());
            multi_workspace.read_with(cx, |multi_workspace, _| {
                assert!(!Arc::ptr_eq(
                    &retirement_fence,
                    &multi_workspace.serialization_fence
                ));
                assert!(retirement_fence.load(Ordering::Relaxed));
                assert!(!multi_workspace.serialization_fence.load(Ordering::Relaxed));
            });
            assert_eq!(
                workspace.read_with(cx, |workspace, _| workspace.session_id.clone()),
                session_id
            );
            assert!(!multi_workspace.read_with(cx, |multi_workspace, _| {
                multi_workspace.is_closing_window()
            }));
            assert!(!plan.was_window_closed(active_id));
            assert_eq!(db.select_bound::<_, (WorkspaceId, Option<String>, Option<u64>)>(
                "SELECT workspace_id, session_id, window_id FROM workspaces WHERE workspace_id IN (?, ?, ?) ORDER BY workspace_id",
            ).unwrap()((active_id, lazy_id, pending_id)).unwrap(), expected_bindings);
            assert_eq!(
                db.select_bound::<_, (String, String)>(
                    "SELECT key, value FROM scoped_kv_store WHERE namespace = ? ORDER BY key",
                )
                .unwrap()(WORKSPACE_RESTORE_NAMESPACE)
                .unwrap(),
                checkpoints
            );
            if failure == "retirement" {
                db.write(|connection| connection.exec("DROP TRIGGER reject_close")?())
                    .await
                    .unwrap();
            }
            armed.store(false, Ordering::Relaxed);
            let mut async_cx = cx.to_async();
            let mut close = Box::pin(crate::prepare_window_to_close(
                window,
                crate::CloseIntent::CloseWindow,
                &mut async_cx,
            ));
            while workspace.read_with(cx, |workspace, _| workspace.session_id().is_some()) {
                assert!(futures::poll!(close.as_mut()).is_pending());
                cx.run_until_parked();
            }
            multi_workspace.update(cx, |multi_workspace, _| {
                assert!(multi_workspace.is_closing_window());
                assert!(multi_workspace.begin_retirement().is_err());
            });
            assert!(close.await.unwrap());
            assert!(!multi_workspace.read_with(cx, |multi_workspace, _| {
                multi_workspace.is_closing_window()
            }));
            assert!(plan.was_window_closed(active_id));
            assert!(!db.workspace_restore_pending(active_id).unwrap());
            assert_eq!(db.select_bound::<_, (WorkspaceId, Option<String>, Option<u64>)>(
                "SELECT workspace_id, session_id, window_id FROM workspaces WHERE workspace_id IN (?, ?, ?) ORDER BY workspace_id",
            ).unwrap()((active_id, lazy_id, pending_id)).unwrap(), vec![
                (active_id, None, None), (lazy_id, None, None), (pending_id, None, None),
            ]);
            assert_eq!(
                db.select_row_bound::<_, u64>(
                    "SELECT contents FROM close_payloads WHERE workspace_id = ?",
                )
                .unwrap()(active_id)
                .unwrap(),
                Some(u64::from(failure == "edit" || failure == "post-retirement"))
            );
            window
                .update(cx, |_, window, _| window.remove_window())
                .unwrap();
        }
        cx.update(|cx| cx.shutdown());
        cx.run_until_parked();
        drop(writer);
    }

    #[gpui::test]
    async fn test_pending_serialization_flushed_on_shutdown(cx: &mut gpui::TestAppContext) {
        crate::tests::init_test(cx);

        let app_state = cx.update(crate::AppState::test);
        cx.update(|cx| crate::init(app_state.clone(), cx));

        let fs = fs::FakeFs::new(cx.executor());
        let dir = unique_test_dir(&fs, "shutdown-flush").await;
        let project = Project::test(fs.clone(), [dir.as_path()], cx).await;

        let (multi_workspace, vcx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));

        let workspace = multi_workspace.read_with(vcx, |mw, _| mw.workspace().clone());

        let db = vcx.update(|_, cx| WorkspaceDb::global(cx));
        let workspace_id = db.next_id().await.unwrap();
        workspace.update(vcx, |ws, _cx| {
            ws.set_database_id(workspace_id);
        });

        multi_workspace.update_in(vcx, |multi_workspace, window, cx| {
            multi_workspace.workspace().update(cx, |workspace, cx| {
                workspace.serialize_workspace(window, cx)
            })
        });

        let serialized_paths = db
            .workspace_for_id(workspace_id)
            .expect("next_id should have reserved a row")
            .paths;
        assert_eq!(
            serialized_paths.paths().len(),
            0,
            "the debounced serialization must not have fired yet"
        );

        drop(workspace);
        drop(multi_workspace);
        cx.update(|cx| cx.shutdown());

        let serialized_paths = db
            .workspace_for_id(workspace_id)
            .expect("the workspace row should still exist after shutdown")
            .paths;
        assert_eq!(
            serialized_paths.paths(),
            std::slice::from_ref(&dir),
            "shutdown should flush the pending workspace serialization"
        );
    }

    #[gpui::test]
    async fn test_create_workspace_serialization(cx: &mut gpui::TestAppContext) {
        crate::tests::init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        let project = Project::test(fs.clone(), [], cx).await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));

        // Give the first workspace a database_id.
        multi_workspace.update_in(cx, |mw, _, cx| {
            mw.set_random_database_id(cx);
        });

        let window_id =
            multi_workspace.update_in(cx, |_, window, _cx| window.window_handle().window_id());

        // Create a new workspace via the MultiWorkspace API (triggers next_id()).
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.create_test_workspace(window, cx).detach();
        });

        // Let the async next_id() and re-serialization tasks complete.
        cx.run_until_parked();

        // The new workspace should now have a database_id.
        let new_workspace_db_id =
            multi_workspace.read_with(cx, |mw, cx| mw.workspace().read(cx).database_id());
        assert!(
            new_workspace_db_id.is_some(),
            "New workspace should have a database_id after run_until_parked"
        );

        // The multi-workspace state should record it as the active workspace.
        let state = cx.update(|_, cx| read_multi_workspace_state(window_id, cx));
        assert_eq!(
            state.active_workspace_id, new_workspace_db_id,
            "Serialized active_workspace_id should match the new workspace's database_id"
        );

        // The individual workspace row should exist with real data
        // (not just the bare DEFAULT VALUES row from next_id).
        let workspace_id = new_workspace_db_id.unwrap();
        let db = cx.update(|_, cx| WorkspaceDb::global(cx));
        let serialized = db.workspace_for_id(workspace_id);
        assert!(
            serialized.is_some(),
            "Newly created workspace should be fully serialized in the DB after database_id assignment"
        );
    }

    #[gpui::test]
    async fn test_remove_workspace_clears_session_binding(cx: &mut gpui::TestAppContext) {
        crate::tests::init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        let dir = unique_test_dir(&fs, "remove").await;
        let project1 = Project::test(fs.clone(), [], cx).await;
        let project2 = Project::test(fs.clone(), [], cx).await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project1.clone(), window, cx));

        multi_workspace.update(cx, |mw, cx| {
            mw.open_sidebar(cx);
        });

        multi_workspace.update_in(cx, |mw, _, cx| {
            mw.set_random_database_id(cx);
        });

        let db = cx.update(|_, cx| WorkspaceDb::global(cx));

        // Get a real DB id for workspace2 so the row actually exists.
        let workspace2_db_id = db.next_id().await.unwrap();

        multi_workspace.update_in(cx, |mw, window, cx| {
            let workspace = cx.new(|cx| crate::Workspace::test_new(project2.clone(), window, cx));
            workspace.update(cx, |ws: &mut crate::Workspace, _cx| {
                ws.set_database_id(workspace2_db_id)
            });
            mw.add(workspace.clone(), window, cx);
        });

        // Save a full workspace row to the DB directly.
        let session_id = format!("remove-test-session-{}", Uuid::new_v4());
        db.save_workspace(SerializedWorkspace {
            id: workspace2_db_id,
            paths: PathList::new(&[&dir]),
            identity_paths: None,
            location: SerializedWorkspaceLocation::Local,
            center_group: Default::default(),
            window_bounds: Default::default(),
            display: Default::default(),
            docks: Default::default(),
            centered_layout: false,
            session_id: Some(session_id.clone()),
            bookmarks: Default::default(),
            breakpoints: Default::default(),
            window_id: Some(99),
            user_toolchains: Default::default(),
            recent_navigation_history: Default::default(),
        })
        .await;

        assert!(
            db.workspace_for_id(workspace2_db_id).is_some(),
            "Workspace2 should exist in DB before removal"
        );

        // Remove workspace at index 1 (the second workspace).
        multi_workspace.update_in(cx, |mw, window, cx| {
            let ws = mw.workspaces().nth(1).unwrap().clone();
            mw.remove([ws], RemovalIntent::CloseProject, window, cx)
                .detach_and_log_err(cx);
        });

        cx.run_until_parked();

        // The row should still exist so it continues to appear in recent
        // projects, but the session binding should be cleared so it is not
        // restored as part of any future session.
        assert!(
            db.workspace_for_id(workspace2_db_id).is_some(),
            "Removed workspace's DB row should be preserved for recent projects"
        );

        let session_workspaces = db
            .last_session_workspace_locations("remove-test-session", None, fs.as_ref())
            .await
            .unwrap();
        let restored_ids: Vec<WorkspaceId> = session_workspaces
            .iter()
            .map(|sw| sw.workspace_id)
            .collect();
        assert!(
            !restored_ids.contains(&workspace2_db_id),
            "Removed workspace should not appear in session restoration"
        );
    }

    #[gpui::test]
    async fn test_remove_workspace_not_restored_as_zombie(cx: &mut gpui::TestAppContext) {
        crate::tests::init_test(cx);
        let fs = fs::FakeFs::new(cx.executor());
        let directory = unique_test_dir(&fs, "zombie").await;
        let project1 = Project::test(fs.clone(), [directory.as_path()], cx).await;
        let project2 = Project::test(fs.clone(), [directory.as_path()], cx).await;
        let db = cx.update(|cx| WorkspaceDb::global(cx));
        let retained_id = db.next_id().await.unwrap();
        let removed_id = db.next_id().await.unwrap();
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project1, window, cx));
        let (session_id, window_id, removed_workspace) =
            multi_workspace.update_in(cx, |multi_workspace, window, cx| {
                multi_workspace.open_sidebar(cx);
                multi_workspace.workspace().update(cx, |workspace, _| {
                    workspace.set_database_id(retained_id);
                });
                let session_id = multi_workspace.workspace().read(cx).session_id().unwrap();
                let workspace = cx.new(|cx| crate::Workspace::test_new(project2, window, cx));
                workspace.update(cx, |workspace, _| {
                    workspace.set_database_id(removed_id);
                    workspace.session_id = Some(session_id.clone());
                });
                multi_workspace.add(workspace.clone(), window, cx);
                (session_id, window.window_handle().window_id(), workspace)
            });
        cx.run_until_parked();
        for id in [retained_id, removed_id] {
            let mut saved = workspace_with(
                id.0 as u64,
                &[directory.as_path()],
                empty_pane_group(),
                Some(&session_id),
            );
            saved.window_id = Some(window_id.as_u64());
            db.save_workspace(saved).await;
        }
        let key = ProjectGroupKey::new(None, PathList::new(&[directory]));
        db.checkpoint_workspace_restores(&[model::SerializedMultiWorkspace {
            active_workspace: db.session_workspace_for_id(retained_id).unwrap(),
            workspace_ids: vec![retained_id, removed_id],
            scratch_workspaces: Vec::new(),
            state: model::MultiWorkspaceState {
                active_workspace_id: Some(retained_id),
                project_groups: vec![model::SerializedProjectGroup::from_group(&key, true)],
                ..model::MultiWorkspaceState::default()
            },
        }])
        .await
        .unwrap();
        assert!(
            multi_workspace
                .update_in(cx, |multi_workspace, window, cx| {
                    multi_workspace.remove(
                        [removed_workspace],
                        RemovalIntent::CloseProject,
                        window,
                        cx,
                    )
                })
                .await
                .unwrap()
        );
        cx.run_until_parked();
        let locations = db
            .last_session_workspace_locations(&session_id, None, fs.as_ref())
            .await
            .unwrap();
        assert_eq!(
            locations
                .iter()
                .map(|workspace| workspace.workspace_id)
                .collect::<Vec<_>>(),
            vec![retained_id]
        );
        let recovered = db.unfinished_workspace_restores().unwrap();
        assert_eq!(recovered.len(), 1);
        assert_eq!(recovered[0].workspace_ids, vec![retained_id]);
        assert_eq!(
            recovered[0]
                .state
                .project_groups
                .iter()
                .cloned()
                .map(ProjectGroupKey::from)
                .collect::<Vec<_>>(),
            vec![key]
        );
        db.checkpoint_workspace_restores(&[]).await.unwrap();
    }

    #[gpui::test]
    async fn test_workspace_retirement_is_scoped_and_cancellation_safe(
        cx: &mut gpui::TestAppContext,
    ) {
        crate::tests::init_test(cx);
        cx.update(crate::register_serializable_item::<TestItem>);
        let paused = Arc::new(AtomicBool::new(false));
        let queued = Arc::new(Mutex::new(VecDeque::<Box<dyn FnOnce() + Send>>::new()));
        let (writes, mut pending_writes) =
            futures::channel::mpsc::unbounded::<Box<dyn FnOnce() + Send>>();
        let writer = cx.background_executor.spawn(async move {
            while let Some(write) = pending_writes.next().await {
                write();
            }
        });
        let connection = ThreadSafeConnection::builder::<db::AppMigrator>(
            &format!("workspace-retirement-{}", Uuid::new_v4()),
            false,
        )
        .with_connection_initialize_query("PRAGMA foreign_keys=TRUE")
        .with_write_queue_constructor(Box::new({
            let paused = paused.clone();
            let queued = queued.clone();
            let writes = writes.clone();
            move || {
                let paused = paused.clone();
                let queued = queued.clone();
                let writes = writes.clone();
                Box::new(move |write| {
                    if paused.load(Ordering::Relaxed) {
                        queued.lock().push_back(write);
                    } else {
                        writes.unbounded_send(write).unwrap();
                    }
                })
            }
        }))
        .build()
        .await
        .unwrap();
        cx.update(|cx| cx.set_global(db::AppDatabase(connection)));
        let db = cx.update(|cx| WorkspaceDb::global(cx));
        for change in ["unrelated", "cancel", "target", "membership", "overlap"] {
            let fs = fs::FakeFs::new(cx.executor());
            let active_path = unique_test_dir(&fs, "retained").await;
            let removed_path = unique_test_dir(&fs, "retired").await;
            let active_project = Project::test(fs.clone(), [active_path.as_path()], cx).await;
            let removed_project = Project::test(fs, [removed_path.as_path()], cx).await;
            let active_id = db.next_id().await.unwrap();
            let removed_id = db.next_id().await.unwrap();
            let additional_id = db.next_id().await.unwrap();
            let (multi_workspace, cx) = cx
                .add_window_view(|window, cx| MultiWorkspace::test_new(active_project, window, cx));
            let synchronous_change = Arc::new(AtomicBool::new(false));
            let unrelated_writes = Arc::new(AtomicUsize::new(0));
            let (active_workspace, active_item, removed_workspace, removed_item) = multi_workspace
                .update_in(cx, |multi_workspace, window, cx| {
                    let active_workspace = multi_workspace.workspace().clone();
                    let active_item = active_workspace.update(cx, |workspace, cx| {
                        workspace.set_database_id(active_id);
                        let unrelated_writes = unrelated_writes.clone();
                        let item = cx.new(|cx| {
                            TestItem::new(cx).with_serialize(move || {
                                unrelated_writes.fetch_add(1, Ordering::Relaxed);
                                Some(Task::ready(Ok(())))
                            })
                        });
                        workspace.add_item_to_active_pane(
                            Box::new(item.clone()),
                            None,
                            true,
                            window,
                            cx,
                        );
                        item
                    });
                    let removed_workspace =
                        cx.new(|cx| Workspace::test_new(removed_project, window, cx));
                    let removed_item = removed_workspace.update(cx, |workspace, cx| {
                        workspace.set_database_id(removed_id);
                        workspace.session_id = active_workspace.read(cx).session_id();
                        let revision = workspace.serialization_revision.clone();
                        let synchronous_change = synchronous_change.clone();
                        let item = cx.new(|cx| {
                            TestItem::new(cx).with_serialize(move || {
                                if synchronous_change.swap(false, Ordering::Relaxed) {
                                    revision.fetch_add(1, Ordering::Relaxed);
                                }
                                Some(Task::ready(Ok(())))
                            })
                        });
                        workspace.add_item_to_active_pane(
                            Box::new(item.clone()),
                            None,
                            false,
                            window,
                            cx,
                        );
                        item
                    });
                    multi_workspace.add(removed_workspace.clone(), window, cx);
                    (
                        active_workspace,
                        active_item,
                        removed_workspace,
                        removed_item,
                    )
                });
            cx.run_until_parked();
            synchronous_change.store(true, Ordering::Relaxed);
            multi_workspace
                .update_in(cx, |multi_workspace, window, cx| {
                    multi_workspace.flush_pending_serialization(window, cx)
                })
                .await
                .unwrap();
            cx.run_until_parked();
            let groups = multi_workspace.read_with(cx, |multi_workspace, _| {
                multi_workspace.project_group_keys()
            });
            db.checkpoint_workspace_restores(&[model::SerializedMultiWorkspace {
                active_workspace: db.session_workspace_for_id(active_id).unwrap(),
                workspace_ids: vec![active_id, removed_id],
                scratch_workspaces: Vec::new(),
                state: model::MultiWorkspaceState {
                    active_workspace_id: Some(active_id),
                    project_groups: groups
                        .iter()
                        .map(|key| model::SerializedProjectGroup::from_group(key, false))
                        .collect(),
                    ..model::MultiWorkspaceState::default()
                },
            }])
            .await
            .unwrap();
            let binding = || {
                db.select_row_bound::<_, (Option<String>, Option<u64>)>(
                    "SELECT session_id, window_id FROM workspaces WHERE workspace_id = ?",
                )
                .unwrap()(removed_id)
                .unwrap()
            };
            let original_binding = binding();
            let original_writes = unrelated_writes.load(Ordering::Relaxed);
            let retirement_fence = multi_workspace.read_with(cx, |multi_workspace, _| {
                multi_workspace.serialization_fence.clone()
            });
            paused.store(true, Ordering::Relaxed);
            let removal = if change == "cancel" {
                cx.update(|_, cx| {
                    let mut settings = AgentSettings::get_global(cx).clone();
                    settings.enabled = false;
                    AgentSettings::override_global(settings, cx);
                });
                None
            } else {
                Some(
                    multi_workspace.update_in(cx, |multi_workspace, window, cx| {
                        multi_workspace.remove(
                            [removed_workspace.clone()],
                            RemovalIntent::CloseProject,
                            window,
                            cx,
                        )
                    }),
                )
            };
            cx.run_until_parked();
            assert_eq!(unrelated_writes.load(Ordering::Relaxed), original_writes);
            if change == "unrelated" {
                active_workspace.update(cx, |workspace, cx| {
                    workspace
                        .enqueue_item_serialization(
                            active_item.to_serializable_item_handle(cx).unwrap(),
                        )
                        .unwrap();
                });
                cx.run_until_parked();
            }
            while binding() != Some((None, None)) {
                let write = queued
                    .lock()
                    .pop_front()
                    .expect("retirement should reach the database");
                write();
                if binding() != Some((None, None)) {
                    cx.run_until_parked();
                }
            }
            assert!(retirement_fence.load(Ordering::Relaxed));
            assert!(!removed_workspace.read_with(cx, |workspace, _| {
                workspace.restoration_cancelled.load(Ordering::Relaxed)
            }));
            if change == "overlap" {
                for compensating in [false, true] {
                    if compensating {
                        removed_workspace.update(cx, |workspace, cx| {
                            workspace
                                .enqueue_item_serialization(
                                    removed_item.to_serializable_item_handle(cx).unwrap(),
                                )
                                .unwrap();
                        });
                        cx.run_until_parked();
                    }
                    let unchanged = multi_workspace.read_with(cx, |multi_workspace, cx| {
                        multi_workspace.serialization_guard(cx)
                    });
                    let queued_writes = queued.lock().len();
                    let competing = multi_workspace.update_in(cx, |multi_workspace, window, cx| {
                        multi_workspace.remove(
                            [removed_workspace.clone()],
                            RemovalIntent::CloseProject,
                            window,
                            cx,
                        )
                    });
                    assert_eq!(
                        competing
                            .now_or_never()
                            .expect("overlapping removal must fail before starting")
                            .unwrap_err()
                            .to_string(),
                        "Workspace retirement is already in progress"
                    );
                    let window = cx.update(|window, _| {
                        window.window_handle().downcast::<MultiWorkspace>().unwrap()
                    });
                    let mut async_cx = cx.cx.to_async();
                    assert_eq!(
                        crate::prepare_window_to_close(
                            window,
                            crate::CloseIntent::CloseWindow,
                            &mut async_cx,
                        )
                        .now_or_never()
                        .expect("overlapping window close must fail before starting")
                        .unwrap_err()
                        .to_string(),
                        "Workspace retirement is already in progress"
                    );
                    for enabled in [false, true, false, true] {
                        cx.update(|_, cx| {
                            let mut settings = AgentSettings::get_global(cx).clone();
                            settings.enabled = enabled;
                            AgentSettings::override_global(settings, cx);
                        });
                    }
                    unchanged().unwrap();
                    assert_eq!(queued.lock().len(), queued_writes);
                    assert_eq!(binding(), Some((None, None)));
                }
            }
            match change {
                "cancel" => {
                    for enabled in [true, false, true] {
                        cx.update(|_, cx| {
                            let mut settings = AgentSettings::get_global(cx).clone();
                            settings.enabled = enabled;
                            AgentSettings::override_global(settings, cx);
                        });
                    }
                }
                "target" => removed_workspace.update(cx, |workspace, cx| {
                    workspace
                        .enqueue_item_serialization(
                            removed_item.to_serializable_item_handle(cx).unwrap(),
                        )
                        .unwrap();
                }),
                "membership" => multi_workspace.update(cx, |multi_workspace, cx| {
                    multi_workspace.restore_workspace_ids(&[additional_id], cx);
                }),
                _ => {}
            }
            if change != "unrelated" {
                cx.run_until_parked();
                multi_workspace.read_with(cx, |multi_workspace, _| {
                    assert!(Arc::ptr_eq(
                        &retirement_fence,
                        &multi_workspace.serialization_fence,
                    ));
                    assert!(multi_workspace.serialization_fence.load(Ordering::Relaxed));
                });
            }
            paused.store(false, Ordering::Relaxed);
            for write in queued.lock().drain(..) {
                writes.unbounded_send(write).unwrap();
            }
            cx.run_until_parked();
            if let Some(removal) = removal {
                let result = removal.await;
                if change == "unrelated" {
                    assert!(result.unwrap());
                } else {
                    assert!(result.is_err());
                }
            }
            let retained = change != "unrelated";
            multi_workspace.read_with(cx, |multi_workspace, cx| {
                assert!(!Arc::ptr_eq(
                    &retirement_fence,
                    &multi_workspace.serialization_fence
                ));
                assert!(retirement_fence.load(Ordering::Relaxed));
                assert!(!multi_workspace.serialization_fence.load(Ordering::Relaxed));
                for workspace in multi_workspace.workspaces() {
                    let workspace = workspace.read(cx);
                    assert!(Arc::ptr_eq(
                        &workspace.serialization_fence,
                        &multi_workspace.serialization_fence,
                    ));
                    assert!(!workspace.restoration_cancelled.load(Ordering::Relaxed));
                }
                assert_eq!(
                    multi_workspace.owns_workspace_id(removed_id, cx),
                    retained,
                    "{change}"
                );
            });
            assert_eq!(
                binding(),
                if retained {
                    original_binding
                } else {
                    Some((None, None))
                },
                "{change}"
            );
            let recovered = db.unfinished_workspace_restores().unwrap();
            let recovered = recovered
                .iter()
                .find(|record| record.active_workspace.workspace_id == active_id)
                .unwrap();
            let mut ids = recovered.workspace_ids.clone();
            ids.sort_unstable();
            assert_eq!(
                ids,
                if retained {
                    vec![active_id, removed_id]
                } else {
                    vec![active_id]
                },
                "{change}"
            );
            if retained {
                assert_eq!(
                    recovered
                        .state
                        .project_groups
                        .iter()
                        .cloned()
                        .map(ProjectGroupKey::from)
                        .collect::<Vec<_>>(),
                    groups
                );
            }
            if change == "overlap" {
                assert!(
                    multi_workspace
                        .update_in(cx, |multi_workspace, window, cx| {
                            multi_workspace.remove(
                                [removed_workspace.clone()],
                                RemovalIntent::CloseProject,
                                window,
                                cx,
                            )
                        })
                        .await
                        .unwrap()
                );
                assert_eq!(binding(), Some((None, None)));
                let recovered = db.unfinished_workspace_restores().unwrap();
                let recovered = recovered
                    .iter()
                    .find(|record| record.active_workspace.workspace_id == active_id)
                    .unwrap();
                assert_eq!(recovered.workspace_ids, vec![active_id]);
                paused.store(true, Ordering::Relaxed);
                let window = cx.update(|window, _| {
                    window.window_handle().downcast::<MultiWorkspace>().unwrap()
                });
                let mut async_cx = cx.cx.to_async();
                let mut closing = Box::pin(crate::prepare_window_to_close(
                    window,
                    crate::CloseIntent::ReplaceWindow,
                    &mut async_cx,
                ));
                assert!(futures::poll!(closing.as_mut()).is_pending());
                cx.run_until_parked();
                let competing = multi_workspace.update_in(cx, |multi_workspace, window, cx| {
                    let key = active_workspace.read(cx).project_group_key(cx);
                    multi_workspace.remove_project_group(&key, window, cx)
                });
                assert_eq!(
                    competing
                        .now_or_never()
                        .expect("window close must exclude project removal")
                        .unwrap_err()
                        .to_string(),
                    "Workspace retirement is already in progress"
                );
                paused.store(false, Ordering::Relaxed);
                for write in queued.lock().drain(..) {
                    writes.unbounded_send(write).unwrap();
                }
                assert!(closing.await.unwrap());
                assert_eq!(db.unfinished_workspace_restores().unwrap().len(), 0);
            }
            cx.update(|window, _| window.remove_window());
            cx.run_until_parked();
        }
        cx.update(|cx| cx.shutdown());
        cx.run_until_parked();
        drop(writer);
    }

    #[gpui::test]
    async fn test_close_window_quit_app_preserves_all_sidebar_workspaces(
        cx: &mut gpui::TestAppContext,
    ) {
        crate::tests::init_test(cx);

        cx.update_global::<settings::SettingsStore, ()>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.workspace.on_last_window_closed =
                    Some(settings::OnLastWindowClosed::QuitApp);
            });
        });

        let fs = fs::FakeFs::new(cx.executor());
        let dir1 = unique_test_dir(&fs, "close-quit-a").await;
        let dir2 = unique_test_dir(&fs, "close-quit-b").await;
        let project1 = Project::test(fs.clone(), [dir1.as_path()], cx).await;
        let project2 = Project::test(fs.clone(), [dir2.as_path()], cx).await;

        let db = cx.update(|cx| WorkspaceDb::global(cx));
        let ws1_id = db.next_id().await.unwrap();
        let ws2_id = db.next_id().await.unwrap();

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project1.clone(), window, cx));

        multi_workspace.update(cx, |mw, cx| {
            mw.open_sidebar(cx);
        });

        let session_id = format!("close-quit-session-{}", Uuid::new_v4());

        let workspace1 = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        workspace1.update(cx, |ws, _| {
            ws.set_database_id(ws1_id);
            ws.session_id = Some(session_id.clone());
        });

        let workspace2 = multi_workspace.update_in(cx, |mw, window, cx| {
            let workspace = cx.new(|cx| crate::Workspace::test_new(project2.clone(), window, cx));
            workspace.update(cx, |ws, _| {
                ws.set_database_id(ws2_id);
                ws.session_id = Some(session_id.clone());
            });
            mw.add(workspace.clone(), window, cx);
            workspace
        });

        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate(workspace2.clone(), None, window, cx);
        });
        cx.run_until_parked();

        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.close_window(&crate::CloseWindow, window, cx);
        });
        cx.run_until_parked();

        let mut restored_ids = db
            .last_session_workspace_locations(&session_id, None, fs.as_ref())
            .await
            .unwrap()
            .into_iter()
            .map(|workspace| workspace.workspace_id)
            .collect::<Vec<_>>();
        restored_ids.sort_by_key(|id| id.0);
        assert_eq!(restored_ids, vec![ws1_id, ws2_id]);
    }

    #[cfg(target_os = "macos")]
    #[gpui::test]
    async fn test_close_window_platform_default_still_removes_from_session(
        cx: &mut gpui::TestAppContext,
    ) {
        crate::tests::init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        let dir1 = unique_test_dir(&fs, "close-default-a").await;
        let dir2 = unique_test_dir(&fs, "close-default-b").await;
        let project1 = Project::test(fs.clone(), [dir1.as_path()], cx).await;
        let project2 = Project::test(fs.clone(), [dir2.as_path()], cx).await;

        let db = cx.update(|cx| WorkspaceDb::global(cx));
        let ws1_id = db.next_id().await.unwrap();
        let ws2_id = db.next_id().await.unwrap();

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project1.clone(), window, cx));

        multi_workspace.update(cx, |mw, cx| {
            mw.open_sidebar(cx);
        });

        let session_id = format!("close-default-session-{}", Uuid::new_v4());

        let workspace1 = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        workspace1.update(cx, |ws, _| {
            ws.set_database_id(ws1_id);
            ws.session_id = Some(session_id.clone());
        });

        let workspace2 = multi_workspace.update_in(cx, |mw, window, cx| {
            let workspace = cx.new(|cx| crate::Workspace::test_new(project2.clone(), window, cx));
            workspace.update(cx, |ws, _| {
                ws.set_database_id(ws2_id);
                ws.session_id = Some(session_id.clone());
            });
            mw.add(workspace.clone(), window, cx);
            workspace
        });

        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate(workspace2.clone(), None, window, cx);
        });
        cx.run_until_parked();

        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.close_window(&crate::CloseWindow, window, cx);
        });
        cx.run_until_parked();

        let restored_ids = db
            .last_session_workspace_locations(&session_id, None, fs.as_ref())
            .await
            .unwrap()
            .into_iter()
            .map(|workspace| workspace.workspace_id)
            .collect::<Vec<_>>();
        assert_eq!(restored_ids, Vec::new());
    }

    #[gpui::test]
    async fn test_create_workspace_bounds_observer_uses_fresh_id(cx: &mut gpui::TestAppContext) {
        crate::tests::init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        let project = Project::test(fs.clone(), [], cx).await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));

        multi_workspace.update_in(cx, |mw, _, cx| {
            mw.set_random_database_id(cx);
        });

        let task =
            multi_workspace.update_in(cx, |mw, window, cx| mw.create_test_workspace(window, cx));
        task.await;

        let new_workspace_db_id =
            multi_workspace.read_with(cx, |mw, cx| mw.workspace().read(cx).database_id());
        assert!(
            new_workspace_db_id.is_some(),
            "After run_until_parked, the workspace should have a database_id"
        );

        let workspace_id = new_workspace_db_id.unwrap();

        let db = cx.update(|_, cx| WorkspaceDb::global(cx));

        assert!(
            db.workspace_for_id(workspace_id).is_some(),
            "The workspace row should exist in the DB"
        );

        cx.simulate_resize(gpui::size(px(1024.0), px(768.0)));

        // Advance the clock past the 100ms debounce timer so the bounds
        // observer task fires
        cx.executor().advance_clock(Duration::from_millis(200));
        cx.run_until_parked();

        let serialized = db
            .workspace_for_id(workspace_id)
            .expect("workspace row should still exist");
        assert!(
            serialized.window_bounds.is_some(),
            "The bounds observer should write bounds for the workspace's real DB ID, \
             even when the workspace was created via create_workspace (where the ID \
             is assigned asynchronously after construction)."
        );
    }

    #[gpui::test]
    async fn test_flush_serialization_writes_bounds(cx: &mut gpui::TestAppContext) {
        crate::tests::init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        let dir = tempfile::TempDir::with_prefix("flush_bounds_test").unwrap();
        fs.insert_tree(dir.path(), json!({})).await;

        let project = Project::test(fs.clone(), [dir.path()], cx).await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));

        let db = cx.update(|_, cx| WorkspaceDb::global(cx));
        let workspace_id = db.next_id().await.unwrap();
        multi_workspace.update_in(cx, |mw, _, cx| {
            mw.workspace().update(cx, |ws, _cx| {
                ws.set_database_id(workspace_id);
            });
        });

        let task = multi_workspace.update_in(cx, |mw, window, cx| {
            mw.workspace()
                .update(cx, |ws, cx| ws.flush_serialization(window, cx))
        });
        task.await;

        let after = db
            .workspace_for_id(workspace_id)
            .expect("workspace row should exist after flush_serialization");
        assert!(
            !after.paths.is_empty(),
            "flush_serialization should have written paths via save_workspace"
        );
        assert!(
            after.window_bounds.is_some(),
            "flush_serialization should ensure window bounds are persisted to the DB \
             before the process exits."
        );
    }

    #[gpui::test]
    async fn test_empty_window_without_items_stays_in_session(cx: &mut gpui::TestAppContext) {
        crate::tests::init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        let project = Project::test(fs.clone(), [], cx).await;
        let db = cx.update(|cx| WorkspaceDb::global(cx));
        let workspace_id = db.next_id().await.unwrap();
        let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
            let workspace = cx.new(|cx| {
                let mut workspace = crate::Workspace::test_new(project, window, cx);
                workspace.set_database_id(workspace_id);
                workspace
            });
            MultiWorkspace::new(workspace, window, cx)
        });
        let session_id = multi_workspace
            .read_with(cx, |multi_workspace, cx| {
                multi_workspace.workspace().read(cx).session_id()
            })
            .unwrap();
        let window_id = cx.update(|window, _| window.window_handle().window_id().as_u64());
        cx.run_until_parked();
        assert_eq!(
            db.select_row_bound::<_, (Option<String>, Option<u64>)>(
                "SELECT session_id, window_id FROM workspaces WHERE workspace_id = ?",
            )
            .unwrap()(workspace_id)
            .unwrap(),
            Some((Some(session_id.clone()), Some(window_id)))
        );

        multi_workspace
            .update_in(cx, |multi_workspace, window, cx| {
                multi_workspace.flush_pending_serialization(window, cx)
            })
            .await
            .unwrap();

        let restored = db
            .last_session_workspace_locations(&session_id, None, fs.as_ref())
            .await
            .unwrap();
        assert_eq!(
            restored
                .iter()
                .map(|workspace| workspace.workspace_id)
                .collect::<Vec<_>>(),
            vec![workspace_id]
        );
    }

    #[gpui::test]
    async fn test_window_with_last_folder_removed_reopens_empty(cx: &mut gpui::TestAppContext) {
        crate::tests::init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({ "main.rs": "" })).await;
        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;
        let db = cx.update(|cx| WorkspaceDb::global(cx));
        let workspace_id = db.next_id().await.unwrap();
        let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
            let workspace = cx.new(|cx| {
                let mut workspace = crate::Workspace::test_new(project.clone(), window, cx);
                workspace.set_database_id(workspace_id);
                workspace
            });
            MultiWorkspace::new(workspace, window, cx)
        });
        let session_id = multi_workspace
            .read_with(cx, |multi_workspace, cx| {
                multi_workspace.workspace().read(cx).session_id()
            })
            .unwrap();
        let window_id = cx.update(|window, _| window.window_handle().window_id().as_u64());
        cx.run_until_parked();
        assert_eq!(
            db.select_row_bound::<_, (Option<String>, Option<u64>)>(
                "SELECT session_id, window_id FROM workspaces WHERE workspace_id = ?",
            )
            .unwrap()(workspace_id)
            .unwrap(),
            Some((Some(session_id.clone()), Some(window_id)))
        );
        multi_workspace
            .update_in(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    workspace.flush_serialization_checked(window, cx)
                })
            })
            .await
            .unwrap();
        assert_eq!(
            db.workspace_for_id(workspace_id).unwrap().paths,
            PathList::new(&[Path::new("/project")])
        );

        let worktree_id = project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        project.update(cx, |project, cx| project.remove_worktree(worktree_id, cx));
        multi_workspace
            .update_in(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    workspace.flush_serialization_checked(window, cx)
                })
            })
            .await
            .unwrap();

        assert_eq!(
            db.workspace_for_id(workspace_id).unwrap().paths,
            PathList::default()
        );
        let restored = db
            .last_session_workspace_locations(&session_id, None, fs.as_ref())
            .await
            .unwrap();
        assert_eq!(
            restored
                .iter()
                .map(|workspace| (workspace.workspace_id, workspace.paths.clone()))
                .collect::<Vec<_>>(),
            vec![(workspace_id, PathList::default())]
        );
    }

    #[gpui::test]
    async fn test_recent_workspace_identity_deduplication(cx: &mut gpui::TestAppContext) {
        let fs = fs::FakeFs::new(cx.executor());

        // Main repo with a linked worktree entry
        fs.insert_tree(
            "/repo",
            json!({
                ".git": {
                    "worktrees": {
                        "feature": {
                            "commondir": "../../",
                            "HEAD": "ref: refs/heads/feature"
                        }
                    }
                },
                "src": { "main.rs": "" }
            }),
        )
        .await;

        // Linked worktree checkout pointing back to /repo
        fs.insert_tree(
            "/worktree",
            json!({
                ".git": "gitdir: /repo/.git/worktrees/feature",
                "src": { "main.rs": "" }
            }),
        )
        .await;

        // A plain non-git project
        fs.insert_tree(
            "/plain-project",
            json!({
                "src": { "main.rs": "" }
            }),
        )
        .await;

        // Another normal git repo (used in mixed-path entry)
        fs.insert_tree(
            "/other-repo",
            json!({
                ".git": {},
                "src": { "lib.rs": "" }
            }),
        )
        .await;

        let t0 = Utc::now() - chrono::Duration::hours(4);
        let t1 = Utc::now() - chrono::Duration::hours(3);
        let t2 = Utc::now() - chrono::Duration::hours(2);
        let t3 = Utc::now() - chrono::Duration::hours(1);

        let workspaces = vec![
            local_recent_workspace(WorkspaceId(1), PathList::new(&["/repo"]), t0, fs.as_ref())
                .await,
            local_recent_workspace(
                WorkspaceId(2),
                PathList::new(&["/worktree"]),
                t1,
                fs.as_ref(),
            )
            .await,
            local_recent_workspace(
                WorkspaceId(3),
                PathList::new(&["/other-repo", "/worktree"]),
                t2,
                fs.as_ref(),
            )
            .await,
            local_recent_workspace(
                WorkspaceId(4),
                PathList::new(&["/plain-project"]),
                t3,
                fs.as_ref(),
            )
            .await,
        ];

        let result = dedupe_recent_workspaces(workspaces);

        // Should have 3 entries: #1 and #2 deduped into one, plus #3 and #4.
        assert_eq!(result.len(), 3);

        // First entry: /repo — deduplicated from #1 and #2.
        // Keeps the position of #1 (first seen), but with #2's later timestamp.
        assert_eq!(result[0].identity_paths.paths(), &[PathBuf::from("/repo")]);
        assert_eq!(result[0].timestamp, t1);

        // Second entry: mixed-path workspace with worktree resolved.
        // /worktree → /repo, so paths become [/other-repo, /repo] (sorted).
        assert_eq!(
            result[1].identity_paths.paths(),
            &[PathBuf::from("/other-repo"), PathBuf::from("/repo")]
        );
        assert_eq!(result[1].workspace_id, WorkspaceId(3));

        // Third entry: non-git project, unchanged.
        assert_eq!(
            result[2].identity_paths.paths(),
            &[PathBuf::from("/plain-project")]
        );
        assert_eq!(result[2].workspace_id, WorkspaceId(4));
    }

    #[gpui::test]
    async fn test_recent_workspace_identity_for_bare_repo(cx: &mut gpui::TestAppContext) {
        let fs = fs::FakeFs::new(cx.executor());

        // Bare repo at /foo/.bare (commondir doesn't end with .git)
        fs.insert_tree(
            "/foo/.bare",
            json!({
                "worktrees": {
                    "my-feature": {
                        "commondir": "../../",
                        "HEAD": "ref: refs/heads/my-feature"
                    }
                }
            }),
        )
        .await;

        // Linked worktree whose commondir resolves to a bare repo (/foo/.bare)
        fs.insert_tree(
            "/foo/my-feature",
            json!({
                ".git": "gitdir: /foo/.bare/worktrees/my-feature",
                "src": { "main.rs": "" }
            }),
        )
        .await;

        let t0 = Utc::now();

        let result = local_recent_workspace(
            WorkspaceId(1),
            PathList::new(&["/foo/my-feature"]),
            t0,
            fs.as_ref(),
        )
        .await;

        // Bare-backed worktrees should resolve to the repo identity path, which
        // is the parent directory users think of as the project root.
        assert_eq!(result.identity_paths.paths(), &[PathBuf::from("/foo")]);
    }

    #[gpui::test]
    async fn test_recent_workspace_identity_for_submodule(cx: &mut gpui::TestAppContext) {
        let fs = fs::FakeFs::new(cx.executor());

        // Superproject `/Foo` with a submodule `Bar`. A submodule's `.git` is a
        // file pointing into the superproject's `.git/modules/<name>` directory,
        // structurally like a linked worktree's `.git` file.
        fs.insert_tree(
            "/Foo",
            json!({
                ".git": {
                    "modules": {
                        "Bar": {
                            "HEAD": "ref: refs/heads/main"
                        }
                    }
                },
                "Bar": {
                    ".git": "gitdir: ../.git/modules/Bar\n",
                    "src": { "main.rs": "" }
                },
                "src": { "lib.rs": "" }
            }),
        )
        .await;

        let t0 = Utc::now();

        let result = local_recent_workspace(
            WorkspaceId(1),
            PathList::new(&["/Foo/Bar"]),
            t0,
            fs.as_ref(),
        )
        .await;

        // Submodules are independent projects: their identity is their own
        // working directory, not the superproject's `.git/modules/<name>`.
        assert_eq!(result.identity_paths.paths(), &[PathBuf::from("/Foo/Bar")]);
    }

    #[gpui::test]
    async fn test_recent_workspace_identity_deduplicates_main_and_linked_worktree(
        cx: &mut gpui::TestAppContext,
    ) {
        let fs = fs::FakeFs::new(cx.executor());

        fs.insert_tree(
            "/the-project",
            json!({
                ".git": "gitdir: ./.bare\n",
                ".bare": {
                    "worktrees": {
                        "feature-a": {
                            "commondir": "../../",
                            "HEAD": "ref: refs/heads/feature-a"
                        }
                    }
                },
                "src": { "main.rs": "" }
            }),
        )
        .await;

        fs.insert_tree(
            "/the-project/feature-a",
            json!({
                ".git": "gitdir: ../.bare/worktrees/feature-a\n",
                "src": { "lib.rs": "" }
            }),
        )
        .await;

        let t0 = Utc::now() - chrono::Duration::hours(1);
        let t1 = Utc::now();
        let workspaces = vec![
            local_recent_workspace(
                WorkspaceId(1),
                PathList::new(&["/the-project"]),
                t0,
                fs.as_ref(),
            )
            .await,
            local_recent_workspace(
                WorkspaceId(2),
                PathList::new(&["/the-project/feature-a"]),
                t1,
                fs.as_ref(),
            )
            .await,
        ];

        let result = dedupe_recent_workspaces(workspaces);

        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0].identity_paths.paths(),
            &[PathBuf::from("/the-project")]
        );
        assert_eq!(result[0].workspace_id, WorkspaceId(2));
        assert_eq!(result[0].timestamp, t1);
    }

    #[gpui::test]
    async fn test_recent_project_workspaces_preserve_reopen_paths(cx: &mut gpui::TestAppContext) {
        let fs = fs::FakeFs::new(cx.executor());
        let db =
            WorkspaceDb::open_test_db("test_recent_project_workspaces_preserve_reopen_paths").await;

        fs.insert_tree(
            "/the-project",
            json!({
                ".git": "gitdir: ./.bare\n",
                ".bare": {
                    "worktrees": {
                        "feature-a": {
                            "commondir": "../../",
                            "HEAD": "ref: refs/heads/feature-a"
                        }
                    }
                },
                "src": { "main.rs": "" }
            }),
        )
        .await;

        fs.insert_tree(
            "/the-project/feature-a",
            json!({
                ".git": "gitdir: ../.bare/worktrees/feature-a\n",
                "src": { "lib.rs": "" }
            }),
        )
        .await;

        db.save_workspace(workspace_with(
            1,
            &[Path::new("/the-project")],
            empty_pane_group(),
            None,
        ))
        .await;
        db.save_workspace(workspace_with(
            2,
            &[Path::new("/the-project/feature-a")],
            empty_pane_group(),
            None,
        ))
        .await;
        db.set_timestamp_for_tests(WorkspaceId(1), "2024-01-01 00:00:00".to_owned())
            .await
            .unwrap();
        db.set_timestamp_for_tests(WorkspaceId(2), "2024-01-01 00:00:01".to_owned())
            .await
            .unwrap();

        let recents = db.recent_project_workspaces(fs.as_ref()).await.unwrap();

        assert_eq!(recents.len(), 1);
        assert_eq!(recents[0].workspace_id, WorkspaceId(2));
        assert_eq!(
            recents[0].paths.paths(),
            &[PathBuf::from("/the-project/feature-a")]
        );
        assert_eq!(
            recents[0].identity_paths.paths(),
            &[PathBuf::from("/the-project")]
        );
    }

    #[gpui::test]
    async fn test_recent_project_workspaces_remote_identity_hint(cx: &mut gpui::TestAppContext) {
        let fs = fs::FakeFs::new(cx.executor());
        let db =
            WorkspaceDb::open_test_db("test_recent_project_workspaces_remote_identity_hint").await;

        let workspace = remote_workspace_with(1, "example.com", &[Path::new("/repo/feature-a")]);
        db.save_workspace(SerializedWorkspace {
            identity_paths: Some(PathList::new(&["/repo"])),
            ..workspace
        })
        .await;

        let recents = db.recent_project_workspaces(fs.as_ref()).await.unwrap();

        assert_eq!(recents.len(), 1);
        assert_eq!(
            recents[0].paths.paths(),
            &[PathBuf::from("/repo/feature-a")]
        );
        assert_eq!(recents[0].identity_paths.paths(), &[PathBuf::from("/repo")]);
    }

    #[gpui::test]
    async fn test_recent_project_workspaces_remote_paths_do_not_use_local_fs_identity(
        cx: &mut gpui::TestAppContext,
    ) {
        let fs = fs::FakeFs::new(cx.executor());
        let db = WorkspaceDb::open_test_db(
            "test_recent_project_workspaces_remote_paths_do_not_use_local_fs_identity",
        )
        .await;

        fs.insert_tree(
            "/repo",
            json!({
                ".git": "gitdir: ./.bare\n",
                ".bare": {
                    "worktrees": {
                        "feature-a": {
                            "commondir": "../../",
                            "HEAD": "ref: refs/heads/feature-a"
                        }
                    }
                },
                "src": { "main.rs": "" }
            }),
        )
        .await;
        fs.insert_tree(
            "/repo/feature-a",
            json!({
                ".git": "gitdir: ../.bare/worktrees/feature-a\n",
                "src": { "lib.rs": "" }
            }),
        )
        .await;

        db.save_workspace(remote_workspace_with(
            1,
            "example.com",
            &[Path::new("/repo/feature-a")],
        ))
        .await;

        let recents = db.recent_project_workspaces(fs.as_ref()).await.unwrap();

        assert_eq!(recents.len(), 1);
        assert_eq!(
            recents[0].identity_paths.paths(),
            &[PathBuf::from("/repo/feature-a")]
        );
    }

    #[gpui::test]
    async fn test_recent_project_workspaces_do_not_dedupe_remote_hosts(
        cx: &mut gpui::TestAppContext,
    ) {
        let fs = fs::FakeFs::new(cx.executor());
        let db =
            WorkspaceDb::open_test_db("test_recent_project_workspaces_do_not_dedupe_remote_hosts")
                .await;

        db.save_workspace(remote_workspace_with(1, "host-a", &[Path::new("/repo")]))
            .await;
        db.save_workspace(remote_workspace_with(2, "host-b", &[Path::new("/repo")]))
            .await;
        db.set_timestamp_for_tests(WorkspaceId(1), "2024-01-01 00:00:00".to_owned())
            .await
            .unwrap();
        db.set_timestamp_for_tests(WorkspaceId(2), "2024-01-01 00:00:01".to_owned())
            .await
            .unwrap();

        let recents = db.recent_project_workspaces(fs.as_ref()).await.unwrap();

        assert_eq!(recents.len(), 2);
        assert_eq!(recents[0].workspace_id, WorkspaceId(2));
        assert_eq!(recents[1].workspace_id, WorkspaceId(1));
    }

    #[gpui::test]
    async fn test_delete_recent_workspace_group_removes_all_matching_rows(
        cx: &mut gpui::TestAppContext,
    ) {
        let fs = fs::FakeFs::new(cx.executor());
        let db = WorkspaceDb(
            db::open_test_db::<(KeyValueStore, WorkspaceDb)>(
                "test_delete_recent_workspace_group_removes_all_matching_rows",
            )
            .await,
        );

        fs.insert_tree(
            "/the-group",
            json!({
                ".git": "gitdir: ./.bare\n",
                ".bare": {
                    "worktrees": {
                        "feature-a": {
                            "commondir": "../../",
                            "HEAD": "ref: refs/heads/feature-a"
                        }
                    }
                },
                "src": { "main.rs": "" }
            }),
        )
        .await;

        fs.insert_tree(
            "/the-group/feature-a",
            json!({
                ".git": "gitdir: ../.bare/worktrees/feature-a\n",
                "src": { "lib.rs": "" }
            }),
        )
        .await;

        db.save_workspace(SerializedWorkspace {
            identity_paths: Some(PathList::new(&["/the-group"])),
            ..workspace_with(1, &[Path::new("/the-group")], empty_pane_group(), None)
        })
        .await;
        db.save_workspace(SerializedWorkspace {
            identity_paths: Some(PathList::new(&["/the-group"])),
            ..workspace_with(
                2,
                &[Path::new("/the-group/feature-a")],
                empty_pane_group(),
                None,
            )
        })
        .await;
        db.set_timestamp_for_tests(WorkspaceId(1), "2024-01-01 00:00:00".to_owned())
            .await
            .unwrap();
        db.set_timestamp_for_tests(WorkspaceId(2), "2024-01-01 00:00:01".to_owned())
            .await
            .unwrap();

        let recents = db.recent_project_workspaces(fs.as_ref()).await.unwrap();
        assert_eq!(recents.len(), 1);

        let deleted = db.delete_recent_workspace_group(&recents[0]).await.unwrap();
        assert_eq!(deleted, vec![WorkspaceId(2), WorkspaceId(1)]);

        let recents = db.recent_project_workspaces(fs.as_ref()).await.unwrap();
        assert!(recents.is_empty());
    }

    #[gpui::test]
    async fn test_restore_window_with_linked_worktree_and_multiple_project_groups(
        cx: &mut gpui::TestAppContext,
    ) {
        crate::tests::init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());

        // Main git repo at /repo
        fs.insert_tree(
            "/repo",
            json!({
                ".git": {
                    "HEAD": "ref: refs/heads/main",
                    "worktrees": {
                        "feature": {
                            "commondir": "../../",
                            "HEAD": "ref: refs/heads/feature"
                        }
                    }
                },
                "src": { "main.rs": "" }
            }),
        )
        .await;

        // Linked worktree checkout pointing back to /repo
        fs.insert_tree(
            "/worktree-feature",
            json!({
                ".git": "gitdir: /repo/.git/worktrees/feature",
                "src": { "lib.rs": "" }
            }),
        )
        .await;

        // --- Phase 1: Set up the original multi-workspace window ---

        let project_1 = Project::test(fs.clone(), ["/repo".as_ref()], cx).await;
        let project_1_linked_worktree =
            Project::test(fs.clone(), ["/worktree-feature".as_ref()], cx).await;

        // Wait for git discovery to finish.
        cx.run_until_parked();

        // Create a second, unrelated project so we have two distinct project groups.
        fs.insert_tree(
            "/other-project",
            json!({
                ".git": { "HEAD": "ref: refs/heads/main" },
                "readme.md": ""
            }),
        )
        .await;
        let project_2 = Project::test(fs.clone(), ["/other-project".as_ref()], cx).await;
        cx.run_until_parked();

        // Create the MultiWorkspace with project_2, then add the main repo
        // and its linked worktree. The linked worktree is added last and
        // becomes the active workspace.
        let (multi_workspace, cx) = cx
            .add_window_view(|window, cx| MultiWorkspace::test_new(project_2.clone(), window, cx));

        multi_workspace.update(cx, |mw, cx| {
            mw.open_sidebar(cx);
        });

        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.test_add_workspace(project_1.clone(), window, cx);
        });

        let workspace_worktree = multi_workspace.update_in(cx, |mw, window, cx| {
            mw.test_add_workspace(project_1_linked_worktree.clone(), window, cx)
        });
        let native_window_state = b"saved-native-window-state";
        let display_uuid = multi_workspace.update_in(cx, |_, window, cx| {
            window.restore_native_window_state(native_window_state);
            window.display(cx).unwrap().uuid().unwrap()
        });

        let tasks =
            multi_workspace.update_in(cx, |mw, window, cx| mw.flush_all_serialization(window, cx));
        cx.run_until_parked();
        for task in tasks {
            task.await;
        }
        cx.run_until_parked();

        let active_db_id = workspace_worktree.read_with(cx, |ws, _| ws.database_id());
        assert!(
            active_db_id.is_some(),
            "Active workspace should have a database ID"
        );

        // --- Phase 2: Read back and verify the serialized state ---

        let session_id = multi_workspace
            .read_with(cx, |mw, cx| mw.workspace().read(cx).session_id())
            .unwrap();
        let db = cx.update(|_, cx| WorkspaceDb::global(cx));
        assert_eq!(
            db.native_window_state(active_db_id.unwrap()).unwrap(),
            Some((Some(display_uuid), Some(native_window_state.to_vec())))
        );
        let session_workspaces = db
            .last_session_workspace_locations(&session_id, None, fs.as_ref())
            .await
            .expect("should load session workspaces");
        assert!(
            !session_workspaces.is_empty(),
            "Should have at least one session workspace"
        );

        let multi_workspaces =
            cx.update(|_, cx| read_serialized_multi_workspaces(session_workspaces, cx));
        assert_eq!(
            multi_workspaces.len(),
            1,
            "All workspaces share one window, so there should be exactly one multi-workspace"
        );

        let serialized = &multi_workspaces[0];
        assert_eq!(
            serialized.active_workspace.workspace_id,
            active_db_id.unwrap(),
        );
        assert_eq!(serialized.state.project_groups.len(), 2,);

        // Verify the serialized project group keys round-trip back to the
        // originals.
        let restored_keys: Vec<ProjectGroupKey> = serialized
            .state
            .project_groups
            .iter()
            .cloned()
            .map(Into::into)
            .collect();
        let expected_keys = vec![
            ProjectGroupKey::new(None, PathList::new(&["/repo"])),
            ProjectGroupKey::new(None, PathList::new(&["/other-project"])),
        ];
        assert_eq!(
            restored_keys, expected_keys,
            "Deserialized project group keys should match the originals"
        );

        // --- Phase 3: Restore the window and verify the result ---

        let app_state =
            multi_workspace.read_with(cx, |mw, cx| mw.workspace().read(cx).app_state().clone());

        let serialized_mw = multi_workspaces.into_iter().next().unwrap();
        multi_workspace.update_in(cx, |_, window, _| window.remove_window());
        let cx = &mut cx.cx;
        let restored_handle: gpui::WindowHandle<MultiWorkspace> = cx
            .update(|cx| {
                cx.spawn(async move |mut cx| {
                    crate::restore_multiworkspace(serialized_mw, app_state, true, &mut cx).await
                })
            })
            .await
            .expect("restore_multiworkspace should succeed");

        cx.run_until_parked();
        assert_eq!(
            restored_handle
                .update(cx, |_, window, _| window.native_window_state())
                .unwrap(),
            Some(native_window_state.to_vec())
        );

        // The restored window should have the same project group keys.
        let restored_keys: Vec<ProjectGroupKey> = restored_handle
            .read_with(cx, |mw: &MultiWorkspace, _cx| mw.project_group_keys())
            .unwrap();
        assert_eq!(
            restored_keys, expected_keys,
            "Restored window should have the same project group keys as the original"
        );

        // The active workspace in the restored window should have the linked
        // worktree paths.
        let active_paths: Vec<PathBuf> = restored_handle
            .read_with(cx, |mw: &MultiWorkspace, cx| {
                mw.workspace()
                    .read(cx)
                    .root_paths(cx)
                    .into_iter()
                    .map(|p: Arc<Path>| p.to_path_buf())
                    .collect()
            })
            .unwrap();
        assert_eq!(
            active_paths,
            vec![PathBuf::from("/worktree-feature")],
            "The restored active workspace should be the linked worktree project"
        );
    }

    #[gpui::test]
    async fn test_remove_project_group_falls_back_to_neighbor(cx: &mut gpui::TestAppContext) {
        crate::tests::init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        let dir_a = unique_test_dir(&fs, "group-a").await;
        let dir_b = unique_test_dir(&fs, "group-b").await;
        let dir_c = unique_test_dir(&fs, "group-c").await;

        let project_a = Project::test(fs.clone(), [dir_a.as_path()], cx).await;
        let project_b = Project::test(fs.clone(), [dir_b.as_path()], cx).await;
        let project_c = Project::test(fs.clone(), [dir_c.as_path()], cx).await;

        // Create a multi-workspace with project A, then add B and C.
        // project_groups stores newest first: [C, B, A].
        // Sidebar displays in the same order: C (top), B (middle), A (bottom).
        let (multi_workspace, cx) = cx
            .add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));

        multi_workspace.update(cx, |mw, cx| mw.open_sidebar(cx));

        let workspace_b = multi_workspace.update_in(cx, |mw, window, cx| {
            mw.test_add_workspace(project_b.clone(), window, cx)
        });
        let _workspace_c = multi_workspace.update_in(cx, |mw, window, cx| {
            mw.test_add_workspace(project_c.clone(), window, cx)
        });
        cx.run_until_parked();

        let key_a = project_a.read_with(cx, |p, cx| p.project_group_key(cx));
        let key_b = project_b.read_with(cx, |p, cx| p.project_group_key(cx));
        let key_c = project_c.read_with(cx, |p, cx| p.project_group_key(cx));

        // Activate workspace B so removing its group exercises the fallback.
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate(workspace_b.clone(), None, window, cx);
        });
        cx.run_until_parked();

        // --- Remove group B (the middle one). ---
        // In the sidebar [C, B, A], "below" B is A.
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.remove_project_group(&key_b, window, cx)
                .detach_and_log_err(cx);
        });
        cx.run_until_parked();

        let active_paths =
            multi_workspace.read_with(cx, |mw, cx| mw.workspace().read(cx).root_paths(cx));
        assert_eq!(
            active_paths
                .iter()
                .map(|p| p.to_path_buf())
                .collect::<Vec<_>>(),
            vec![dir_a.clone()],
            "After removing the middle group, should fall back to the group below (A)"
        );

        // After removing B, keys = [A, C], sidebar = [C, A].
        // Activate workspace A (the bottom) so removing it tests the
        // "fall back upward" path.
        let workspace_a =
            multi_workspace.read_with(cx, |mw, _cx| mw.workspaces().next().unwrap().clone());
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate(workspace_a.clone(), None, window, cx);
        });
        cx.run_until_parked();

        // --- Remove group A (the bottom one in sidebar). ---
        // Nothing below A, so should fall back upward to C.
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.remove_project_group(&key_a, window, cx)
                .detach_and_log_err(cx);
        });
        cx.run_until_parked();

        let active_paths =
            multi_workspace.read_with(cx, |mw, cx| mw.workspace().read(cx).root_paths(cx));
        assert_eq!(
            active_paths
                .iter()
                .map(|p| p.to_path_buf())
                .collect::<Vec<_>>(),
            vec![dir_c.clone()],
            "After removing the bottom group, should fall back to the group above (C)"
        );

        // --- Remove group C (the only one remaining). ---
        // Should create an empty workspace.
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.remove_project_group(&key_c, window, cx)
                .detach_and_log_err(cx);
        });
        cx.run_until_parked();

        let active_paths =
            multi_workspace.read_with(cx, |mw, cx| mw.workspace().read(cx).root_paths(cx));
        assert!(
            active_paths.is_empty(),
            "After removing the only remaining group, should have an empty workspace"
        );
    }

    /// Regression test for a crash where `find_or_create_local_workspace`
    /// returned a workspace that was about to be removed, hitting an assert
    /// in `MultiWorkspace::remove`.
    ///
    /// The scenario: two workspaces share the same root paths (e.g. due to
    /// a provisional key mismatch). When the first is removed and the
    /// fallback searches for the same paths, `workspace_for_paths` must
    /// skip the doomed workspace so the assert in `remove` is satisfied.
    #[gpui::test]
    async fn test_remove_fallback_skips_excluded_workspaces(cx: &mut gpui::TestAppContext) {
        crate::tests::init_test(cx);

        let fs = fs::FakeFs::new(cx.executor());
        let dir = unique_test_dir(&fs, "shared").await;

        // Two projects that open the same directory — this creates two
        // workspaces whose root_paths are identical.
        let project_a = Project::test(fs.clone(), [dir.as_path()], cx).await;
        let project_b = Project::test(fs.clone(), [dir.as_path()], cx).await;

        let (multi_workspace, cx) = cx
            .add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));

        multi_workspace.update(cx, |mw, cx| mw.open_sidebar(cx));

        let workspace_b = multi_workspace.update_in(cx, |mw, window, cx| {
            mw.test_add_workspace(project_b.clone(), window, cx)
        });
        cx.run_until_parked();

        // workspace_a is first in the workspaces vec.
        let workspace_a =
            multi_workspace.read_with(cx, |mw, _| mw.workspaces().next().cloned().unwrap());
        assert_ne!(workspace_a, workspace_b);

        // Activate workspace_a so removing it triggers the fallback path.
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.activate(workspace_a.clone(), None, window, cx);
        });
        cx.run_until_parked();

        // Remove workspace_a. Its replacement is looked up by the same paths,
        // so workspace_a itself has to be skipped or it would be picked again.
        multi_workspace.update_in(cx, |mw, window, cx| {
            mw.remove(
                vec![workspace_a.clone()],
                RemovalIntent::CloseProject,
                window,
                cx,
            )
            .detach_and_log_err(cx);
        });
        cx.run_until_parked();

        // workspace_b should now be active — workspace_a was removed.
        multi_workspace.read_with(cx, |mw, _cx| {
            assert_eq!(
                mw.workspace(),
                &workspace_b,
                "fallback should have found workspace_b, not the excluded workspace_a"
            );
        });
    }

    struct SuspendedItemRestore {
        barrier: Option<(
            oneshot::Sender<WeakEntity<Workspace>>,
            oneshot::Receiver<()>,
        )>,
        writes: Arc<AtomicUsize>,
        cleanups: usize,
    }

    impl gpui::Global for SuspendedItemRestore {}

    async fn write_multi_workspace_state(
        kvp: &KeyValueStore,
        window_id: WindowId,
        state: model::MultiWorkspaceState,
    ) -> Result<()> {
        let json = serde_json::to_string(&state)?;
        kvp.scoped("multi_workspace_state")
            .write(window_id.as_u64().to_string(), json)
            .await
    }
}
