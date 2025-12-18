use acp_thread::AgentSessionConfigOptions;
use agent_client_protocol as acp;
use gpui::{Context, Entity, FocusHandle, WeakEntity, Window, prelude::*};
use std::rc::Rc;
use ui::{
    Button, ContextMenu, ContextMenuEntry, DocumentationEdge, DocumentationSide, PopoverMenu,
    PopoverMenuHandle, Tooltip, prelude::*,
};

pub struct ConfigOptionSelector {
    config_options: Rc<dyn AgentSessionConfigOptions>,
    config_id: acp::SessionConfigId,
    menu_handle: PopoverMenuHandle<ContextMenu>,
    #[allow(dead_code)]
    focus_handle: FocusHandle,
    setting_value: bool,
}

impl ConfigOptionSelector {
    pub fn new(
        config_options: Rc<dyn AgentSessionConfigOptions>,
        config_id: acp::SessionConfigId,
        focus_handle: FocusHandle,
    ) -> Self {
        Self {
            config_options,
            config_id,
            menu_handle: PopoverMenuHandle::default(),
            focus_handle,
            setting_value: false,
        }
    }

    pub fn menu_handle(&self) -> PopoverMenuHandle<ContextMenu> {
        self.menu_handle.clone()
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

        ContextMenu::build(window, cx, move |mut menu, _window, _cx| {
            let side = DocumentationSide::Left;

            match &option.kind {
                acp::SessionConfigKind::Select(select) => match &select.options {
                    acp::SessionConfigSelectOptions::Ungrouped(options) => {
                        for opt in options {
                            let is_selected = current_value.as_ref() == Some(&opt.value);
                            let entry = ContextMenuEntry::new(opt.name.clone())
                                .toggleable(IconPosition::End, is_selected);

                            let entry = if let Some(description) = &opt.description {
                                entry.documentation_aside(side, DocumentationEdge::Bottom, {
                                    let description = description.clone();
                                    move |_| Label::new(description.clone()).into_any_element()
                                })
                            } else {
                                entry
                            };

                            menu.push_item(entry.handler({
                                let value = opt.value.clone();
                                let weak_self = weak_self.clone();
                                move |_window, cx| {
                                    weak_self
                                        .update(cx, |this, cx| {
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
                                let entry = ContextMenuEntry::new(opt.name.clone())
                                    .toggleable(IconPosition::End, is_selected);

                                let entry = if let Some(description) = &opt.description {
                                    entry.documentation_aside(side, DocumentationEdge::Bottom, {
                                        let description = description.clone();
                                        move |_| Label::new(description.clone()).into_any_element()
                                    })
                                } else {
                                    entry
                                };

                                menu.push_item(entry.handler({
                                    let value = opt.value.clone();
                                    let weak_self = weak_self.clone();
                                    move |_window, cx| {
                                        weak_self
                                            .update(cx, |this, cx| {
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
}

impl Render for ConfigOptionSelector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(option) = self.current_option() else {
            return div().into_any_element();
        };

        let current_value_name = self.current_value_name();
        let option_name = option.name.clone();
        let option_description = option.description.clone();

        let this = cx.weak_entity();

        let icon = if self.menu_handle.is_deployed() {
            IconName::ChevronUp
        } else {
            IconName::ChevronDown
        };

        let trigger_button = Button::new(
            ElementId::Name(format!("config-option-{}", option.id.0).into()),
            current_value_name,
        )
        .label_size(LabelSize::Small)
        .color(Color::Muted)
        .icon(icon)
        .icon_size(IconSize::XSmall)
        .icon_position(IconPosition::End)
        .icon_color(Color::Muted)
        .disabled(self.setting_value);

        let menu_id = format!("config-option-menu-{}", option.id.0);

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
            .with_handle(self.menu_handle.clone())
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
