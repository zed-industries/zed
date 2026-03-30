use std::sync::Arc;

use acp_thread::AgentSessionListRequest;
use agent::ThreadStore;
use agent_client_protocol as acp;
use chrono::Utc;
use collections::HashSet;
use fs::Fs;
use futures::FutureExt as _;
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Render, SharedString,
    Task, WeakEntity, Window,
};
use notifications::status_toast::{StatusToast, ToastIcon};
use picker::{Picker, PickerDelegate};
use project::{AgentId, AgentRegistryStore, AgentServerStore};
use ui::{Checkbox, CommonAnimationExt as _, KeyBinding, ListItem, ListItemSpacing, prelude::*};
use util::ResultExt;
use workspace::{ModalView, MultiWorkspace, Workspace};

use crate::{
    Agent, AgentPanel,
    agent_connection_store::AgentConnectionStore,
    thread_metadata_store::{ThreadMetadata, ThreadMetadataStore},
};

#[derive(Clone)]
struct AgentEntry {
    agent_id: AgentId,
    display_name: SharedString,
    icon_path: Option<SharedString>,
}

pub struct ThreadImportModal {
    picker: Entity<Picker<ThreadImportPickerDelegate>>,
    workspace: WeakEntity<Workspace>,
    multi_workspace: WeakEntity<MultiWorkspace>,
    agent_ids: Vec<AgentId>,
    unchecked_agents: HashSet<AgentId>,
    is_importing: bool,
    last_error: Option<SharedString>,
}

impl ThreadImportModal {
    pub fn new(
        agent_server_store: Entity<AgentServerStore>,
        agent_registry_store: Entity<AgentRegistryStore>,
        workspace: WeakEntity<Workspace>,
        multi_workspace: WeakEntity<MultiWorkspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let agent_entries = agent_server_store
            .read(cx)
            .external_agents()
            .map(|agent_id| {
                let display_name = agent_server_store
                    .read(cx)
                    .agent_display_name(agent_id)
                    .or_else(|| {
                        agent_registry_store
                            .read(cx)
                            .agent(agent_id)
                            .map(|agent| agent.name().clone())
                    })
                    .unwrap_or_else(|| agent_id.0.clone());
                let icon_path = agent_server_store
                    .read(cx)
                    .agent_icon(agent_id)
                    .or_else(|| {
                        agent_registry_store
                            .read(cx)
                            .agent(agent_id)
                            .and_then(|agent| agent.icon_path().cloned())
                    });

                AgentEntry {
                    agent_id: agent_id.clone(),
                    display_name,
                    icon_path,
                }
            })
            .collect::<Vec<_>>();

        let agent_ids = agent_entries
            .iter()
            .map(|entry| entry.agent_id.clone())
            .collect::<Vec<_>>();

        let thread_import_modal = cx.entity().downgrade();
        let picker: Entity<Picker<ThreadImportPickerDelegate>> = cx.new(|cx| {
            Picker::uniform_list(
                ThreadImportPickerDelegate::new(thread_import_modal, agent_entries),
                window,
                cx,
            )
        });

        Self {
            picker,
            workspace,
            multi_workspace,
            agent_ids,
            unchecked_agents: HashSet::default(),
            is_importing: false,
            last_error: None,
        }
    }

    fn set_agent_checked(&mut self, agent_id: AgentId, state: ToggleState, cx: &mut Context<Self>) {
        match state {
            ToggleState::Selected => {
                self.unchecked_agents.remove(&agent_id);
            }
            ToggleState::Unselected | ToggleState::Indeterminate => {
                self.unchecked_agents.insert(agent_id);
            }
        }
        cx.notify();
    }

    fn toggle_agent_checked(&mut self, agent_id: AgentId, cx: &mut Context<Self>) {
        if self.unchecked_agents.contains(&agent_id) {
            self.unchecked_agents.remove(&agent_id);
        } else {
            self.unchecked_agents.insert(agent_id);
        }
        cx.notify();
    }

    fn import_threads(&mut self, _: &menu::Confirm, _: &mut Window, cx: &mut Context<Self>) {
        if self.is_importing {
            return;
        }

        let Some(multi_workspace) = self.multi_workspace.upgrade() else {
            self.is_importing = false;
            cx.notify();
            return;
        };

        let stores = resolve_agent_connection_stores(&multi_workspace, cx);
        if stores.is_empty() {
            log::error!("Did not find any workspaces to import from");
            self.is_importing = false;
            cx.notify();
            return;
        }

        self.is_importing = true;
        self.last_error = None;
        cx.notify();

        let agent_ids = self
            .agent_ids
            .iter()
            .filter(|agent_id| !self.unchecked_agents.contains(*agent_id))
            .cloned()
            .collect::<Vec<_>>();

        let existing_sessions = ThreadMetadataStore::global(cx)
            .read(cx)
            .entry_ids()
            .collect::<HashSet<_>>();

        let task = find_threads_to_import(agent_ids, existing_sessions, stores, cx);
        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| match result {
                Ok(threads) => {
                    let imported_count = threads.len();
                    ThreadMetadataStore::global(cx)
                        .update(cx, |store, cx| store.save_all(threads, cx));
                    this.is_importing = false;
                    this.last_error = None;
                    this.show_imported_threads_toast(imported_count, cx);
                    cx.emit(DismissEvent);
                }
                Err(error) => {
                    this.is_importing = false;
                    this.last_error = Some(error.to_string().into());
                    cx.notify();
                }
            })
        })
        .detach_and_log_err(cx);
    }

    fn show_imported_threads_toast(&self, imported_count: usize, cx: &mut App) {
        let message = if imported_count == 1 {
            "Imported 1 thread.".to_string()
        } else {
            format!("Imported {imported_count} threads.")
        };

        self.workspace
            .update(cx, |workspace, cx| {
                let status_toast = StatusToast::new(message, cx, |this, _cx| {
                    this.icon(ToastIcon::new(IconName::Check).color(Color::Success))
                });

                workspace.toggle_status_toast(status_toast, cx);
            })
            .log_err();
    }
}

impl EventEmitter<DismissEvent> for ThreadImportModal {}

impl Focusable for ThreadImportModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl ModalView for ThreadImportModal {}

impl Render for ThreadImportModal {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("ThreadImportModal")
            .w(rems(34.))
            .child(self.picker.clone())
    }
}

struct ThreadImportPickerDelegate {
    thread_import_modal: WeakEntity<ThreadImportModal>,
    agent_entries: Vec<AgentEntry>,
    filtered_indices: Vec<usize>,
    selected_index: usize,
}

impl ThreadImportPickerDelegate {
    fn new(
        thread_import_modal: WeakEntity<ThreadImportModal>,
        agent_entries: Vec<AgentEntry>,
    ) -> Self {
        let filtered_indices = (0..agent_entries.len()).collect();
        Self {
            thread_import_modal,
            agent_entries,
            filtered_indices,
            selected_index: 0,
        }
    }
}

impl PickerDelegate for ThreadImportPickerDelegate {
    type ListItem = AnyElement;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search ACP agents…".into()
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some(if self.agent_entries.is_empty() {
            "No ACP agents available.".into()
        } else {
            "No ACP agents match your search.".into()
        })
    }

    fn match_count(&self) -> usize {
        self.filtered_indices.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix.min(self.filtered_indices.len().saturating_sub(1));
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let query = query.to_lowercase();
        self.filtered_indices = self
            .agent_entries
            .iter()
            .enumerate()
            .filter(|(_, entry)| {
                query.is_empty() || entry.display_name.to_lowercase().contains(&query)
            })
            .map(|(index, _)| index)
            .collect();
        self.selected_index = self
            .selected_index
            .min(self.filtered_indices.len().saturating_sub(1));
        cx.notify();
        Task::ready(())
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if secondary {
            self.thread_import_modal
                .update(cx, |thread_import_modal, cx| {
                    if thread_import_modal.is_importing {
                        return;
                    }

                    thread_import_modal.import_threads(&menu::Confirm, window, cx);
                })
                .log_err();
            return;
        }

        let Some(entry) = self
            .filtered_indices
            .get(self.selected_index)
            .and_then(|index| self.agent_entries.get(*index))
        else {
            return;
        };

        self.thread_import_modal
            .update(cx, |thread_import_modal, cx| {
                thread_import_modal.toggle_agent_checked(entry.agent_id.clone(), cx);
            })
            .log_err();
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.thread_import_modal
            .update(cx, |_thread_import_modal, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let entry = self.agent_entries.get(*self.filtered_indices.get(ix)?)?;
        let is_checked = self
            .thread_import_modal
            .read_with(cx, |modal, _cx| {
                !modal.unchecked_agents.contains(&entry.agent_id)
            })
            .ok()
            .unwrap_or(false);

        Some(
            ListItem::new(("thread-import-agent", ix))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .start_slot(
                    Checkbox::new(
                        ("thread-import-agent-checkbox", ix),
                        if is_checked {
                            ToggleState::Selected
                        } else {
                            ToggleState::Unselected
                        },
                    )
                    .on_click({
                        let thread_import_modal = self.thread_import_modal.clone();
                        let agent_id = entry.agent_id.clone();
                        move |state, _window, cx| {
                            thread_import_modal
                                .update(cx, |thread_import_modal, cx| {
                                    thread_import_modal.set_agent_checked(
                                        agent_id.clone(),
                                        *state,
                                        cx,
                                    );
                                })
                                .log_err();
                        }
                    }),
                )
                .child(
                    h_flex()
                        .w_full()
                        .gap_2()
                        .child(if let Some(icon_path) = entry.icon_path.clone() {
                            Icon::from_external_svg(icon_path)
                                .color(Color::Muted)
                                .size(IconSize::Small)
                        } else {
                            Icon::new(IconName::Sparkle)
                                .color(Color::Muted)
                                .size(IconSize::Small)
                        })
                        .child(Label::new(entry.display_name.clone())),
                )
                .into_any_element(),
        )
    }

    fn render_footer(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        let (is_importing, last_error) = self
            .thread_import_modal
            .read_with(cx, |thread_import_modal, _cx| {
                (
                    thread_import_modal.is_importing,
                    thread_import_modal.last_error.clone(),
                )
            })
            .ok()
            .unwrap_or((false, None));

        Some(
            h_flex()
                .w_full()
                .p_1p5()
                .gap_2()
                .items_center()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .when_some(last_error, |this, error| {
                            this.child(
                                Label::new(error)
                                    .size(LabelSize::Small)
                                    .color(Color::Error)
                                    .truncate(),
                            )
                        }),
                )
                .child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .when(is_importing, |this| {
                            this.child(
                                Icon::new(IconName::ArrowCircle)
                                    .size(IconSize::Small)
                                    .color(Color::Muted)
                                    .with_rotate_animation(2),
                            )
                        })
                        .child(
                            Button::new("import-threads", "Import Threads")
                                .disabled(is_importing)
                                .key_binding(
                                    KeyBinding::for_action(&menu::SecondaryConfirm, cx)
                                        .map(|kb| kb.size(rems_from_px(12.))),
                                )
                                .on_click({
                                    let thread_import_modal = self.thread_import_modal.clone();
                                    move |_, window, cx| {
                                        thread_import_modal
                                            .update(cx, |thread_import_modal, cx| {
                                                if thread_import_modal.is_importing {
                                                    return;
                                                }

                                                thread_import_modal.import_threads(
                                                    &menu::Confirm,
                                                    window,
                                                    cx,
                                                );
                                            })
                                            .log_err();
                                    }
                                }),
                        ),
                )
                .into_any_element(),
        )
    }
}

fn resolve_agent_connection_stores(
    multi_workspace: &Entity<MultiWorkspace>,
    cx: &App,
) -> Vec<Entity<AgentConnectionStore>> {
    let mut stores = Vec::new();
    let mut included_local_store = false;

    for workspace in multi_workspace.read(cx).workspaces() {
        let workspace = workspace.read(cx);
        let project = workspace.project().read(cx);

        // We only want to include scores from one local workspace, since we
        // know that they live on the same machine
        let include_store = if project.is_remote() {
            true
        } else if project.is_local() && !included_local_store {
            included_local_store = true;
            true
        } else {
            false
        };

        if !include_store {
            continue;
        }

        if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
            stores.push(panel.read(cx).connection_store().clone());
        }
    }

    stores
}

fn find_threads_to_import(
    agent_ids: Vec<AgentId>,
    existing_sessions: HashSet<acp::SessionId>,
    stores: Vec<Entity<AgentConnectionStore>>,
    cx: &mut App,
) -> Task<anyhow::Result<Vec<ThreadMetadata>>> {
    let mut wait_for_connection_tasks = Vec::new();

    for store in stores {
        for agent_id in agent_ids.clone() {
            let agent = Agent::from(agent_id.clone());
            let server = agent.server(<dyn Fs>::global(cx), ThreadStore::global(cx));
            let entry = store.update(cx, |store, cx| store.request_connection(agent, server, cx));
            wait_for_connection_tasks
                .push(entry.read(cx).wait_for_connection().map(|s| (agent_id, s)));
        }
    }

    let mut session_list_tasks = Vec::new();
    cx.spawn(async move |cx| {
        let results = futures::future::join_all(wait_for_connection_tasks).await;
        for (agent, result) in results {
            let Some(state) = result.log_err() else {
                continue;
            };
            let Some(list) = cx.update(|cx| state.connection.session_list(cx)) else {
                continue;
            };
            let task = cx.update(|cx| {
                list.list_sessions(AgentSessionListRequest::default(), cx)
                    .map(|r| (agent, r))
            });
            session_list_tasks.push(task);
        }

        let mut sessions_by_agent = Vec::new();
        let results = futures::future::join_all(session_list_tasks).await;
        for (agent_id, result) in results {
            let Some(response) = result.log_err() else {
                continue;
            };
            sessions_by_agent.push((agent_id, response.sessions));
        }

        Ok(collect_importable_threads(
            sessions_by_agent,
            existing_sessions,
        ))
    })
}

fn collect_importable_threads(
    sessions_by_agent: Vec<(AgentId, Vec<acp_thread::AgentSessionInfo>)>,
    mut existing_sessions: HashSet<acp::SessionId>,
) -> Vec<ThreadMetadata> {
    let mut to_insert = Vec::new();
    for (agent_id, sessions) in sessions_by_agent {
        for session in sessions {
            if !existing_sessions.insert(session.session_id.clone()) {
                continue;
            }
            let Some(folder_paths) = session.work_dirs else {
                continue;
            };
            to_insert.push(ThreadMetadata {
                session_id: session.session_id,
                agent_id: agent_id.clone(),
                title: session
                    .title
                    .unwrap_or_else(|| crate::DEFAULT_THREAD_TITLE.into()),
                updated_at: session.updated_at.unwrap_or_else(|| Utc::now()),
                created_at: session.created_at,
                folder_paths,
                archived: true,
            });
        }
    }
    to_insert
}

#[cfg(test)]
mod tests {
    use super::*;
    use acp_thread::AgentSessionInfo;
    use chrono::Utc;
    use std::path::Path;
    use workspace::PathList;

    fn make_session(
        session_id: &str,
        title: Option<&str>,
        work_dirs: Option<PathList>,
        updated_at: Option<chrono::DateTime<Utc>>,
        created_at: Option<chrono::DateTime<Utc>>,
    ) -> AgentSessionInfo {
        AgentSessionInfo {
            session_id: acp::SessionId::new(session_id),
            title: title.map(|t| SharedString::from(t.to_string())),
            work_dirs,
            updated_at,
            created_at,
            meta: None,
        }
    }

    #[test]
    fn test_collect_skips_sessions_already_in_existing_set() {
        let existing = HashSet::from_iter(vec![acp::SessionId::new("existing-1")]);
        let paths = PathList::new(&[Path::new("/project")]);

        let sessions_by_agent = vec![(
            AgentId::new("agent-a"),
            vec![
                make_session(
                    "existing-1",
                    Some("Already There"),
                    Some(paths.clone()),
                    None,
                    None,
                ),
                make_session("new-1", Some("Brand New"), Some(paths), None, None),
            ],
        )];

        let result = collect_importable_threads(sessions_by_agent, existing);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].session_id.0.as_ref(), "new-1");
        assert_eq!(result[0].title.as_ref(), "Brand New");
    }

    #[test]
    fn test_collect_skips_sessions_without_work_dirs() {
        let existing = HashSet::default();
        let paths = PathList::new(&[Path::new("/project")]);

        let sessions_by_agent = vec![(
            AgentId::new("agent-a"),
            vec![
                make_session("has-dirs", Some("With Dirs"), Some(paths), None, None),
                make_session("no-dirs", Some("No Dirs"), None, None, None),
            ],
        )];

        let result = collect_importable_threads(sessions_by_agent, existing);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].session_id.0.as_ref(), "has-dirs");
    }

    #[test]
    fn test_collect_marks_all_imported_threads_as_archived() {
        let existing = HashSet::default();
        let paths = PathList::new(&[Path::new("/project")]);

        let sessions_by_agent = vec![(
            AgentId::new("agent-a"),
            vec![
                make_session("s1", Some("Thread 1"), Some(paths.clone()), None, None),
                make_session("s2", Some("Thread 2"), Some(paths), None, None),
            ],
        )];

        let result = collect_importable_threads(sessions_by_agent, existing);

        assert_eq!(result.len(), 2);
        assert!(result.iter().all(|t| t.archived));
    }

    #[test]
    fn test_collect_assigns_correct_agent_id_per_session() {
        let existing = HashSet::default();
        let paths = PathList::new(&[Path::new("/project")]);

        let sessions_by_agent = vec![
            (
                AgentId::new("agent-a"),
                vec![make_session(
                    "s1",
                    Some("From A"),
                    Some(paths.clone()),
                    None,
                    None,
                )],
            ),
            (
                AgentId::new("agent-b"),
                vec![make_session("s2", Some("From B"), Some(paths), None, None)],
            ),
        ];

        let result = collect_importable_threads(sessions_by_agent, existing);

        assert_eq!(result.len(), 2);
        let s1 = result
            .iter()
            .find(|t| t.session_id.0.as_ref() == "s1")
            .unwrap();
        let s2 = result
            .iter()
            .find(|t| t.session_id.0.as_ref() == "s2")
            .unwrap();
        assert_eq!(s1.agent_id.as_ref(), "agent-a");
        assert_eq!(s2.agent_id.as_ref(), "agent-b");
    }

    #[test]
    fn test_collect_deduplicates_across_agents() {
        let existing = HashSet::default();
        let paths = PathList::new(&[Path::new("/project")]);

        let sessions_by_agent = vec![
            (
                AgentId::new("agent-a"),
                vec![make_session(
                    "shared-session",
                    Some("From A"),
                    Some(paths.clone()),
                    None,
                    None,
                )],
            ),
            (
                AgentId::new("agent-b"),
                vec![make_session(
                    "shared-session",
                    Some("From B"),
                    Some(paths),
                    None,
                    None,
                )],
            ),
        ];

        let result = collect_importable_threads(sessions_by_agent, existing);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].session_id.0.as_ref(), "shared-session");
        assert_eq!(
            result[0].agent_id.as_ref(),
            "agent-a",
            "first agent encountered should win"
        );
    }

    #[test]
    fn test_collect_all_existing_returns_empty() {
        let paths = PathList::new(&[Path::new("/project")]);
        let existing =
            HashSet::from_iter(vec![acp::SessionId::new("s1"), acp::SessionId::new("s2")]);

        let sessions_by_agent = vec![(
            AgentId::new("agent-a"),
            vec![
                make_session("s1", Some("T1"), Some(paths.clone()), None, None),
                make_session("s2", Some("T2"), Some(paths), None, None),
            ],
        )];

        let result = collect_importable_threads(sessions_by_agent, existing);
        assert!(result.is_empty());
    }
}
