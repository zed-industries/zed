use acp_thread::AgentSessionListRequest;
use agent::ThreadStore;
use agent_client_protocol as acp;
use chrono::Utc;
use collections::HashSet;
use db::kvp::Dismissable;
use fs::Fs;
use futures::FutureExt as _;
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, MouseDownEvent,
    Render, SharedString, Task, WeakEntity, Window,
};
use notifications::status_toast::{StatusToast, ToastIcon};
use project::{AgentId, AgentRegistryStore, AgentServerStore};
use remote::RemoteConnectionOptions;
use ui::{
    Checkbox, KeyBinding, ListItem, ListItemSpacing, Modal, ModalFooter, ModalHeader, Section,
    prelude::*,
};
use util::ResultExt;
use workspace::{ModalView, MultiWorkspace, PathList, Workspace};

use crate::{
    Agent, AgentPanel,
    agent_connection_store::AgentConnectionStore,
    thread_metadata_store::{ThreadMetadata, ThreadMetadataStore},
};

pub struct AcpThreadImportOnboarding;

impl AcpThreadImportOnboarding {
    pub fn dismissed(cx: &App) -> bool {
        <Self as Dismissable>::dismissed(cx)
    }

    pub fn dismiss(cx: &mut App) {
        <Self as Dismissable>::set_dismissed(true, cx);
    }
}

impl Dismissable for AcpThreadImportOnboarding {
    const KEY: &'static str = "dismissed-acp-thread-import";
}

#[derive(Clone)]
struct AgentEntry {
    agent_id: AgentId,
    display_name: SharedString,
    icon_path: Option<SharedString>,
}

pub struct ThreadImportModal {
    focus_handle: FocusHandle,
    workspace: WeakEntity<Workspace>,
    multi_workspace: WeakEntity<MultiWorkspace>,
    agent_entries: Vec<AgentEntry>,
    unchecked_agents: HashSet<AgentId>,
    selected_index: Option<usize>,
    is_importing: bool,
    last_error: Option<SharedString>,
}

impl ThreadImportModal {
    pub fn new(
        agent_server_store: Entity<AgentServerStore>,
        agent_registry_store: Entity<AgentRegistryStore>,
        workspace: WeakEntity<Workspace>,
        multi_workspace: WeakEntity<MultiWorkspace>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        AcpThreadImportOnboarding::dismiss(cx);

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

        Self {
            focus_handle: cx.focus_handle(),
            workspace,
            multi_workspace,
            agent_entries,
            unchecked_agents: HashSet::default(),
            selected_index: None,
            is_importing: false,
            last_error: None,
        }
    }

    fn agent_ids(&self) -> Vec<AgentId> {
        self.agent_entries
            .iter()
            .map(|entry| entry.agent_id.clone())
            .collect()
    }

    fn toggle_agent_checked(&mut self, agent_id: AgentId, cx: &mut Context<Self>) {
        if self.unchecked_agents.contains(&agent_id) {
            self.unchecked_agents.remove(&agent_id);
        } else {
            self.unchecked_agents.insert(agent_id);
        }
        cx.notify();
    }

    fn select_next(&mut self, _: &menu::SelectNext, _window: &mut Window, cx: &mut Context<Self>) {
        if self.agent_entries.is_empty() {
            return;
        }
        self.selected_index = Some(match self.selected_index {
            Some(ix) if ix + 1 >= self.agent_entries.len() => 0,
            Some(ix) => ix + 1,
            None => 0,
        });
        cx.notify();
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.agent_entries.is_empty() {
            return;
        }
        self.selected_index = Some(match self.selected_index {
            Some(0) => self.agent_entries.len() - 1,
            Some(ix) => ix - 1,
            None => self.agent_entries.len() - 1,
        });
        cx.notify();
    }

    fn confirm(&mut self, _: &menu::Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ix) = self.selected_index {
            if let Some(entry) = self.agent_entries.get(ix) {
                self.toggle_agent_checked(entry.agent_id.clone(), cx);
            }
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn import_threads(
        &mut self,
        _: &menu::SecondaryConfirm,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
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
            .agent_ids()
            .into_iter()
            .filter(|agent_id| !self.unchecked_agents.contains(agent_id))
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
        let status_toast = if imported_count == 0 {
            StatusToast::new("No threads found to import.", cx, |this, _cx| {
                this.icon(ToastIcon::new(IconName::Info).color(Color::Muted))
                    .dismiss_button(true)
            })
        } else {
            let message = if imported_count == 1 {
                "Imported 1 thread.".to_string()
            } else {
                format!("Imported {imported_count} threads.")
            };
            StatusToast::new(message, cx, |this, _cx| {
                this.icon(ToastIcon::new(IconName::Check).color(Color::Success))
                    .dismiss_button(true)
            })
        };

        self.workspace
            .update(cx, |workspace, cx| {
                workspace.toggle_status_toast(status_toast, cx);
            })
            .log_err();
    }
}

impl EventEmitter<DismissEvent> for ThreadImportModal {}

impl Focusable for ThreadImportModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for ThreadImportModal {}

impl Render for ThreadImportModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_agents = !self.agent_entries.is_empty();
        let disabled_import_thread = self.is_importing
            || !has_agents
            || self.unchecked_agents.len() == self.agent_entries.len();

        let agent_rows = self
            .agent_entries
            .iter()
            .enumerate()
            .map(|(ix, entry)| {
                let is_checked = !self.unchecked_agents.contains(&entry.agent_id);
                let is_focused = self.selected_index == Some(ix);

                ListItem::new(("thread-import-agent", ix))
                    .rounded()
                    .spacing(ListItemSpacing::Sparse)
                    .focused(is_focused)
                    .disabled(self.is_importing)
                    .child(
                        h_flex()
                            .w_full()
                            .gap_2()
                            .when(!is_checked, |this| this.opacity(0.6))
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
                    .end_slot(Checkbox::new(
                        ("thread-import-agent-checkbox", ix),
                        if is_checked {
                            ToggleState::Selected
                        } else {
                            ToggleState::Unselected
                        },
                    ))
                    .on_click({
                        let agent_id = entry.agent_id.clone();
                        cx.listener(move |this, _event, _window, cx| {
                            this.toggle_agent_checked(agent_id.clone(), cx);
                        })
                    })
            })
            .collect::<Vec<_>>();

        v_flex()
            .id("thread-import-modal")
            .key_context("ThreadImportModal")
            .w(rems(34.))
            .elevation_3(cx)
            .overflow_hidden()
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::import_threads))
            .on_any_mouse_down(cx.listener(|this, _: &MouseDownEvent, window, cx| {
                this.focus_handle.focus(window, cx);
            }))
            .child(
                Modal::new("import-threads", None)
                    .header(
                        ModalHeader::new()
                            .headline("Import ACP Threads")
                            .description(
                                "Import threads from your ACP agents — whether started in Zed or another client. \
                                Choose which agents to include, and their threads will appear in your archive."
                            )
                            .show_dismiss_button(true),

                    )
                    .section(
                        Section::new().child(
                            v_flex()
                                .id("thread-import-agent-list")
                                .max_h(rems_from_px(320.))
                                .pb_1()
                                .overflow_y_scroll()
                                .when(has_agents, |this| this.children(agent_rows))
                                .when(!has_agents, |this| {
                                    this.child(
                                        Label::new("No ACP agents available.")
                                            .color(Color::Muted)
                                            .size(LabelSize::Small),
                                    )
                                }),
                        ),
                    )
                    .footer(
                        ModalFooter::new()
                            .when_some(self.last_error.clone(), |this, error| {
                                this.start_slot(
                                    Label::new(error)
                                        .size(LabelSize::Small)
                                        .color(Color::Error)
                                        .truncate(),
                                )
                            })
                            .end_slot(
                                Button::new("import-threads", "Import Threads")
                                    .loading(self.is_importing)
                                    .disabled(disabled_import_thread)
                                    .key_binding(
                                        KeyBinding::for_action(&menu::SecondaryConfirm, cx)
                                            .map(|kb| kb.size(rems_from_px(12.))),
                                    )
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.import_threads(&menu::SecondaryConfirm, window, cx);
                                    })),
                            ),
                    ),
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
        let remote_connection = store
            .read(cx)
            .project()
            .read(cx)
            .remote_connection_options(cx);

        for agent_id in agent_ids.clone() {
            let agent = Agent::from(agent_id.clone());
            let server = agent.server(<dyn Fs>::global(cx), ThreadStore::global(cx));
            let entry = store.update(cx, |store, cx| store.request_connection(agent, server, cx));

            wait_for_connection_tasks.push(entry.read(cx).wait_for_connection().map({
                let remote_connection = remote_connection.clone();
                move |state| (agent_id, remote_connection, state)
            }));
        }
    }

    let mut session_list_tasks = Vec::new();
    cx.spawn(async move |cx| {
        let results = futures::future::join_all(wait_for_connection_tasks).await;
        for (agent_id, remote_connection, result) in results {
            let Some(state) = result.log_err() else {
                continue;
            };
            let Some(list) = cx.update(|cx| state.connection.session_list(cx)) else {
                continue;
            };
            let task = cx.update(|cx| {
                list.list_sessions(AgentSessionListRequest::default(), cx)
                    .map({
                        let remote_connection = remote_connection.clone();
                        move |response| (agent_id, remote_connection, response)
                    })
            });
            session_list_tasks.push(task);
        }

        let mut sessions_by_agent = Vec::new();
        let results = futures::future::join_all(session_list_tasks).await;
        for (agent_id, remote_connection, result) in results {
            let Some(response) = result.log_err() else {
                continue;
            };
            sessions_by_agent.push(SessionByAgent {
                agent_id,
                remote_connection,
                sessions: response.sessions,
            });
        }

        Ok(collect_importable_threads(
            sessions_by_agent,
            existing_sessions,
        ))
    })
}

struct SessionByAgent {
    agent_id: AgentId,
    remote_connection: Option<RemoteConnectionOptions>,
    sessions: Vec<acp_thread::AgentSessionInfo>,
}

fn collect_importable_threads(
    sessions_by_agent: Vec<SessionByAgent>,
    mut existing_sessions: HashSet<acp::SessionId>,
) -> Vec<ThreadMetadata> {
    let mut to_insert = Vec::new();
    for SessionByAgent {
        agent_id,
        remote_connection,
        sessions,
    } in sessions_by_agent
    {
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
                main_worktree_paths: PathList::default(),
                remote_connection: remote_connection.clone(),
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

        let sessions_by_agent = vec![SessionByAgent {
            agent_id: AgentId::new("agent-a"),
            remote_connection: None,
            sessions: vec![
                make_session(
                    "existing-1",
                    Some("Already There"),
                    Some(paths.clone()),
                    None,
                    None,
                ),
                make_session("new-1", Some("Brand New"), Some(paths), None, None),
            ],
        }];

        let result = collect_importable_threads(sessions_by_agent, existing);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].session_id.0.as_ref(), "new-1");
        assert_eq!(result[0].title.as_ref(), "Brand New");
    }

    #[test]
    fn test_collect_skips_sessions_without_work_dirs() {
        let existing = HashSet::default();
        let paths = PathList::new(&[Path::new("/project")]);

        let sessions_by_agent = vec![SessionByAgent {
            agent_id: AgentId::new("agent-a"),
            remote_connection: None,
            sessions: vec![
                make_session("has-dirs", Some("With Dirs"), Some(paths), None, None),
                make_session("no-dirs", Some("No Dirs"), None, None, None),
            ],
        }];

        let result = collect_importable_threads(sessions_by_agent, existing);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].session_id.0.as_ref(), "has-dirs");
    }

    #[test]
    fn test_collect_marks_all_imported_threads_as_archived() {
        let existing = HashSet::default();
        let paths = PathList::new(&[Path::new("/project")]);

        let sessions_by_agent = vec![SessionByAgent {
            agent_id: AgentId::new("agent-a"),
            remote_connection: None,
            sessions: vec![
                make_session("s1", Some("Thread 1"), Some(paths.clone()), None, None),
                make_session("s2", Some("Thread 2"), Some(paths), None, None),
            ],
        }];

        let result = collect_importable_threads(sessions_by_agent, existing);

        assert_eq!(result.len(), 2);
        assert!(result.iter().all(|t| t.archived));
    }

    #[test]
    fn test_collect_assigns_correct_agent_id_per_session() {
        let existing = HashSet::default();
        let paths = PathList::new(&[Path::new("/project")]);

        let sessions_by_agent = vec![
            SessionByAgent {
                agent_id: AgentId::new("agent-a"),
                remote_connection: None,
                sessions: vec![make_session(
                    "s1",
                    Some("From A"),
                    Some(paths.clone()),
                    None,
                    None,
                )],
            },
            SessionByAgent {
                agent_id: AgentId::new("agent-b"),
                remote_connection: None,
                sessions: vec![make_session("s2", Some("From B"), Some(paths), None, None)],
            },
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
            SessionByAgent {
                agent_id: AgentId::new("agent-a"),
                remote_connection: None,
                sessions: vec![make_session(
                    "shared-session",
                    Some("From A"),
                    Some(paths.clone()),
                    None,
                    None,
                )],
            },
            SessionByAgent {
                agent_id: AgentId::new("agent-b"),
                remote_connection: None,
                sessions: vec![make_session(
                    "shared-session",
                    Some("From B"),
                    Some(paths),
                    None,
                    None,
                )],
            },
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

        let sessions_by_agent = vec![SessionByAgent {
            agent_id: AgentId::new("agent-a"),
            remote_connection: None,
            sessions: vec![
                make_session("s1", Some("T1"), Some(paths.clone()), None, None),
                make_session("s2", Some("T2"), Some(paths), None, None),
            ],
        }];

        let result = collect_importable_threads(sessions_by_agent, existing);
        assert!(result.is_empty());
    }
}
