use acp_thread::{AgentConnection, StubAgentConnection};
use agent_client_protocol as acp;
use agent_servers::{AgentServer, AgentServerDelegate};
use gpui::{Entity, Task, TestAppContext, VisualTestContext};
use project::AgentId;
use settings::SettingsStore;
use std::any::Any;
use std::rc::Rc;

use crate::AgentPanel;
use crate::agent_panel;

pub struct StubAgentServer<C> {
    connection: C,
}

impl<C> StubAgentServer<C> {
    pub fn new(connection: C) -> Self {
        Self { connection }
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
        ui::IconName::Ai
    }

    fn agent_id(&self) -> AgentId {
        "Test".into()
    }

    fn connect(
        &self,
        _delegate: AgentServerDelegate,
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
        theme::init(theme::LoadThemes::JustBase, cx);
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

pub fn send_message(panel: &Entity<AgentPanel>, cx: &mut VisualTestContext) {
    let thread_view = panel.read_with(cx, |panel, cx| panel.as_active_thread_view(cx).unwrap());
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
