use anyhow::Result;
use async_recursion::async_recursion;
use collections::HashSet;
use futures::future::join_all;
use gpui::{AppContext as _, AsyncWindowContext, Axis, Entity, Task, WeakEntity};
use project::Project;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ui::{App, Context, Window};
use util::ResultExt as _;

use db::{
    kvp::KeyValueStore,
    query,
    sqlez::{domain::Domain, statement::Statement, thread_safe_connection::ThreadSafeConnection},
    sqlez_macros::sql,
};
use workspace::{
    ItemId, Member, Pane, PaneAxis, PaneGroup, SerializableItem as _, SplitDirection, Workspace,
    WorkspaceDb, WorkspaceId,
};

use crate::{
    TerminalView, default_working_directory,
    terminal_panel::{TerminalPanel, new_terminal_pane},
};

pub(crate) fn serialize_pane_group(
    pane_group: &PaneGroup,
    active_pane: &Entity<Pane>,
    workspace: &mut Workspace,
    cx: &mut App,
) -> Result<(SerializedPaneGroup, Vec<Task<Result<()>>>)> {
    let mut tasks = Vec::new();
    let group =
        build_serialized_pane_group(&pane_group.root, active_pane, workspace, &mut tasks, cx)?;
    Ok((group, tasks))
}

fn build_serialized_pane_group(
    pane_group: &Member,
    active_pane: &Entity<Pane>,
    workspace: &mut Workspace,
    tasks: &mut Vec<Task<Result<()>>>,
    cx: &mut App,
) -> Result<SerializedPaneGroup> {
    Ok(match pane_group {
        Member::Axis(PaneAxis {
            axis,
            members,
            flexes,
            bounding_boxes: _,
        }) => SerializedPaneGroup::Group {
            axis: SerializedAxis(*axis),
            children: members
                .iter()
                .map(|member| {
                    build_serialized_pane_group(member, active_pane, workspace, tasks, cx)
                })
                .collect::<Result<Vec<_>>>()?,
            flexes: Some(flexes.lock().clone()),
        },
        Member::Pane(pane_handle) => SerializedPaneGroup::Pane(serialize_pane(
            pane_handle,
            pane_handle == active_pane,
            workspace,
            tasks,
            cx,
        )?),
    })
}

fn serialize_pane(
    pane: &Entity<Pane>,
    active: bool,
    workspace: &mut Workspace,
    tasks: &mut Vec<Task<Result<()>>>,
    cx: &mut App,
) -> Result<SerializedPane> {
    let pane = pane.read(cx);
    let active_runtime_id = pane.active_item().map(|item| item.item_id());
    let pinned_count = pane.pinned_count();
    let terminals = pane
        .items()
        .filter_map(|item| item.act_as::<TerminalView>(cx))
        .filter(|terminal| terminal.read(cx).terminal().read(cx).task().is_none())
        .collect::<Vec<_>>();
    let mut children = Vec::new();
    let mut active_item = None;
    for terminal in terminals {
        let item_id = workspace.serialization_id(
            TerminalView::serialized_item_kind(),
            terminal.entity_id(),
            cx,
        )?;
        if Some(terminal.entity_id()) == active_runtime_id {
            active_item = Some(item_id);
        }
        if let Some(task) = terminal.update(cx, |terminal, cx| {
            terminal.serialize(workspace, item_id, false, cx)
        }) {
            tasks.push(task);
        }
        children.push(item_id);
    }
    Ok(SerializedPane {
        active,
        children,
        active_item,
        pinned_count,
    })
}

pub(crate) fn deserialize_terminal_panel(
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    database_id: WorkspaceId,
    serialized_panel: SerializedTerminalPanel,
    terminal_panel: WeakEntity<TerminalPanel>,
    window: &mut Window,
    cx: &mut App,
) -> Task<anyhow::Result<usize>> {
    window.spawn(cx, async move |cx| {
        let restored_items = match &serialized_panel.items {
            SerializedItems::NoSplits(item_ids) => {
                let items = deserialize_terminal_views(
                    database_id,
                    project,
                    workspace,
                    item_ids.as_slice(),
                    cx,
                )
                .await;
                let restored_items = items.len();
                let active_item = serialized_panel.active_item_id;
                terminal_panel.update_in(cx, |terminal_panel, window, cx| {
                    terminal_panel.active_pane.update(cx, |pane, cx| {
                        populate_pane_items(pane, items, active_item, window, cx);
                    });
                })?;
                restored_items
            }
            SerializedItems::WithSplits(serialized_pane_group) => {
                let center_pane = deserialize_pane_group(
                    workspace,
                    project,
                    terminal_panel.clone(),
                    database_id,
                    serialized_pane_group,
                    cx,
                )
                .await;
                if let Some((center_group, active_pane)) = center_pane {
                    terminal_panel.update_in(cx, |terminal_panel, window, cx| {
                        let interim_panes = terminal_panel
                            .center
                            .panes()
                            .into_iter()
                            .filter(|pane| pane.read(cx).items_len() > 0)
                            .cloned()
                            .collect::<Vec<_>>();
                        let focused_interim_pane = interim_panes
                            .iter()
                            .find(|pane| pane.read(cx).has_focus(window, cx))
                            .cloned();
                        terminal_panel.center = PaneGroup::with_root(center_group);
                        terminal_panel.active_pane =
                            active_pane.unwrap_or_else(|| terminal_panel.center.first_pane());
                        let restored_items = terminal_panel
                            .center
                            .panes()
                            .into_iter()
                            .map(|pane| pane.read(cx).items_len())
                            .sum::<usize>();
                        let restored_pane = terminal_panel.active_pane.clone();
                        for interim_pane in &interim_panes {
                            terminal_panel.center.split(
                                &restored_pane,
                                interim_pane,
                                SplitDirection::Right,
                                cx,
                            );
                        }
                        if let Some(focused_interim_pane) = focused_interim_pane {
                            terminal_panel.active_pane = focused_interim_pane;
                        }
                        restored_items
                    })?
                } else {
                    0
                }
            }
        };

        Ok(restored_items)
    })
}

fn populate_pane_items(
    pane: &mut Pane,
    items: Vec<(ItemId, Entity<TerminalView>)>,
    active_item: Option<u64>,
    window: &mut Window,
    cx: &mut Context<Pane>,
) {
    let interim_active_item = (pane.items_len() > 0).then(|| pane.active_item()).flatten();
    let mut active_item_index = None;
    for (item_index, (item_id, item)) in (pane.items_len()..).zip(items) {
        if Some(item_id) == active_item {
            active_item_index = Some(item_index);
        }
        pane.add_item(Box::new(item), false, false, None, window, cx);
    }
    if let Some(interim_active_item) = interim_active_item {
        if let Some(index) = pane.index_for_item(interim_active_item.as_ref()) {
            pane.activate_item(index, false, false, window, cx);
        }
    } else if let Some(index) = active_item_index {
        pane.activate_item(index, false, false, window, cx);
    }
}

#[async_recursion(?Send)]
async fn deserialize_pane_group(
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    panel: WeakEntity<TerminalPanel>,
    workspace_id: WorkspaceId,
    serialized: &SerializedPaneGroup,
    cx: &mut AsyncWindowContext,
) -> Option<(Member, Option<Entity<Pane>>)> {
    match serialized {
        SerializedPaneGroup::Group {
            axis,
            flexes,
            children,
        } => {
            let mut current_active_pane = None;
            let mut members = Vec::new();
            for child in children {
                if let Some((new_member, active_pane)) = deserialize_pane_group(
                    workspace.clone(),
                    project.clone(),
                    panel.clone(),
                    workspace_id,
                    child,
                    cx,
                )
                .await
                {
                    members.push(new_member);
                    current_active_pane = current_active_pane.or(active_pane);
                }
            }

            if members.is_empty() {
                return None;
            }

            if members.len() == 1 {
                return Some((members.remove(0), current_active_pane));
            }

            Some((
                Member::Axis(PaneAxis::load(axis.0, members, flexes.clone())),
                current_active_pane,
            ))
        }
        SerializedPaneGroup::Pane(serialized_pane) => {
            let active = serialized_pane.active;

            let pane = panel
                .update_in(cx, |terminal_panel, window, cx| {
                    new_terminal_pane(
                        workspace.clone(),
                        project.clone(),
                        terminal_panel.active_pane.read(cx).is_zoomed(),
                        window,
                        cx,
                    )
                })
                .log_err()?;
            let active_item = serialized_pane.active_item;
            let pinned_count = serialized_pane.pinned_count;
            let new_items = deserialize_terminal_views(
                workspace_id,
                project.clone(),
                workspace.clone(),
                serialized_pane.children.as_slice(),
                cx,
            );
            cx.spawn({
                let pane = pane.downgrade();
                async move |cx| {
                    let new_items = new_items.await;

                    let items = pane.update_in(cx, |pane, window, cx| {
                        populate_pane_items(pane, new_items, active_item, window, cx);
                        pane.set_pinned_count(pinned_count.min(pane.items_len()));
                        pane.items_len()
                    });
                    // Avoid blank panes in splits
                    if items.is_ok_and(|items| items == 0) {
                        let working_directory = workspace
                            .update(cx, |workspace, cx| default_working_directory(workspace, cx))
                            .ok()
                            .flatten();
                        let terminal = project
                            .update(cx, |project, cx| {
                                project.create_terminal_shell(working_directory, cx)
                            })
                            .await
                            .log_err();
                        let Some(terminal) = terminal else {
                            return;
                        };
                        pane.update_in(cx, |pane, window, cx| {
                            let terminal_view = Box::new(cx.new(|cx| {
                                TerminalView::new(
                                    terminal,
                                    workspace.clone(),
                                    Some(workspace_id),
                                    project.downgrade(),
                                    window,
                                    cx,
                                )
                            }));
                            pane.add_item(terminal_view, true, false, None, window, cx);
                        })
                        .ok();
                    }
                }
            })
            .await;
            Some((Member::Pane(pane.clone()), active.then_some(pane)))
        }
    }
}

fn deserialize_terminal_views(
    workspace_id: WorkspaceId,
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    item_ids: &[u64],
    cx: &mut AsyncWindowContext,
) -> impl Future<Output = Vec<(ItemId, Entity<TerminalView>)>> + use<> {
    let deserialized_items = join_all(item_ids.iter().filter_map(|item_id| {
        let item_id = *item_id;
        cx.update(|window, cx| {
            let task = TerminalView::deserialize(
                project.clone(),
                workspace.clone(),
                workspace_id,
                item_id,
                window,
                cx,
            );
            window.spawn(cx, {
                let workspace = workspace.clone();
                async move |cx| {
                    let item = task.await?;
                    workspace.update(cx, |workspace, cx| {
                        workspace.register_serialized_item_id(
                            TerminalView::serialized_item_kind(),
                            item.entity_id(),
                            item_id,
                            cx,
                        )
                    })??;
                    anyhow::Ok((item_id, item))
                }
            })
        })
        .log_err()
    }));
    async move {
        deserialized_items
            .await
            .into_iter()
            .filter_map(|item| item.log_err())
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SerializedTerminalPanel {
    pub items: SerializedItems,
    // A deprecated field, kept for backwards compatibility for the code before terminal splits were introduced.
    pub active_item_id: Option<u64>,
}

impl SerializedTerminalPanel {
    pub(crate) fn item_ids(&self) -> Vec<ItemId> {
        let mut item_ids = Vec::new();
        match &self.items {
            SerializedItems::NoSplits(items) => item_ids.extend(items),
            SerializedItems::WithSplits(group) => group.collect_item_ids(&mut item_ids),
        }
        item_ids.extend(self.active_item_id);
        item_ids
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum SerializedItems {
    // The data stored before terminal splits were introduced.
    NoSplits(Vec<u64>),
    WithSplits(SerializedPaneGroup),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum SerializedPaneGroup {
    Pane(SerializedPane),
    Group {
        axis: SerializedAxis,
        flexes: Option<Vec<f32>>,
        children: Vec<SerializedPaneGroup>,
    },
}

impl SerializedPaneGroup {
    fn collect_item_ids(&self, item_ids: &mut Vec<ItemId>) {
        match self {
            Self::Pane(pane) => {
                item_ids.extend(&pane.children);
                item_ids.extend(pane.active_item);
            }
            Self::Group { children, .. } => {
                for child in children {
                    child.collect_item_ids(item_ids);
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SerializedPane {
    pub active: bool,
    pub children: Vec<u64>,
    pub active_item: Option<u64>,
    #[serde(default)]
    pub pinned_count: usize,
}

#[derive(Debug)]
pub(crate) struct SerializedAxis(pub Axis);

impl Serialize for SerializedAxis {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.0 {
            Axis::Horizontal => serializer.serialize_str("horizontal"),
            Axis::Vertical => serializer.serialize_str("vertical"),
        }
    }
}

impl<'de> Deserialize<'de> for SerializedAxis {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "horizontal" => Ok(SerializedAxis(Axis::Horizontal)),
            "vertical" => Ok(SerializedAxis(Axis::Vertical)),
            invalid => Err(serde::de::Error::custom(format!(
                "Invalid axis value: '{invalid}'"
            ))),
        }
    }
}

pub struct TerminalDb(ThreadSafeConnection);

impl Domain for TerminalDb {
    const NAME: &str = stringify!(TerminalDb);

    const MIGRATIONS: &[&str] = &[
        sql!(
            CREATE TABLE terminals (
                workspace_id INTEGER,
                item_id INTEGER UNIQUE,
                working_directory BLOB,
                PRIMARY KEY(workspace_id, item_id),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
            ) STRICT;
        ),
        // Remove the unique constraint on the item_id table
        // SQLite doesn't have a way of doing this automatically, so
        // we have to do this silly copying.
        sql!(
            CREATE TABLE terminals2 (
                workspace_id INTEGER,
                item_id INTEGER,
                working_directory BLOB,
                PRIMARY KEY(workspace_id, item_id),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
            ) STRICT;

            INSERT INTO terminals2 (workspace_id, item_id, working_directory)
            SELECT workspace_id, item_id, working_directory FROM terminals;

            DROP TABLE terminals;

            ALTER TABLE terminals2 RENAME TO terminals;
        ),
        sql! (
            ALTER TABLE terminals ADD COLUMN working_directory_path TEXT;
            UPDATE terminals SET working_directory_path = CAST(working_directory AS TEXT);
        ),
        sql! (
            ALTER TABLE terminals ADD COLUMN custom_title TEXT;
        ),
    ];
}

db::static_connection!(TerminalDb, [WorkspaceDb, KeyValueStore]);

impl TerminalDb {
    query! {
        pub fn item_ids(workspace_id: WorkspaceId) -> Result<Vec<ItemId>> {
            SELECT item_id FROM terminals WHERE workspace_id = ? ORDER BY item_id
        }
    }

    pub fn save_terminal(
        &self,
        item_id: ItemId,
        workspace_id: WorkspaceId,
        working_directory: Option<PathBuf>,
        custom_title: Option<String>,
    ) -> impl Future<Output = Result<()>> + use<> {
        self.write(move |connection| {
            let mut statement = Statement::prepare(
                connection,
                "INSERT INTO terminals (
                    item_id, workspace_id, working_directory, working_directory_path, custom_title
                ) VALUES (?1, ?2, ?3, ?4, ?5)
                ON CONFLICT (workspace_id, item_id) DO UPDATE SET
                    working_directory = COALESCE(excluded.working_directory, terminals.working_directory),
                    working_directory_path = COALESCE(excluded.working_directory_path, terminals.working_directory_path),
                    custom_title = excluded.custom_title",
            )?;
            let mut next_index = statement.bind(&item_id, 1)?;
            next_index = statement.bind(&workspace_id, next_index)?;
            next_index = statement.bind(&working_directory, next_index)?;
            next_index = statement.bind(
                &working_directory.map(|path| path.to_string_lossy().into_owned()),
                next_index,
            )?;
            statement.bind(&custom_title, next_index)?;
            statement.exec()
        })
    }

    pub fn cleanup(
        &self,
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
    ) -> impl Future<Output = Result<()>> + use<> {
        self.cleanup_candidates(workspace_id, alive_items, None)
    }

    query! {
        pub fn get_working_directory(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<PathBuf>> {
            SELECT working_directory
            FROM terminals
            WHERE item_id = ? AND workspace_id = ?
        }
    }

    query! {
        pub fn get_custom_title(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<String>> {
            SELECT custom_title
            FROM terminals
            WHERE item_id = ? AND workspace_id = ?
        }
    }

    pub(crate) fn prepare_cleanup(
        &self,
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
    ) -> impl Future<Output = Result<()>> + use<> {
        let candidates = self.write(move |connection| {
            connection.select_bound::<WorkspaceId, ItemId>(
                "SELECT item_id FROM terminals WHERE workspace_id = ?",
            )?(workspace_id)
        });
        let db = self.clone();
        async move {
            let candidates = candidates.await?;
            db.cleanup_candidates(workspace_id, alive_items, Some(candidates))
                .await
        }
    }

    fn cleanup_candidates(
        &self,
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
        candidates: Option<Vec<ItemId>>,
    ) -> impl Future<Output = Result<()>> + use<> {
        self.write(move |connection| {
            connection.with_savepoint("cleanup_terminals", || {
                let mut retained = alive_items.into_iter().collect::<HashSet<_>>();
                retained.extend(connection.select_bound::<(WorkspaceId, &str), ItemId>(
                    "SELECT item_id FROM items WHERE workspace_id = ? AND kind = ?",
                )?((
                    workspace_id,
                    TerminalView::serialized_item_kind(),
                ))?);
                let panel_key = TerminalPanel::serialization_key_for_workspace_id(workspace_id);
                if let Some(panel) = connection
                    .select_row_bound::<&str, String>("SELECT value FROM kv_store WHERE key = ?")?(
                    &panel_key,
                )? {
                    let panel = serde_json::from_str::<SerializedTerminalPanel>(&panel)?;
                    retained.extend(panel.item_ids());
                }
                let item_ids = match candidates {
                    Some(candidates) => candidates,
                    None => connection.select_bound::<WorkspaceId, ItemId>(
                        "SELECT item_id FROM terminals WHERE workspace_id = ?",
                    )?(workspace_id)?,
                };
                let mut delete = connection.exec_bound::<(WorkspaceId, ItemId)>(
                    "DELETE FROM terminals WHERE workspace_id = ? AND item_id = ?",
                )?;
                for item_id in item_ids {
                    if !retained.contains(&item_id) {
                        delete((workspace_id, item_id))?;
                    }
                }
                Ok(())
            })
        })
    }
}
