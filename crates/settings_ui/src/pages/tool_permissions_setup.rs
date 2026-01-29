use agent_settings::AgentSettings;
use gpui::{FontWeight, ReadGlobal, ScrollHandle, prelude::*};
use settings::{Settings as _, SettingsStore, ToolPermissionMode};
use std::sync::Arc;
use ui::{ContextMenu, PopoverMenu, prelude::*};

use crate::{
    SettingsWindow, USER,
    components::{SettingsInputField, SettingsSectionHeader},
};

/// Tools that support permission rules
const TOOLS: &[ToolInfo] = &[
    ToolInfo {
        id: "terminal",
        name: "Terminal",
        description: "Commands executed in the terminal",
    },
    ToolInfo {
        id: "edit_file",
        name: "Edit File",
        description: "File editing operations",
    },
    ToolInfo {
        id: "delete_path",
        name: "Delete Path",
        description: "File and directory deletion",
    },
    ToolInfo {
        id: "move_path",
        name: "Move Path",
        description: "File and directory moves/renames",
    },
    ToolInfo {
        id: "create_directory",
        name: "Create Directory",
        description: "Directory creation",
    },
    ToolInfo {
        id: "save_file",
        name: "Save File",
        description: "File saving operations",
    },
    ToolInfo {
        id: "fetch",
        name: "Fetch",
        description: "HTTP requests to URLs",
    },
    ToolInfo {
        id: "web_search",
        name: "Web Search",
        description: "Web search queries",
    },
];

struct ToolInfo {
    id: &'static str,
    name: &'static str,
    description: &'static str,
}

/// Renders the main tool permissions setup page showing a list of tools
pub(crate) fn render_tool_permissions_setup_page(
    settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let tool_items: Vec<AnyElement> = TOOLS
        .iter()
        .map(|tool| render_tool_list_item(settings_window, tool, window, cx))
        .collect();

    div()
        .size_full()
        .child(
            v_flex()
                .id("tool-permissions-page")
                .min_w_0()
                .size_full()
                .px_8()
                .pb_16()
                .overflow_y_scroll()
                .track_scroll(scroll_handle)
                .child(
                    v_flex()
                        .pt_8()
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
                .child(v_flex().mt_4().gap_0().children(tool_items)),
        )
        .into_any_element()
}

fn render_tool_list_item(
    _settings_window: &SettingsWindow,
    tool: &'static ToolInfo,
    _window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let rules = get_tool_rules(tool.id, cx);
    let rule_count =
        rules.always_allow.len() + rules.always_deny.len() + rules.always_confirm.len();

    let rule_summary = if rule_count > 0 {
        Some(format!("{} rules", rule_count))
    } else {
        None
    };

    let render_fn = get_tool_render_fn(tool.id);

    v_flex()
        .w_full()
        .border_b_1()
        .border_color(cx.theme().colors().border_variant)
        .child(
            h_flex()
                .id(tool.id)
                .w_full()
                .py_3()
                .gap_2()
                .cursor_pointer()
                .hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
                .on_click({
                    let tool_name = tool.name;
                    cx.listener(move |this, _, window, cx| {
                        this.push_dynamic_sub_page(
                            tool_name,
                            "Configure Tool Rules",
                            None,
                            render_fn,
                            window,
                            cx,
                        );
                    })
                })
                .child(
                    v_flex()
                        .flex_1()
                        .child(
                            h_flex()
                                .gap_2()
                                .child(Label::new(tool.name).weight(FontWeight::MEDIUM))
                                .when_some(rule_summary, |this, summary| {
                                    this.child(
                                        Label::new(summary)
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    )
                                }),
                        )
                        .child(
                            Label::new(tool.description)
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                )
                .child({
                    let tool_name = tool.name;
                    Button::new(format!("configure-{}", tool.id), "Configure")
                        .icon(IconName::ChevronRight)
                        .icon_position(IconPosition::End)
                        .icon_color(Color::Muted)
                        .icon_size(IconSize::Small)
                        .style(ButtonStyle::OutlinedGhost)
                        .size(ButtonSize::Medium)
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.push_dynamic_sub_page(
                                tool_name,
                                "Configure Tool Rules",
                                None,
                                render_fn,
                                window,
                                cx,
                            );
                        }))
                }),
        )
        .into_any_element()
}

fn get_tool_render_fn(
    tool_id: &str,
) -> fn(&SettingsWindow, &ScrollHandle, &mut Window, &mut Context<SettingsWindow>) -> AnyElement {
    match tool_id {
        "terminal" => render_terminal_tool_config,
        "edit_file" => render_edit_file_tool_config,
        "delete_path" => render_delete_path_tool_config,
        "move_path" => render_move_path_tool_config,
        "create_directory" => render_create_directory_tool_config,
        "save_file" => render_save_file_tool_config,
        "fetch" => render_fetch_tool_config,
        "web_search" => render_web_search_tool_config,
        _ => render_terminal_tool_config, // fallback
    }
}

/// Renders an individual tool's permission configuration page
pub(crate) fn render_tool_config_page(
    tool_id: &'static str,
    _settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    _window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let tool = TOOLS.iter().find(|t| t.id == tool_id).unwrap();
    let rules = get_tool_rules(tool_id, cx);

    div()
        .size_full()
        .child(
            v_flex()
                .id(format!("tool-config-page-{}", tool_id))
                .min_w_0()
                .size_full()
                .px_8()
                .pb_16()
                .overflow_y_scroll()
                .track_scroll(scroll_handle)
                .child(
                    v_flex()
                        .pt_8()
                        .gap_1()
                        .child(SettingsSectionHeader::new(tool.name).no_padding(true))
                        .child(
                            Label::new(tool.description)
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                )
                .child(render_rule_section(
                    tool_id,
                    "Always Allow",
                    "Patterns that auto-approve without prompting",
                    Color::Success,
                    RuleType::Allow,
                    &rules.always_allow,
                    cx,
                ))
                .child(render_rule_section(
                    tool_id,
                    "Always Deny",
                    "Patterns that auto-reject (highest priority)",
                    Color::Error,
                    RuleType::Deny,
                    &rules.always_deny,
                    cx,
                ))
                .child(render_rule_section(
                    tool_id,
                    "Always Confirm",
                    "Patterns that always require confirmation",
                    Color::Warning,
                    RuleType::Confirm,
                    &rules.always_confirm,
                    cx,
                ))
                .child(render_default_mode_section(tool_id, rules.default_mode, cx)),
        )
        .into_any_element()
}

fn render_rule_section(
    tool_id: &'static str,
    title: &'static str,
    description: &'static str,
    color: Color,
    rule_type: RuleType,
    patterns: &[String],
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    v_flex()
        .mt_6()
        .gap_2()
        .child(
            h_flex().justify_between().child(
                v_flex()
                    .child(
                        Label::new(title)
                            .size(LabelSize::Default)
                            .weight(FontWeight::MEDIUM)
                            .color(color),
                    )
                    .child(
                        Label::new(description)
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            ),
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
                .when(patterns.is_empty(), |this| {
                    this.child(
                        Label::new("No patterns configured")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                })
                .children(patterns.iter().enumerate().map(|(index, pattern)| {
                    let pattern_clone = pattern.clone();
                    let tool_id_owned = tool_id.to_string();
                    let delete_id = format!("{}-{:?}-{}", tool_id, rule_type, index);
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
                            IconButton::new(delete_id, IconName::Trash)
                                .icon_size(IconSize::Small)
                                .icon_color(Color::Muted)
                                .on_click(cx.listener(move |_, _, _, cx| {
                                    delete_pattern(&tool_id_owned, rule_type, &pattern_clone, cx);
                                })),
                        )
                }))
                .child(render_add_pattern_input(tool_id, rule_type, cx)),
        )
        .into_any_element()
}

fn render_add_pattern_input(
    tool_id: &'static str,
    rule_type: RuleType,
    _cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let tool_id_owned = tool_id.to_string();

    h_flex()
        .w_full()
        .gap_2()
        .pt_2()
        .child(
            SettingsInputField::new()
                .with_placeholder("Enter regex pattern and press Enter...")
                .tab_index(0)
                .on_confirm(move |pattern, _window, cx| {
                    if let Some(pattern) = pattern {
                        if !pattern.trim().is_empty() {
                            save_pattern(&tool_id_owned, rule_type, pattern.trim().to_string(), cx);
                        }
                    }
                }),
        )
        .into_any_element()
}

fn render_default_mode_section(
    tool_id: &'static str,
    current_mode: ToolPermissionMode,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let mode_label = match current_mode {
        ToolPermissionMode::Allow => "Allow",
        ToolPermissionMode::Deny => "Deny",
        ToolPermissionMode::Confirm => "Confirm",
    };

    let tool_id_owned = tool_id.to_string();

    v_flex()
        .mt_8()
        .pt_4()
        .border_t_1()
        .border_color(cx.theme().colors().border_variant)
        .gap_2()
        .child(
            v_flex()
                .child(
                    Label::new("Default Action")
                        .size(LabelSize::Default)
                        .weight(FontWeight::MEDIUM),
                )
                .child(
                    Label::new("Action to take when no patterns match")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                ),
        )
        .child(
            h_flex()
                .w_full()
                .justify_between()
                .child(Label::new("When no patterns match:").size(LabelSize::Small))
                .child(
                    PopoverMenu::new(format!("default-mode-{}", tool_id))
                        .trigger(
                            Button::new(format!("mode-trigger-{}", tool_id), mode_label)
                                .style(ButtonStyle::Subtle)
                                .icon(IconName::ChevronDown)
                                .icon_position(IconPosition::End)
                                .icon_size(IconSize::XSmall)
                                .label_size(LabelSize::Small),
                        )
                        .menu(move |window, cx| {
                            let tool_id = tool_id_owned.clone();
                            Some(ContextMenu::build(window, cx, move |menu, _, _| {
                                let tool_id_confirm = tool_id.clone();
                                let tool_id_allow = tool_id.clone();
                                let tool_id_deny = tool_id;

                                menu.entry("Confirm", None, move |_, cx| {
                                    set_default_mode(
                                        &tool_id_confirm,
                                        ToolPermissionMode::Confirm,
                                        cx,
                                    );
                                })
                                .entry("Allow", None, move |_, cx| {
                                    set_default_mode(&tool_id_allow, ToolPermissionMode::Allow, cx);
                                })
                                .entry(
                                    "Deny",
                                    None,
                                    move |_, cx| {
                                        set_default_mode(
                                            &tool_id_deny,
                                            ToolPermissionMode::Deny,
                                            cx,
                                        );
                                    },
                                )
                            }))
                        })
                        .anchor(gpui::Corner::TopRight),
                ),
        )
        .into_any_element()
}

// Helper types and functions

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum RuleType {
    Allow,
    Deny,
    Confirm,
}

struct ToolRulesView {
    default_mode: ToolPermissionMode,
    always_allow: Vec<String>,
    always_deny: Vec<String>,
    always_confirm: Vec<String>,
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

fn save_pattern(tool_name: &str, rule_type: RuleType, pattern: String, cx: &mut App) {
    let tool_name = tool_name.to_string();

    SettingsStore::global(cx).update_settings_file(<dyn fs::Fs>::global(cx), move |settings, _| {
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
    });
}

fn delete_pattern(tool_name: &str, rule_type: RuleType, pattern: &str, cx: &mut App) {
    let tool_name = tool_name.to_string();
    let pattern = pattern.to_string();

    SettingsStore::global(cx).update_settings_file(<dyn fs::Fs>::global(cx), move |settings, _| {
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
    });
}

fn set_default_mode(tool_name: &str, mode: ToolPermissionMode, cx: &mut App) {
    let tool_name = tool_name.to_string();

    SettingsStore::global(cx).update_settings_file(<dyn fs::Fs>::global(cx), move |settings, _| {
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
    });
}

// Macro to generate render functions for each tool
macro_rules! tool_config_page_fn {
    ($fn_name:ident, $tool_id:literal) => {
        pub fn $fn_name(
            settings_window: &SettingsWindow,
            scroll_handle: &ScrollHandle,
            window: &mut Window,
            cx: &mut Context<SettingsWindow>,
        ) -> AnyElement {
            render_tool_config_page($tool_id, settings_window, scroll_handle, window, cx)
        }
    };
}

tool_config_page_fn!(render_terminal_tool_config, "terminal");
tool_config_page_fn!(render_edit_file_tool_config, "edit_file");
tool_config_page_fn!(render_delete_path_tool_config, "delete_path");
tool_config_page_fn!(render_move_path_tool_config, "move_path");
tool_config_page_fn!(render_create_directory_tool_config, "create_directory");
tool_config_page_fn!(render_save_file_tool_config, "save_file");
tool_config_page_fn!(render_fetch_tool_config, "fetch");
tool_config_page_fn!(render_web_search_tool_config, "web_search");
