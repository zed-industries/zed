use acp_thread::AgentSessionModes;
use agent_client_protocol::schema::v1 as acp;
use agent_servers::AgentServer;

use fs::Fs;
use gpui::{Context, Entity, WeakEntity, Window, prelude::*};

use std::{rc::Rc, sync::Arc};
use ui::{
    Button, ContextMenu, ContextMenuEntry, KeyBinding, PopoverMenu, PopoverMenuHandle, Tooltip,
    prelude::*,
};

use crate::{CycleModeSelector, ToggleProfileSelector, ui::documentation_aside_side};

pub struct ModeSelector {
    connection: Rc<dyn AgentSessionModes>,
    agent_server: Rc<dyn AgentServer>,
    menu_handle: PopoverMenuHandle<ContextMenu>,
    fs: Arc<dyn Fs>,
    setting_mode: bool,
}

impl ModeSelector {
    pub fn new(
        session_modes: Rc<dyn AgentSessionModes>,
        agent_server: Rc<dyn AgentServer>,
        fs: Arc<dyn Fs>,
    ) -> Self {
        Self {
            connection: session_modes,
            agent_server,
            menu_handle: PopoverMenuHandle::default(),
            fs,
            setting_mode: false,
        }
    }

    pub fn menu_handle(&self) -> PopoverMenuHandle<ContextMenu> {
        self.menu_handle.clone()
    }

    pub fn cycle_mode(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let all_modes = self.connection.all_modes();
        if all_modes.is_empty() {
            return;
        }

        let current_mode = self.connection.current_mode();

        let current_index = all_modes
            .iter()
            .position(|mode| mode.id.0 == current_mode.0)
            .unwrap_or(0);

        if let Some(next_mode) = all_modes.get((current_index + 1) % all_modes.len()) {
            self.set_mode(next_mode.id.clone(), cx);
        }
    }

    pub fn mode(&self) -> acp::SessionModeId {
        self.connection.current_mode()
    }

    pub fn set_mode(&mut self, mode: acp::SessionModeId, cx: &mut Context<Self>) {
        self.agent_server
            .set_default_mode(Some(mode.clone()), self.fs.clone(), cx);

        let task = self.connection.set_mode(mode, cx);
        self.setting_mode = true;
        cx.notify();

        cx.spawn(async move |this: WeakEntity<ModeSelector>, cx| {
            if let Err(err) = task.await {
                log::error!("Failed to set session mode: {:?}", err);
            }
            this.update(cx, |this, cx| {
                this.setting_mode = false;
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    fn build_context_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        let weak_self = cx.weak_entity();

        ContextMenu::build(window, cx, move |mut menu, _window, cx| {
            let all_modes = self.connection.all_modes();
            let current_mode = self.connection.current_mode();

            let side = documentation_aside_side(cx);

            for mode in all_modes {
                let is_selected = &mode.id == &current_mode;
                let entry = ContextMenuEntry::new(mode.name.clone())
                    .toggleable(IconPosition::End, is_selected);

                let entry = if let Some(description) = &mode.description {
                    entry.documentation_aside(side, {
                        let description = description.clone();

                        move |_| Label::new(description.clone()).into_any_element()
                    })
                } else {
                    entry
                };

                menu.push_item(entry.handler({
                    let mode_id = mode.id.clone();
                    let weak_self = weak_self.clone();
                    move |_window, cx| {
                        weak_self
                            .update(cx, |this, cx| {
                                this.set_mode(mode_id.clone(), cx);
                            })
                            .ok();
                    }
                }));
            }

            menu.key_context("ModeSelector")
        })
    }
}

impl Render for ModeSelector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let current_mode_id = self.connection.current_mode();
        let current_mode_name = self
            .connection
            .all_modes()
            .iter()
            .find(|mode| mode.id == current_mode_id)
            .map(|mode| mode.name.clone())
            .unwrap_or_else(|| "Unknown".into());

        let this = cx.weak_entity();

        let icon = if self.menu_handle.is_deployed() {
            IconName::ChevronUp
        } else {
            IconName::ChevronDown
        };

        let trigger_button = Button::new("mode-selector-trigger", current_mode_name)
            .label_size(LabelSize::Small)
            .color(Color::Muted)
            .end_icon(Icon::new(icon).size(IconSize::XSmall).color(Color::Muted))
            .disabled(self.setting_mode);

        PopoverMenu::new("mode-selector")
            .trigger_with_tooltip(
                trigger_button,
                Tooltip::element({
                    move |_window, cx| {
                        v_flex()
                            .gap_1()
                            .child(
                                h_flex()
                                    .gap_2()
                                    .justify_between()
                                    .child(Label::new("Change Mode"))
                                    .child(KeyBinding::for_action(&ToggleProfileSelector, cx)),
                            )
                            .child(
                                h_flex()
                                    .pt_1()
                                    .gap_2()
                                    .border_t_1()
                                    .border_color(cx.theme().colors().border_variant)
                                    .justify_between()
                                    .child(Label::new("Cycle Through Modes"))
                                    .child(KeyBinding::for_action(&CycleModeSelector, cx)),
                            )
                            .into_any()
                    }
                }),
            )
            .anchor(gpui::Anchor::BottomRight)
            .with_handle(self.menu_handle.clone())
            .offset(gpui::Point {
                x: px(0.0),
                y: px(-2.0),
            })
            .menu(move |window, cx| {
                this.update(cx, |this, cx| this.build_context_menu(window, cx))
                    .ok()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use acp_thread::AgentConnection;
    use fs::FakeFs;
    use gpui::{App, Task, TestAppContext};
    use parking_lot::Mutex;
    use project::{AgentId, Project};
    use std::{any::Any, cell::RefCell};

    #[gpui::test]
    fn setting_mode_saves_selected_mode_as_default(cx: &mut TestAppContext) {
        let agent_server = Rc::new(TestAgentServer::default());
        let session_modes = Rc::new(TestSessionModes::new());
        let fs: Arc<dyn Fs> = FakeFs::new(cx.executor());

        cx.update(|cx| {
            let session_modes: Rc<dyn AgentSessionModes> = session_modes.clone();
            let agent_server: Rc<dyn AgentServer> = agent_server.clone();
            let selector = cx.new(|_| ModeSelector::new(session_modes, agent_server, fs));

            selector.update(cx, |selector, cx| {
                selector.set_mode(acp::SessionModeId::new("manual"), cx);
            });
        });

        assert_eq!(
            agent_server.saved_defaults.lock().as_slice(),
            &[Some(acp::SessionModeId::new("manual"))]
        );
        assert_eq!(
            session_modes.set_modes.borrow().as_slice(),
            &[acp::SessionModeId::new("manual")]
        );
    }

    #[derive(Default)]
    struct TestAgentServer {
        saved_defaults: Arc<Mutex<Vec<Option<acp::SessionModeId>>>>,
    }

    impl AgentServer for TestAgentServer {
        fn logo(&self) -> IconName {
            IconName::ZedAssistant
        }

        fn agent_id(&self) -> AgentId {
            AgentId::new("test-agent")
        }

        fn connect(
            &self,
            _delegate: agent_servers::AgentServerDelegate,
            _project: Entity<Project>,
            _cx: &mut App,
        ) -> Task<anyhow::Result<Rc<dyn AgentConnection>>> {
            Task::ready(Err(anyhow::anyhow!("test agent server cannot connect")))
        }

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }

        fn set_default_mode(
            &self,
            mode_id: Option<acp::SessionModeId>,
            _fs: Arc<dyn Fs>,
            _cx: &mut App,
        ) {
            self.saved_defaults.lock().push(mode_id);
        }
    }

    struct TestSessionModes {
        current_mode: RefCell<acp::SessionModeId>,
        set_modes: RefCell<Vec<acp::SessionModeId>>,
    }

    impl TestSessionModes {
        fn new() -> Self {
            Self {
                current_mode: RefCell::new(acp::SessionModeId::new("auto")),
                set_modes: RefCell::new(Vec::new()),
            }
        }
    }

    impl AgentSessionModes for TestSessionModes {
        fn current_mode(&self) -> acp::SessionModeId {
            self.current_mode.borrow().clone()
        }

        fn all_modes(&self) -> Vec<acp::SessionMode> {
            vec![
                acp::SessionMode::new("auto", "Auto"),
                acp::SessionMode::new("manual", "Manual"),
            ]
        }

        fn set_mode(&self, mode: acp::SessionModeId, _cx: &mut App) -> Task<anyhow::Result<()>> {
            *self.current_mode.borrow_mut() = mode.clone();
            self.set_modes.borrow_mut().push(mode);
            Task::ready(Ok(()))
        }
    }
}
