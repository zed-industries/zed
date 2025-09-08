use acp_thread::AgentSessionModes;
use agent_client_protocol as acp;
use gpui::{Context, WeakEntity, Window, prelude::*};
use std::rc::Rc;
use ui::{Button, prelude::*};

use crate::CycleModeSelector;

pub struct ModeSelector {
    connection: Rc<dyn AgentSessionModes>,
    setting_mode: bool,
}

impl ModeSelector {
    pub fn new(session_modes: Rc<dyn AgentSessionModes>) -> Self {
        Self {
            connection: session_modes,
            setting_mode: false,
        }
    }

    pub fn cycle_mode(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let all_modes = self.connection.all_modes();
        let current_mode = self.connection.current_mode();

        let current_index = all_modes
            .iter()
            .position(|mode| mode.id.0 == current_mode.0)
            .unwrap_or(0);

        let next_index = (current_index + 1) % all_modes.len();
        self.set_mode(all_modes[next_index].id.clone(), cx);
    }

    pub fn set_mode(&mut self, mode: acp::SessionModeId, cx: &mut Context<Self>) {
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
}

impl Render for ModeSelector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let current_mode_id = self.connection.current_mode();
        let current_mode_name = self
            .connection
            .all_modes()
            .iter()
            .find(|mode| mode.id == current_mode_id)
            .unwrap()
            .name
            .clone();

        Button::new("mode-selector-trigger", current_mode_name)
            .label_size(LabelSize::Small)
            .style(ButtonStyle::Subtle)
            .color(Color::Muted)
            .tooltip(|window, cx| {
                ui::Tooltip::for_action("Cycle through modes", &CycleModeSelector, window, cx)
            })
            .on_click(cx.listener(|this, _event, window, cx| {
                this.cycle_mode(window, cx);
            }))
    }
}
