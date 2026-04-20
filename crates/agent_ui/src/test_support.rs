use acp_thread::{AgentConnection, StubAgentConnection};
use agent_client_protocol as acp;
use agent_servers::{AgentServer, AgentServerDelegate};
use gpui::{Entity, Task, TestAppContext, VisualTestContext};
use project::AgentId;
use project::Project;
use settings::SettingsStore;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

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
        cx.set_global(acp_thread::StubSessionCounter(
            std::sync::atomic::AtomicUsize::new(0),
        ));
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
        release_channel::init("0.0.0".parse().unwrap(), cx);
        agent_panel::init(cx);
    });
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
