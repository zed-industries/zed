use agent::ToolPermissionDecision;
use agent_settings::AgentSettings;
use gpui::{ReadGlobal, ScrollHandle, prelude::*};
use settings::{Settings as _, SettingsStore, ToolPermissionMode};
use std::sync::Arc;
use ui::{ContextMenu, Divider, PopoverMenu, Tooltip, prelude::*};
use util::shell::ShellKind;

use crate::{SettingsWindow, components::SettingsInputField};

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

    let page_description =
        "Configure regex patterns to control which tool actions require confirmation.";

    v_flex()
        .id("tool-permissions-page")
        .min_w_0()
        .size_full()
        .pt_2p5()
        .px_8()
        .pb_16()
        .overflow_y_scroll()
        .track_scroll(scroll_handle)
        .child(Label::new("Tool Permission Rules").size(LabelSize::Large))
        .child(
            Label::new(page_description)
                .size(LabelSize::Small)
                .color(Color::Muted),
        )
        .child(
            v_flex()
                .mt_4()
                .children(tool_items.into_iter().enumerate().flat_map(|(i, item)| {
                    let mut elements: Vec<AnyElement> = vec![item];
                    if i + 1 < TOOLS.len() {
                        elements.push(Divider::horizontal().into_any_element());
                    }
                    elements
                })),
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

    h_flex()
        .w_full()
        .py_3()
        .justify_between()
        .child(
            v_flex()
                .child(h_flex().gap_1().child(Label::new(tool.name)).when_some(
                    rule_summary,
                    |this, summary| {
                        this.child(
                            Label::new(summary)
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                    },
                ))
                .child(
                    Label::new(tool.description)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                ),
        )
        .child({
            let tool_name = tool.name;
            Button::new(format!("configure-{}", tool.id), "Configure")
                .style(ButtonStyle::OutlinedGhost)
                .size(ButtonSize::Medium)
                .icon(IconName::ChevronRight)
                .icon_position(IconPosition::End)
                .icon_color(Color::Muted)
                .icon_size(IconSize::Small)
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
        })
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
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let tool = TOOLS.iter().find(|t| t.id == tool_id).unwrap();
    let rules = get_tool_rules(tool_id, cx);
    let page_title = format!("{} Tool", tool.name);

    v_flex()
        .id(format!("tool-config-page-{}", tool_id))
        .min_w_0()
        .size_full()
        .pt_2p5()
        .px_8()
        .pb_16()
        .overflow_y_scroll()
        .track_scroll(scroll_handle)
        .child(Label::new(page_title).size(LabelSize::Large))
        .child(render_verification_section(tool_id, window, cx))
        .child(
            v_flex()
                .mt_6()
                .gap_5()
                .child(render_default_mode_section(tool_id, rules.default_mode, cx))
                .child(Divider::horizontal().color(ui::DividerColor::BorderFaded))
                .child(render_rule_section(
                    tool_id,
                    "Always Deny",
                    "Patterns that auto-reject (highest priority).",
                    RuleType::Deny,
                    &rules.always_deny,
                    cx,
                ))
                .child(Divider::horizontal().color(ui::DividerColor::BorderFaded))
                .child(render_rule_section(
                    tool_id,
                    "Always Allow",
                    "Patterns that auto-approve without prompting.",
                    RuleType::Allow,
                    &rules.always_allow,
                    cx,
                ))
                .child(Divider::horizontal().color(ui::DividerColor::BorderFaded))
                .child(render_rule_section(
                    tool_id,
                    "Always Confirm",
                    "Patterns that always require confirmation.",
                    RuleType::Confirm,
                    &rules.always_confirm,
                    cx,
                )),
        )
        .into_any_element()
}

fn render_verification_section(
    tool_id: &'static str,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let input_id = format!("{}-verification-input", tool_id);

    let settings = AgentSettings::get_global(cx);
    let always_allow_enabled = settings.always_allow_tool_actions;

    let editor = window.use_keyed_state(input_id.clone(), cx, |window, cx| {
        let mut editor = editor::Editor::single_line(window, cx);
        editor.set_placeholder_text("Enter a test input to see how rules apply...", window, cx);
        editor
    });

    cx.observe(&editor, |_, _, cx| cx.notify()).detach();

    let current_text = editor.read(cx).text(cx);
    let decision = if current_text.is_empty() {
        None
    } else {
        Some(evaluate_test_input(tool_id, &current_text, cx))
    };

    let theme_colors = cx.theme().colors();

    v_flex()
        .id(format!("{}-verification-section", tool_id))
        .mt_4()
        .p_3()
        .rounded_md()
        .bg(cx.theme().colors().surface_background)
        .border_1()
        .border_color(cx.theme().colors().border_variant)
        .child(
            Label::new("Test Your Rules")
                .size(LabelSize::Small)
                .color(Color::Muted),
        )
        .child(
            h_flex()
                .mt_2()
                .gap_3()
                .items_start()
                .child(
                    div()
                        .flex_1()
                        .py_1()
                        .px_2()
                        .rounded_md()
                        .border_1()
                        .border_color(theme_colors.border)
                        .bg(theme_colors.editor_background)
                        .child(editor),
                )
                .child(render_verification_result(decision.as_ref(), cx)),
        )
        .when(always_allow_enabled, |this| {
            this.child(
                h_flex()
                    .mt_2()
                    .gap_1()
                    .items_center()
                    .child(
                        Icon::new(IconName::Warning)
                            .size(IconSize::Small)
                            .color(Color::Warning),
                    )
                    .child(
                        Button::new("always-allow-link", "Always allow tool actions")
                            .style(ButtonStyle::Transparent)
                            .color(Color::Warning)
                            .label_size(LabelSize::Small)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.navigate_to_setting(
                                    "agent.always_allow_tool_actions",
                                    window,
                                    cx,
                                );
                            })),
                    )
                    .child(
                        Label::new(
                            "is enabled — all tools will be allowed regardless of these rules.",
                        )
                        .size(LabelSize::Small)
                        .color(Color::Warning),
                    ),
            )
        })
        .into_any_element()
}

fn evaluate_test_input(tool_id: &str, input: &str, cx: &App) -> ToolPermissionDecision {
    let settings = AgentSettings::get_global(cx);

    let shell_kind = if tool_id == "terminal" {
        ShellKind::system()
    } else {
        None
    };

    // Always pass false for always_allow_tool_actions so we test the actual rules,
    // not the global override that bypasses all checks.
    ToolPermissionDecision::from_input(
        tool_id,
        input,
        &settings.tool_permissions,
        false,
        shell_kind,
    )
}

fn render_verification_result(decision: Option<&ToolPermissionDecision>, cx: &App) -> AnyElement {
    let (label, color, icon) = match decision {
        Some(ToolPermissionDecision::Allow) => ("Allowed", Color::Success, IconName::Check),
        Some(ToolPermissionDecision::Deny(_)) => ("Denied", Color::Error, IconName::XCircle),
        Some(ToolPermissionDecision::Confirm) => ("Confirm", Color::Warning, IconName::Info),
        None => ("", Color::Muted, IconName::Dash),
    };

    let has_result = decision.is_some();
    let deny_reason = decision.and_then(|d| {
        if let ToolPermissionDecision::Deny(reason) = d {
            Some(reason.clone())
        } else {
            None
        }
    });

    h_flex()
        .h_7()
        .px_2()
        .gap_1()
        .items_center()
        .rounded_md()
        .when(has_result, |this| this.bg(color.color(cx).opacity(0.1)))
        .when(has_result, |this| {
            this.child(Icon::new(icon).size(IconSize::Small).color(color))
                .child(Label::new(label).size(LabelSize::Small).color(color))
        })
        .when_some(deny_reason, |this, reason| {
            this.child(
                Label::new(format!("— {}", reason))
                    .size(LabelSize::XSmall)
                    .color(Color::Muted),
            )
        })
        .into_any_element()
}

fn render_rule_section(
    tool_id: &'static str,
    title: &'static str,
    description: &'static str,
    rule_type: RuleType,
    patterns: &[String],
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let section_id = format!("{}-{:?}-section", tool_id, rule_type);

    v_flex()
        .id(section_id)
        .child(Label::new(title))
        .child(
            Label::new(description)
                .size(LabelSize::Small)
                .color(Color::Muted),
        )
        .child(
            v_flex()
                .w_full()
                .mt_2()
                .gap_1()
                .when(patterns.is_empty(), |this| {
                    this.child(render_pattern_empty_state(cx))
                })
                .children(patterns.iter().enumerate().map(|(index, pattern)| {
                    render_pattern_row(tool_id, rule_type, index, pattern.clone(), cx)
                }))
                .child(render_add_pattern_input(tool_id, rule_type, cx)),
        )
        .into_any_element()
}

fn render_pattern_empty_state(cx: &mut Context<SettingsWindow>) -> AnyElement {
    h_flex()
        .p_2()
        .rounded_md()
        .border_1()
        .border_dashed()
        .border_color(cx.theme().colors().border_variant)
        .child(
            Label::new("No patterns configured")
                .size(LabelSize::Small)
                .color(Color::Disabled),
        )
        .into_any_element()
}

fn render_pattern_row(
    tool_id: &'static str,
    rule_type: RuleType,
    index: usize,
    pattern: String,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let pattern_for_delete = pattern.clone();
    let pattern_for_update = pattern.clone();
    let tool_id_for_delete = tool_id.to_string();
    let tool_id_for_update = tool_id.to_string();
    let input_id = format!("{}-{:?}-pattern-{}", tool_id, rule_type, index);
    let delete_id = format!("{}-{:?}-delete-{}", tool_id, rule_type, index);
    let group = format!("{}-pattern", tool_id);

    div()
        .group(&group)
        .relative()
        .w_full()
        .child(
            SettingsInputField::new()
                .with_id(input_id)
                .with_initial_text(pattern)
                .tab_index(0)
                .with_buffer_font()
                .on_confirm(move |new_pattern, _window, cx| {
                    if let Some(new_pattern) = new_pattern {
                        let new_pattern = new_pattern.trim().to_string();
                        if !new_pattern.is_empty() && new_pattern != pattern_for_update {
                            update_pattern(
                                &tool_id_for_update,
                                rule_type,
                                &pattern_for_update,
                                new_pattern,
                                cx,
                            );
                        }
                    }
                }),
        )
        .child(
            div()
                .visible_on_hover(group.clone())
                .absolute()
                .top_1p5()
                .right_1p5()
                .child(
                    IconButton::new(delete_id, IconName::Trash)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Muted)
                        .tooltip(Tooltip::text("Delete Pattern"))
                        .on_click(cx.listener(move |_, _, _, cx| {
                            delete_pattern(&tool_id_for_delete, rule_type, &pattern_for_delete, cx);
                        })),
                ),
        )
        .into_any_element()
}

fn render_add_pattern_input(
    tool_id: &'static str,
    rule_type: RuleType,
    _cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let tool_id_owned = tool_id.to_string();
    let input_id = format!("{}-{:?}-new-pattern", tool_id, rule_type);
    let group = format!("{}-pattern", tool_id);

    div()
        .group(&group)
        .relative()
        .w_full()
        .child(
            SettingsInputField::new()
                .with_id(input_id)
                .with_placeholder("Add regex pattern…")
                .tab_index(0)
                .with_buffer_font()
                .on_confirm(move |pattern, _window, cx| {
                    if let Some(pattern) = pattern {
                        if !pattern.trim().is_empty() {
                            save_pattern(&tool_id_owned, rule_type, pattern.trim().to_string(), cx);
                        }
                    }
                }),
        )
        .child(
            div()
                .visible_on_hover(group.clone())
                .absolute()
                .top_1p5()
                .right_1p5(), // .child(
                              //     IconButton::new(delete_id, IconName::Trash)
                              //         .icon_size(IconSize::Small)
                              //         .icon_color(Color::Muted)
                              //         .tooltip(Tooltip::text("Delete Pattern"))
                              //         .on_click(cx.listener(move |_, _, _, cx| {
                              //             delete_pattern(&tool_id_for_delete, rule_type, &pattern_for_delete, cx);
                              //         })),
                              // ),
        )
        .into_any_element()
}

fn render_default_mode_section(
    tool_id: &'static str,
    current_mode: ToolPermissionMode,
    _cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let mode_label = match current_mode {
        ToolPermissionMode::Allow => "Allow",
        ToolPermissionMode::Deny => "Deny",
        ToolPermissionMode::Confirm => "Confirm",
    };

    let tool_id_owned = tool_id.to_string();

    h_flex()
        .justify_between()
        .child(
            v_flex().child(Label::new("Default Action")).child(
                Label::new("Action to take when no patterns match.")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            ),
        )
        .child(
            PopoverMenu::new(format!("default-mode-{}", tool_id))
                .trigger(
                    Button::new(format!("mode-trigger-{}", tool_id), mode_label)
                        .style(ButtonStyle::Outlined)
                        .size(ButtonSize::Medium)
                        .icon(IconName::ChevronDown)
                        .icon_position(IconPosition::End)
                        .icon_size(IconSize::Small),
                )
                .menu(move |window, cx| {
                    let tool_id = tool_id_owned.clone();
                    Some(ContextMenu::build(window, cx, move |menu, _, _| {
                        let tool_id_confirm = tool_id.clone();
                        let tool_id_allow = tool_id.clone();
                        let tool_id_deny = tool_id;

                        menu.entry("Confirm", None, move |_, cx| {
                            set_default_mode(&tool_id_confirm, ToolPermissionMode::Confirm, cx);
                        })
                        .entry("Allow", None, move |_, cx| {
                            set_default_mode(&tool_id_allow, ToolPermissionMode::Allow, cx);
                        })
                        .entry("Deny", None, move |_, cx| {
                            set_default_mode(&tool_id_deny, ToolPermissionMode::Deny, cx);
                        })
                    }))
                })
                .anchor(gpui::Corner::TopRight),
        )
        .into_any_element()
}

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

fn update_pattern(
    tool_name: &str,
    rule_type: RuleType,
    old_pattern: &str,
    new_pattern: String,
    cx: &mut App,
) {
    let tool_name = tool_name.to_string();
    let old_pattern = old_pattern.to_string();

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
                if let Some(rule) = list.0.iter_mut().find(|r| r.pattern == old_pattern) {
                    rule.pattern = new_pattern;
                }
            }
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
