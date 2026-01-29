use agent_settings::AgentSettings;
use gpui::{
    Entity, FocusHandle, Focusable, FontWeight, ReadGlobal, ScrollHandle, WeakEntity, prelude::*,
};
use settings::{Settings as _, SettingsStore, ToolPermissionMode};
use std::sync::Arc;
use ui::{ContextMenu, Disclosure, PopoverMenu, prelude::*};

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

pub(crate) fn render_tool_permissions_setup_page(
    _settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let settings_window_entity = cx.entity().clone();
    let page: Entity<ToolPermissionsSetupPage> = window.use_state(cx, |_, cx| {
        ToolPermissionsSetupPage::new(settings_window_entity, cx)
    });
    let scroll_handle = scroll_handle.clone();

    ToolPermissionsSetupPageView {
        page,
        scroll_handle,
    }
    .into_any_element()
}

struct ToolPermissionsSetupPageView {
    page: Entity<ToolPermissionsSetupPage>,
    scroll_handle: ScrollHandle,
}

impl IntoElement for ToolPermissionsSetupPageView {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        self.into_any_element()
    }
}

impl RenderOnce for ToolPermissionsSetupPageView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let page = self.page.read(cx);
        let scroll_handle = self.scroll_handle;
        let focus_handle = page.focus_handle.clone();
        let expanded_tools = page.expanded_tools.clone();
        let editing_pattern = page.editing_pattern.clone();
        let weak_page = self.page.downgrade();

        let mut tool_sections = Vec::new();
        for (index, (id, name, desc)) in TOOLS.iter().enumerate() {
            tool_sections.push(render_tool_section(
                weak_page.clone(),
                index,
                id,
                name,
                desc,
                expanded_tools.get(index).copied().unwrap_or(false),
                editing_pattern.clone(),
                cx,
            ));
        }

        div()
            .size_full()
            .track_focus(&focus_handle)
            .child(
                v_flex()
                    .id("tool-permissions-page")
                    .min_w_0()
                    .size_full()
                    .overflow_y_scroll()
                    .track_scroll(&scroll_handle)
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

pub struct ToolPermissionsSetupPage {
    #[allow(dead_code)]
    settings_window: Entity<SettingsWindow>,
    expanded_tools: Vec<bool>,
    editing_pattern: Option<EditingPattern>,
    focus_handle: FocusHandle,
}

#[derive(Clone)]
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

struct ToolRulesView {
    default_mode: ToolPermissionMode,
    always_allow: Vec<String>,
    always_deny: Vec<String>,
    always_confirm: Vec<String>,
}

impl ToolPermissionsSetupPage {
    fn new(settings_window: Entity<SettingsWindow>, cx: &mut App) -> Self {
        Self {
            settings_window,
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
}

impl Focusable for ToolPermissionsSetupPage {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

fn get_tool_rules(tool_name: &str, cx: &App) -> ToolRulesView {
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

fn render_tool_section(
    weak_page: WeakEntity<ToolPermissionsSetupPage>,
    index: usize,
    tool_id: &'static str,
    tool_name: &'static str,
    tool_description: &'static str,
    is_expanded: bool,
    editing_pattern: Option<EditingPattern>,
    cx: &App,
) -> AnyElement {
    let rules = get_tool_rules(tool_id, cx);

    let has_rules = !rules.always_allow.is_empty()
        || !rules.always_deny.is_empty()
        || !rules.always_confirm.is_empty();

    let rule_count =
        rules.always_allow.len() + rules.always_deny.len() + rules.always_confirm.len();

    let weak_page_toggle = weak_page.clone();
    let weak_page_disclosure = weak_page.clone();

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
                .on_click(move |_, _, cx| {
                    weak_page_toggle
                        .update(cx, |this: &mut ToolPermissionsSetupPage, cx| {
                            this.toggle_tool(index, cx);
                        })
                        .ok();
                })
                .child(
                    Disclosure::new(("tool-disclosure", index), is_expanded).on_click(
                        move |_, _, cx| {
                            weak_page_disclosure
                                .update(cx, |this: &mut ToolPermissionsSetupPage, cx| {
                                    this.toggle_tool(index, cx);
                                })
                                .ok();
                        },
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
                .child(render_default_mode_dropdown(
                    weak_page.clone(),
                    index,
                    tool_id,
                    &rules,
                )),
        )
        .when(is_expanded, |this| {
            this.child(
                v_flex()
                    .w_full()
                    .px_4()
                    .pb_4()
                    .gap_4()
                    .child(render_rule_section(
                        weak_page.clone(),
                        index,
                        tool_id,
                        RuleType::Allow,
                        &rules,
                        editing_pattern.clone(),
                        cx,
                    ))
                    .child(render_rule_section(
                        weak_page.clone(),
                        index,
                        tool_id,
                        RuleType::Deny,
                        &rules,
                        editing_pattern.clone(),
                        cx,
                    ))
                    .child(render_rule_section(
                        weak_page,
                        index,
                        tool_id,
                        RuleType::Confirm,
                        &rules,
                        editing_pattern,
                        cx,
                    )),
            )
        })
        .into_any_element()
}

fn render_default_mode_dropdown(
    weak_page: WeakEntity<ToolPermissionsSetupPage>,
    index: usize,
    tool_id: &'static str,
    rules: &ToolRulesView,
) -> AnyElement {
    let current_mode = rules.default_mode;
    let mode_label = match current_mode {
        ToolPermissionMode::Allow => "Allow",
        ToolPermissionMode::Deny => "Deny",
        ToolPermissionMode::Confirm => "Confirm",
    };

    let tool_id_owned = tool_id.to_string();

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
            let weak_page = weak_page.clone();
            Some(ContextMenu::build(window, cx, move |menu, _, _| {
                let tool_id_allow = tool_id.clone();
                let tool_id_confirm = tool_id.clone();
                let tool_id_deny = tool_id;
                let weak_page_allow = weak_page.clone();
                let weak_page_confirm = weak_page.clone();
                let weak_page_deny = weak_page;

                menu.entry("Confirm (default)", None, move |_, cx| {
                    weak_page_confirm
                        .update(cx, |this: &mut ToolPermissionsSetupPage, cx| {
                            this.set_default_mode(
                                tool_id_confirm.clone(),
                                ToolPermissionMode::Confirm,
                                cx,
                            );
                        })
                        .ok();
                })
                .entry("Allow", None, move |_, cx| {
                    weak_page_allow
                        .update(cx, |this: &mut ToolPermissionsSetupPage, cx| {
                            this.set_default_mode(
                                tool_id_allow.clone(),
                                ToolPermissionMode::Allow,
                                cx,
                            );
                        })
                        .ok();
                })
                .entry("Deny", None, move |_, cx| {
                    weak_page_deny
                        .update(cx, |this: &mut ToolPermissionsSetupPage, cx| {
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
    weak_page: WeakEntity<ToolPermissionsSetupPage>,
    tool_index: usize,
    tool_id: &'static str,
    rule_type: RuleType,
    rules: &ToolRulesView,
    editing_pattern: Option<EditingPattern>,
    cx: &App,
) -> AnyElement {
    let patterns: Vec<String> = match rule_type {
        RuleType::Allow => rules.always_allow.clone(),
        RuleType::Deny => rules.always_deny.clone(),
        RuleType::Confirm => rules.always_confirm.clone(),
    };

    let is_editing = editing_pattern
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
                .when(!is_editing, {
                    let tool_id = tool_id.to_string();
                    let weak_page = weak_page.clone();
                    move |this| {
                        this.child(
                            Button::new(("add-pattern", section_id), "Add Pattern")
                                .style(ButtonStyle::Subtle)
                                .icon(IconName::Plus)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::XSmall)
                                .label_size(LabelSize::Small)
                                .on_click({
                                    let tool_id = tool_id.clone();
                                    let weak_page = weak_page.clone();
                                    move |_, _, cx| {
                                        weak_page
                                            .update(
                                                cx,
                                                |this: &mut ToolPermissionsSetupPage, cx| {
                                                    this.start_adding_pattern(
                                                        tool_id.clone(),
                                                        rule_type,
                                                        cx,
                                                    );
                                                },
                                            )
                                            .ok();
                                    }
                                }),
                        )
                    }
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
                    let weak_page = weak_page.clone();
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
                                .on_click(move |_, _, cx| {
                                    weak_page
                                        .update(cx, |this: &mut ToolPermissionsSetupPage, cx| {
                                            this.delete_pattern(
                                                tool_id.clone(),
                                                rule_type,
                                                pattern_clone.clone(),
                                                cx,
                                            );
                                        })
                                        .ok();
                                }),
                        )
                }))
                .when(is_editing, |this| {
                    this.child(render_pattern_input(weak_page.clone()))
                }),
        )
        .into_any_element()
}

fn render_pattern_input(weak_page: WeakEntity<ToolPermissionsSetupPage>) -> AnyElement {
    let weak_page_save = weak_page.clone();
    let weak_page_cancel = weak_page;

    h_flex()
        .w_full()
        .gap_2()
        .p_1()
        .child(
            SettingsInputField::new()
                .with_placeholder("Enter regex pattern...")
                .tab_index(0)
                .on_confirm({
                    move |pattern, _window, cx| {
                        if let Some(pattern) = pattern {
                            weak_page_save
                                .update(cx, |this: &mut ToolPermissionsSetupPage, cx| {
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
                    weak_page_cancel
                        .update(cx, |this: &mut ToolPermissionsSetupPage, cx| {
                            this.cancel_editing(cx);
                        })
                        .ok();
                }),
        )
        .into_any_element()
}
