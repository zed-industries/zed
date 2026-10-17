use anyhow::{Result, anyhow};
use collections::HashSet;
use futures::{
    FutureExt as _,
    channel::oneshot,
    future::{Either, Shared, join_all, select},
};
use gpui::{AppContext as _, Axis, Entity, Task, WeakEntity};
use project::Project;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use ui::{App, Context, SharedString, Window};
use util::ResultExt as _;

use db::{
    query,
    sqlez::{domain::Domain, statement::Statement, thread_safe_connection::ThreadSafeConnection},
    sqlez_macros::sql,
};
use workspace::{
    ItemId, Member, Pane, PaneAxis, PaneGroup, SerializableItem as _, SerializedItem,
    SplitDirection, Workspace, WorkspaceDb, WorkspaceId, invalid_item_view::RestoringItem,
    item::item_persistence_id,
};

use crate::{
    TerminalView, default_working_directory,
    terminal_panel::{TerminalPanel, new_terminal_pane},
};

pub(crate) type PersistenceTask = Shared<Task<Result<(), Arc<anyhow::Error>>>>;

pub(crate) fn serialize_pane_group(
    pane_group: &PaneGroup,
    active_pane: &Entity<Pane>,
    cx: &mut App,
) -> Task<Result<SerializedPaneGroup>> {
    build_serialized_pane_group(&pane_group.root, active_pane, cx)
}

fn build_serialized_pane_group(
    pane_group: &Member,
    active_pane: &Entity<Pane>,
    cx: &mut App,
) -> Task<Result<SerializedPaneGroup>> {
    match pane_group {
        Member::Axis(PaneAxis {
            axis,
            members,
            flexes,
            bounding_boxes: _,
        }) => {
            let axis = SerializedAxis(*axis);
            let children = members
                .iter()
                .map(|member| build_serialized_pane_group(member, active_pane, cx))
                .collect::<Vec<_>>();
            let flexes = Some(flexes.lock().clone());
            cx.background_spawn(async move {
                Ok(SerializedPaneGroup::Group {
                    axis,
                    children: futures::future::try_join_all(children).await?,
                    flexes,
                })
            })
        }
        Member::Pane(pane_handle) => serialize_pane(pane_handle, pane_handle == active_pane, cx),
    }
}

fn serialize_pane(
    pane: &Entity<Pane>,
    active: bool,
    cx: &mut App,
) -> Task<Result<SerializedPaneGroup>> {
    let pane = pane.read(cx);
    let active_item = pane.active_item().map(|item| item.item_id());
    let pinned_count = pane.pinned_count();
    let items = pane
        .items()
        .enumerate()
        .map(|(index, item)| {
            let active = Some(item.item_id()) == active_item;
            (item.boxed_clone(), active, index < pinned_count)
        })
        .collect::<Vec<_>>();
    let items = items
        .into_iter()
        .filter_map(|(item, active, pinned)| {
            let item_id = if let Some(placeholder) = item.downcast::<RestoringItem>() {
                let item = &placeholder.read(cx).serialized_item;
                if item.kind.as_ref() != "Terminal" {
                    return None;
                }
                Task::ready(Ok(item.item_id)).shared()
            } else {
                let terminal = item.act_as::<TerminalView>(cx)?;
                if terminal.read(cx).terminal().read(cx).task().is_some() {
                    return None;
                }
                item_persistence_id(&terminal, cx)
            };
            Some((item_id, active, pinned))
        })
        .collect::<Vec<_>>();
    cx.background_spawn(async move {
        let mut pane = SerializedPane {
            active,
            children: Vec::new(),
            active_item: None,
            pinned_count: 0,
        };
        for (item_id, active, pinned) in items {
            let item_id = item_id.await.map_err(|error| anyhow!(error))?;
            pane.children.push(item_id);
            if active {
                pane.active_item = Some(item_id);
            }
            pane.pinned_count += usize::from(pinned);
        }
        Ok(SerializedPaneGroup::Pane(pane))
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
    let restore_empty_panes = matches!(&serialized_panel.items, SerializedItems::WithSplits(_));
    let initialized = initialize_terminal_panel(
        workspace.clone(),
        project.clone(),
        serialized_panel,
        &terminal_panel,
        window,
        cx,
    );
    let placeholders = match initialized {
        Ok(placeholders) => placeholders,
        Err(error) => return Task::ready(Err(error)),
    };
    let mut tasks = Vec::new();
    for placeholder in placeholders {
        let item_id = placeholder.read(cx).serialized_item.item_id;
        let deserialize = TerminalView::deserialize(
            project.clone(),
            workspace.clone(),
            database_id,
            item_id,
            window,
            cx,
        );
        tasks.push(hydrate_terminal(
            workspace.clone(),
            placeholder,
            deserialize,
            window,
            cx,
        ));
    }
    let empty_panes = terminal_panel
        .read_with(cx, |panel, cx| {
            panel
                .center
                .panes()
                .into_iter()
                .filter(|pane| restore_empty_panes && pane.read(cx).items_len() == 0)
                .map(Entity::downgrade)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    for pane in empty_panes {
        let workspace = workspace.clone();
        let project = project.clone();
        let terminal_panel = terminal_panel.clone();
        tasks.push(window.spawn(cx, async move |cx| {
            let cwd = workspace
                .read_with(cx, |workspace, cx| default_working_directory(workspace, cx))
                .ok()
                .flatten();
            let Some(terminal) = project
                .update(cx, |project, cx| project.create_terminal_shell(cwd, cx))
                .await
                .log_err()
            else {
                return false;
            };
            if !terminal_panel
                .read_with(cx, |panel, _| {
                    panel
                        .center
                        .panes()
                        .into_iter()
                        .any(|candidate| candidate.downgrade() == pane)
                })
                .unwrap_or(false)
            {
                return false;
            }
            pane.update_in(cx, |pane, window, cx| {
                if pane.items_len() > 0 {
                    return false;
                }
                let terminal = cx.new(|cx| {
                    TerminalView::new(
                        terminal,
                        workspace,
                        Some(database_id),
                        project.downgrade(),
                        window,
                        cx,
                    )
                });
                pane.add_item(Box::new(terminal), false, false, None, window, cx);
                true
            })
            .unwrap_or(false)
        }));
    }
    cx.background_spawn(async move {
        Ok(join_all(tasks)
            .await
            .into_iter()
            .filter(|restored| *restored)
            .count())
    })
}

pub(crate) fn initialize_terminal_panel(
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    serialized_panel: SerializedTerminalPanel,
    terminal_panel: &WeakEntity<TerminalPanel>,
    window: &mut Window,
    cx: &mut App,
) -> Result<Vec<Entity<RestoringItem>>> {
    terminal_panel.update(cx, |panel, cx| {
        panel.has_serialized_state = true;
        let mut placeholders = Vec::new();
        match serialized_panel.items {
            SerializedItems::NoSplits(children) => {
                let serialized = SerializedPane {
                    active: true,
                    children,
                    active_item: serialized_panel.active_item_id,
                    pinned_count: 0,
                };
                placeholders.extend(populate_pane_items(
                    &panel.active_pane,
                    &serialized,
                    window,
                    cx,
                ));
            }
            SerializedItems::WithSplits(serialized) => {
                let interim_panes = panel
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
                let Some((member, active_pane)) = deserialize_pane_group(
                    workspace,
                    project,
                    &serialized,
                    &mut placeholders,
                    window,
                    cx,
                ) else {
                    return placeholders;
                };
                let zoomed = panel.active_pane.read(cx).is_zoomed();
                panel.center = PaneGroup::with_root(member);
                for pane in panel.center.panes() {
                    pane.update(cx, |pane, cx| pane.set_zoomed(zoomed, cx));
                }
                panel.active_pane = active_pane.unwrap_or_else(|| panel.center.first_pane());
                let restored_pane = panel.active_pane.clone();
                for pane in interim_panes {
                    panel
                        .center
                        .split(&restored_pane, &pane, SplitDirection::Right, cx);
                }
                if let Some(pane) = focused_interim_pane {
                    panel.active_pane = pane;
                }
            }
        }
        cx.notify();
        placeholders
    })
}

pub(crate) fn hydrate_terminal(
    workspace: WeakEntity<Workspace>,
    placeholder: Entity<RestoringItem>,
    deserialize: Task<Result<Entity<TerminalView>>>,
    window: &mut Window,
    cx: &mut App,
) -> Task<bool> {
    let (released, release) = oneshot::channel();
    let subscription = cx.observe_release(&placeholder, move |_, _| {
        released.send(()).ok();
    });
    let placeholder = placeholder.downgrade();
    window.spawn(cx, async move |cx| {
        let _subscription = subscription;
        let restored = match select(deserialize, release).await {
            Either::Left((restored, _)) => restored,
            Either::Right(_) => return false,
        };
        let Some(placeholder) = placeholder.upgrade() else {
            return false;
        };
        let pane = workspace
            .read_with(cx, |workspace, _| {
                workspace.pane_for_item_id(placeholder.entity_id())
            })
            .ok()
            .flatten();
        let Some(pane) = pane else {
            return false;
        };
        pane.update_in(cx, |pane, window, cx| match restored {
            Ok(terminal) => {
                pane.replace_item(placeholder.entity_id(), Box::new(terminal), window, cx)
            }
            Err(error) => {
                placeholder.update(cx, |placeholder, cx| {
                    placeholder.set_error(SharedString::from(format!("{error:#}")), cx);
                });
                false
            }
        })
        .unwrap_or(false)
    })
}

fn populate_pane_items(
    pane: &Entity<Pane>,
    serialized: &SerializedPane,
    window: &mut Window,
    cx: &mut App,
) -> Vec<Entity<RestoringItem>> {
    pane.update(cx, |pane, cx| {
        let pinned_count = pane.pinned_count();
        let had_items = pane.items_len() > 0;
        let mut placeholders = Vec::new();
        for item_id in &serialized.children {
            let placeholder = cx.new(|cx| {
                RestoringItem::new(SerializedItem::new("Terminal", *item_id, false, false), cx)
            });
            let active = !had_items
                && serialized
                    .active_item
                    .map_or(placeholders.is_empty(), |active_item| {
                        *item_id == active_item
                    });
            pane.add_restoring_item(placeholder.clone(), active, window, cx);
            placeholders.push(placeholder);
        }
        pane.set_pinned_count(if had_items {
            pinned_count
        } else {
            serialized.pinned_count.min(placeholders.len())
        });

        placeholders
    })
}

fn deserialize_pane_group(
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    serialized: &SerializedPaneGroup,
    placeholders: &mut Vec<Entity<RestoringItem>>,
    window: &mut Window,
    cx: &mut Context<TerminalPanel>,
) -> Option<(Member, Option<Entity<Pane>>)> {
    if let SerializedPaneGroup::Group {
        axis,
        flexes,
        children,
    } = serialized
    {
        let mut active_pane = None;
        let mut members = children
            .iter()
            .filter_map(|child| {
                let (member, active) = deserialize_pane_group(
                    workspace.clone(),
                    project.clone(),
                    child,
                    placeholders,
                    window,
                    cx,
                )?;
                active_pane = active_pane.take().or(active);
                Some(member)
            })
            .collect::<Vec<_>>();
        if members.len() <= 1 {
            return members.pop().map(|member| (member, active_pane));
        }
        let flexes = (members.len() == children.len())
            .then(|| flexes.clone())
            .flatten();
        return Some((
            Member::Axis(PaneAxis::load(axis.0, members, flexes)),
            active_pane,
        ));
    }
    let pane = new_terminal_pane(workspace, project, false, window, cx);
    let mut active = false;
    if let SerializedPaneGroup::Pane(serialized) = serialized {
        placeholders.extend(populate_pane_items(&pane, serialized, window, cx));
        active = serialized.active;
    }
    Some((Member::Pane(pane.clone()), active.then_some(pane)))
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SerializedTerminalPanel {
    pub items: SerializedItems,
    // A deprecated field, kept for backwards compatibility for the code before terminal splits were introduced.
    pub active_item_id: Option<u64>,
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

impl SerializedTerminalPanel {
    pub(crate) fn into_pane_group(self) -> SerializedPaneGroup {
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

impl SerializedPaneGroup {
    pub(crate) fn item_ids(&self) -> Vec<ItemId> {
        match self {
            Self::Pane(pane) => pane.children.clone(),
            Self::Group { children, .. } => children.iter().flat_map(Self::item_ids).collect(),
        }
    }
}

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

db::static_connection!(TerminalDb, [WorkspaceDb]);

impl TerminalDb {
    pub async fn cleanup(
        &self,
        workspace_id: WorkspaceId,
        candidates: Vec<ItemId>,
        mut alive_items: HashSet<ItemId>,
    ) -> Result<()> {
        self.write(move |connection| {
            connection.with_savepoint("cleanup_terminals", || {
                let key = TerminalPanel::database_serialization_key(workspace_id);
                if let Some(state) = connection.select_row_bound::<_, String>(
                    "SELECT value FROM kv_store WHERE key = ?",
                )?(key.as_str())? {
                    let Some(panel) = serde_json::from_str::<SerializedTerminalPanel>(&state).log_err() else {
                        return Ok(());
                    };
                    alive_items.extend(panel.into_pane_group().item_ids());
                }
                let mut statement = Statement::prepare(connection,
                    "DELETE FROM terminals WHERE workspace_id = ?1 AND item_id = ?2
                     AND NOT EXISTS (SELECT 1 FROM items WHERE workspace_id = ?1 AND item_id = ?2 AND kind = 'Terminal')"
                )?;
                for item_id in candidates {
                    if !alive_items.contains(&item_id) {
                        statement.bind(&workspace_id, 1)?;
                        statement.bind(&item_id, 2)?;
                        statement.exec()?;
                        statement.reset();
                    }
                }
                Ok(())
            })
        }).await
    }

    query! {
        pub fn item_ids(workspace_id: WorkspaceId) -> Result<Vec<ItemId>> {
            SELECT item_id FROM terminals WHERE workspace_id = ?
        }
    }

    query! {
       pub async fn update_workspace_id(
            new_id: WorkspaceId,
            old_id: WorkspaceId,
            item_id: ItemId
        ) -> Result<()> {
            UPDATE terminals
            SET workspace_id = ?
            WHERE workspace_id = ? AND item_id = ?
        }
    }

    pub async fn save_working_directory(
        &self,
        item_id: ItemId,
        workspace_id: WorkspaceId,
        working_directory: PathBuf,
    ) -> Result<()> {
        if working_directory.as_os_str().is_empty() {
            return Ok(());
        }
        log::debug!(
            "Saving working directory {working_directory:?} for item {item_id} in workspace {workspace_id:?}"
        );
        let query =
            "INSERT INTO terminals(item_id, workspace_id, working_directory, working_directory_path)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT DO UPDATE SET
                item_id = ?1,
                workspace_id = ?2,
                working_directory = ?3,
                working_directory_path = ?4"
        ;
        self.write(move |conn| {
            let mut statement = Statement::prepare(conn, query)?;
            let mut next_index = statement.bind(&item_id, 1)?;
            next_index = statement.bind(&workspace_id, next_index)?;
            next_index = statement.bind(&working_directory, next_index)?;
            statement.bind(
                &working_directory.to_string_lossy().into_owned(),
                next_index,
            )?;
            statement.exec()
        })
        .await
    }

    query! {
        pub fn get_working_directory(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<PathBuf>> {
            SELECT working_directory
            FROM terminals
            WHERE item_id = ? AND workspace_id = ?
        }
    }

    pub async fn save_custom_title(
        &self,
        item_id: ItemId,
        workspace_id: WorkspaceId,
        custom_title: Option<String>,
    ) -> Result<()> {
        log::debug!(
            "Saving custom title {:?} for item {} in workspace {:?}",
            custom_title,
            item_id,
            workspace_id
        );
        self.write(move |conn| {
            let query = "INSERT INTO terminals (item_id, workspace_id, custom_title)
                VALUES (?1, ?2, ?3)
                ON CONFLICT (workspace_id, item_id) DO UPDATE SET
                    custom_title = excluded.custom_title";
            let mut statement = Statement::prepare(conn, query)?;
            let mut next_index = statement.bind(&item_id, 1)?;
            next_index = statement.bind(&workspace_id, next_index)?;
            statement.bind(&custom_title, next_index)?;
            statement.exec()
        })
        .await
    }

    query! {
        pub fn get_custom_title(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<String>> {
            SELECT custom_title
            FROM terminals
            WHERE item_id = ? AND workspace_id = ?
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[gpui::test]
    async fn test_terminal_metadata_preserves_directory_when_unavailable(cx: &mut TestAppContext) {
        cx.update(|cx| cx.set_global(db::AppDatabase::test_new()));
        let workspace_id = cx
            .update(|cx| WorkspaceDb::global(cx))
            .next_id()
            .await
            .unwrap();
        let db = cx.update(|cx| TerminalDb::global(cx));
        let directory = PathBuf::from("/saved/terminal");
        db.save_working_directory(1, workspace_id, directory.clone())
            .await
            .unwrap();
        db.save_working_directory(1, workspace_id, PathBuf::new())
            .await
            .unwrap();
        assert_eq!(
            db.get_working_directory(1, workspace_id).unwrap(),
            Some(directory)
        );
    }

    #[gpui::test]
    async fn test_terminal_cleanup_preserves_saved_center_and_new_rows(cx: &mut TestAppContext) {
        cx.update(|cx| cx.set_global(db::AppDatabase::test_new()));
        let workspace_id = cx
            .update(|cx| WorkspaceDb::global(cx))
            .next_id()
            .await
            .unwrap();
        let db = cx.update(|cx| TerminalDb::global(cx));
        db.write(move |connection| {
            connection.exec_bound("INSERT INTO panes (pane_id, workspace_id, active) VALUES (1, ?, 1)")?(workspace_id)?;
            connection.exec_bound("INSERT INTO items (item_id, workspace_id, pane_id, kind, position, active) VALUES (1, ?, 1, 'Terminal', 0, 1)")?(workspace_id)
        }).await.unwrap();
        for item_id in [1, 2, 3, 5] {
            db.save_custom_title(item_id, workspace_id, Some(item_id.to_string()))
                .await
                .unwrap();
        }
        let candidates = db.item_ids(workspace_id).unwrap();
        let cleanup = db.cleanup(workspace_id, candidates, HashSet::from_iter([3]));
        db.save_custom_title(4, workspace_id, Some("new".to_string()))
            .await
            .unwrap();
        let kvp = cx.update(|cx| db::kvp::KeyValueStore::global(cx));
        kvp.write_kvp(
            TerminalPanel::database_serialization_key(workspace_id),
            serde_json::to_string(&SerializedTerminalPanel {
                items: SerializedItems::NoSplits(vec![5]),
                active_item_id: Some(5),
            })
            .unwrap(),
        )
        .await
        .unwrap();
        cleanup.await.unwrap();
        let mut remaining = db.item_ids(workspace_id).unwrap();
        remaining.sort_unstable();
        assert_eq!(remaining, vec![1, 3, 4, 5]);
    }
}
