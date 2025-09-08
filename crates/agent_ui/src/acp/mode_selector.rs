use acp_thread::AgentSessionModes;
use agent_client_protocol as acp;
use gpui::{Context, Entity, WeakEntity, Window, prelude::*};
use std::rc::Rc;
use ui::{
    Button, ContextMenu, ContextMenuEntry, PopoverMenu, PopoverMenuHandle, Tooltip, prelude::*,
};

use crate::ToggleModeSelector;

pub struct ModeSelector {
    connection: Rc<dyn AgentSessionModes>,
    menu_handle: PopoverMenuHandle<ContextMenu>,
    setting_mode: bool,
}

impl ModeSelector {
    pub fn new(session_modes: Rc<dyn AgentSessionModes>) -> Self {
        Self {
            connection: session_modes,
            menu_handle: PopoverMenuHandle::default(),
            setting_mode: false,
        }
    }

    pub fn menu_handle(&self) -> PopoverMenuHandle<ContextMenu> {
        self.menu_handle.clone()
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

    fn build_context_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        let all_modes = self.connection.all_modes();
        let current_mode = self.connection.current_mode();

        ContextMenu::build(window, cx, |mut menu, _window, _cx| {
            for mode in all_modes {
                menu.push_item(
                    ContextMenuEntry::new(mode.name.clone())
                        .toggleable(IconPosition::End, mode.id == current_mode)
                        .handler({
                            let mode_id = mode.id.clone();
                            let connection = self.connection.clone();
                            move |_window, cx| {
                                let task = connection.set_mode(mode_id.clone(), cx);
                                cx.spawn(async move |_cx| {
                                    if let Err(err) = task.await {
                                        log::error!("Failed to set session mode: {:?}", err);
                                    }
                                    anyhow::Ok(())
                                })
                                .detach();
                            }
                        }),
                );
            }

            menu
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

        let this = cx.entity();
        let trigger_button = Button::new("mode-selector-trigger", current_mode_name)
            .label_size(LabelSize::Small)
            .style(ButtonStyle::Subtle)
            .color(Color::Muted)
            .icon(IconName::ChevronDown)
            .icon_size(IconSize::XSmall)
            .icon_position(IconPosition::End)
            .icon_color(Color::Muted)
            .disabled(self.setting_mode);

        PopoverMenu::new("mode-selector")
            .trigger_with_tooltip(trigger_button, move |window, cx| {
                Tooltip::for_action("Toggle Mode Menu", &ToggleModeSelector, window, cx)
            })
            .anchor(gpui::Corner::BottomLeft)
            .with_handle(self.menu_handle.clone())
            .menu(move |window, cx| {
                Some(this.update(cx, |this, cx| this.build_context_menu(window, cx)))
            })
    }
}
