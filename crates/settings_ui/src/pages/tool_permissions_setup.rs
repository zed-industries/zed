use agent::{HARDCODED_SECURITY_RULES, ToolPermissionDecision};
use agent_settings::AgentSettings;
use gpui::{Focusable, ReadGlobal, ScrollHandle, TextStyleRefinement, point, prelude::*};
use settings::{Settings as _, SettingsStore, ToolPermissionMode};
use std::sync::Arc;
use theme::ThemeSettings;
use ui::{Banner, ContextMenu, Divider, PopoverMenu, Tooltip, prelude::*};
use util::shell::ShellKind;

use crate::{SettingsWindow, components::SettingsInputField};

/// Tools that support permission rules
const TOOLS: &[ToolInfo] = &[
    ToolInfo {
        id: "terminal",
        name: "Terminal",
        description: "Commands executed in the terminal",
        regex_explanation: "Patterns are matched against each command in the input. Commands chained with &&, ||, ;, or pipes are split and checked individually.",
    },
    ToolInfo {
        id: "edit_file",
        name: "Edit File",
        description: "File editing operations",
        regex_explanation: "Patterns are matched against the file path being edited.",
    },
    ToolInfo {
        id: "delete_path",
        name: "Delete Path",
        description: "File and directory deletion",
        regex_explanation: "Patterns are matched against the path being deleted.",
    },
    ToolInfo {
        id: "move_path",
        name: "Move Path",
        description: "File and directory moves/renames",
        regex_explanation: "Patterns are matched against both the source and destination paths.",
    },
    ToolInfo {
        id: "create_directory",
        name: "Create Directory",
        description: "Directory creation",
        regex_explanation: "Patterns are matched against the directory path being created.",
    },
    ToolInfo {
        id: "save_file",
        name: "Save File",
        description: "File saving operations",
        regex_explanation: "Patterns are matched against the file path being saved.",
    },
    ToolInfo {
        id: "fetch",
        name: "Fetch",
        description: "HTTP requests to URLs",
        regex_explanation: "Patterns are matched against the URL being fetched.",
    },
    ToolInfo {
        id: "web_search",
        name: "Web Search",
        description: "Web search queries",
        regex_explanation: "Patterns are matched against the search query.",
    },
];

struct ToolInfo {
    id: &'static str,
    name: &'static str,
    description: &'static str,
    regex_explanation: &'static str,
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
        .enumerate()
        .map(|(i, tool)| render_tool_list_item(settings_window, tool, i, window, cx))
        .collect();

    let page_description =
        "Configure regex patterns to control which tool actions require confirmation.";

    let settings = AgentSettings::get_global(cx);
    let global_default = settings.tool_permissions.default;

    let scroll_step = px(40.);

    v_flex()
        .id("tool-permissions-page")
        .on_action({
            let scroll_handle = scroll_handle.clone();
            move |_: &menu::SelectNext, window, cx| {
                window.focus_next(cx);
                let current_offset = scroll_handle.offset();
                scroll_handle.set_offset(point(current_offset.x, current_offset.y - scroll_step));
            }
        })
        .on_action({
            let scroll_handle = scroll_handle.clone();
            move |_: &menu::SelectPrevious, window, cx| {
                window.focus_prev(cx);
                let current_offset = scroll_handle.offset();
                scroll_handle.set_offset(point(current_offset.x, current_offset.y + scroll_step));
            }
        })
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
                .mt_2()
                .child(render_global_default_mode_section(global_default))
                .child(Divider::horizontal())
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
    tool_index: usize,
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
                .tab_index(tool_index as isize)
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
    let scroll_step = px(80.);

    v_flex()
        .id(format!("tool-config-page-{}", tool_id))
        .on_action({
            let scroll_handle = scroll_handle.clone();
            move |_: &menu::SelectNext, window, cx| {
                window.focus_next(cx);
                let current_offset = scroll_handle.offset();
                scroll_handle.set_offset(point(current_offset.x, current_offset.y - scroll_step));
            }
        })
        .on_action({
            let scroll_handle = scroll_handle.clone();
            move |_: &menu::SelectPrevious, window, cx| {
                window.focus_prev(cx);
                let current_offset = scroll_handle.offset();
                scroll_handle.set_offset(point(current_offset.x, current_offset.y + scroll_step));
            }
        })
        .min_w_0()
        .size_full()
        .pt_2p5()
        .px_8()
        .pb_16()
        .overflow_y_scroll()
        .track_scroll(scroll_handle)
        .child(
            v_flex()
                .min_w_0()
                .child(Label::new(page_title).size(LabelSize::Large))
                .child(
                    Label::new(tool.regex_explanation)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                ),
        )
        .when(tool_id == "terminal", |this| {
            this.child(render_hardcoded_security_banner(cx))
        })
        .child(render_verification_section(tool_id, window, cx))
        .child(
            v_flex()
                .mt_6()
                .min_w_0()
                .w_full()
                .gap_5()
                .child(render_default_mode_section(tool_id, rules.default, cx))
                .child(Divider::horizontal().color(ui::DividerColor::BorderFaded))
                .child(render_rule_section(
                    tool_id,
                    "Always Deny",
                    "If any of these regexes match, the tool action will be denied.",
                    RuleType::Deny,
                    &rules.always_deny,
                    cx,
                ))
                .child(Divider::horizontal().color(ui::DividerColor::BorderFaded))
                .child(render_rule_section(
                    tool_id,
                    "Always Allow",
                    "If any of these regexes match, the tool action will be approved—unless an Always Confirm or Always Deny regex matches.",
                    RuleType::Allow,
                    &rules.always_allow,
                    cx,
                ))
                .child(Divider::horizontal().color(ui::DividerColor::BorderFaded))
                .child(render_rule_section(
                    tool_id,
                    "Always Confirm",
                    "If any of these regexes match, a confirmation will be shown unless an Always Deny regex matches.",
                    RuleType::Confirm,
                    &rules.always_confirm,
                    cx,
                )),
        )
        .into_any_element()
}

fn render_hardcoded_security_banner(cx: &mut Context<SettingsWindow>) -> AnyElement {
    let pattern_labels = HARDCODED_SECURITY_RULES.terminal_deny.iter().map(|rule| {
        h_flex()
            .gap_1()
            .child(
                Icon::new(IconName::Dash)
                    .color(Color::Hidden)
                    .size(IconSize::Small),
            )
            .child(
                Label::new(rule.pattern.clone())
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .buffer_font(cx),
            )
    });

    v_flex()
        .mt_3()
        .child(
            Banner::new().child(
                v_flex()
                    .py_1()
                    .gap_1()
                    .child(
                        Label::new(
                            "The following patterns are always blocked and cannot be overridden:",
                        )
                        .size(LabelSize::Small),
                    )
                    .children(pattern_labels),
            ),
        )
        .into_any_element()
}

fn render_verification_section(
    tool_id: &'static str,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let input_id = format!("{}-verification-input", tool_id);

    let editor = window.use_keyed_state(input_id, cx, |window, cx| {
        let mut editor = editor::Editor::single_line(window, cx);
        editor.set_placeholder_text("Enter a rule to see how it applies…", window, cx);

        let global_settings = ThemeSettings::get_global(cx);
        editor.set_text_style_refinement(TextStyleRefinement {
            font_family: Some(global_settings.buffer_font.family.clone()),
            font_size: Some(rems(0.75).into()),
            ..Default::default()
        });

        editor
    });

    cx.observe(&editor, |_, _, cx| cx.notify()).detach();

    let focus_handle = editor.focus_handle(cx).tab_index(0).tab_stop(true);

    let current_text = editor.read(cx).text(cx);
    let (decision, matched_patterns) = if current_text.is_empty() {
        (None, Vec::new())
    } else {
        let decision = evaluate_test_input(tool_id, &current_text, cx);
        let tool_name = TOOLS
            .iter()
            .find(|t| t.id == tool_id)
            .map(|t| t.name)
            .unwrap_or(tool_id);
        let matches = find_matched_patterns(tool_id, tool_name, &current_text, cx);
        (Some(decision), matches)
    };

    let theme_colors = cx.theme().colors();

    v_flex()
        .mt_3()
        .min_w_0()
        .gap_2()
        .child(
            v_flex()
                .p_2p5()
                .gap_1p5()
                .bg(theme_colors.surface_background.opacity(0.15))
                .border_1()
                .border_dashed()
                .border_color(theme_colors.border_variant)
                .rounded_sm()
                .child(
                    Label::new("Test Your Rules")
                        .color(Color::Muted)
                        .size(LabelSize::Small),
                )
                .child(
                    h_flex()
                        .w_full()
                        .h_8()
                        .px_2()
                        .rounded_md()
                        .border_1()
                        .border_color(theme_colors.border)
                        .bg(theme_colors.editor_background)
                        .track_focus(&focus_handle)
                        .child(editor),
                )
                .when(decision.is_some(), |this| {
                    this.child(render_matched_patterns(&matched_patterns, cx))
                }),
        )
        .into_any_element()
}

fn mode_to_rule_type(mode: ToolPermissionMode) -> RuleType {
    match mode {
        ToolPermissionMode::Allow => RuleType::Allow,
        ToolPermissionMode::Deny => RuleType::Deny,
        ToolPermissionMode::Confirm => RuleType::Confirm,
    }
}

#[derive(Clone, Debug)]
struct MatchedPattern {
    label: String,
    rule_type: RuleType,
    is_overridden: bool,
    is_regex: bool,
}

fn find_matched_patterns(
    tool_id: &str,
    tool_name: &str,
    input: &str,
    cx: &App,
) -> Vec<MatchedPattern> {
    let settings = AgentSettings::get_global(cx);
    let rules = settings.tool_permissions.tools.get(tool_id);

    let mut matched = Vec::new();
    let mut has_deny_match = false;
    let mut has_confirm_match = false;
    let mut has_allow_match = false;

    if let Some(rules) = rules {
        for rule in &rules.always_deny {
            if rule.is_match(input) {
                has_deny_match = true;
                matched.push(MatchedPattern {
                    label: rule.pattern.clone(),
                    rule_type: RuleType::Deny,
                    is_overridden: false,
                    is_regex: true,
                });
            }
        }

        for rule in &rules.always_confirm {
            if rule.is_match(input) {
                has_confirm_match = true;
                matched.push(MatchedPattern {
                    label: rule.pattern.clone(),
                    rule_type: RuleType::Confirm,
                    is_overridden: has_deny_match,
                    is_regex: true,
                });
            }
        }

        for rule in &rules.always_allow {
            if rule.is_match(input) {
                has_allow_match = true;
                matched.push(MatchedPattern {
                    label: rule.pattern.clone(),
                    rule_type: RuleType::Allow,
                    is_overridden: has_deny_match || has_confirm_match,
                    is_regex: true,
                });
            }
        }
    }

    let any_pattern_matched = has_deny_match || has_confirm_match || has_allow_match;
    let tool_specific_default = rules.and_then(|r| r.default);
    let global_default = settings.tool_permissions.default;
    let has_tool_default = tool_specific_default.is_some();

    if let Some(tool_default) = tool_specific_default {
        matched.push(MatchedPattern {
            label: format!("Default permission for {} tool", tool_name.to_lowercase()),
            rule_type: mode_to_rule_type(tool_default),
            is_overridden: any_pattern_matched,
            is_regex: false,
        });
    }

    matched.push(MatchedPattern {
        label: "Default permission for all tools".to_string(),
        rule_type: mode_to_rule_type(global_default),
        is_overridden: any_pattern_matched || has_tool_default,
        is_regex: false,
    });

    matched
}

fn render_matched_patterns(patterns: &[MatchedPattern], cx: &App) -> AnyElement {
    v_flex()
        .gap_1()
        .children(patterns.iter().map(|pattern| {
            let (type_label, color) = if pattern.is_regex {
                match pattern.rule_type {
                    RuleType::Deny => ("Always Deny", Color::Error),
                    RuleType::Confirm => ("Always Confirm", Color::Warning),
                    RuleType::Allow => ("Always Allow", Color::Success),
                }
            } else {
                match pattern.rule_type {
                    RuleType::Deny => ("Deny", Color::Error),
                    RuleType::Confirm => ("Confirm", Color::Warning),
                    RuleType::Allow => ("Allow", Color::Success),
                }
            };

            let type_color = if pattern.is_overridden {
                Color::Muted
            } else {
                color
            };

            h_flex()
                .gap_1()
                .child(
                    Label::new(pattern.label.clone())
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                        .when(pattern.is_regex, |this| this.buffer_font(cx))
                        .when(pattern.is_overridden, |this| this.strikethrough()),
                )
                .child(
                    Icon::new(IconName::Dash)
                        .size(IconSize::Small)
                        .color(Color::Custom(cx.theme().colors().icon_muted.opacity(0.4))),
                )
                .child(
                    Label::new(type_label)
                        .size(LabelSize::XSmall)
                        .color(type_color)
                        .when(pattern.is_overridden, |this| {
                            this.strikethrough().alpha(0.5)
                        }),
                )
        }))
        .into_any_element()
}

fn evaluate_test_input(tool_id: &str, input: &str, cx: &App) -> ToolPermissionDecision {
    let settings = AgentSettings::get_global(cx);

    // ShellKind is only used for terminal tool's hardcoded security rules;
    // for other tools, the check returns None immediately.
    ToolPermissionDecision::from_input(
        tool_id,
        input,
        &settings.tool_permissions,
        ShellKind::system(),
    )
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

    let user_patterns: Vec<_> = patterns.iter().enumerate().collect();

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
                .mt_2()
                .w_full()
                .gap_1p5()
                .when(patterns.is_empty(), |this| {
                    this.child(render_pattern_empty_state(cx))
                })
                .when(!user_patterns.is_empty(), |this| {
                    this.child(v_flex().gap_1p5().children(user_patterns.iter().map(
                        |(index, pattern)| {
                            render_user_pattern_row(
                                tool_id,
                                rule_type,
                                *index,
                                (*pattern).clone(),
                                cx,
                            )
                        },
                    )))
                })
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

fn render_user_pattern_row(
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

    SettingsInputField::new()
        .with_id(input_id)
        .with_initial_text(pattern)
        .tab_index(0)
        .with_buffer_font()
        .color(Color::Default)
        .action_slot(
            IconButton::new(delete_id, IconName::Trash)
                .icon_size(IconSize::Small)
                .icon_color(Color::Muted)
                .tooltip(Tooltip::text("Delete Pattern"))
                .on_click(cx.listener(move |_, _, _, cx| {
                    delete_pattern(&tool_id_for_delete, rule_type, &pattern_for_delete, cx);
                })),
        )
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
        })
        .into_any_element()
}

fn render_add_pattern_input(
    tool_id: &'static str,
    rule_type: RuleType,
    _cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let tool_id_owned = tool_id.to_string();
    let input_id = format!("{}-{:?}-new-pattern", tool_id, rule_type);

    SettingsInputField::new()
        .with_id(input_id)
        .with_placeholder("Add regex pattern…")
        .tab_index(0)
        .with_buffer_font()
        .display_clear_button()
        .display_confirm_button()
        .on_confirm(move |pattern, _window, cx| {
            if let Some(pattern) = pattern {
                if !pattern.trim().is_empty() {
                    save_pattern(&tool_id_owned, rule_type, pattern.trim().to_string(), cx);
                }
            }
        })
        .into_any_element()
}

fn render_global_default_mode_section(current_mode: ToolPermissionMode) -> AnyElement {
    let mode_label = match current_mode {
        ToolPermissionMode::Allow => "Allow",
        ToolPermissionMode::Deny => "Deny",
        ToolPermissionMode::Confirm => "Confirm",
    };

    h_flex()
        .mt_4()
        .justify_between()
        .child(
            v_flex()
                .child(Label::new("Default Permission"))
                .child(
                    Label::new(
                        "Controls the default behavior for all tool actions. Per-tool rules and patterns can override this.",
                    )
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                ),
        )
        .child(
            PopoverMenu::new("global-default-mode")
                .trigger(
                    Button::new("global-mode-trigger", mode_label)
                        .tab_index(0_isize)
                        .style(ButtonStyle::Outlined)
                        .size(ButtonSize::Medium)
                        .icon(IconName::ChevronDown)
                        .icon_position(IconPosition::End)
                        .icon_size(IconSize::Small),
                )
                .menu(move |window, cx| {
                    Some(ContextMenu::build(window, cx, move |menu, _, _| {
                        menu.entry("Confirm", None, move |_, cx| {
                            set_global_default_permission(ToolPermissionMode::Confirm, cx);
                        })
                        .entry("Allow", None, move |_, cx| {
                            set_global_default_permission(ToolPermissionMode::Allow, cx);
                        })
                        .entry("Deny", None, move |_, cx| {
                            set_global_default_permission(ToolPermissionMode::Deny, cx);
                        })
                    }))
                })
                .anchor(gpui::Corner::TopRight),
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
                        .tab_index(0_isize)
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
    default: ToolPermissionMode,
    always_allow: Vec<String>,
    always_deny: Vec<String>,
    always_confirm: Vec<String>,
}

fn get_tool_rules(tool_name: &str, cx: &App) -> ToolRulesView {
    let settings = AgentSettings::get_global(cx);

    let tool_rules = settings.tool_permissions.tools.get(tool_name);

    match tool_rules {
        Some(rules) => ToolRulesView {
            default: rules.default.unwrap_or(settings.tool_permissions.default),
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
            default: settings.tool_permissions.default,
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

fn set_global_default_permission(mode: ToolPermissionMode, cx: &mut App) {
    SettingsStore::global(cx).update_settings_file(<dyn fs::Fs>::global(cx), move |settings, _| {
        settings
            .agent
            .get_or_insert_default()
            .tool_permissions
            .get_or_insert_default()
            .default = Some(mode);
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
        tool_rules.default = Some(mode);
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
