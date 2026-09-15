use anyhow::{Context as _, Result};
use async_recursion::async_recursion;
use collections::HashSet;
use futures::future::join_all;
use gpui::{AppContext as _, AsyncWindowContext, Axis, Entity, Task, WeakEntity};
use project::Project;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ui::{App, Context, Window};

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

pub(crate) struct TerminalSerializationAdmission {
    unknown_item_ids: Vec<(bool, HashSet<ItemId>)>,
}

impl TerminalSerializationAdmission {
    pub(crate) fn new(
        workspace: &mut Workspace,
        workspace_id: WorkspaceId,
        recovery: bool,
        known_item_ids: HashSet<ItemId>,
        cx: &App,
    ) -> Result<Self> {
        workspace.refresh_serialized_item_ids(workspace_id, "Terminal", cx)?;
        let db = TerminalDb::global(cx);
        let mut unknown_item_ids = Vec::new();
        for saved_recovery in [false, true] {
            let saved = match db.saved_panel(workspace_id, saved_recovery) {
                Ok(Some(saved)) => saved,
                Ok(None) => continue,
                Err(error) if !saved_recovery && recovery && error.is::<serde_json::Error>() => {
                    continue;
                }
                Err(error) => return Err(error),
            };
            saved.validate_child_item_ids()?;
            let mut item_ids = saved.item_ids();
            item_ids.extend(saved.primary_item_ids);
            workspace.reserve_serialized_item_ids(workspace_id, "Terminal", &item_ids, cx)?;
            unknown_item_ids.push((
                saved_recovery,
                item_ids
                    .into_iter()
                    .filter(|item_id| !known_item_ids.contains(item_id))
                    .collect(),
            ));
        }
        Ok(Self { unknown_item_ids })
    }

    pub(crate) fn validate(&self, live_ids: &HashSet<ItemId>) -> Result<()> {
        for (saved_recovery, item_ids) in &self.unknown_item_ids {
            let mut conflicts = live_ids
                .iter()
                .copied()
                .filter(|item_id| item_ids.contains(item_id))
                .collect::<Vec<_>>();
            conflicts.sort_unstable();
            let layout = if *saved_recovery {
                "Recovery layout"
            } else {
                "Saved layout"
            };
            anyhow::ensure!(
                conflicts.is_empty(),
                "{layout} references conflict with new terminal IDs {conflicts:?}; repair the saved references before retrying"
            );
        }
        Ok(())
    }
}

pub(crate) fn serialize_pane_group(
    pane_group: &PaneGroup,
    active_pane: &Entity<Pane>,
    workspace: &mut Workspace,
    admission: &TerminalSerializationAdmission,
    cx: &mut App,
) -> Result<(SerializedPaneGroup, Vec<Task<Result<()>>>)> {
    let mut tasks = Vec::new();
    let group = build_serialized_pane_group(
        &pane_group.root,
        active_pane,
        workspace,
        admission,
        &mut tasks,
        cx,
    )?;
    Ok((group, tasks))
}

fn build_serialized_pane_group(
    pane_group: &Member,
    active_pane: &Entity<Pane>,
    workspace: &mut Workspace,
    admission: &TerminalSerializationAdmission,
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
                    build_serialized_pane_group(
                        member,
                        active_pane,
                        workspace,
                        admission,
                        tasks,
                        cx,
                    )
                })
                .collect::<Result<Vec<_>>>()?,
            flexes: Some(flexes.lock().clone()),
        },
        Member::Pane(pane_handle) => SerializedPaneGroup::Pane(serialize_pane(
            pane_handle,
            pane_handle == active_pane,
            workspace,
            admission,
            tasks,
            cx,
        )?),
    })
}

fn serialize_pane(
    pane: &Entity<Pane>,
    active: bool,
    workspace: &mut Workspace,
    admission: &TerminalSerializationAdmission,
    tasks: &mut Vec<Task<Result<()>>>,
    cx: &mut App,
) -> Result<SerializedPane> {
    let pane = pane.read(cx);
    let active_runtime_id = pane.active_item().map(|item| item.item_id());
    let terminals = pane
        .items()
        .enumerate()
        .filter_map(|(index, item)| {
            item.act_as::<TerminalView>(cx)
                .map(|terminal| (index, terminal))
        })
        .filter(|(_, terminal)| terminal.read(cx).terminal().read(cx).task().is_none())
        .collect::<Vec<_>>();
    let pinned_count = terminals
        .iter()
        .take_while(|(index, _)| *index < pane.pinned_count())
        .count();
    let mut children = Vec::new();
    let mut active_item = None;
    for (_, terminal) in terminals {
        let item_id = workspace.serialization_id(
            TerminalView::serialized_item_kind(),
            terminal.entity_id(),
            cx,
        )?;
        if Some(terminal.entity_id()) == active_runtime_id {
            active_item = Some(item_id);
        }
        if let Some(task) = terminal.update(cx, |terminal, cx| {
            terminal.serialize_with_admission(workspace, item_id, Some(admission), cx)
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
        serialized_panel.validate_child_item_ids()?;
        let known_items = terminal_panel.read_with(cx, |panel, cx| {
            panel
                .center
                .panes()
                .into_iter()
                .flat_map(|pane| {
                    pane.read(cx)
                        .items_of_type::<TerminalView>()
                        .filter_map(|view| view.read(cx).serialization_identity())
                        .filter_map(|(workspace_id, item_id)| {
                            (workspace_id == database_id).then_some(item_id)
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<HashSet<_>>()
        })?;
        let primary_item_ids = serialized_panel.primary_item_ids.clone();
        let Some(serialized_panel) = serialized_panel
            .without_items(&known_items)
            .filter(|panel| !panel.item_ids().is_empty())
        else {
            terminal_panel.update(cx, |panel, _| {
                panel.primary_item_ids.extend(primary_item_ids);
            })?;
            return Ok(0);
        };
        let restored_items = match &serialized_panel.items {
            SerializedItems::NoSplits(item_ids) => {
                let items = deserialize_terminal_views(
                    database_id,
                    project,
                    workspace,
                    item_ids.as_slice(),
                    cx,
                )
                .await?;
                let restored_items = items.len();
                let active_item = serialized_panel.active_item_id;
                terminal_panel.update_in(cx, |terminal_panel, window, cx| {
                    terminal_panel.primary_item_ids.extend(primary_item_ids);
                    terminal_panel.active_pane.update(cx, |pane, cx| {
                        populate_pane_items(pane, items, active_item, window, cx);
                    });
                })?;
                restored_items
            }
            SerializedItems::WithSplits(serialized_pane_group) => {
                let mut prepared_panes = Vec::new();
                let center_pane = deserialize_pane_group(
                    workspace,
                    project,
                    terminal_panel.clone(),
                    database_id,
                    serialized_pane_group,
                    &mut prepared_panes,
                    cx,
                )
                .await?;
                if let Some((center_group, active_pane)) = center_pane {
                    terminal_panel.update_in(cx, |terminal_panel, window, cx| {
                        terminal_panel.primary_item_ids.extend(primary_item_ids);
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
                        for prepared in prepared_panes {
                            prepared.pane.update(cx, |pane, cx| {
                                populate_pane_items(
                                    pane,
                                    prepared.items,
                                    prepared.active_item,
                                    window,
                                    cx,
                                );
                                pane.set_pinned_count(prepared.pinned_count);
                                if let Some(terminal) = prepared.default_terminal {
                                    pane.add_item(
                                        Box::new(terminal),
                                        true,
                                        false,
                                        None,
                                        window,
                                        cx,
                                    );
                                }
                            });
                        }
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

struct PreparedTerminalPane {
    pane: Entity<Pane>,
    items: Vec<(ItemId, Entity<TerminalView>)>,
    active_item: Option<ItemId>,
    pinned_count: usize,
    default_terminal: Option<Entity<TerminalView>>,
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
    prepared_panes: &mut Vec<PreparedTerminalPane>,
    cx: &mut AsyncWindowContext,
) -> Result<Option<(Member, Option<Entity<Pane>>)>> {
    Ok(match serialized {
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
                    prepared_panes,
                    cx,
                )
                .await?
                {
                    members.push(new_member);
                    current_active_pane = current_active_pane.or(active_pane);
                }
            }

            if members.is_empty() {
                return Ok(None);
            }

            if members.len() == 1 {
                return Ok(Some((members.remove(0), current_active_pane)));
            }

            Some((
                Member::Axis(PaneAxis::load(axis.0, members, flexes.clone())),
                current_active_pane,
            ))
        }
        SerializedPaneGroup::Pane(serialized_pane) => {
            let active = serialized_pane.active;

            let pane = panel.update_in(cx, |terminal_panel, window, cx| {
                new_terminal_pane(
                    workspace.clone(),
                    project.clone(),
                    terminal_panel.active_pane.read(cx).is_zoomed(),
                    window,
                    cx,
                )
            })?;
            let active_item = serialized_pane.active_item;
            let pinned_ids = serialized_pane
                .children
                .iter()
                .take(serialized_pane.pinned_count)
                .copied()
                .collect::<HashSet<_>>();
            let new_items = deserialize_terminal_views(
                workspace_id,
                project.clone(),
                workspace.clone(),
                serialized_pane.children.as_slice(),
                cx,
            );
            let new_items = new_items.await?;
            let pinned_count = new_items
                .iter()
                .take_while(|(item_id, _)| pinned_ids.contains(item_id))
                .count();
            // Avoid blank panes in splits
            let default_terminal = if new_items.is_empty() {
                let working_directory = workspace
                    .update(cx, |workspace, cx| default_working_directory(workspace, cx))?;
                let terminal = project
                    .update(cx, |project, cx| {
                        project.create_terminal_shell(working_directory, cx)
                    })
                    .await?;
                Some(cx.update(|window, cx| {
                    cx.new(|cx| {
                        TerminalView::new(
                            terminal,
                            workspace.clone(),
                            project.downgrade(),
                            window,
                            cx,
                        )
                    })
                })?)
            } else {
                None
            };
            prepared_panes.push(PreparedTerminalPane {
                pane: pane.clone(),
                items: new_items,
                active_item,
                pinned_count,
                default_terminal,
            });
            Some((Member::Pane(pane.clone()), active.then_some(pane)))
        }
    })
}

fn deserialize_terminal_views(
    workspace_id: WorkspaceId,
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    item_ids: &[u64],
    cx: &mut AsyncWindowContext,
) -> impl Future<Output = Result<Vec<(ItemId, Entity<TerminalView>)>>> + use<> {
    let deserialized_items = item_ids
        .iter()
        .map(|item_id| {
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
                    let project = project.clone();
                    async move |cx| {
                        let item = match task.await {
                            Ok(item) => item,
                            Err(error) => {
                                log::error!("Failed to restore terminal {item_id}: {error:#}");
                                cx.update(|window, cx| {
                                    cx.new(|cx| {
                                        TerminalView::failed_restoration(
                                            workspace.clone(),
                                            project.downgrade(),
                                            workspace_id,
                                            item_id,
                                            error,
                                            window,
                                            cx,
                                        )
                                    })
                                })?
                            }
                        };
                        workspace.update(cx, |workspace, cx| {
                            workspace.register_serialized_item_id(
                                TerminalView::serialized_item_kind(),
                                item.entity_id(),
                                item_id,
                                cx,
                            )?;
                            workspace.track_serialized_item(Box::new(item.downgrade()));
                            anyhow::Ok(())
                        })??;
                        anyhow::Ok((item_id, item))
                    }
                })
            })
        })
        .collect::<Vec<_>>();
    async move {
        let tasks = deserialized_items.into_iter().collect::<Result<Vec<_>>>()?;
        join_all(tasks).await.into_iter().collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SerializedTerminalPanel {
    pub items: SerializedItems,
    // A deprecated field, kept for backwards compatibility for the code before terminal splits were introduced.
    pub active_item_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub primary_item_ids: Vec<ItemId>,
}

impl SerializedTerminalPanel {
    pub(crate) fn validate_child_item_ids(&self) -> Result<()> {
        let mut child_ids = HashSet::default();
        match &self.items {
            SerializedItems::NoSplits(items) => {
                for item_id in items {
                    anyhow::ensure!(
                        child_ids.insert(*item_id),
                        "Terminal layout contains duplicate child ID {item_id}"
                    );
                }
            }
            SerializedItems::WithSplits(group) => {
                let mut groups = vec![group];
                while let Some(group) = groups.pop() {
                    match group {
                        SerializedPaneGroup::Pane(pane) => {
                            for item_id in &pane.children {
                                anyhow::ensure!(
                                    child_ids.insert(*item_id),
                                    "Terminal layout contains duplicate child ID {item_id}"
                                );
                            }
                        }
                        SerializedPaneGroup::Group { children, .. } => groups.extend(children),
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn without_items(self, excluded: &HashSet<ItemId>) -> Option<Self> {
        let active_item_id = self
            .active_item_id
            .filter(|item_id| !excluded.contains(item_id));
        let items = match self.items {
            SerializedItems::NoSplits(mut items) => {
                let was_empty = items.is_empty();
                items.retain(|item_id| !excluded.contains(item_id));
                if items.is_empty() && !was_empty {
                    return None;
                }
                SerializedItems::NoSplits(items)
            }
            SerializedItems::WithSplits(group) => {
                SerializedItems::WithSplits(group.without_items(excluded)?)
            }
        };
        Some(Self {
            items,
            active_item_id,
            primary_item_ids: self.primary_item_ids,
        })
    }

    pub(crate) fn merge(
        mut self,
        mut previous: Self,
        previous_live_ids: &HashSet<ItemId>,
    ) -> Result<Self> {
        let mut known = self.item_ids().into_iter().collect::<HashSet<_>>();
        let primary_ids = self
            .primary_item_ids
            .iter()
            .copied()
            .collect::<HashSet<_>>();
        let previous_primary_ids = previous
            .primary_item_ids
            .iter()
            .copied()
            .collect::<HashSet<_>>();
        let mut conflicts = previous
            .item_ids()
            .into_iter()
            .filter(|item_id| {
                known.contains(item_id)
                    && !previous_live_ids.contains(item_id)
                    && !(primary_ids.contains(item_id) && previous_primary_ids.contains(item_id))
            })
            .collect::<Vec<_>>();
        conflicts.sort_unstable();
        conflicts.dedup();
        anyhow::ensure!(
            conflicts.is_empty(),
            "Recovery layout references conflict with new terminal IDs {conflicts:?}; repair the saved references before retrying"
        );
        known.extend(primary_ids);
        known.extend(previous_live_ids);
        self.primary_item_ids.extend(&previous.primary_item_ids);
        self.primary_item_ids.sort_unstable();
        self.primary_item_ids.dedup();
        previous.primary_item_ids = self.primary_item_ids.clone();
        let Some(previous) = previous.without_items(&known) else {
            return Ok(self);
        };
        if previous.item_ids().is_empty() {
            return Ok(self);
        }
        if self.item_ids().is_empty() {
            return Ok(previous);
        }
        let primary_item_ids = self.primary_item_ids.clone();
        Ok(Self {
            primary_item_ids,
            items: SerializedItems::WithSplits(SerializedPaneGroup::Group {
                axis: SerializedAxis(Axis::Horizontal),
                flexes: None,
                children: vec![self.into_group(), previous.into_group()],
            }),
            active_item_id: None,
        })
    }

    pub(crate) fn item_ids(&self) -> Vec<ItemId> {
        let mut item_ids = Vec::new();
        match &self.items {
            SerializedItems::NoSplits(items) => item_ids.extend(items),
            SerializedItems::WithSplits(group) => group.collect_item_ids(&mut item_ids),
        }
        item_ids.extend(self.active_item_id);
        item_ids
    }

    fn into_group(self) -> SerializedPaneGroup {
        match self.items {
            SerializedItems::WithSplits(group) => group,
            SerializedItems::NoSplits(children) => SerializedPaneGroup::Pane(SerializedPane {
                active: true,
                children,
                active_item: self.active_item_id,
                pinned_count: 0,
            }),
        }
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
    fn without_items(self, excluded: &HashSet<ItemId>) -> Option<Self> {
        match self {
            Self::Pane(mut pane) => {
                let was_empty = pane.children.is_empty();
                let pinned_count = pane
                    .children
                    .iter()
                    .take(pane.pinned_count)
                    .filter(|item_id| !excluded.contains(item_id))
                    .count();
                pane.children.retain(|item_id| !excluded.contains(item_id));
                if pane.children.is_empty() && !was_empty {
                    return None;
                }
                pane.pinned_count = pinned_count;
                pane.active_item = pane
                    .active_item
                    .filter(|item_id| !excluded.contains(item_id));
                Some(Self::Pane(pane))
            }
            Self::Group {
                axis,
                children,
                flexes,
            } => {
                let mut retained_flexes = Vec::new();
                let mut children = children
                    .into_iter()
                    .enumerate()
                    .filter_map(|(index, child)| {
                        let child = child.without_items(excluded)?;
                        if let Some(flex) = flexes.as_ref().and_then(|flexes| flexes.get(index)) {
                            retained_flexes.push(*flex);
                        }
                        Some(child)
                    })
                    .collect::<Vec<_>>();
                if children.is_empty() {
                    return None;
                }
                if children.len() == 1 {
                    return children.pop();
                }
                let flexes = (retained_flexes.len() == children.len()).then_some(retained_flexes);
                Some(Self::Group {
                    axis,
                    children,
                    flexes,
                })
            }
        }
    }

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

    pub(crate) fn get_terminal(
        &self,
        item_id: ItemId,
        workspace_id: WorkspaceId,
    ) -> Result<(Option<PathBuf>, Option<String>)> {
        self.select_row_bound::<(ItemId, WorkspaceId), (Option<PathBuf>, Option<String>)>(
            "SELECT working_directory, custom_title FROM terminals WHERE item_id = ? AND workspace_id = ?",
        )?((item_id, workspace_id))?
        .with_context(|| format!("Saved terminal {item_id} has no payload"))
    }

    pub(crate) fn saved_panel(
        &self,
        workspace_id: WorkspaceId,
        recovery: bool,
    ) -> Result<Option<SerializedTerminalPanel>> {
        let key = if recovery {
            TerminalPanel::recovery_key_for_workspace_id(workspace_id)
        } else {
            TerminalPanel::serialization_key_for_workspace_id(workspace_id)
        };
        self.select_row_bound::<&str, String>("SELECT value FROM kv_store WHERE key = ?")?(&key)?
            .map(|raw| serde_json::from_str(&raw).map_err(anyhow::Error::from))
            .transpose()
    }

    pub(crate) fn serialized_item_ids(&self, workspace_id: WorkspaceId) -> Result<Vec<ItemId>> {
        let mut item_ids = self.item_ids(workspace_id)?;
        for recovery in [false, true] {
            let panel = match self.saved_panel(workspace_id, recovery) {
                Ok(Some(panel)) => panel,
                Ok(None) => continue,
                Err(error) if !recovery && error.is::<serde_json::Error>() => {
                    log::error!("Saved terminal layout is unreadable: {error:#}");
                    continue;
                }
                Err(error) => {
                    return Err(error).context(
                        "Cannot reserve terminal IDs because the recovery layout is unreadable",
                    );
                }
            };
            item_ids.extend(panel.item_ids());
            item_ids.extend(panel.primary_item_ids);
        }
        item_ids.sort_unstable();
        item_ids.dedup();
        Ok(item_ids)
    }

    pub(crate) fn save_panel(
        &self,
        workspace_id: WorkspaceId,
        mut panel: SerializedTerminalPanel,
        recovery: bool,
        recovery_loaded: bool,
        previous_live_ids: HashSet<ItemId>,
    ) -> impl Future<Output = Result<()>> + use<> {
        self.write(move |connection| {
            connection.with_savepoint("save_terminal_panel", || {
                let primary_key = TerminalPanel::serialization_key_for_workspace_id(workspace_id);
                let recovery_key = TerminalPanel::recovery_key_for_workspace_id(workspace_id);
                if recovery && !recovery_loaded {
                    if let Some(previous) = connection.select_row_bound::<&str, String>(
                        "SELECT value FROM kv_store WHERE key = ?",
                    )?(&recovery_key)?
                    {
                        panel = panel.merge(
                            serde_json::from_str(&previous).context(
                                "Cannot save terminal changes because the recovery layout is unreadable; keep this window open and repair the recovery layout before retrying",
                            )?,
                            &previous_live_ids,
                        )?;
                    }
                }
                if recovery {
                    panel.primary_item_ids.sort_unstable();
                    panel.primary_item_ids.dedup();
                } else {
                    panel.primary_item_ids.clear();
                }
                let key = if recovery {
                    &recovery_key
                } else {
                    &primary_key
                };
                let serialized = serde_json::to_string(&panel)?;
                connection.exec_bound::<(&str, &str)>(
                    "INSERT OR REPLACE INTO kv_store (key, value) VALUES (?, ?)",
                )?((key, &serialized))?;
                if !recovery {
                    connection.exec_bound::<&str>("DELETE FROM kv_store WHERE key = ?")?(
                        &recovery_key,
                    )?;
                }
                Ok(())
            })
        })
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
                for panel_key in [
                    TerminalPanel::serialization_key_for_workspace_id(workspace_id),
                    TerminalPanel::recovery_key_for_workspace_id(workspace_id),
                ] {
                    if let Some(panel) = connection.select_row_bound::<&str, String>(
                        "SELECT value FROM kv_store WHERE key = ?",
                    )?(&panel_key)?
                    {
                        let panel = serde_json::from_str::<SerializedTerminalPanel>(&panel)?;
                        retained.extend(panel.item_ids());
                    }
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
