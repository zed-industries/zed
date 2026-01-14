use agent_settings::AgentSettings;
use gpui::{Entity, FocusHandle, Focusable, FontWeight, ReadGlobal, ScrollHandle, prelude::*};
use settings::{Settings as _, SettingsStore, ToolPermissionMode};
use std::sync::Arc;
use ui::{ContextMenu, Disclosure, PopoverMenu, WithScrollbar, prelude::*};

use crate::{SettingsWindow, components::SettingsInputField};

/// Tools that support permission rules
const TOOLS: &[(&str, &str, &str)] = &[
    ("terminal", "Terminal", "Commands executed in the terminal"),
    ("edit_file", "Edit File", "File editing operations"),
    ("delete_path", "Delete Path", "File and directory deletion"),
    ("move_path", "Move Path", "File and directory moves/renames"),
    ("create_directory", "Create Directory", "Directory creation"),
    ("save_file", "Save File", "File saving operations"),
    ("fetch", "Fetch", "HTTP requests to URLs"),
    ("web_search", "Web Search", "Web search queries"),
];

pub struct ToolPermissionsSetupPage {
    #[allow(dead_code)]
    settings_window: Entity<SettingsWindow>,
    scroll_handle: ScrollHandle,
    expanded_tools: Vec<bool>,
    editing_pattern: Option<EditingPattern>,
    focus_handle: FocusHandle,
}

struct EditingPattern {
    tool_name: String,
    rule_type: RuleType,
}

#[derive(Clone, Copy, PartialEq)]
enum RuleType {
    Allow,
    Deny,
    Confirm,
}

impl RuleType {
    fn label(&self) -> &'static str {
        match self {
            RuleType::Allow => "Always Allow",
            RuleType::Deny => "Always Deny",
            RuleType::Confirm => "Always Confirm",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            RuleType::Allow => "Patterns that auto-approve without prompting",
            RuleType::Deny => "Patterns that auto-reject (highest priority)",
            RuleType::Confirm => "Patterns that always require confirmation",
        }
    }
}

/// View model for tool rules (simplified from settings content)
struct ToolRulesView {
    default_mode: ToolPermissionMode,
    always_allow: Vec<String>,
    always_deny: Vec<String>,
    always_confirm: Vec<String>,
}

impl ToolPermissionsSetupPage {
    pub fn new(settings_window: Entity<SettingsWindow>, cx: &mut App) -> Self {
        Self {
            settings_window,
            scroll_handle: ScrollHandle::new(),
            expanded_tools: vec![false; TOOLS.len()],
            editing_pattern: None,
            focus_handle: cx.focus_handle(),
        }
    }

    fn toggle_tool(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.expanded_tools.len() {
            self.expanded_tools[index] = !self.expanded_tools[index];
            cx.notify();
        }
    }

    fn get_tool_rules(&self, tool_name: &str, cx: &App) -> ToolRulesView {
        let settings = AgentSettings::get_global(cx);

        let tool_rules = settings.tool_permissions.tools.get(tool_name);

        match tool_rules {
            Some(rules) => ToolRulesView {
                default_mode: rules.default_mode,
                always_allow: rules
                    .always_allow
                    .iter()
                    .map(|r| r.pattern.clone())
                    .collect(),
                always_deny: rules
                    .always_deny
                    .iter()
                    .map(|r| r.pattern.clone())
                    .collect(),
                always_confirm: rules
                    .always_confirm
                    .iter()
                    .map(|r| r.pattern.clone())
                    .collect(),
            },
            None => ToolRulesView {
                default_mode: ToolPermissionMode::Confirm,
                always_allow: Vec::new(),
                always_deny: Vec::new(),
                always_confirm: Vec::new(),
            },
        }
    }

    fn start_adding_pattern(
        &mut self,
        tool_name: String,
        rule_type: RuleType,
        cx: &mut Context<Self>,
    ) {
        self.editing_pattern = Some(EditingPattern {
            tool_name,
            rule_type,
        });
        cx.notify();
    }

    fn cancel_editing(&mut self, cx: &mut Context<Self>) {
        self.editing_pattern = None;
        cx.notify();
    }

    fn save_pattern(&mut self, pattern: String, cx: &mut Context<Self>) {
        let Some(editing) = self.editing_pattern.take() else {
            return;
        };

        let pattern = pattern.trim().to_string();

        if pattern.is_empty() {
            cx.notify();
            return;
        }

        let tool_name = editing.tool_name;
        let rule_type = editing.rule_type;

        // Update settings via SettingsStore
        SettingsStore::global(cx).update_settings_file(
            <dyn fs::Fs>::global(cx),
            move |settings, _| {
                let tool_permissions = settings
                    .agent
                    .get_or_insert_default()
                    .tool_permissions
                    .get_or_insert_default();
                let tool_rules = tool_permissions
                    .tools
                    .entry(Arc::from(tool_name.as_str()))
                    .or_default();

                let rule = settings::ToolRegexRule {
                    pattern,
                    case_sensitive: None,
                };

                let rules_list = match rule_type {
                    RuleType::Allow => tool_rules.always_allow.get_or_insert_default(),
                    RuleType::Deny => tool_rules.always_deny.get_or_insert_default(),
                    RuleType::Confirm => tool_rules.always_confirm.get_or_insert_default(),
                };

                // Don't add duplicates
                if !rules_list.0.iter().any(|r| r.pattern == rule.pattern) {
                    rules_list.0.push(rule);
                }
            },
        );

        cx.notify();
    }

    fn delete_pattern(
        &mut self,
        tool_name: String,
        rule_type: RuleType,
        pattern: String,
        cx: &mut Context<Self>,
    ) {
        SettingsStore::global(cx).update_settings_file(
            <dyn fs::Fs>::global(cx),
            move |settings, _| {
                let tool_permissions = settings
                    .agent
                    .get_or_insert_default()
                    .tool_permissions
                    .get_or_insert_default();

                if let Some(tool_rules) = tool_permissions.tools.get_mut(tool_name.as_str()) {
                    let rules_list = match rule_type {
                        RuleType::Allow => &mut tool_rules.always_allow,
                        RuleType::Deny => &mut tool_rules.always_deny,
                        RuleType::Confirm => &mut tool_rules.always_confirm,
                    };

                    if let Some(list) = rules_list {
                        list.0.retain(|r| r.pattern != pattern);
                    }
                }
            },
        );

        cx.notify();
    }

    fn set_default_mode(
        &mut self,
        tool_name: String,
        mode: ToolPermissionMode,
        cx: &mut Context<Self>,
    ) {
        SettingsStore::global(cx).update_settings_file(
            <dyn fs::Fs>::global(cx),
            move |settings, _| {
                let tool_permissions = settings
                    .agent
                    .get_or_insert_default()
                    .tool_permissions
                    .get_or_insert_default();
                let tool_rules = tool_permissions
                    .tools
                    .entry(Arc::from(tool_name.as_str()))
                    .or_default();
                tool_rules.default_mode = Some(mode);
            },
        );

        cx.notify();
    }

    fn render_tool_section(
        &self,
        index: usize,
        tool_id: &'static str,
        tool_name: &'static str,
        tool_description: &'static str,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let is_expanded = self.expanded_tools.get(index).copied().unwrap_or(false);
        let rules = self.get_tool_rules(tool_id, cx);

        let has_rules = !rules.always_allow.is_empty()
            || !rules.always_deny.is_empty()
            || !rules.always_confirm.is_empty();

        let rule_count =
            rules.always_allow.len() + rules.always_deny.len() + rules.always_confirm.len();

        v_flex()
            .w_full()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .child(
                h_flex()
                    .id(("tool-row", index))
                    .w_full()
                    .px_4()
                    .py_3()
                    .gap_2()
                    .cursor_pointer()
                    .hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.toggle_tool(index, cx);
                    }))
                    .child(
                        Disclosure::new(("tool-disclosure", index), is_expanded).on_click(
                            cx.listener(move |this, _, _, cx| {
                                this.toggle_tool(index, cx);
                            }),
                        ),
                    )
                    .child(
                        v_flex()
                            .flex_1()
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(Label::new(tool_name).weight(FontWeight::MEDIUM))
                                    .when(has_rules, |this| {
                                        this.child(
                                            Label::new(format!("{} rules", rule_count))
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        )
                                    }),
                            )
                            .child(
                                Label::new(tool_description)
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            ),
                    )
                    .child(self.render_default_mode_dropdown(index, tool_id, &rules, cx)),
            )
            .when(is_expanded, |this| {
                this.child(
                    v_flex()
                        .w_full()
                        .px_4()
                        .pb_4()
                        .gap_4()
                        .child(self.render_rule_section(
                            index,
                            tool_id,
                            RuleType::Allow,
                            &rules,
                            cx,
                        ))
                        .child(self.render_rule_section(index, tool_id, RuleType::Deny, &rules, cx))
                        .child(self.render_rule_section(
                            index,
                            tool_id,
                            RuleType::Confirm,
                            &rules,
                            cx,
                        )),
                )
            })
            .into_any_element()
    }

    fn render_default_mode_dropdown(
        &self,
        index: usize,
        tool_id: &'static str,
        rules: &ToolRulesView,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let current_mode = rules.default_mode;
        let mode_label = match current_mode {
            ToolPermissionMode::Allow => "Allow",
            ToolPermissionMode::Deny => "Deny",
            ToolPermissionMode::Confirm => "Confirm",
        };

        let tool_id_owned = tool_id.to_string();
        let weak_self = cx.weak_entity();

        PopoverMenu::new(("default-mode", index))
            .trigger(
                Button::new(("mode-trigger", index), mode_label)
                    .style(ButtonStyle::Subtle)
                    .icon(IconName::ChevronDown)
                    .icon_position(IconPosition::End)
                    .icon_size(IconSize::XSmall)
                    .label_size(LabelSize::Small),
            )
            .menu(move |window, cx| {
                let tool_id = tool_id_owned.clone();
                let weak_self = weak_self.clone();
                Some(ContextMenu::build(window, cx, move |menu, _, _| {
                    let tool_id_allow = tool_id.clone();
                    let tool_id_confirm = tool_id.clone();
                    let tool_id_deny = tool_id;
                    let weak_self_allow = weak_self.clone();
                    let weak_self_confirm = weak_self.clone();
                    let weak_self_deny = weak_self;

                    menu.entry("Confirm (default)", None, move |_, cx| {
                        weak_self_confirm
                            .update(cx, |this, cx| {
                                this.set_default_mode(
                                    tool_id_confirm.clone(),
                                    ToolPermissionMode::Confirm,
                                    cx,
                                );
                            })
                            .ok();
                    })
                    .entry("Allow", None, move |_, cx| {
                        weak_self_allow
                            .update(cx, |this, cx| {
                                this.set_default_mode(
                                    tool_id_allow.clone(),
                                    ToolPermissionMode::Allow,
                                    cx,
                                );
                            })
                            .ok();
                    })
                    .entry("Deny", None, move |_, cx| {
                        weak_self_deny
                            .update(cx, |this, cx| {
                                this.set_default_mode(
                                    tool_id_deny.clone(),
                                    ToolPermissionMode::Deny,
                                    cx,
                                );
                            })
                            .ok();
                    })
                }))
            })
            .anchor(gpui::Corner::TopRight)
            .into_any_element()
    }

    fn render_rule_section(
        &self,
        tool_index: usize,
        tool_id: &'static str,
        rule_type: RuleType,
        rules: &ToolRulesView,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let patterns: Vec<String> = match rule_type {
            RuleType::Allow => rules.always_allow.clone(),
            RuleType::Deny => rules.always_deny.clone(),
            RuleType::Confirm => rules.always_confirm.clone(),
        };

        let is_editing = self
            .editing_pattern
            .as_ref()
            .is_some_and(|e| e.tool_name == tool_id && e.rule_type == rule_type);

        let color = match rule_type {
            RuleType::Allow => Color::Success,
            RuleType::Deny => Color::Error,
            RuleType::Confirm => Color::Warning,
        };

        let rule_type_index = match rule_type {
            RuleType::Allow => 0usize,
            RuleType::Deny => 1,
            RuleType::Confirm => 2,
        };
        let section_id = tool_index * 3 + rule_type_index;

        v_flex()
            .gap_2()
            .child(
                h_flex()
                    .justify_between()
                    .child(
                        v_flex()
                            .child(
                                Label::new(rule_type.label())
                                    .size(LabelSize::Small)
                                    .weight(FontWeight::MEDIUM)
                                    .color(color),
                            )
                            .child(
                                Label::new(rule_type.description())
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted),
                            ),
                    )
                    .when(!is_editing, |this| {
                        let tool_id = tool_id.to_string();
                        this.child(
                            Button::new(("add-pattern", section_id), "Add Pattern")
                                .style(ButtonStyle::Subtle)
                                .icon(IconName::Plus)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::XSmall)
                                .label_size(LabelSize::Small)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.start_adding_pattern(tool_id.clone(), rule_type, cx);
                                })),
                        )
                    }),
            )
            .child(
                v_flex()
                    .w_full()
                    .rounded_md()
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .bg(cx.theme().colors().editor_background)
                    .p_2()
                    .gap_1()
                    .when(patterns.is_empty() && !is_editing, |this| {
                        this.child(
                            Label::new("No patterns configured")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                    })
                    .children(patterns.iter().enumerate().map(|(pattern_index, pattern)| {
                        let tool_id = tool_id.to_string();
                        let pattern_clone = pattern.clone();
                        let delete_id = section_id * 100 + pattern_index;
                        h_flex()
                            .w_full()
                            .justify_between()
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
                            .child(
                                Label::new(pattern.clone())
                                    .size(LabelSize::Small)
                                    .single_line(),
                            )
                            .child(
                                IconButton::new(("delete-pattern", delete_id), IconName::Trash)
                                    .icon_size(IconSize::Small)
                                    .icon_color(Color::Muted)
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.delete_pattern(
                                            tool_id.clone(),
                                            rule_type,
                                            pattern_clone.clone(),
                                            cx,
                                        );
                                    })),
                            )
                    }))
                    .when(is_editing, |this| this.child(self.render_pattern_input(cx))),
            )
            .into_any_element()
    }

    fn render_pattern_input(&self, cx: &mut Context<Self>) -> AnyElement {
        let weak_self = cx.weak_entity();
        let weak_self_cancel = weak_self.clone();

        h_flex()
            .w_full()
            .gap_2()
            .p_1()
            .child(
                SettingsInputField::new()
                    .with_placeholder("Enter regex pattern...")
                    .tab_index(0)
                    .on_confirm({
                        move |pattern, cx| {
                            if let Some(pattern) = pattern {
                                weak_self
                                    .update(cx, |this, cx| {
                                        this.save_pattern(pattern, cx);
                                    })
                                    .ok();
                            }
                        }
                    }),
            )
            .child(
                Button::new("cancel-pattern", "Cancel")
                    .style(ButtonStyle::Subtle)
                    .label_size(LabelSize::Small)
                    .on_click(move |_, _, cx| {
                        weak_self_cancel
                            .update(cx, |this, cx| {
                                this.cancel_editing(cx);
                            })
                            .ok();
                    }),
            )
            .into_any_element()
    }
}

impl Focusable for ToolPermissionsSetupPage {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ToolPermissionsSetupPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Build tool sections first to avoid closure capture issues
        let mut tool_sections = Vec::new();
        for (index, (id, name, desc)) in TOOLS.iter().enumerate() {
            tool_sections.push(self.render_tool_section(index, id, name, desc, cx));
        }

        div()
            .size_full()
            .track_focus(&self.focus_handle)
            .vertical_scrollbar_for(&self.scroll_handle, window, cx)
            .child(
                v_flex()
                    .id("tool-permissions-page")
                    .min_w_0()
                    .size_full()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .child(
                        v_flex()
                            .px_4()
                            .py_3()
                            .gap_1()
                            .child(
                                Label::new("Tool Permission Rules")
                                    .size(LabelSize::Large)
                                    .weight(FontWeight::SEMIBOLD),
                            )
                            .child(
                                Label::new(
                                    "Configure regex patterns to control which tool actions require confirmation.",
                                )
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                            ),
                    )
                    .children(tool_sections),
            )
    }
}
