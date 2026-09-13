use crate::{
    Agent, AgentPanel, ConversationView,
    agent_panel::{NewThreadMenuParams, NewThreadMenuTarget, build_new_thread_menu},
    thread_metadata_store::{ThreadId, ThreadMetadataStore},
};
use agent_client_protocol::schema::v1 as acp;
use anyhow::{Context as _, Result};
use db::{
    query,
    sqlez::{domain::Domain, statement::Statement, thread_safe_connection::ThreadSafeConnection},
    sqlez_macros::sql,
};
use gpui::{
    Action, App, Context, Entity, EventEmitter, FocusHandle, Focusable, IntoElement, Render,
    SharedString, Subscription, Task, WeakEntity, Window,
};
use project::{AgentId, DisableAiSettings, Project};
use settings::{Settings as _, SettingsStore};
use theme_settings::ThemeSettings;
use ui::{ContextMenu, Tooltip, prelude::*, utils::WithRemSize};
use util::ResultExt as _;
use workspace::{
    Event as WorkspaceEvent, Item, ItemId, NewCenterTerminal, SerializableItem, Workspace,
    WorkspaceDb, WorkspaceId, delete_unloaded_items,
    item::{ItemEvent, TabTooltipContent},
};

pub struct ThreadItem {
    state: ThreadItemState,
    workspace: WeakEntity<Workspace>,
    workspace_id: Option<WorkspaceId>,
    focus_handle: FocusHandle,
    last_tab_title: SharedString,
    needs_serialize: bool,
    discarded: bool,
    suspended_prompt: Option<Vec<acp::ContentBlock>>,
    pending_prompt_persist: bool,
    _title_observations: Vec<Subscription>,
    _workspace_subscription: Option<Subscription>,
    _settings_subscription: Option<Subscription>,
}

enum ThreadItemState {
    Pending { agent: Agent, thread_id: ThreadId },
    Loaded(Entity<ConversationView>),
}

impl ThreadItem {
    pub(crate) fn new(
        conversation_view: Entity<ConversationView>,
        workspace: WeakEntity<Workspace>,
        workspace_id: Option<WorkspaceId>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut item = Self {
            state: ThreadItemState::Loaded(conversation_view),
            workspace,
            workspace_id,
            focus_handle: cx.focus_handle(),
            last_tab_title: SharedString::default(),
            needs_serialize: true,
            discarded: false,
            suspended_prompt: None,
            pending_prompt_persist: false,
            _title_observations: Vec::new(),
            _workspace_subscription: None,
            _settings_subscription: None,
        };
        item.observe_title(cx);
        item
    }

    pub fn pending(
        agent: Agent,
        thread_id: ThreadId,
        workspace: WeakEntity<Workspace>,
        workspace_id: Option<WorkspaceId>,
        cx: &mut Context<Self>,
    ) -> Self {
        let last_tab_title = ThreadMetadataStore::try_global(cx)
            .and_then(|store| {
                store
                    .read(cx)
                    .entry(thread_id)
                    .and_then(|metadata| metadata.title())
            })
            .unwrap_or_else(|| "New Thread".into());
        let mut item = Self {
            state: ThreadItemState::Pending { agent, thread_id },
            workspace,
            workspace_id,
            focus_handle: cx.focus_handle(),
            last_tab_title,
            needs_serialize: true,
            discarded: false,
            suspended_prompt: None,
            pending_prompt_persist: false,
            _title_observations: Vec::new(),
            _workspace_subscription: None,
            _settings_subscription: None,
        };
        item.observe_title(cx);
        item
    }

    pub fn thread_id(&self, cx: &App) -> ThreadId {
        match &self.state {
            ThreadItemState::Pending { thread_id, .. } => *thread_id,
            ThreadItemState::Loaded(view) => view.read(cx).thread_id,
        }
    }

    pub fn agent(&self, cx: &App) -> Agent {
        match &self.state {
            ThreadItemState::Pending { agent, .. } => agent.clone(),
            ThreadItemState::Loaded(view) => view.read(cx).agent_key().clone(),
        }
    }

    pub fn conversation_view(&self) -> Option<&Entity<ConversationView>> {
        match &self.state {
            ThreadItemState::Pending { .. } => None,
            ThreadItemState::Loaded(view) => Some(view),
        }
    }

    pub(crate) fn deploy(
        workspace: &mut Workspace,
        agent: Option<Agent>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        if DisableAiSettings::get_global(cx).disable_ai {
            return;
        }
        let Some(panel) = workspace.panel::<AgentPanel>(cx) else {
            return;
        };
        if !panel.read(cx).has_open_project(cx) {
            return;
        }
        let agent = agent.unwrap_or_else(|| panel.read(cx).selected_agent(cx));
        if workspace.project().read(cx).is_via_collab() && !agent.is_native() {
            return;
        }
        let view = panel.update(cx, |panel, cx| {
            panel.create_center_thread(agent, None, window, cx)
        });
        let item =
            cx.new(|cx| Self::new(view, workspace.weak_handle(), workspace.database_id(), cx));
        workspace.add_item_to_active_pane(Box::new(item), None, true, window, cx);
    }

    fn tab_title(&self, cx: &App) -> SharedString {
        ThreadMetadataStore::try_global(cx)
            .and_then(|store| {
                store
                    .read(cx)
                    .entry(self.thread_id(cx))
                    .and_then(|metadata| match &self.state {
                        ThreadItemState::Pending { .. } => metadata.title(),
                        ThreadItemState::Loaded(_) => metadata.title_override.clone(),
                    })
            })
            .unwrap_or_else(|| match &self.state {
                ThreadItemState::Loaded(view) => view.read(cx).title(cx),
                ThreadItemState::Pending { .. } => self.last_tab_title.clone(),
            })
    }

    fn observe_title(&mut self, cx: &mut Context<Self>) {
        self.last_tab_title = self.tab_title(cx);
        self._title_observations.clear();
        if let ThreadItemState::Loaded(view) = &self.state {
            self._title_observations
                .push(cx.observe(view, |this, _view, cx| {
                    this.refresh_tab_title(cx);
                    cx.notify();
                }));
        }
        if let Some(store) = ThreadMetadataStore::try_global(cx) {
            self._title_observations
                .push(cx.observe(&store, |this, _store, cx| {
                    this.refresh_tab_title(cx);
                }));
        }
    }

    fn refresh_tab_title(&mut self, cx: &mut Context<Self>) {
        let title = self.tab_title(cx);
        // Streaming output and unrelated metadata changes also notify, but
        // only title changes need to invalidate the tab bar.
        if title != self.last_tab_title {
            self.last_tab_title = title;
            cx.emit(ItemEvent::UpdateTab);
            cx.notify();
        }
    }

    fn attach_to_panel(
        &mut self,
        panel: &Entity<AgentPanel>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let thread_id = self.thread_id(cx);
        let item = cx.weak_entity();
        panel.update(cx, |panel, _cx| {
            panel.register_center_thread(thread_id, item)
        });
        self.resolve_pending(panel, window, cx);
    }

    fn resolve_pending(
        &mut self,
        panel: &Entity<AgentPanel>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if DisableAiSettings::get_global(cx).disable_ai
            || self.pending_prompt_persist
            || self.discarded
        {
            return;
        }
        let ThreadItemState::Pending { agent, thread_id } = &self.state else {
            return;
        };
        let agent = agent.clone();
        let thread_id = *thread_id;
        // Deserialization awaits the store reload, but deletion or archival can
        // still happen before the panel attaches. Restoring must not undo either.
        if !ThreadMetadataStore::try_global(cx).is_some_and(|store| {
            store
                .read(cx)
                .entry(thread_id)
                .is_some_and(|metadata| !metadata.archived)
        }) {
            panel.update(cx, |panel, _cx| panel.unregister_center_thread(thread_id));
            self.discard(window, cx);
            return;
        }
        let had_focus = self.focus_handle.contains_focused(window, cx);
        let view = panel.update(cx, |panel, cx| {
            panel.create_center_thread_with_initial_content(
                agent,
                Some(thread_id),
                self.suspended_prompt.clone().map(|blocks| {
                    crate::AgentInitialContent::ContentBlock {
                        blocks,
                        auto_submit: false,
                    }
                }),
                window,
                cx,
            )
        });
        self.state = ThreadItemState::Loaded(view.clone());
        self.observe_title(cx);
        self.needs_serialize = true;
        if had_focus {
            view.read(cx).activation_focus_handle(cx).focus(window, cx);
        }
        cx.emit(ItemEvent::UpdateTab);
        cx.notify();
    }

    fn settings_changed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if DisableAiSettings::get_global(cx).disable_ai {
            let Some(view) = self.conversation_view().cloned() else {
                cx.notify();
                return;
            };
            let thread_id = self.thread_id(cx);
            let agent = self.agent(cx);
            if view.read(cx).focus_handle(cx).contains_focused(window, cx) {
                self.focus_handle.focus(window, cx);
            }
            if let Some(snapshot) = view.update(cx, |view, cx| view.prepare_for_ai_disable(cx)) {
                self.suspended_prompt = Some(snapshot);
            }
            self.state = ThreadItemState::Pending { agent, thread_id };
            self.observe_title(cx);
            let is_draft = ThreadMetadataStore::try_global(cx).is_some_and(|store| {
                store
                    .read(cx)
                    .entry(thread_id)
                    .is_some_and(|metadata| metadata.is_draft())
            });
            if is_draft && let Some(snapshot) = &self.suspended_prompt {
                let persist = if snapshot.is_empty() {
                    crate::draft_prompt_store::delete(thread_id, cx)
                } else {
                    crate::draft_prompt_store::write(thread_id, snapshot, cx)
                };
                self.pending_prompt_persist = true;
                // The save must outlive a closed tab without retaining its live view.
                cx.spawn_in(window, async move |this, cx| {
                    persist.await.log_err();
                    if let Some(this) = this.upgrade() {
                        this.update_in(cx, |this, window, cx| {
                            this.pending_prompt_persist = false;
                            this.settings_changed(window, cx);
                        })
                        .log_err();
                    }
                })
                .detach();
            }
            cx.emit(ItemEvent::UpdateTab);
            cx.notify();
        } else if let Some(panel) = self
            .workspace
            .upgrade()
            .and_then(|workspace| workspace.read(cx).panel::<AgentPanel>(cx))
        {
            self.resolve_pending(&panel, window, cx);
        }
    }

    pub(crate) fn discard(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.discarded = true;
        self.close_tab(window, cx);
    }

    fn close_tab(&self, window: &mut Window, cx: &mut Context<Self>) {
        let item = cx.entity();
        let workspace = self.workspace.clone();
        // Callers can hold a workspace update. Remove the item after that
        // update ends.
        window.defer(cx, move |window, cx| {
            let Some(workspace) = workspace.upgrade() else {
                return;
            };
            workspace.update(cx, |workspace, cx| {
                if let Some(pane) = workspace.pane_for(&item) {
                    pane.update(cx, |pane, cx| {
                        pane.remove_item(item.entity_id(), false, true, window, cx);
                    });
                }
            });
        });
    }
}

impl EventEmitter<ItemEvent> for ThreadItem {}

impl Focusable for ThreadItem {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        if DisableAiSettings::get_global(cx).disable_ai {
            return self.focus_handle.clone();
        }
        match &self.state {
            // The conversation container does not forward focus to its editor.
            ThreadItemState::Loaded(view) => view.read(cx).activation_focus_handle(cx),
            ThreadItemState::Pending { .. } => self.focus_handle.clone(),
        }
    }
}

impl Render for ThreadItem {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match &self.state {
            ThreadItemState::Loaded(view) if !DisableAiSettings::get_global(cx).disable_ai => {
                let theme_settings = ThemeSettings::get_global(cx);
                WithRemSize::new(theme_settings.agent_ui_font_size(cx))
                    .size_full()
                    .font_family(theme_settings.agent_ui_font_family().clone())
                    .child(view.clone())
                    .into_any_element()
            }
            _ => v_flex()
                .size_full()
                .items_center()
                .justify_center()
                .track_focus(&self.focus_handle)
                .child(
                    Label::new(if DisableAiSettings::get_global(cx).disable_ai {
                        "AI is disabled"
                    } else {
                        "Loading…"
                    })
                    .color(Color::Muted),
                )
                .into_any_element(),
        }
    }
}

impl Item for ThreadItem {
    type Event = ItemEvent;

    fn to_item_events(event: &Self::Event, emit: &mut dyn FnMut(ItemEvent)) {
        emit(*event);
    }

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        self.tab_title(cx)
    }

    fn tab_icon(&self, _window: &Window, cx: &App) -> Option<Icon> {
        let thread_icons = self
            .conversation_view()
            .and_then(|view| view.read(cx).root_thread_view())
            .map(|view| {
                let view = view.read(cx);
                (view.agent_icon, view.agent_icon_from_external_svg.clone())
            });
        let icon = match thread_icons {
            Some((_, Some(path))) => Icon::from_external_svg(path),
            Some((icon, None)) => Icon::new(icon),
            None if self.agent(cx).is_native() => Icon::new(IconName::ZedAgent),
            None => Icon::new(IconName::Sparkle),
        };
        Some(icon.color(Color::Muted))
    }

    fn tab_tooltip_content(&self, cx: &App) -> Option<TabTooltipContent> {
        let title = self.tab_content_text(0, cx);
        let agent_name = match &self.state {
            ThreadItemState::Loaded(view) => view.read(cx).agent_display_name(cx),
            ThreadItemState::Pending { agent, .. } => agent.label(),
        };
        Some(TabTooltipContent::Custom(Box::new(Tooltip::element(
            move |_window, _cx| {
                v_flex()
                    .child(Label::new(title.clone()))
                    .child(
                        Label::new(agent_name.clone())
                            .color(Color::Muted)
                            .size(LabelSize::Small),
                    )
                    .into_any_element()
            },
        ))))
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Agent Thread Opened")
    }

    fn include_in_nav_history() -> bool {
        false
    }

    fn can_split(&self) -> bool {
        false
    }

    fn is_dirty(&self, _cx: &App) -> bool {
        false
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self._settings_subscription =
            Some(cx.observe_global_in::<SettingsStore>(window, Self::settings_changed));
        if DisableAiSettings::get_global(cx).disable_ai {
            self.settings_changed(window, cx);
        }
        // The workspace is being updated. Subscribing is safe, but reading
        // through its entity handle here would borrow it again.
        if let Some(workspace_entity) = self.workspace.upgrade() {
            self._workspace_subscription = Some(cx.subscribe_in(
                &workspace_entity,
                window,
                |this, _workspace, event: &WorkspaceEvent, window, cx| match event {
                    WorkspaceEvent::PanelAdded(view) => {
                        if let Ok(panel) = view.clone().downcast::<AgentPanel>() {
                            this.attach_to_panel(&panel, window, cx);
                        }
                    }
                    WorkspaceEvent::ActiveItemChanged | WorkspaceEvent::ZoomChanged => {
                        if let Some(view) = this.conversation_view() {
                            view.update(cx, |view, cx| {
                                view.dismiss_notifications_if_visible(window, cx);
                            });
                        }
                    }
                    _ => {}
                },
            ));
        }
        if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
            self.attach_to_panel(&panel, window, cx);
        }
    }

    fn on_removed(&self, cx: &mut Context<Self>) {
        if self.discarded {
            return;
        }
        let view = self.conversation_view().cloned();
        let thread_id = self.thread_id(cx);
        let item_id = cx.entity_id();
        let workspace = self.workspace.clone();
        // Moves remove the item before inserting it into the destination pane.
        // App::defer also survives release of the item when the tab is closed.
        App::defer(cx, move |cx| {
            let Some(workspace) = workspace.upgrade() else {
                return;
            };
            if workspace
                .read(cx)
                .items_of_type::<ThreadItem>(cx)
                .any(|item| item.entity_id() == item_id)
            {
                return;
            }
            if let Some(panel) = workspace.read(cx).panel::<AgentPanel>(cx) {
                panel.update(cx, |panel, cx| {
                    if let Some(view) = view {
                        panel.retain_closed_center_thread(view, cx);
                    } else {
                        panel.unregister_center_thread(thread_id);
                    }
                });
            }
        });
    }
}

impl SerializableItem for ThreadItem {
    fn serialized_item_kind() -> &'static str {
        "AgentThread"
    }

    fn cleanup(
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let database = ThreadItemDb::global(cx);
        delete_unloaded_items(
            alive_items,
            workspace_id,
            "agent_thread_items",
            &database,
            cx,
        )
    }

    fn serialize(
        &mut self,
        _workspace: &mut Workspace,
        item_id: ItemId,
        _closing: bool,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        if !self.needs_serialize {
            return None;
        }
        let workspace_id = self.workspace_id?;
        // Entity ids change across restarts. Even pending restored tabs must
        // save their new id before cleanup deletes their previous row.
        let thread_id = self.thread_id(cx);
        let agent_id = self.agent(cx).id().0.to_string();
        self.needs_serialize = false;
        let database = ThreadItemDb::global(cx);
        Some(cx.background_spawn(async move {
            database
                .save_thread_item(item_id, workspace_id, thread_id, agent_id)
                .await
        }))
    }

    fn should_serialize(&self, _event: &Self::Event) -> bool {
        self.needs_serialize
    }

    fn deserialize(
        _project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        workspace_id: WorkspaceId,
        item_id: ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let row = ThreadItemDb::global(cx).get_thread_item(item_id, workspace_id);
        let metadata_store = ThreadMetadataStore::try_global(cx);
        window.spawn(cx, async move |cx| {
            let (thread_id, agent_id) = row?.context("no agent thread item row")?;
            let metadata_store =
                metadata_store.context("the thread metadata store is not initialized")?;
            let reload = cx.update(|_window, cx| metadata_store.read(cx).reload_task())?;
            reload.await;
            let exists =
                cx.update(|_window, cx| metadata_store.read(cx).entry(thread_id).is_some())?;
            anyhow::ensure!(exists, "thread {thread_id:?} no longer exists");
            let agent = Agent::from(AgentId::new(agent_id));
            cx.update(|_window, cx| {
                cx.new(|cx| Self::pending(agent, thread_id, workspace, Some(workspace_id), cx))
            })
        })
    }
}

pub(crate) struct ThreadItemDb(ThreadSafeConnection);

impl Domain for ThreadItemDb {
    const NAME: &'static str = stringify!(ThreadItemDb);
    const MIGRATIONS: &'static [&'static str] = &[sql!(
        CREATE TABLE agent_thread_items (
            workspace_id INTEGER,
            item_id INTEGER,
            thread_id BLOB NOT NULL,
            agent_id TEXT NOT NULL,
            PRIMARY KEY(workspace_id, item_id),
            FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
            ON DELETE CASCADE
        ) STRICT;
    )];
}

db::static_connection!(ThreadItemDb, [WorkspaceDb]);

impl ThreadItemDb {
    pub async fn save_thread_item(
        &self,
        item_id: ItemId,
        workspace_id: WorkspaceId,
        thread_id: ThreadId,
        agent_id: String,
    ) -> Result<()> {
        self.write(move |connection| {
            let mut statement = Statement::prepare(
                connection,
                "INSERT INTO agent_thread_items (item_id, workspace_id, thread_id, agent_id)
                VALUES (?1, ?2, ?3, ?4)
                ON CONFLICT (workspace_id, item_id) DO UPDATE SET
                    thread_id = excluded.thread_id,
                    agent_id = excluded.agent_id",
            )?;
            let mut next_index = statement.bind(&item_id, 1)?;
            next_index = statement.bind(&workspace_id, next_index)?;
            next_index = statement.bind(&thread_id, next_index)?;
            statement.bind(&agent_id, next_index)?;
            statement.exec()
        })
        .await
    }

    query! {
        pub fn get_thread_item(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<(ThreadId, String)>> {
            SELECT thread_id, agent_id
            FROM agent_thread_items
            WHERE item_id = ? AND workspace_id = ?
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum CenterMenuKind {
    Terminal,
    ThreadSubmenu,
}

pub(crate) fn center_menu_kind(workspace: &WeakEntity<Workspace>, cx: &App) -> CenterMenuKind {
    if !DisableAiSettings::get_global(cx).disable_ai
        && workspace
            .upgrade()
            .is_some_and(|workspace| workspace.read(cx).panel::<AgentPanel>(cx).is_some())
    {
        CenterMenuKind::ThreadSubmenu
    } else {
        CenterMenuKind::Terminal
    }
}

pub(crate) fn center_entries(
    menu: ContextMenu,
    workspace: &WeakEntity<Workspace>,
    _window: &mut Window,
    cx: &mut App,
) -> ContextMenu {
    match center_menu_kind(workspace, cx) {
        CenterMenuKind::Terminal => menu.action(
            "New Center Terminal",
            NewCenterTerminal::default().boxed_clone(),
        ),
        CenterMenuKind::ThreadSubmenu => {
            let workspace = workspace.clone();
            menu.submenu("New Center Thread", move |menu, _window, cx| {
                let Some(params) = center_menu_params(&workspace, cx) else {
                    return menu;
                };
                build_new_thread_menu(menu, &params, cx)
            })
        }
    }
}

fn center_menu_params(workspace: &WeakEntity<Workspace>, cx: &App) -> Option<NewThreadMenuParams> {
    let workspace_entity = workspace.upgrade()?;
    let workspace_ref = workspace_entity.read(cx);
    let panel = workspace_ref.panel::<AgentPanel>(cx)?;
    let panel = panel.read(cx);
    let project = workspace_ref.project().read(cx);
    Some(NewThreadMenuParams {
        target: NewThreadMenuTarget::Center,
        selected_agent: panel.selected_agent(cx),
        showing_terminal: false,
        supports_terminal: project.supports_terminal(cx),
        is_via_collab: project.is_via_collab(),
        agent_server_store: project.agent_server_store().clone(),
        focus_handle: workspace_ref.focus_handle(cx),
        workspace: workspace.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AgentThreadSource, NewCenterThread, ThreadOpened, conversation_view::tests::init_test,
    };
    use acp_thread::StubAgentConnection;
    use agent_client_protocol::schema::v1 as acp;
    use fs::{FakeFs, Fs};
    use gpui::{TestAppContext, VisualTestContext};
    use std::{cell::Cell, path::Path, rc::Rc};
    use workspace::{CloseActiveItem, MultiWorkspace, SplitDirection};

    async fn setup_workspace(
        cx: &mut TestAppContext,
    ) -> (Entity<Workspace>, &mut VisualTestContext) {
        init_test(cx);
        let filesystem = FakeFs::new(cx.executor());
        filesystem
            .insert_tree("/project", serde_json::json!({ "src": {} }))
            .await;
        cx.update(|cx| {
            <dyn Fs>::set_global(filesystem.clone(), cx);
            cx.set_global(acp_thread::StubSessionCounter(
                std::sync::atomic::AtomicUsize::new(0),
            ));
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });
        let project = Project::test(filesystem, [Path::new("/project")], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = multi_workspace.read_with(cx, |multi_workspace, _cx| {
            multi_workspace.workspace().clone()
        });
        cx.run_until_parked();
        (workspace, cx)
    }

    fn add_panel(workspace: &Entity<Workspace>, cx: &mut VisualTestContext) -> Entity<AgentPanel> {
        workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        })
    }

    async fn setup(
        cx: &mut TestAppContext,
    ) -> (
        Entity<Workspace>,
        Entity<AgentPanel>,
        &mut VisualTestContext,
    ) {
        let (workspace, cx) = setup_workspace(cx).await;
        let panel = add_panel(&workspace, cx);
        cx.run_until_parked();
        (workspace, panel, cx)
    }

    fn active_thread_item(
        workspace: &Entity<Workspace>,
        cx: &VisualTestContext,
    ) -> Entity<ThreadItem> {
        workspace.read_with(cx, |workspace, cx| {
            workspace
                .active_item(cx)
                .and_then(|item| item.downcast::<ThreadItem>())
                .expect("active center thread")
        })
    }

    fn deploy_stub_thread(
        workspace: &Entity<Workspace>,
        cx: &mut VisualTestContext,
    ) -> Entity<ThreadItem> {
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.focus_handle(cx).focus(window, cx);
        });
        cx.dispatch_action(NewCenterThread {
            agent: Some(Agent::Stub.id()),
        });
        cx.run_until_parked();
        active_thread_item(workspace, cx)
    }

    fn conversation_view(
        item: &Entity<ThreadItem>,
        cx: &VisualTestContext,
    ) -> Entity<ConversationView> {
        item.read_with(cx, |item, _cx| {
            item.conversation_view()
                .expect("loaded center thread")
                .clone()
        })
    }

    fn set_message(item: &Entity<ThreadItem>, text: &str, cx: &mut VisualTestContext) {
        let view = conversation_view(item, cx);
        let thread_view = view.read_with(cx, |view, _cx| {
            view.root_thread_view().expect("connected thread")
        });
        let editor = thread_view.read_with(cx, |view, _cx| view.message_editor.clone());
        editor.update_in(cx, |editor, window, cx| editor.set_text(text, window, cx));
        cx.run_until_parked();
        cx.executor()
            .advance_clock(crate::conversation_view::DRAFT_PROMPT_PERSIST_DEBOUNCE * 2);
        cx.run_until_parked();
    }

    fn stub_connection(
        view: &Entity<ConversationView>,
        cx: &VisualTestContext,
    ) -> Rc<StubAgentConnection> {
        view.read_with(cx, |view, cx| {
            view.root_thread(cx)
                .expect("connected thread")
                .read(cx)
                .connection()
                .clone()
                .into_any()
                .downcast::<StubAgentConnection>()
                .expect("stub connection")
        })
    }

    fn send_message(item: &Entity<ThreadItem>, cx: &mut VisualTestContext) {
        set_message(item, "Hello", cx);
        let view = conversation_view(item, cx);
        let thread_view = view.read_with(cx, |view, _cx| {
            view.root_thread_view().expect("connected thread")
        });
        thread_view.update_in(cx, |view, window, cx| view.send(window, cx));
        cx.run_until_parked();
    }

    fn message_editor_is_focused(item: &Entity<ThreadItem>, cx: &mut VisualTestContext) -> bool {
        let view = conversation_view(item, cx);
        cx.update(|window, cx| {
            view.read(cx)
                .root_thread_view()
                .expect("connected thread")
                .read(cx)
                .message_editor
                .read(cx)
                .focus_handle(cx)
                .is_focused(window)
        })
    }

    fn metadata_exists(thread_id: ThreadId, cx: &mut VisualTestContext) -> bool {
        cx.update(|_window, cx| {
            ThreadMetadataStore::global(cx)
                .read(cx)
                .entry(thread_id)
                .is_some()
        })
    }

    fn views_for(thread_id: ThreadId, panel: &Entity<AgentPanel>, cx: &VisualTestContext) -> usize {
        panel.read_with(cx, |panel, cx| {
            panel
                .conversation_views(cx)
                .iter()
                .filter(|view| view.read(cx).thread_id == thread_id)
                .count()
        })
    }

    async fn close_active_item(workspace: &Entity<Workspace>, cx: &mut VisualTestContext) {
        let pane = workspace.read_with(cx, |workspace, _cx| workspace.active_pane().clone());
        pane.update_in(cx, |pane, window, cx| {
            pane.close_active_item(&CloseActiveItem::default(), window, cx)
        })
        .await
        .expect("close center tab");
        cx.run_until_parked();
    }

    fn add_pending_item(
        workspace: &Entity<Workspace>,
        thread_id: ThreadId,
        cx: &mut VisualTestContext,
    ) -> Entity<ThreadItem> {
        let item = cx.update(|_window, cx| {
            cx.new(|cx| {
                ThreadItem::pending(Agent::Stub, thread_id, workspace.downgrade(), None, cx)
            })
        });
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(Box::new(item.clone()), None, true, window, cx);
        });
        cx.run_until_parked();
        item
    }

    fn open_panel_thread(panel: &Entity<AgentPanel>, cx: &mut VisualTestContext) -> ThreadId {
        panel.update_in(cx, |panel, window, cx| {
            panel.test_set_selected_agent(Agent::Stub);
            panel.activate_draft(true, AgentThreadSource::AgentPanel, window, cx);
        });
        cx.run_until_parked();
        let view = panel.read_with(cx, |panel, _cx| {
            panel
                .active_conversation_view()
                .expect("active thread")
                .clone()
        });
        stub_connection(&view, cx).set_next_prompt_updates(vec![
            acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new("Done".into())),
        ]);
        crate::test_support::send_message(panel, cx);
        crate::test_support::active_thread_id(panel, cx)
    }

    #[gpui::test]
    async fn test_new_center_thread_opens_registered_item_and_focuses_editor(
        cx: &mut TestAppContext,
    ) {
        let (workspace, panel, cx) = setup(cx).await;
        let item = deploy_stub_thread(&workspace, cx);
        let thread_id = item.read_with(cx, |item, cx| {
            assert_eq!(item.agent(cx), Agent::Stub);
            item.thread_id(cx)
        });
        panel.read_with(cx, |panel, _cx| {
            assert!(panel.is_center_thread(thread_id));
            assert!(panel.active_conversation_view().is_none());
        });
        assert!(message_editor_is_focused(&item, cx));
        assert_eq!(views_for(thread_id, &panel, cx), 1);
        panel.read_with(cx, |panel, cx| {
            assert!(panel.conversation_view_for_id(&thread_id, cx).is_some())
        });
        assert!(panel.update(cx, |panel, cx| panel.cancel_thread(&thread_id, cx)));
    }

    #[gpui::test]
    async fn test_new_center_thread_defaults_to_selected_agent(cx: &mut TestAppContext) {
        let (workspace, panel, cx) = setup(cx).await;
        panel.update(cx, |panel, _cx| panel.test_set_selected_agent(Agent::Stub));
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.focus_handle(cx).focus(window, cx)
        });
        cx.dispatch_action(NewCenterThread::default());
        cx.run_until_parked();
        active_thread_item(&workspace, cx)
            .read_with(cx, |item, cx| assert_eq!(item.agent(cx), Agent::Stub));
    }

    #[gpui::test]
    async fn test_disable_ai_releases_center_view_and_restores_latest_draft(
        cx: &mut TestAppContext,
    ) {
        let (workspace, panel, cx) = setup(cx).await;
        let item = deploy_stub_thread(&workspace, cx);
        let view = conversation_view(&item, cx);
        let old_view = view.downgrade();
        let thread_id = item.read_with(cx, |item, cx| item.thread_id(cx));
        let editor = view.read_with(cx, |view, cx| {
            view.root_thread_view()
                .expect("connected thread")
                .read(cx)
                .message_editor
                .clone()
        });
        editor.update_in(cx, |editor, window, cx| {
            editor.set_text("The latest unsaved draft", window, cx);
            project::DisableAiSettings::override_global(
                project::DisableAiSettings { disable_ai: true },
                cx,
            );
        });
        drop(editor);
        drop(view);
        cx.run_until_parked();
        item.read_with(cx, |item, cx| {
            assert!(item.conversation_view().is_none());
            assert_eq!(item.thread_id(cx), thread_id);
        });
        assert!(old_view.upgrade().is_none());
        cx.update(|window, cx| {
            assert!(item.read(cx).focus_handle(cx).is_focused(window));
            assert_eq!(
                center_menu_kind(&workspace.downgrade(), cx),
                CenterMenuKind::Terminal
            );
        });
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.remove_panel(&panel, window, cx);
        });
        drop(panel);
        cx.update(|_window, cx| {
            project::DisableAiSettings::override_global(
                project::DisableAiSettings { disable_ai: false },
                cx,
            );
        });
        let panel = add_panel(&workspace, cx);
        cx.run_until_parked();
        assert_eq!(views_for(thread_id, &panel, cx), 1);
        assert!(message_editor_is_focused(&item, cx));
        let view = conversation_view(&item, cx);
        view.read_with(cx, |view, cx| {
            assert_eq!(
                view.root_thread_view()
                    .expect("restored thread")
                    .read(cx)
                    .message_editor
                    .read(cx)
                    .text(cx),
                "The latest unsaved draft"
            );
        });
    }

    #[gpui::test]
    async fn test_disable_ai_cancels_running_center_thread(cx: &mut TestAppContext) {
        let (workspace, _panel, cx) = setup(cx).await;
        let item = deploy_stub_thread(&workspace, cx);
        let view = conversation_view(&item, cx);
        stub_connection(&view, cx).set_next_prompt_updates(Vec::new());
        send_message(&item, cx);
        let thread = view.read_with(cx, |view, cx| view.root_thread(cx).expect("connected"));
        assert_eq!(
            thread.read_with(cx, |thread, _cx| thread.status()),
            acp_thread::ThreadStatus::Generating
        );
        drop(view);
        cx.update(|_window, cx| {
            project::DisableAiSettings::override_global(
                project::DisableAiSettings { disable_ai: true },
                cx,
            );
        });
        cx.run_until_parked();
        assert!(item.read_with(cx, |item, _cx| item.conversation_view().is_none()));
        assert_eq!(
            thread.read_with(cx, |thread, _cx| thread.status()),
            acp_thread::ThreadStatus::Idle
        );
    }

    #[gpui::test]
    async fn test_disable_ai_prevents_pending_restore_and_new_center_threads(
        cx: &mut TestAppContext,
    ) {
        let (workspace, _panel, cx) = setup(cx).await;
        let item = deploy_stub_thread(&workspace, cx);
        let thread_id = item.read_with(cx, |item, cx| item.thread_id(cx));
        set_message(&item, "Saved draft", cx);
        close_active_item(&workspace, cx).await;
        drop(item);
        cx.update(|_window, cx| {
            project::DisableAiSettings::override_global(
                project::DisableAiSettings { disable_ai: true },
                cx,
            );
        });
        let item = add_pending_item(&workspace, thread_id, cx);
        assert!(item.read_with(cx, |item, _cx| item.conversation_view().is_none()));
        workspace.update_in(cx, |workspace, window, cx| {
            ThreadItem::deploy(workspace, Some(Agent::Stub), window, cx);
        });
        cx.run_until_parked();
        workspace.read_with(cx, |workspace, cx| {
            assert_eq!(workspace.items_of_type::<ThreadItem>(cx).count(), 1);
        });
        cx.update(|_window, cx| {
            project::DisableAiSettings::override_global(
                project::DisableAiSettings { disable_ai: false },
                cx,
            );
        });
        cx.run_until_parked();
        assert!(item.read_with(cx, |item, _cx| item.conversation_view().is_some()));
    }

    fn edit_and_disable_ai(
        item: &Entity<ThreadItem>,
        text: &str,
        cx: &mut VisualTestContext,
    ) -> WeakEntity<ConversationView> {
        let view = conversation_view(item, cx);
        let old_view = view.downgrade();
        let editor = view.read_with(cx, |view, cx| {
            view.root_thread_view()
                .expect("connected thread")
                .read(cx)
                .message_editor
                .clone()
        });
        editor.update_in(cx, |editor, window, cx| {
            editor.set_text(text, window, cx);
            DisableAiSettings::override_global(DisableAiSettings { disable_ai: true }, cx);
        });
        old_view
    }

    fn assert_center_prompt(item: &Entity<ThreadItem>, expected: &str, cx: &VisualTestContext) {
        conversation_view(item, cx).read_with(cx, |view, cx| {
            assert_eq!(
                view.root_thread_view()
                    .expect("connected thread")
                    .read(cx)
                    .message_editor
                    .read(cx)
                    .text(cx),
                expected
            );
        });
    }

    #[gpui::test]
    async fn test_rapid_ai_reenable_waits_for_draft_save_with_replacement_panel(
        cx: &mut TestAppContext,
    ) {
        let (workspace, panel, cx) = setup(cx).await;
        let item = deploy_stub_thread(&workspace, cx);
        let thread_id = item.read_with(cx, |item, cx| item.thread_id(cx));
        let thread_view = conversation_view(&item, cx).read_with(cx, |view, _cx| {
            view.root_thread_view().expect("connected thread")
        });
        // Sending sets this before async content resolution clears the editor.
        thread_view.update(cx, |view, _cx| view.is_loading_contents = true);
        drop(thread_view);
        let old_view = edit_and_disable_ai(&item, "Draft before rapid re-enable", cx);
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.remove_panel(&panel, window, cx);
        });
        drop(panel);
        cx.update(|_window, cx| {
            DisableAiSettings::override_global(DisableAiSettings { disable_ai: false }, cx);
        });
        let panel = add_panel(&workspace, cx);
        item.read_with(cx, |item, _cx| {
            assert!(item.pending_prompt_persist);
            assert!(item.conversation_view().is_none());
        });
        cx.run_until_parked();
        assert!(old_view.upgrade().is_none());
        assert_eq!(views_for(thread_id, &panel, cx), 1);
        assert_center_prompt(&item, "Draft before rapid re-enable", cx);
        assert!(message_editor_is_focused(&item, cx));
    }

    #[gpui::test]
    async fn test_rapid_ai_toggles_keep_center_thread_disabled_after_save(cx: &mut TestAppContext) {
        let (workspace, panel, cx) = setup(cx).await;
        let item = deploy_stub_thread(&workspace, cx);
        let thread_id = item.read_with(cx, |item, cx| item.thread_id(cx));
        let old_view = edit_and_disable_ai(&item, "Draft while AI stays disabled", cx);
        cx.update(|_window, cx| {
            DisableAiSettings::override_global(DisableAiSettings { disable_ai: false }, cx);
        });
        cx.update(|_window, cx| {
            DisableAiSettings::override_global(DisableAiSettings { disable_ai: true }, cx);
        });
        cx.run_until_parked();
        assert!(old_view.upgrade().is_none());
        assert_eq!(views_for(thread_id, &panel, cx), 0);
        item.read_with(cx, |item, _cx| {
            assert!(!item.pending_prompt_persist);
            assert!(item.conversation_view().is_none());
        });
        panel.read_with(cx, |panel, _cx| assert!(panel.is_center_thread(thread_id)));
        cx.update(|_window, cx| {
            DisableAiSettings::override_global(DisableAiSettings { disable_ai: false }, cx);
        });
        cx.run_until_parked();
        assert_eq!(views_for(thread_id, &panel, cx), 1);
        assert_center_prompt(&item, "Draft while AI stays disabled", cx);
    }

    #[gpui::test]
    async fn test_rapid_ai_reenable_preserves_native_unsent_prompt(cx: &mut TestAppContext) {
        let (workspace, panel, cx) = setup(cx).await;
        workspace.update_in(cx, |workspace, window, cx| {
            ThreadItem::deploy(workspace, Some(Agent::NativeAgent), window, cx);
        });
        cx.run_until_parked();
        let item = active_thread_item(&workspace, cx);
        let thread_id = item.read_with(cx, |item, cx| item.thread_id(cx));
        let old_view = edit_and_disable_ai(&item, "Native draft before rapid re-enable", cx);
        cx.update(|_window, cx| {
            DisableAiSettings::override_global(DisableAiSettings { disable_ai: false }, cx);
        });
        cx.run_until_parked();
        assert!(old_view.upgrade().is_none());
        assert_eq!(views_for(thread_id, &panel, cx), 1);
        assert_center_prompt(&item, "Native draft before rapid re-enable", cx);
        assert!(message_editor_is_focused(&item, cx));

        send_message(&item, cx);
        assert!(panel.update(cx, |panel, cx| panel.cancel_thread(&thread_id, cx)));
        cx.run_until_parked();
        cx.update(|_window, cx| {
            assert!(
                !ThreadMetadataStore::global(cx)
                    .read(cx)
                    .entry(thread_id)
                    .expect("native thread metadata")
                    .is_draft()
            );
        });
        let old_view = edit_and_disable_ai(&item, "Unsent next prompt in native thread", cx);
        cx.update(|_window, cx| {
            DisableAiSettings::override_global(DisableAiSettings { disable_ai: false }, cx);
        });
        cx.run_until_parked();
        assert!(old_view.upgrade().is_none());
        assert_eq!(views_for(thread_id, &panel, cx), 1);
        assert_center_prompt(&item, "Unsent next prompt in native thread", cx);
        assert!(message_editor_is_focused(&item, cx));
    }

    #[test]
    fn test_new_center_thread_accepts_legacy_agent_names() {
        for value in [serde_json::json!({}), serde_json::json!({"agent": null})] {
            let action: NewCenterThread = serde_json::from_value(value).expect("optional agent");
            assert!(action.agent.is_none());
        }
        for value in [
            serde_json::json!({"agent": "NativeAgent"}),
            serde_json::json!({"agent": "native_agent"}),
            serde_json::json!({"agent": "TextThread"}),
        ] {
            let action: NewCenterThread =
                serde_json::from_value(value).expect("legacy native agent");
            assert_eq!(action.agent, Some(Agent::NativeAgent.id()));
        }
        for value in [
            serde_json::json!({"agent": "claude"}),
            serde_json::json!({"agent": {"Custom": {"name": "claude"}}}),
        ] {
            let action: NewCenterThread = serde_json::from_value(value).expect("external agent");
            assert_eq!(action.agent, Some(AgentId::new("claude")));
        }
        assert!(
            serde_json::from_value::<NewCenterThread>(serde_json::json!({"unknown": true}))
                .is_err()
        );
    }

    #[gpui::test]
    async fn test_tab_title_only_updates_when_title_changes(cx: &mut TestAppContext) {
        let (workspace, _panel, cx) = setup(cx).await;
        let item = deploy_stub_thread(&workspace, cx);
        let updates = Rc::new(Cell::new(0usize));
        let _subscription = cx.update(|_window, cx| {
            let updates = updates.clone();
            cx.subscribe(&item, move |_item, event: &ItemEvent, _cx| {
                if matches!(event, ItemEvent::UpdateTab) {
                    updates.set(updates.get() + 1);
                }
            })
        });
        let view = conversation_view(&item, cx);
        let thread_view = view.read_with(cx, |view, _cx| {
            view.root_thread_view().expect("connected thread")
        });
        thread_view.update_in(cx, |view, window, cx| {
            view.rename("Renamed thread".into(), window, cx)
        });
        cx.run_until_parked();
        assert_eq!(
            item.read_with(cx, |item, cx| item.tab_content_text(0, cx)),
            "Renamed thread"
        );
        assert_eq!(updates.get(), 1);
        view.update(cx, |_view, cx| cx.notify());
        cx.run_until_parked();
        assert_eq!(updates.get(), 1);
        let thread_id = item.read_with(cx, |item, cx| item.thread_id(cx));
        cx.update(|_window, cx| {
            ThreadMetadataStore::global(cx).update(cx, |store, cx| {
                store.set_title_override(thread_id, "Metadata title".into(), cx);
            });
        });
        cx.run_until_parked();
        assert_eq!(
            item.read_with(cx, |item, cx| item.tab_content_text(0, cx)),
            "Metadata title"
        );
        assert_eq!(updates.get(), 2);
        cx.update(|_window, cx| {
            ThreadMetadataStore::global(cx).update(cx, |_store, cx| cx.notify());
        });
        cx.run_until_parked();
        assert_eq!(updates.get(), 2);
        cx.update(|_window, cx| {
            ThreadMetadataStore::global(cx).update(cx, |store, cx| {
                store.set_generated_title(thread_id, "Renamed thread".into(), cx);
            });
        });
        cx.run_until_parked();
        assert_eq!(
            item.read_with(cx, |item, cx| item.tab_content_text(0, cx)),
            "Renamed thread"
        );
        assert_eq!(updates.get(), 3);
    }

    #[gpui::test]
    async fn test_loading_a_center_thread_activates_its_existing_tab(cx: &mut TestAppContext) {
        let (workspace, panel, cx) = setup(cx).await;
        let first = deploy_stub_thread(&workspace, cx);
        let thread_id = first.read_with(cx, |item, cx| item.thread_id(cx));
        let original_view = conversation_view(&first, cx);
        let _second = deploy_stub_thread(&workspace, cx);
        workspace.update_in(cx, |_workspace, window, cx| {
            let opened = panel.update(cx, |panel, cx| {
                panel.load_agent_thread(
                    Agent::Stub,
                    thread_id,
                    None,
                    None,
                    true,
                    AgentThreadSource::Sidebar,
                    window,
                    cx,
                )
            });
            assert_eq!(opened, ThreadOpened::CenterPane);
        });
        cx.run_until_parked();
        assert_eq!(active_thread_item(&workspace, cx), first);
        assert_eq!(conversation_view(&first, cx), original_view);
        assert_eq!(views_for(thread_id, &panel, cx), 1);
        assert!(message_editor_is_focused(&first, cx));
    }

    #[gpui::test]
    async fn test_pending_tab_adopts_panel_active_thread(cx: &mut TestAppContext) {
        let (workspace, panel, cx) = setup(cx).await;
        let thread_id = open_panel_thread(&panel, cx);
        let original = panel.read_with(cx, |panel, _cx| {
            panel
                .active_conversation_view()
                .expect("active thread")
                .clone()
        });
        let item = add_pending_item(&workspace, thread_id, cx);
        assert_eq!(conversation_view(&item, cx), original);
        assert_eq!(views_for(thread_id, &panel, cx), 1);
        panel.read_with(cx, |panel, cx| {
            assert!(!panel.retained_threads().contains_key(&thread_id));
            assert!(panel.active_view_is_new_draft(cx));
        });
    }

    #[gpui::test]
    async fn test_pending_tab_adopts_active_draft_from_both_panel_slots(cx: &mut TestAppContext) {
        let (workspace, panel, cx) = setup(cx).await;
        panel.update_in(cx, |panel, window, cx| {
            panel.test_set_selected_agent(Agent::Stub);
            panel.activate_draft(true, AgentThreadSource::AgentPanel, window, cx);
        });
        cx.run_until_parked();
        let thread_id = crate::test_support::active_thread_id(&panel, cx);
        let original = panel.read_with(cx, |panel, _cx| {
            panel.test_draft_thread().expect("draft slot").clone()
        });
        let item = add_pending_item(&workspace, thread_id, cx);
        assert_eq!(conversation_view(&item, cx), original);
        assert_eq!(views_for(thread_id, &panel, cx), 1);
        panel.read_with(cx, |panel, cx| {
            assert!(
                panel
                    .test_draft_thread()
                    .is_some_and(|draft| draft != &original)
            );
            assert!(panel.active_view_is_new_draft(cx));
            assert!(!panel.retained_threads().contains_key(&thread_id));
        });
    }

    #[gpui::test]
    async fn test_pending_tab_adopts_retained_thread_at_cache_limit(cx: &mut TestAppContext) {
        let (workspace, panel, cx) = setup(cx).await;
        cx.update(|_window, cx| cx.set_global(crate::MaxIdleRetainedThreads(1)));
        let thread_id = open_panel_thread(&panel, cx);
        panel.update_in(cx, |panel, window, cx| {
            panel.new_thread(&crate::NewThread, window, cx)
        });
        cx.run_until_parked();
        let original = panel.read_with(cx, |panel, _cx| {
            panel
                .retained_threads()
                .get(&thread_id)
                .expect("retained thread")
                .clone()
        });
        let replacement = panel.read_with(cx, |panel, _cx| {
            panel
                .active_conversation_view()
                .expect("active draft")
                .clone()
        });
        let item = add_pending_item(&workspace, thread_id, cx);
        assert_eq!(conversation_view(&item, cx), original);
        assert_eq!(views_for(thread_id, &panel, cx), 1);
        panel.read_with(cx, |panel, _cx| {
            assert!(!panel.retained_threads().contains_key(&thread_id));
            assert_eq!(panel.active_conversation_view(), Some(&replacement));
        });
    }

    #[gpui::test]
    async fn test_missing_pending_thread_closes_without_retention(cx: &mut TestAppContext) {
        let (workspace, panel, cx) = setup(cx).await;
        let thread_id = ThreadId::new();
        let item = add_pending_item(&workspace, thread_id, cx);
        workspace.read_with(cx, |workspace, _cx| {
            assert!(workspace.pane_for(&item).is_none())
        });
        panel.read_with(cx, |panel, _cx| {
            assert!(!panel.is_center_thread(thread_id));
            assert!(!panel.retained_threads().contains_key(&thread_id));
        });
    }

    #[gpui::test]
    async fn test_moving_a_center_tab_keeps_its_registry_and_metadata(cx: &mut TestAppContext) {
        let (workspace, panel, cx) = setup(cx).await;
        let item = deploy_stub_thread(&workspace, cx);
        let thread_id = item.read_with(cx, |item, cx| item.thread_id(cx));
        let original = conversation_view(&item, cx);
        workspace.update_in(cx, |workspace, window, cx| {
            let source = workspace.active_pane().clone();
            let destination =
                workspace.split_pane(source.clone(), SplitDirection::Right, window, cx);
            workspace::move_item(&source, &destination, item.entity_id(), 0, true, window, cx);
        });
        cx.run_until_parked();
        assert_eq!(active_thread_item(&workspace, cx), item);
        assert_eq!(conversation_view(&item, cx), original);
        assert!(metadata_exists(thread_id, cx));
        panel.read_with(cx, |panel, _cx| {
            assert!(panel.is_center_thread(thread_id));
            assert!(!panel.retained_threads().contains_key(&thread_id));
        });
    }

    #[gpui::test]
    async fn test_cancel_thread_reaches_a_center_thread(cx: &mut TestAppContext) {
        let (workspace, panel, cx) = setup(cx).await;
        let item = deploy_stub_thread(&workspace, cx);
        let thread_id = item.read_with(cx, |item, cx| item.thread_id(cx));
        let view = conversation_view(&item, cx);
        let thread = view.read_with(cx, |view, cx| {
            view.root_thread(cx).expect("connected thread")
        });
        stub_connection(&view, cx).set_next_prompt_updates(Vec::new());
        send_message(&item, cx);
        assert_eq!(
            thread.read_with(cx, |thread, _cx| thread.status()),
            acp_thread::ThreadStatus::Generating
        );

        assert!(panel.update(cx, |panel, cx| panel.cancel_thread(&thread_id, cx)));
        cx.run_until_parked();

        assert_eq!(
            thread.read_with(cx, |thread, _cx| thread.status()),
            acp_thread::ThreadStatus::Idle
        );
        assert_eq!(active_thread_item(&workspace, cx), item);
        panel.read_with(cx, |panel, _cx| {
            assert!(panel.is_center_thread(thread_id));
            assert!(!panel.retained_threads().contains_key(&thread_id));
        });
    }

    #[gpui::test]
    async fn test_closing_running_center_thread_retains_it(cx: &mut TestAppContext) {
        let (workspace, panel, cx) = setup(cx).await;
        let item = deploy_stub_thread(&workspace, cx);
        let thread_id = item.read_with(cx, |item, cx| item.thread_id(cx));
        let view = conversation_view(&item, cx);
        stub_connection(&view, cx).set_next_prompt_updates(Vec::new());
        send_message(&item, cx);
        assert!(view.read_with(cx, |view, cx| {
            view.root_thread(cx).expect("connected").read(cx).status()
                == acp_thread::ThreadStatus::Generating
        }));
        close_active_item(&workspace, cx).await;
        panel.read_with(cx, |panel, _cx| {
            assert!(!panel.is_center_thread(thread_id));
            assert_eq!(panel.retained_threads().get(&thread_id), Some(&view));
        });
        assert!(metadata_exists(thread_id, cx));
        assert!(panel.update(cx, |panel, cx| panel.cancel_thread(&thread_id, cx)));
        cx.run_until_parked();
    }

    #[gpui::test]
    async fn test_closing_empty_center_draft_deletes_metadata(cx: &mut TestAppContext) {
        let (workspace, panel, cx) = setup(cx).await;
        let item = deploy_stub_thread(&workspace, cx);
        let thread_id = item.read_with(cx, |item, cx| item.thread_id(cx));
        assert!(metadata_exists(thread_id, cx));
        close_active_item(&workspace, cx).await;
        assert!(!metadata_exists(thread_id, cx));
        panel.read_with(cx, |panel, _cx| {
            assert!(!panel.is_center_thread(thread_id));
            assert!(!panel.retained_threads().contains_key(&thread_id));
        });
    }

    #[gpui::test]
    async fn test_discarding_center_draft_with_content_closes_without_retention(
        cx: &mut TestAppContext,
    ) {
        let (workspace, panel, cx) = setup(cx).await;
        let item = deploy_stub_thread(&workspace, cx);
        let thread_id = item.read_with(cx, |item, cx| item.thread_id(cx));
        set_message(&item, "typed draft", cx);
        panel.update_in(cx, |panel, window, cx| {
            panel.remove_thread(thread_id, window, cx)
        });
        cx.run_until_parked();
        workspace.read_with(cx, |workspace, _cx| {
            assert!(workspace.pane_for(&item).is_none())
        });
        assert!(!metadata_exists(thread_id, cx));
        panel.read_with(cx, |panel, _cx| {
            assert!(!panel.is_center_thread(thread_id));
            assert!(!panel.retained_threads().contains_key(&thread_id));
        });
    }

    #[gpui::test]
    async fn test_center_menu_falls_back_without_panel(cx: &mut TestAppContext) {
        let (workspace, cx) = setup_workspace(cx).await;
        cx.update(|_window, cx| {
            assert_eq!(
                center_menu_kind(&workspace.downgrade(), cx),
                CenterMenuKind::Terminal
            )
        });
        add_panel(&workspace, cx);
        cx.update(|_window, cx| {
            assert_eq!(
                center_menu_kind(&workspace.downgrade(), cx),
                CenterMenuKind::ThreadSubmenu
            )
        });
    }

    #[gpui::test]
    async fn test_center_menu_keeps_terminal_without_open_folder(cx: &mut TestAppContext) {
        let (workspace, panel, cx) = setup(cx).await;
        let project = workspace.read_with(cx, |workspace, _cx| workspace.project().clone());
        project.update(cx, |project, cx| {
            let worktree_ids = project
                .worktrees(cx)
                .map(|worktree| worktree.read(cx).id())
                .collect::<Vec<_>>();
            for worktree_id in worktree_ids {
                project.remove_worktree(worktree_id, cx);
            }
        });
        cx.run_until_parked();
        assert!(!panel.read_with(cx, |panel, cx| panel.supports_terminal(cx)));
        cx.update(|_window, cx| {
            let params = center_menu_params(&workspace.downgrade(), cx).expect("center menu");
            assert!(
                crate::agent_panel::new_thread_menu_entries(&params, cx)
                    .iter()
                    .any(|entry| entry.kind
                        == crate::agent_panel::NewThreadMenuEntryKind::Terminal
                        && !entry.disabled)
            );
        });
    }

    #[gpui::test]
    async fn test_activating_center_tab_dismisses_notification(cx: &mut TestAppContext) {
        let (workspace, _panel, cx) = setup(cx).await;
        let first = deploy_stub_thread(&workspace, cx);
        let _second = deploy_stub_thread(&workspace, cx);
        let view = conversation_view(&first, cx);
        let is_visible = |cx: &mut VisualTestContext| {
            view.update_in(cx, |view, window, cx| {
                view.test_agent_status_visible(window, cx)
            })
        };
        assert!(!is_visible(cx));
        stub_connection(&view, cx).set_next_prompt_updates(vec![
            acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new("Done".into())),
        ]);
        send_message(&first, cx);
        let has_notification = |cx: &VisualTestContext| {
            cx.windows()
                .iter()
                .any(|window| window.downcast::<crate::ui::AgentNotification>().is_some())
        };
        assert!(has_notification(cx));
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.activate_item(&first, true, true, window, cx);
        });
        cx.run_until_parked();
        assert!(is_visible(cx));
        assert!(!has_notification(cx));
    }

    #[gpui::test]
    async fn test_center_thread_hidden_by_zoom_shows_notification(cx: &mut TestAppContext) {
        let (workspace, panel, cx) = setup(cx).await;
        let item = deploy_stub_thread(&workspace, cx);
        let view = conversation_view(&item, cx);
        let source = workspace.read_with(cx, |workspace, _cx| workspace.active_pane().clone());
        let destination = workspace.update_in(cx, |workspace, window, cx| {
            let destination =
                workspace.split_pane(source.clone(), SplitDirection::Right, window, cx);
            destination.focus_handle(cx).focus(window, cx);
            destination
        });
        cx.run_until_parked();
        let _other = deploy_stub_thread(&workspace, cx);
        let is_visible = |cx: &mut VisualTestContext| {
            view.update_in(cx, |view, window, cx| {
                view.test_agent_status_visible(window, cx)
            })
        };
        assert!(is_visible(cx));
        destination.update_in(cx, |pane, window, cx| {
            pane.zoom_in(&workspace::ZoomIn, window, cx);
        });
        cx.run_until_parked();
        assert!(!is_visible(cx));

        stub_connection(&view, cx).set_next_prompt_updates(vec![
            acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new("Done".into())),
        ]);
        send_message(&item, cx);
        assert!(
            cx.windows()
                .iter()
                .any(|window| window.downcast::<crate::ui::AgentNotification>().is_some())
        );

        destination.update_in(cx, |pane, window, cx| {
            pane.zoom_out(&workspace::ZoomOut, window, cx);
        });
        cx.run_until_parked();
        assert!(is_visible(cx));
        assert!(
            !cx.windows()
                .iter()
                .any(|window| window.downcast::<crate::ui::AgentNotification>().is_some())
        );

        workspace.update_in(cx, |workspace, window, cx| {
            workspace.toggle_editor_zoom(&workspace::ToggleEditorZoom, window, cx);
        });
        cx.run_until_parked();
        assert!(!is_visible(cx));
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.toggle_editor_zoom(&workspace::ToggleEditorZoom, window, cx);
        });
        cx.run_until_parked();
        assert!(is_visible(cx));

        workspace.update_in(cx, |workspace, window, cx| {
            workspace.activate_item(&item, true, true, window, cx);
        });
        source.update_in(cx, |pane, window, cx| {
            pane.zoom_in(&workspace::ZoomIn, window, cx);
        });
        cx.run_until_parked();
        assert!(is_visible(cx));

        source.update_in(cx, |pane, window, cx| {
            pane.zoom_out(&workspace::ZoomOut, window, cx);
        });
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.toggle_editor_zoom(&workspace::ToggleEditorZoom, window, cx);
        });
        cx.run_until_parked();
        assert!(is_visible(cx));
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.toggle_panel_focus::<AgentPanel>(window, cx);
        });
        cx.run_until_parked();
        assert!(is_visible(cx));
        panel.update_in(cx, |panel, window, cx| {
            workspace::Panel::set_zoomed(panel, true, window, cx);
        });
        cx.run_until_parked();
        assert!(!is_visible(cx));
    }

    #[gpui::test]
    async fn test_center_tab_restores_after_metadata_reload_and_panel_creation(
        cx: &mut TestAppContext,
    ) {
        let (workspace, panel, cx) = setup(cx).await;
        let item = deploy_stub_thread(&workspace, cx);
        let thread_id = item.read_with(cx, |item, cx| item.thread_id(cx));
        set_message(&item, "typed draft", cx);
        let workspace_database = cx.update(|_window, cx| WorkspaceDb::global(cx));
        cx.update(|_window, cx| {
            ThreadMetadataStore::global(cx).update(cx, |store, cx| {
                store.set_title_override(thread_id, "Restored draft title".into(), cx);
            });
        });
        cx.run_until_parked();
        let workspace_id = workspace_database
            .next_id()
            .await
            .expect("workspace database row");
        let item_id = item.entity_id().as_u64();
        let project = workspace.read_with(cx, |workspace, _cx| workspace.project().clone());
        workspace
            .update(cx, |workspace, cx| {
                item.update(cx, |item, cx| {
                    item.workspace_id = Some(workspace_id);
                    item.serialize(workspace, item_id, false, cx)
                })
            })
            .expect("fresh item needs serialization")
            .await
            .expect("save center thread row");
        close_active_item(&workspace, cx).await;
        panel.update(cx, |panel, _cx| {
            assert!(panel.test_unload_retained_thread(thread_id))
        });
        drop(item);
        let filesystem = project.read_with(cx, |project, _cx| project.fs().clone());
        let project = Project::test(filesystem, [Path::new("/project")], cx).await;
        let restored_workspace = cx.update(|window, cx| {
            let multi_workspace = window
                .root::<MultiWorkspace>()
                .flatten()
                .expect("multi workspace window");
            multi_workspace.update(cx, |multi_workspace, cx| {
                multi_workspace.test_add_workspace(project.clone(), window, cx)
            })
        });
        cx.update(|_window, cx| ThreadMetadataStore::init_global(cx));
        let restored = cx
            .update(|window, cx| {
                ThreadItem::deserialize(
                    project.clone(),
                    restored_workspace.downgrade(),
                    workspace_id,
                    item_id,
                    window,
                    cx,
                )
            })
            .await
            .expect("restore waits for metadata reload");
        restored.read_with(cx, |item, cx| {
            assert_eq!(item.thread_id(cx), thread_id);
            assert_eq!(item.tab_content_text(0, cx), "Restored draft title");
            assert!(item.conversation_view().is_none());
            assert!(item.needs_serialize);
        });
        let restored_item_id = restored.entity_id().as_u64();
        restored_workspace
            .update(cx, |workspace, cx| {
                restored.update(cx, |item, cx| {
                    item.serialize(workspace, restored_item_id, false, cx)
                })
            })
            .expect("pending item needs serialization")
            .await
            .expect("save pending restored item");
        cx.update(|window, cx| {
            ThreadItem::cleanup(workspace_id, vec![restored_item_id], window, cx)
        })
        .await
        .expect("remove old serialized item");
        cx.update(|_window, cx| {
            let database = ThreadItemDb::global(cx);
            assert!(
                database
                    .get_thread_item(item_id, workspace_id)
                    .expect("old item row")
                    .is_none()
            );
            assert_eq!(
                database
                    .get_thread_item(restored_item_id, workspace_id)
                    .expect("restored row"),
                Some((thread_id, "stub".into()))
            );
        });
        restored_workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(Box::new(restored.clone()), None, true, window, cx);
        });
        cx.run_until_parked();
        let restored_panel = restored_workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            assert!(
                panel.read(cx).is_center_thread(thread_id),
                "registered before PanelAdded"
            );
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });
        cx.run_until_parked();
        assert!(message_editor_is_focused(&restored, cx));
        restored.read_with(cx, |item, cx| {
            assert_eq!(item.tab_content_text(0, cx), "Restored draft title");
        });
        assert_eq!(views_for(thread_id, &restored_panel, cx), 1);
        restored.read_with(cx, |item, _cx| assert!(item.needs_serialize));
        let missing_row = cx
            .update(|window, cx| {
                ThreadItem::deserialize(
                    project.clone(),
                    restored_workspace.downgrade(),
                    workspace_id,
                    item_id,
                    window,
                    cx,
                )
            })
            .await;
        assert!(missing_row.is_err());
        cx.update(|_window, cx| {
            ThreadMetadataStore::global(cx).update(cx, |store, cx| store.delete(thread_id, cx));
        });
        let missing_thread = cx
            .update(|window, cx| {
                ThreadItem::deserialize(
                    project,
                    restored_workspace.downgrade(),
                    workspace_id,
                    restored_item_id,
                    window,
                    cx,
                )
            })
            .await;
        assert!(missing_thread.is_err());
    }
}
