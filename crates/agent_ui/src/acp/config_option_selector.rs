use acp_thread::AgentSessionConfigOptions;
use agent_client_protocol as acp;
use agent_servers::AgentServer;
use agent_settings::AgentSettings;
use fs::Fs;
use gpui::{Context, Entity, WeakEntity, Window, prelude::*};
use picker::popover_menu::PickerPopoverMenu;
use settings::Settings as _;
use std::rc::Rc;
use std::sync::Arc;
use ui::{
    Button, ContextMenu, ContextMenuEntry, DocumentationEdge, DocumentationSide, PopoverMenu,
    PopoverMenuHandle, Tooltip, prelude::*,
};

use super::config_option_picker::{ConfigOptionPicker, config_option_picker, count_config_options};
use crate::ui::HoldForDefault;

const PICKER_THRESHOLD: usize = 4;

pub struct ConfigOptionSelector {
    config_options: Rc<dyn AgentSessionConfigOptions>,
    config_id: acp::SessionConfigId,
    agent_server: Rc<dyn AgentServer>,
    fs: Arc<dyn Fs>,
    context_menu_handle: PopoverMenuHandle<ContextMenu>,
    picker_handle: PopoverMenuHandle<ConfigOptionPicker>,
    picker: Option<Entity<ConfigOptionPicker>>,
    setting_value: bool,
    use_picker: bool,
}

impl ConfigOptionSelector {
    pub fn new(
        config_options: Rc<dyn AgentSessionConfigOptions>,
        config_id: acp::SessionConfigId,
        agent_server: Rc<dyn AgentServer>,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let option_count = config_options
            .config_options()
            .iter()
            .find(|opt| opt.id == config_id)
            .map(|opt| count_config_options(opt))
            .unwrap_or(0);

        let use_picker = option_count >= PICKER_THRESHOLD;

        let picker = if use_picker {
            Some(cx.new(|cx| {
                config_option_picker(
                    config_options.clone(),
                    config_id.clone(),
                    agent_server.clone(),
                    fs.clone(),
                    window,
                    cx,
                )
            }))
        } else {
            None
        };

        Self {
            config_options,
            config_id,
            agent_server,
            fs,
            context_menu_handle: PopoverMenuHandle::default(),
            picker_handle: PopoverMenuHandle::default(),
            picker,
            setting_value: false,
            use_picker,
        }
    }

    fn current_option(&self) -> Option<acp::SessionConfigOption> {
        self.config_options
            .config_options()
            .into_iter()
            .find(|opt| opt.id == self.config_id)
    }

    fn current_value(&self) -> Option<acp::SessionConfigValueId> {
        self.current_option().and_then(|opt| match &opt.kind {
            acp::SessionConfigKind::Select(select) => Some(select.current_value.clone()),
            _ => None,
        })
    }

    fn current_value_name(&self) -> String {
        let Some(option) = self.current_option() else {
            return "Unknown".to_string();
        };

        match &option.kind {
            acp::SessionConfigKind::Select(select) => {
                Self::find_option_name(&select.options, &select.current_value)
                    .unwrap_or_else(|| "Unknown".to_string())
            }
            _ => "Unknown".to_string(),
        }
    }

    fn find_option_name(
        options: &acp::SessionConfigSelectOptions,
        value_id: &acp::SessionConfigValueId,
    ) -> Option<String> {
        match options {
            acp::SessionConfigSelectOptions::Ungrouped(opts) => opts
                .iter()
                .find(|o| &o.value == value_id)
                .map(|o| o.name.clone()),
            acp::SessionConfigSelectOptions::Grouped(groups) => groups.iter().find_map(|group| {
                group
                    .options
                    .iter()
                    .find(|o| &o.value == value_id)
                    .map(|o| o.name.clone())
            }),
            _ => None,
        }
    }

    pub fn set_value(&mut self, value: acp::SessionConfigValueId, cx: &mut Context<Self>) {
        let task = self
            .config_options
            .set_config_option(self.config_id.clone(), value, cx);
        self.setting_value = true;
        cx.notify();

        cx.spawn(async move |this: WeakEntity<ConfigOptionSelector>, cx| {
            if let Err(err) = task.await {
                log::error!("Failed to set config option: {:?}", err);
            }
            this.update(cx, |this, cx| {
                this.setting_value = false;
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
        let Some(option) = self.current_option() else {
            return ContextMenu::build(window, cx, |menu, _, _| menu);
        };

        let current_value = self.current_value();
        let agent_server = self.agent_server.clone();
        let config_id = self.config_id.clone();

        ContextMenu::build(window, cx, move |mut menu, _window, cx| {
            let settings = AgentSettings::get_global(cx);
            let side = match settings.dock {
                settings::DockPosition::Left => DocumentationSide::Right,
                settings::DockPosition::Bottom | settings::DockPosition::Right => {
                    DocumentationSide::Left
                }
            };

            match &option.kind {
                acp::SessionConfigKind::Select(select) => match &select.options {
                    acp::SessionConfigSelectOptions::Ungrouped(options) => {
                        for opt in options {
                            let is_selected = current_value.as_ref() == Some(&opt.value);
                            let default_value =
                                agent_server.default_config_option(&config_id.0, cx);
                            let is_default = default_value.as_deref() == Some(opt.value.0.as_ref());

                            let entry = ContextMenuEntry::new(opt.name.clone())
                                .toggleable(IconPosition::End, is_selected);

                            let entry =
                                entry.documentation_aside(side, DocumentationEdge::Bottom, {
                                    let description = opt.description.clone();
                                    move |_| {
                                        v_flex()
                                            .gap_1()
                                            .when_some(description.clone(), |this, desc| {
                                                this.child(Label::new(desc))
                                            })
                                            .child(HoldForDefault::new(is_default))
                                            .into_any_element()
                                    }
                                });

                            menu.push_item(entry.handler({
                                let value = opt.value.clone();
                                let weak_self = weak_self.clone();
                                let config_id = config_id.clone();
                                move |window, cx| {
                                    weak_self
                                        .update(cx, |this, cx| {
                                            if window.modifiers().secondary() {
                                                this.agent_server.set_default_config_option(
                                                    config_id.0.as_ref(),
                                                    if is_default {
                                                        None
                                                    } else {
                                                        Some(value.0.as_ref())
                                                    },
                                                    this.fs.clone(),
                                                    cx,
                                                );
                                            }

                                            this.set_value(value.clone(), cx);
                                        })
                                        .ok();
                                }
                            }));
                        }
                    }
                    acp::SessionConfigSelectOptions::Grouped(groups) => {
                        for (group_idx, group) in groups.iter().enumerate() {
                            if group_idx > 0 {
                                menu = menu.separator();
                            }
                            menu = menu.header(group.name.clone());

                            for opt in &group.options {
                                let is_selected = current_value.as_ref() == Some(&opt.value);
                                let default_value =
                                    agent_server.default_config_option(config_id.0.as_ref(), cx);
                                let is_default =
                                    default_value.as_deref() == Some(opt.value.0.as_ref());

                                let entry = ContextMenuEntry::new(opt.name.clone())
                                    .toggleable(IconPosition::End, is_selected);

                                let entry =
                                    entry.documentation_aside(side, DocumentationEdge::Bottom, {
                                        let description = opt.description.clone();
                                        move |_| {
                                            v_flex()
                                                .gap_1()
                                                .when_some(description.clone(), |this, desc| {
                                                    this.child(Label::new(desc))
                                                })
                                                .child(HoldForDefault::new(is_default))
                                                .into_any_element()
                                        }
                                    });

                                menu.push_item(entry.handler({
                                    let value = opt.value.clone();
                                    let weak_self = weak_self.clone();
                                    let config_id = config_id.clone();
                                    move |window, cx| {
                                        weak_self
                                            .update(cx, |this, cx| {
                                                if window.modifiers().secondary() {
                                                    this.agent_server.set_default_config_option(
                                                        config_id.0.as_ref(),
                                                        if is_default {
                                                            None
                                                        } else {
                                                            Some(value.0.as_ref())
                                                        },
                                                        this.fs.clone(),
                                                        cx,
                                                    );
                                                }

                                                this.set_value(value.clone(), cx);
                                            })
                                            .ok();
                                    }
                                }));
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }

            menu.key_context("ConfigOptionSelector")
        })
    }

    fn render_trigger_button(&self, _window: &mut Window, _cx: &mut Context<Self>) -> Button {
        let Some(option) = self.current_option() else {
            return Button::new(
                format!("config-option-trigger-{}", &*self.config_id.0),
                "Unknown",
            )
            .label_size(LabelSize::Small)
            .color(Color::Muted)
            .disabled(true);
        };

        let current_value_name = self.current_value_name();

        let icon = if self.use_picker {
            if self.picker_handle.is_deployed() {
                IconName::ChevronUp
            } else {
                IconName::ChevronDown
            }
        } else if self.context_menu_handle.is_deployed() {
            IconName::ChevronUp
        } else {
            IconName::ChevronDown
        };

        Button::new(
            ElementId::Name(format!("config-option-{}-{}", self.config_id, option.id.0).into()),
            current_value_name,
        )
        .label_size(LabelSize::Small)
        .color(Color::Muted)
        .icon(icon)
        .icon_size(IconSize::XSmall)
        .icon_position(IconPosition::End)
        .icon_color(Color::Muted)
        .disabled(self.setting_value)
    }

    fn render_with_picker(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let Some(picker) = self.picker.clone() else {
            return div().into_any_element();
        };

        let Some(option) = self.current_option() else {
            return div().into_any_element();
        };

        let option_name = option.name.clone();
        let option_description = option.description;

        let trigger_button = self.render_trigger_button(window, cx);

        let tooltip = Tooltip::element({
            move |_window, _cx| {
                let mut content = v_flex().gap_1().child(Label::new(option_name.clone()));

                if let Some(ref desc) = option_description {
                    content = content.child(
                        Label::new(desc.clone())
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    );
                }

                content.into_any()
            }
        });

        PickerPopoverMenu::new(
            picker,
            trigger_button,
            tooltip,
            gpui::Corner::BottomRight,
            cx,
        )
        .with_handle(self.picker_handle.clone())
        .render(window, cx)
        .into_any_element()
    }

    fn render_with_context_menu(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(option) = self.current_option() else {
            return div().into_any_element();
        };

        let option_name = option.name.clone();
        let option_description = option.description.clone();

        let this = cx.weak_entity();

        let icon = if self.context_menu_handle.is_deployed() {
            IconName::ChevronUp
        } else {
            IconName::ChevronDown
        };

        let trigger_button = Button::new(
            ElementId::Name(format!("config-option-{}-{}", self.config_id, option.id.0).into()),
            self.current_value_name(),
        )
        .label_size(LabelSize::Small)
        .color(Color::Muted)
        .icon(icon)
        .icon_size(IconSize::XSmall)
        .icon_position(IconPosition::End)
        .icon_color(Color::Muted)
        .disabled(self.setting_value);

        let menu_id = format!("config-option-menu-{}-{}", self.config_id, option.id.0);

        PopoverMenu::new(menu_id)
            .trigger_with_tooltip(
                trigger_button,
                Tooltip::element({
                    move |_window, _cx| {
                        let mut content = v_flex().gap_1().child(Label::new(option_name.clone()));

                        if let Some(desc) = &option_description {
                            content = content.child(
                                Label::new(desc.clone())
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            );
                        }

                        content.into_any()
                    }
                }),
            )
            .anchor(gpui::Corner::BottomRight)
            .with_handle(self.context_menu_handle.clone())
            .offset(gpui::Point {
                x: px(0.0),
                y: px(-2.0),
            })
            .menu(move |window, cx| {
                this.update(cx, |this, cx| this.build_context_menu(window, cx))
                    .ok()
            })
            .into_any_element()
    }
}

impl Render for ConfigOptionSelector {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.current_option().is_none() {
            return div().into_any_element();
        }

        if self.use_picker {
            self.render_with_picker(window, cx)
        } else {
            self.render_with_context_menu(window, cx)
        }
    }
}
