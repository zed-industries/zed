use acp_thread::AgentSessionModes;
use agent_client_protocol as acp;
use agent_servers::AgentServer;
use agent_settings::AgentSettings;
use fs::Fs;
use gpui::{Context, Entity, WeakEntity, Window, prelude::*};
use settings::Settings as _;
use std::{rc::Rc, sync::Arc};
use ui::{
    Button, ContextMenu, ContextMenuEntry, DocumentationSide, KeyBinding, PopoverMenu,
    PopoverMenuHandle, Tooltip, prelude::*,
};

use crate::{CycleModeSelector, ToggleProfileSelector, ui::HoldForDefault};

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
        let current_mode = self.connection.current_mode();

        let current_index = all_modes
            .iter()
            .position(|mode| mode.id.0 == current_mode.0)
            .unwrap_or(0);

        let next_index = (current_index + 1) % all_modes.len();
        self.set_mode(all_modes[next_index].id.clone(), cx);
    }

    pub fn mode(&self) -> acp::SessionModeId {
        self.connection.current_mode()
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
        let weak_self = cx.weak_entity();

        ContextMenu::build(window, cx, move |mut menu, _window, cx| {
            let all_modes = self.connection.all_modes();
            let current_mode = self.connection.current_mode();
            let default_mode = self.agent_server.default_mode(cx);

            let settings = AgentSettings::get_global(cx);
            let side = match settings.dock {
                settings::DockPosition::Left => DocumentationSide::Right,
                settings::DockPosition::Bottom | settings::DockPosition::Right => {
                    DocumentationSide::Left
                }
            };

            for mode in all_modes {
                let is_selected = &mode.id == &current_mode;
                let is_default = Some(&mode.id) == default_mode.as_ref();
                let entry = ContextMenuEntry::new(mode.name.clone())
                    .toggleable(IconPosition::End, is_selected);

                let entry = if let Some(description) = &mode.description {
                    entry.documentation_aside(side, {
                        let description = description.clone();

                        move |_| {
                            v_flex()
                                .gap_1()
                                .child(Label::new(description.clone()))
                                .child(HoldForDefault::new(is_default))
                                .into_any_element()
                        }
                    })
                } else {
                    entry
                };

                menu.push_item(entry.handler({
                    let mode_id = mode.id.clone();
                    let weak_self = weak_self.clone();
                    move |window, cx| {
                        weak_self
                            .update(cx, |this, cx| {
                                if window.modifiers().secondary() {
                                    this.agent_server.set_default_mode(
                                        if is_default {
                                            None
                                        } else {
                                            Some(mode_id.clone())
                                        },
                                        this.fs.clone(),
                                        cx,
                                    );
                                }

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
            .icon(icon)
            .icon_size(IconSize::XSmall)
            .icon_position(IconPosition::End)
            .icon_color(Color::Muted)
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
            .anchor(gpui::Corner::BottomRight)
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
