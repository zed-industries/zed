use acp_thread::{AgentConnection, StubAgentConnection};
use agent_client_protocol::schema as acp;
use agent_servers::{AgentServer, AgentServerDelegate};
use gpui::{
    App, AppContext as _, Context, Entity, EventEmitter, FocusHandle, Focusable, IntoElement,
    Pixels, Render, Task, TestAppContext, VisualTestContext, Window, div, px,
};
use project::AgentId;
use project::Project;
use settings::SettingsStore;
use std::any::Any;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use workspace::{MultiWorkspace, Sidebar as WorkspaceSidebar, SidebarEvent, SidebarSide};

use crate::AgentPanel;
use crate::agent_panel;

thread_local! {
    static STUB_AGENT_CONNECTION: RefCell<Option<StubAgentConnection>> = const { RefCell::new(None) };
}

/// Registers a `StubAgentConnection` that will be used by `Agent::Stub`.
///
/// Returns the same connection so callers can hold onto it and control
/// the stub's behavior (e.g. `connection.set_next_prompt_updates(...)`).
pub fn set_stub_agent_connection(connection: StubAgentConnection) -> StubAgentConnection {
    STUB_AGENT_CONNECTION.with(|cell| {
        *cell.borrow_mut() = Some(connection.clone());
    });
    connection
}

/// Returns the shared `StubAgentConnection` used by `Agent::Stub`,
/// creating a default one if none was registered.
pub fn stub_agent_connection() -> StubAgentConnection {
    STUB_AGENT_CONNECTION.with(|cell| {
        let mut borrow = cell.borrow_mut();
        borrow.get_or_insert_with(StubAgentConnection::new).clone()
    })
}

pub struct StubAgentServer<C> {
    connection: C,
    agent_id: AgentId,
}

impl<C> StubAgentServer<C>
where
    C: AgentConnection,
{
    pub fn new(connection: C) -> Self {
        Self {
            connection,
            agent_id: "Test".into(),
        }
    }

    pub fn with_connection_agent_id(mut self) -> Self {
        self.agent_id = self.connection.agent_id();
        self
    }
}

impl StubAgentServer<StubAgentConnection> {
    pub fn default_response() -> Self {
        let conn = StubAgentConnection::new();
        conn.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("Default response".into()),
        )]);
        Self::new(conn)
    }
}

impl<C> AgentServer for StubAgentServer<C>
where
    C: 'static + AgentConnection + Send + Clone,
{
    fn logo(&self) -> ui::IconName {
        ui::IconName::ZedAgent
    }

    fn agent_id(&self) -> AgentId {
        self.agent_id.clone()
    }

    fn connect(
        &self,
        _delegate: AgentServerDelegate,
        _project: Entity<Project>,
        _cx: &mut gpui::App,
    ) -> Task<gpui::Result<Rc<dyn AgentConnection>>> {
        Task::ready(Ok(Rc::new(self.connection.clone())))
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

pub fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        // Use an isolated DB so parallel tests can't see each other's
        // persisted records (e.g. created-worktree records).
        cx.set_global(db::AppDatabase::test_new());
        cx.set_global(acp_thread::StubSessionCounter(
            std::sync::atomic::AtomicUsize::new(0),
        ));
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
        release_channel::init("0.0.0".parse().unwrap(), cx);
        agent_panel::init(cx);
        crate::terminal_thread_metadata_store::TerminalThreadMetadataStore::init_global(cx);
    });
}

/// Returns the creation time assigned to a linked worktree's git metadata
/// directory, mirroring `FakeGitRepository::worktree_created_at` (which uses
/// the FakeFs directory mtime as a stand-in for the creation time).
pub async fn fake_worktree_created_at(fs: &dyn fs::Fs, worktree_path: &Path) -> SystemTime {
    let git_file = fs.load(&worktree_path.join(".git")).await.unwrap();
    let git_dir = worktree_path.join(git_file.strip_prefix("gitdir:").unwrap().trim());
    let (seconds, nanos) = fs
        .metadata(&git_dir)
        .await
        .unwrap()
        .unwrap()
        .mtime
        .to_seconds_and_nanos_for_persistence()
        .unwrap();
    UNIX_EPOCH + Duration::new(seconds, nanos)
}

/// Records the worktree in the created-worktrees registry with its actual
/// (fake) creation time, as the worktree creation flow would. Tests that
/// expect a worktree to be archivable must call this after setting it up.
pub async fn record_zed_created_worktree(
    fs: &dyn fs::Fs,
    worktree_path: &Path,
    remote: Option<&remote::RemoteConnectionOptions>,
    cx: &mut TestAppContext,
) {
    let created_at = fake_worktree_created_at(fs, worktree_path).await;
    cx.update(|cx| {
        git_ui::created_worktrees::record_created_worktree(worktree_path, remote, created_at, cx)
    })
    .await
    .unwrap();
}

pub struct TestWorkspaceSidebar {
    focus_handle: FocusHandle,
    threads_list_active: bool,
}

impl TestWorkspaceSidebar {
    fn new(threads_list_active: bool, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            threads_list_active,
        }
    }
}

impl EventEmitter<SidebarEvent> for TestWorkspaceSidebar {}

impl Focusable for TestWorkspaceSidebar {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl WorkspaceSidebar for TestWorkspaceSidebar {
    fn width(&self, _cx: &App) -> Pixels {
        px(300.)
    }

    fn set_width(&mut self, _width: Option<Pixels>, _cx: &mut Context<Self>) {}

    fn has_notifications(&self, _cx: &App) -> bool {
        false
    }

    fn side(&self, _cx: &App) -> SidebarSide {
        SidebarSide::Left
    }

    fn is_threads_list_view_active(&self) -> bool {
        self.threads_list_active
    }
}

impl Render for TestWorkspaceSidebar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

pub fn register_test_sidebar(
    threads_list_active: bool,
    cx: &mut VisualTestContext,
) -> Entity<TestWorkspaceSidebar> {
    cx.update(|window, cx| {
        let multi_workspace = window
            .root::<MultiWorkspace>()
            .flatten()
            .expect("test window should have a MultiWorkspace root");
        let sidebar = cx.new(|cx| TestWorkspaceSidebar::new(threads_list_active, cx));
        multi_workspace.update(cx, |multi_workspace, cx| {
            multi_workspace.register_sidebar(sidebar.clone(), cx);
        });
        sidebar
    })
}

pub fn open_thread_with_connection(
    panel: &Entity<AgentPanel>,
    connection: StubAgentConnection,
    cx: &mut VisualTestContext,
) {
    panel.update_in(cx, |panel, window, cx| {
        panel.open_external_thread_with_server(
            Rc::new(StubAgentServer::new(connection)),
            window,
            cx,
        );
    });
    cx.run_until_parked();
}

/// Opens a draft thread against a stub server so the panel's `draft_thread`
/// pointer is populated for tests that care about draft UX.
pub fn open_draft_with_connection(
    panel: &Entity<AgentPanel>,
    connection: StubAgentConnection,
    cx: &mut VisualTestContext,
) {
    panel.update_in(cx, |panel, window, cx| {
        panel.open_draft_with_server(Rc::new(StubAgentServer::new(connection)), window, cx);
    });
    cx.run_until_parked();
}

pub fn open_thread_with_custom_connection<C>(
    panel: &Entity<AgentPanel>,
    connection: C,
    cx: &mut VisualTestContext,
) where
    C: 'static + AgentConnection + Send + Clone,
{
    panel.update_in(cx, |panel, window, cx| {
        panel.open_external_thread_with_server(
            Rc::new(StubAgentServer::new(connection).with_connection_agent_id()),
            window,
            cx,
        );
    });
    cx.run_until_parked();
}

pub fn send_message(panel: &Entity<AgentPanel>, cx: &mut VisualTestContext) {
    let thread_view = panel.read_with(cx, |panel, cx| panel.active_thread_view(cx).unwrap());
    let message_editor = thread_view.read_with(cx, |view, _cx| view.message_editor.clone());
    message_editor.update_in(cx, |editor, window, cx| {
        editor.set_text("Hello", window, cx);
    });
    thread_view.update_in(cx, |view, window, cx| view.send(window, cx));
    cx.run_until_parked();
}

pub fn type_draft_prompt(panel: &Entity<AgentPanel>, text: &str, cx: &mut VisualTestContext) {
    let thread_view = panel.read_with(cx, |panel, cx| panel.active_thread_view(cx).unwrap());
    let message_editor = thread_view.read_with(cx, |view, _cx| view.message_editor.clone());
    message_editor.update_in(cx, |editor, window, cx| {
        editor.set_text(text, window, cx);
    });
    cx.run_until_parked();
    // Drain the debounced draft-prompt persist task so the kvp write has
    // landed by the time we return.
    cx.executor()
        .advance_clock(crate::conversation_view::DRAFT_PROMPT_PERSIST_DEBOUNCE * 2);
    cx.run_until_parked();
}

pub fn active_session_id(panel: &Entity<AgentPanel>, cx: &VisualTestContext) -> acp::SessionId {
    panel.read_with(cx, |panel, cx| {
        let thread = panel.active_agent_thread(cx).unwrap();
        thread.read(cx).session_id().clone()
    })
}

pub fn active_thread_id(
    panel: &Entity<AgentPanel>,
    cx: &VisualTestContext,
) -> crate::thread_metadata_store::ThreadId {
    panel.read_with(cx, |panel, cx| panel.active_thread_id(cx).unwrap())
}
