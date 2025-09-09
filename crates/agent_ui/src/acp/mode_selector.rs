use acp_thread::AgentSessionModes;
use agent_client_protocol as acp;
use agent_servers::AgentServer;
use fs::Fs;
use gpui::{Context, Entity, FocusHandle, WeakEntity, Window, prelude::*};
use std::{rc::Rc, sync::Arc};
use ui::{
    Button, ContextMenu, ContextMenuEntry, KeyBinding, PopoverMenu, PopoverMenuHandle, Tooltip,
    prelude::*,
};

use crate::{CycleModeSelector, ToggleProfileSelector};

pub struct ModeSelector {
    connection: Rc<dyn AgentSessionModes>,
    agent_server: Rc<dyn AgentServer>,
    menu_handle: PopoverMenuHandle<ContextMenu>,
    focus_handle: FocusHandle,
    fs: Arc<dyn Fs>,
    setting_mode: bool,
}

impl ModeSelector {
    pub fn new(
        session_modes: Rc<dyn AgentSessionModes>,
        agent_server: Rc<dyn AgentServer>,
        fs: Arc<dyn Fs>,
        focus_handle: FocusHandle,
    ) -> Self {
        Self {
            connection: session_modes,
            agent_server,
            menu_handle: PopoverMenuHandle::default(),
            fs,
            setting_mode: false,
            focus_handle,
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
        let weak_self = cx.weak_entity();

        ContextMenu::build(window, cx, move |mut menu, _window, cx| {
            let all_modes = self.connection.all_modes();
            let current_mode = self.connection.current_mode();
            let default_mode = self.agent_server.default_mode(cx);

            for mode in all_modes {
                let is_selected = &mode.id == &current_mode;
                let is_default = Some(&mode.id) == default_mode.as_ref();
                let entry = ContextMenuEntry::new(mode.name.clone())
                    .toggleable(IconPosition::End, is_selected);

                let entry = if let Some(description) = &mode.description {
                    entry.documentation_aside(ui::DocumentationSide::Left, {
                        let description = description.clone();

                        move |cx| {
                            v_flex()
                                .gap_1()
                                .child(Label::new(description.clone()))
                                .child(
                                    h_flex()
                                        .pt_1()
                                        .border_t_1()
                                        .border_color(cx.theme().colors().border_variant)
                                        .gap_0p5()
                                        .text_sm()
                                        .text_color(Color::Muted.color(cx))
                                        .child("Hold")
                                        .child(div().pt_0p5().children(ui::render_modifiers(
                                            &gpui::Modifiers::secondary_key(),
                                            PlatformStyle::platform(),
                                            None,
                                            Some(ui::TextSize::Default.rems(cx).into()),
                                            true,
                                        )))
                                        .child(div().map(|this| {
                                            if is_default {
                                                this.child("to also unset as default")
                                            } else {
                                                this.child("to also set as default")
                                            }
                                        })),
                                )
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
            .trigger_with_tooltip(
                trigger_button,
                Tooltip::element({
                    let focus_handle = self.focus_handle.clone();
                    move |window, cx| {
                        v_flex()
                            .gap_1()
                            .child(
                                h_flex()
                                    .pb_1()
                                    .gap_2()
                                    .justify_between()
                                    .border_b_1()
                                    .border_color(cx.theme().colors().border_variant)
                                    .child(Label::new("Cycle Through Modes"))
                                    .children(KeyBinding::for_action_in(
                                        &CycleModeSelector,
                                        &focus_handle,
                                        window,
                                        cx,
                                    )),
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .justify_between()
                                    .child(Label::new("Toggle Mode Menu"))
                                    .children(KeyBinding::for_action_in(
                                        &ToggleProfileSelector,
                                        &focus_handle,
                                        window,
                                        cx,
                                    )),
                            )
                            .into_any()
                    }
                }),
            )
            .anchor(gpui::Corner::BottomRight)
            .with_handle(self.menu_handle.clone())
            .menu(move |window, cx| {
                Some(this.update(cx, |this, cx| this.build_context_menu(window, cx)))
            })
    }
}
