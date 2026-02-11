use agent::{AgentTool, TerminalTool, ToolPermissionDecision};
use agent_settings::AgentSettings;
use gpui::{
    Focusable, HighlightStyle, ReadGlobal, ScrollHandle, StyledText, TextStyleRefinement, point,
    prelude::*,
};
use settings::{Settings as _, SettingsStore, ToolPermissionMode};
use shell_command_parser::extract_commands;
use std::sync::Arc;
use theme::ThemeSettings;
use ui::{Banner, ContextMenu, Divider, PopoverMenu, Severity, Tooltip, prelude::*};
use util::ResultExt as _;
use util::shell::ShellKind;

use crate::{SettingsWindow, components::SettingsInputField};

const HARDCODED_RULES_DESCRIPTION: &str =
    "`rm -rf` commands are always blocked when run on `$HOME`, `~`, `.`, `..`, or `/`";
const SETTINGS_DISCLAIMER: &str = "Note: custom tool permissions only apply to the Zed native agent and don’t extend to external agents connected through the Agent Client Protocol (ACP).";

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
        id: "copy_path",
        name: "Copy Path",
        description: "File and directory copying",
        regex_explanation: "Patterns are matched independently against the source path and the destination path. Enter either path below to test.",
    },
    ToolInfo {
        id: "move_path",
        name: "Move Path",
        description: "File and directory moves/renames",
        regex_explanation: "Patterns are matched independently against the source path and the destination path. Enter either path below to test.",
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
    ToolInfo {
        id: "restore_file_from_disk",
        name: "Restore File from Disk",
        description: "Discards unsaved changes by reloading from disk",
        regex_explanation: "Patterns are matched against the file path being restored.",
    },
];

pub(crate) struct ToolInfo {
    id: &'static str,
    name: &'static str,
    description: &'static str,
    regex_explanation: &'static str,
}

const fn const_str_eq(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// Finds the index of a tool in `TOOLS` by its ID. Panics (compile error in
/// const context) if the ID is not found, so every macro-generated render
/// function is validated at compile time.
const fn tool_index(id: &str) -> usize {
    let mut i = 0;
    while i < TOOLS.len() {
        if const_str_eq(TOOLS[i].id, id) {
            return i;
        }
        i += 1;
    }
    panic!("tool ID not found in TOOLS array")
}

/// Parses a string containing backtick-delimited code spans into a `StyledText`
/// with code background highlights applied to each span.
fn render_inline_code_markdown(text: &str, cx: &App) -> StyledText {
    let code_background = cx.theme().colors().surface_background;
    let mut plain = String::new();
    let mut highlights: Vec<(std::ops::Range<usize>, HighlightStyle)> = Vec::new();
    let mut in_code = false;
    let mut code_start = 0;

    for ch in text.chars() {
        if ch == '`' {
            if in_code {
                highlights.push((
                    code_start..plain.len(),
                    HighlightStyle {
                        background_color: Some(code_background),
                        ..Default::default()
                    },
                ));
            } else {
                code_start = plain.len();
            }
            in_code = !in_code;
        } else {
            plain.push(ch);
        }
    }

    StyledText::new(plain).with_highlights(highlights)
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
        .child(
            Banner::new().child(
                Label::new(SETTINGS_DISCLAIMER)
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .mt_0p5(),
            ),
        )
        .child(
            v_flex()
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
    let invalid_count = rules.invalid_patterns.len();

    let rule_summary = if rule_count > 0 || invalid_count > 0 {
        let mut parts = Vec::new();
        if rule_count > 0 {
            if rule_count == 1 {
                parts.push("1 rule".to_string());
            } else {
                parts.push(format!("{} rules", rule_count));
            }
        }
        if invalid_count > 0 {
            parts.push(format!("{} invalid", invalid_count));
        }
        Some(parts.join(", "))
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
                        "Tool Permissions",
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
        "copy_path" => render_copy_path_tool_config,
        "move_path" => render_move_path_tool_config,
        "create_directory" => render_create_directory_tool_config,
        "save_file" => render_save_file_tool_config,
        "fetch" => render_fetch_tool_config,
        "web_search" => render_web_search_tool_config,
        "restore_file_from_disk" => render_restore_file_from_disk_tool_config,
        _ => render_terminal_tool_config, // fallback
    }
}

/// Renders an individual tool's permission configuration page
pub(crate) fn render_tool_config_page(
    tool: &ToolInfo,
    settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let rules = get_tool_rules(tool.id, cx);
    let page_title = format!("{} Tool", tool.name);
    let scroll_step = px(80.);

    v_flex()
        .id(format!("tool-config-page-{}", tool.id))
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
        .when(tool.id == TerminalTool::NAME, |this| {
            this.child(render_hardcoded_security_banner(cx))
        })
        .child(render_verification_section(tool.id, window, cx))
        .when_some(
            settings_window.regex_validation_error.clone(),
            |this, error| {
                this.child(
                    Banner::new()
                        .severity(Severity::Warning)
                        .child(Label::new(error).size(LabelSize::Small))
                        .action_slot(
                            Button::new("dismiss-regex-error", "Dismiss")
                                .style(ButtonStyle::Tinted(ui::TintColor::Warning))
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.regex_validation_error = None;
                                    cx.notify();
                                })),
                        ),
                )
            },
        )
        .child(
            v_flex()
                .mt_6()
                .min_w_0()
                .w_full()
                .gap_5()
                .child(render_default_mode_section(tool.id, rules.default, cx))
                .child(Divider::horizontal().color(ui::DividerColor::BorderFaded))
                .child(render_rule_section(
                    tool.id,
                    "Always Deny",
                    "If any of these regexes match, the tool action will be denied.",
                    ToolPermissionMode::Deny,
                    &rules.always_deny,
                    cx,
                ))
                .child(Divider::horizontal().color(ui::DividerColor::BorderFaded))
                .child(render_rule_section(
                    tool.id,
                    "Always Allow",
                    "If any of these regexes match, the action will be approved—unless an Always Confirm or Always Deny matches.",
                    ToolPermissionMode::Allow,
                    &rules.always_allow,
                    cx,
                ))
                .child(Divider::horizontal().color(ui::DividerColor::BorderFaded))
                .child(render_rule_section(
                    tool.id,
                    "Always Confirm",
                    "If any of these regexes match, a confirmation will be shown unless an Always Deny regex matches.",
                    ToolPermissionMode::Confirm,
                    &rules.always_confirm,
                    cx,
                ))
                .when(!rules.invalid_patterns.is_empty(), |this| {
                    this.child(Divider::horizontal().color(ui::DividerColor::BorderFaded))
                        .child(render_invalid_patterns_section(
                            tool.id,
                            &rules.invalid_patterns,
                            cx,
                        ))
                }),
        )
        .into_any_element()
}

fn render_hardcoded_rules(smaller_font_size: bool, cx: &App) -> AnyElement {
    div()
        .map(|this| {
            if smaller_font_size {
                this.text_xs()
            } else {
                this.text_sm()
            }
        })
        .text_color(cx.theme().colors().text_muted)
        .child(render_inline_code_markdown(HARDCODED_RULES_DESCRIPTION, cx))
        .into_any_element()
}

fn render_hardcoded_security_banner(cx: &mut Context<SettingsWindow>) -> AnyElement {
    div()
        .mt_3()
        .child(Banner::new().child(render_hardcoded_rules(false, cx)))
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
        editor.set_placeholder_text("Enter a tool input to test your rules…", window, cx);

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
        let matches = find_matched_patterns(tool_id, &current_text, cx);
        let decision = evaluate_test_input(tool_id, &current_text, cx);
        (Some(decision), matches)
    };

    let default_mode = get_tool_rules(tool_id, cx).default;
    let is_hardcoded_denial = matches!(
        &decision,
        Some(ToolPermissionDecision::Deny(reason))
            if reason.contains("built-in security rule")
    );
    let denial_reason = match &decision {
        Some(ToolPermissionDecision::Deny(reason))
            if !reason.is_empty() && !is_hardcoded_denial =>
        {
            Some(reason.clone())
        }
        _ => None,
    };
    let (authoritative_mode, patterns_agree) = match &decision {
        Some(decision) => {
            let authoritative = decision_to_mode(decision);
            let implied = implied_mode_from_patterns(&matched_patterns, default_mode);
            let agrees = authoritative == implied;
            if !agrees {
                log::error!(
                    "Tool permission verdict disagreement for '{}': \
                     engine={}, pattern_preview={}. \
                     Showing authoritative verdict only.",
                    tool_id,
                    mode_display_label(authoritative),
                    mode_display_label(implied),
                );
            }
            (Some(authoritative), agrees)
        }
        None => (None, true),
    };

    let color = cx.theme().colors();

    v_flex()
        .mt_3()
        .min_w_0()
        .gap_2()
        .child(
            v_flex()
                .p_2p5()
                .gap_1p5()
                .bg(color.surface_background.opacity(0.15))
                .border_1()
                .border_dashed()
                .border_color(color.border_variant)
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
                        .border_color(color.border)
                        .bg(color.editor_background)
                        .track_focus(&focus_handle)
                        .child(editor),
                )
                .when_some(authoritative_mode, |this, mode| {
                    this.when(patterns_agree, |this| {
                        if matched_patterns.is_empty() {
                            this.child(
                                Label::new("No regex matches, using the default action.")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                        } else {
                            this.child(render_matched_patterns(&matched_patterns, cx))
                        }
                    })
                    .when(!patterns_agree, |this| {
                        if is_hardcoded_denial {
                            this.child(render_hardcoded_rules(true, cx))
                        } else if let Some(reason) = &denial_reason {
                            this.child(
                                Label::new(format!("Denied: {}", reason))
                                    .size(LabelSize::XSmall)
                                    .color(Color::Warning),
                            )
                        } else {
                            this.child(
                                Label::new(
                                    "Pattern preview differs from engine — showing authoritative result.",
                                )
                                .size(LabelSize::XSmall)
                                .color(Color::Warning),
                            )
                        }
                    })
                    .when(is_hardcoded_denial && patterns_agree, |this| {
                        this.child(render_hardcoded_rules(true, cx))
                    })
                    .child(render_verdict_label(mode))
                    .when_some(
                        denial_reason.filter(|_| patterns_agree && !is_hardcoded_denial),
                        |this, reason| {
                            this.child(
                                Label::new(format!("Reason: {}", reason))
                                    .size(LabelSize::XSmall)
                                    .color(Color::Error),
                            )
                        },
                    )
                }),
        )
        .into_any_element()
}

#[derive(Clone, Debug)]
struct MatchedPattern {
    pattern: String,
    rule_type: ToolPermissionMode,
    is_overridden: bool,
}

fn find_matched_patterns(tool_id: &str, input: &str, cx: &App) -> Vec<MatchedPattern> {
    let settings = AgentSettings::get_global(cx);
    let rules = match settings.tool_permissions.tools.get(tool_id) {
        Some(rules) => rules,
        None => return Vec::new(),
    };

    let mut matched = Vec::new();

    // For terminal commands, parse chained commands (&&, ||, ;) so the preview
    // matches the real permission engine's behavior.
    // When parsing fails (extract_commands returns None), the real engine
    // ignores always_allow rules, so we track parse success to mirror that.
    let (inputs_to_check, allow_enabled) = if tool_id == TerminalTool::NAME {
        match extract_commands(input) {
            Some(cmds) => (cmds, true),
            None => (vec![input.to_string()], false),
        }
    } else {
        (vec![input.to_string()], true)
    };

    let mut has_deny_match = false;
    let mut has_confirm_match = false;

    for rule in &rules.always_deny {
        if inputs_to_check.iter().any(|cmd| rule.is_match(cmd)) {
            has_deny_match = true;
            matched.push(MatchedPattern {
                pattern: rule.pattern.clone(),
                rule_type: ToolPermissionMode::Deny,
                is_overridden: false,
            });
        }
    }

    for rule in &rules.always_confirm {
        if inputs_to_check.iter().any(|cmd| rule.is_match(cmd)) {
            has_confirm_match = true;
            matched.push(MatchedPattern {
                pattern: rule.pattern.clone(),
                rule_type: ToolPermissionMode::Confirm,
                is_overridden: has_deny_match,
            });
        }
    }

    // The real engine requires ALL commands to match at least one allow
    // pattern for the overall verdict to be Allow. Compute that first,
    // then show individual patterns with correct override status.
    let all_commands_matched_allow = !inputs_to_check.is_empty()
        && inputs_to_check
            .iter()
            .all(|cmd| rules.always_allow.iter().any(|rule| rule.is_match(cmd)));

    for rule in &rules.always_allow {
        if inputs_to_check.iter().any(|cmd| rule.is_match(cmd)) {
            matched.push(MatchedPattern {
                pattern: rule.pattern.clone(),
                rule_type: ToolPermissionMode::Allow,
                is_overridden: !allow_enabled
                    || has_deny_match
                    || has_confirm_match
                    || !all_commands_matched_allow,
            });
        }
    }

    matched
}

fn render_matched_patterns(patterns: &[MatchedPattern], cx: &App) -> AnyElement {
    v_flex()
        .gap_1()
        .children(patterns.iter().map(|pattern| {
            let (type_label, color) = match pattern.rule_type {
                ToolPermissionMode::Deny => ("Always Deny", Color::Error),
                ToolPermissionMode::Confirm => ("Always Confirm", Color::Warning),
                ToolPermissionMode::Allow => ("Always Allow", Color::Success),
            };

            let type_color = if pattern.is_overridden {
                Color::Muted
            } else {
                color
            };

            h_flex()
                .gap_1()
                .child(
                    Label::new(pattern.pattern.clone())
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                        .buffer_font(cx)
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
        &[input.to_string()],
        &settings.tool_permissions,
        ShellKind::system(),
    )
}

fn decision_to_mode(decision: &ToolPermissionDecision) -> ToolPermissionMode {
    match decision {
        ToolPermissionDecision::Allow => ToolPermissionMode::Allow,
        ToolPermissionDecision::Deny(_) => ToolPermissionMode::Deny,
        ToolPermissionDecision::Confirm => ToolPermissionMode::Confirm,
    }
}

fn implied_mode_from_patterns(
    patterns: &[MatchedPattern],
    default_mode: ToolPermissionMode,
) -> ToolPermissionMode {
    let has_active_deny = patterns
        .iter()
        .any(|p| matches!(p.rule_type, ToolPermissionMode::Deny) && !p.is_overridden);
    let has_active_confirm = patterns
        .iter()
        .any(|p| matches!(p.rule_type, ToolPermissionMode::Confirm) && !p.is_overridden);
    let has_active_allow = patterns
        .iter()
        .any(|p| matches!(p.rule_type, ToolPermissionMode::Allow) && !p.is_overridden);

    if has_active_deny {
        ToolPermissionMode::Deny
    } else if has_active_confirm {
        ToolPermissionMode::Confirm
    } else if has_active_allow {
        ToolPermissionMode::Allow
    } else {
        default_mode
    }
}

fn mode_display_label(mode: ToolPermissionMode) -> &'static str {
    match mode {
        ToolPermissionMode::Allow => "Allow",
        ToolPermissionMode::Deny => "Deny",
        ToolPermissionMode::Confirm => "Confirm",
    }
}

fn verdict_color(mode: ToolPermissionMode) -> Color {
    match mode {
        ToolPermissionMode::Allow => Color::Success,
        ToolPermissionMode::Deny => Color::Error,
        ToolPermissionMode::Confirm => Color::Warning,
    }
}

fn render_verdict_label(mode: ToolPermissionMode) -> AnyElement {
    h_flex()
        .gap_1()
        .child(
            Label::new("Result:")
                .size(LabelSize::Small)
                .color(Color::Muted),
        )
        .child(
            Label::new(mode_display_label(mode))
                .size(LabelSize::Small)
                .color(verdict_color(mode)),
        )
        .into_any_element()
}

fn render_invalid_patterns_section(
    tool_id: &'static str,
    invalid_patterns: &[InvalidPatternView],
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let section_id = format!("{}-invalid-patterns-section", tool_id);
    let theme_colors = cx.theme().colors();

    v_flex()
        .id(section_id)
        .child(
            h_flex()
                .gap_1()
                .child(
                    Icon::new(IconName::Warning)
                        .size(IconSize::Small)
                        .color(Color::Error),
                )
                .child(Label::new("Invalid Patterns").color(Color::Error)),
        )
        .child(
            Label::new(
                "These patterns failed to compile as regular expressions. \
                 The tool will be blocked until they are fixed or removed.",
            )
            .size(LabelSize::Small)
            .color(Color::Muted),
        )
        .child(
            v_flex()
                .mt_2()
                .w_full()
                .gap_1p5()
                .children(invalid_patterns.iter().map(|invalid| {
                    let rule_type_label = match invalid.rule_type.as_str() {
                        "always_allow" => "Always Allow",
                        "always_deny" => "Always Deny",
                        "always_confirm" => "Always Confirm",
                        other => other,
                    };

                    let pattern_for_delete = invalid.pattern.clone();
                    let rule_type = match invalid.rule_type.as_str() {
                        "always_allow" => ToolPermissionMode::Allow,
                        "always_deny" => ToolPermissionMode::Deny,
                        _ => ToolPermissionMode::Confirm,
                    };
                    let tool_id_for_delete = tool_id.to_string();
                    let delete_id =
                        format!("{}-invalid-delete-{}", tool_id, invalid.pattern.clone());

                    v_flex()
                        .p_2()
                        .rounded_md()
                        .border_1()
                        .border_color(theme_colors.border_variant)
                        .bg(theme_colors.surface_background.opacity(0.15))
                        .gap_1()
                        .child(
                            h_flex()
                                .justify_between()
                                .child(
                                    h_flex()
                                        .gap_1p5()
                                        .min_w_0()
                                        .child(
                                            Label::new(invalid.pattern.clone())
                                                .size(LabelSize::Small)
                                                .color(Color::Error)
                                                .buffer_font(cx),
                                        )
                                        .child(
                                            Label::new(format!("({})", rule_type_label))
                                                .size(LabelSize::XSmall)
                                                .color(Color::Muted),
                                        ),
                                )
                                .child(
                                    IconButton::new(delete_id, IconName::Trash)
                                        .icon_size(IconSize::Small)
                                        .icon_color(Color::Muted)
                                        .tooltip(Tooltip::text("Delete Invalid Pattern"))
                                        .on_click(cx.listener(move |_, _, _, cx| {
                                            delete_pattern(
                                                &tool_id_for_delete,
                                                rule_type,
                                                &pattern_for_delete,
                                                cx,
                                            );
                                        })),
                                ),
                        )
                        .child(
                            Label::new(format!("Error: {}", invalid.error))
                                .size(LabelSize::XSmall)
                                .color(Color::Muted),
                        )
                })),
        )
        .into_any_element()
}

fn render_rule_section(
    tool_id: &'static str,
    title: &'static str,
    description: &'static str,
    rule_type: ToolPermissionMode,
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
    rule_type: ToolPermissionMode,
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
    let settings_window = cx.entity().downgrade();

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
                    let updated = update_pattern(
                        &tool_id_for_update,
                        rule_type,
                        &pattern_for_update,
                        new_pattern.clone(),
                        cx,
                    );

                    let validation_error = if !updated {
                        Some(
                            "A pattern with that name already exists in this rule list."
                                .to_string(),
                        )
                    } else {
                        match regex::Regex::new(&new_pattern) {
                            Err(err) => Some(format!(
                                "Invalid regex: {err}. Pattern saved but will block this tool until fixed or removed."
                            )),
                            Ok(_) => None,
                        }
                    };
                    settings_window
                        .update(cx, |this, cx| {
                            this.regex_validation_error = validation_error;
                            cx.notify();
                        })
                        .log_err();
                }
            }
        })
        .into_any_element()
}

fn render_add_pattern_input(
    tool_id: &'static str,
    rule_type: ToolPermissionMode,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let tool_id_owned = tool_id.to_string();
    let input_id = format!("{}-{:?}-new-pattern", tool_id, rule_type);
    let settings_window = cx.entity().downgrade();

    SettingsInputField::new()
        .with_id(input_id)
        .with_placeholder("Add regex pattern…")
        .tab_index(0)
        .with_buffer_font()
        .display_clear_button()
        .display_confirm_button()
        .clear_on_confirm()
        .on_confirm(move |pattern, _window, cx| {
            if let Some(pattern) = pattern {
                let trimmed = pattern.trim().to_string();
                if !trimmed.is_empty() {
                    save_pattern(&tool_id_owned, rule_type, trimmed.clone(), cx);

                    let validation_error = match regex::Regex::new(&trimmed) {
                        Err(err) => Some(format!(
                            "Invalid regex: {err}. Pattern saved but will block this tool until fixed or removed."
                        )),
                        Ok(_) => None,
                    };
                    settings_window
                        .update(cx, |this, cx| {
                            this.regex_validation_error = validation_error;
                            cx.notify();
                        })
                        .log_err();
                }
            }
        })
        .into_any_element()
}

fn render_global_default_mode_section(current_mode: ToolPermissionMode) -> AnyElement {
    let mode_label = current_mode.to_string();

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

struct InvalidPatternView {
    pattern: String,
    rule_type: String,
    error: String,
}

struct ToolRulesView {
    default: ToolPermissionMode,
    always_allow: Vec<String>,
    always_deny: Vec<String>,
    always_confirm: Vec<String>,
    invalid_patterns: Vec<InvalidPatternView>,
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
            invalid_patterns: rules
                .invalid_patterns
                .iter()
                .map(|p| InvalidPatternView {
                    pattern: p.pattern.clone(),
                    rule_type: p.rule_type.clone(),
                    error: p.error.clone(),
                })
                .collect(),
        },
        None => ToolRulesView {
            default: settings.tool_permissions.default,
            always_allow: Vec::new(),
            always_deny: Vec::new(),
            always_confirm: Vec::new(),
            invalid_patterns: Vec::new(),
        },
    }
}

fn save_pattern(tool_name: &str, rule_type: ToolPermissionMode, pattern: String, cx: &mut App) {
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
            ToolPermissionMode::Allow => tool_rules.always_allow.get_or_insert_default(),
            ToolPermissionMode::Deny => tool_rules.always_deny.get_or_insert_default(),
            ToolPermissionMode::Confirm => tool_rules.always_confirm.get_or_insert_default(),
        };

        if !rules_list.0.iter().any(|r| r.pattern == rule.pattern) {
            rules_list.0.push(rule);
        }
    });
}

fn update_pattern(
    tool_name: &str,
    rule_type: ToolPermissionMode,
    old_pattern: &str,
    new_pattern: String,
    cx: &mut App,
) -> bool {
    let settings = AgentSettings::get_global(cx);
    if let Some(tool_rules) = settings.tool_permissions.tools.get(tool_name) {
        let patterns = match rule_type {
            ToolPermissionMode::Allow => &tool_rules.always_allow,
            ToolPermissionMode::Deny => &tool_rules.always_deny,
            ToolPermissionMode::Confirm => &tool_rules.always_confirm,
        };
        if patterns.iter().any(|r| r.pattern == new_pattern) {
            return false;
        }
    }

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
                ToolPermissionMode::Allow => &mut tool_rules.always_allow,
                ToolPermissionMode::Deny => &mut tool_rules.always_deny,
                ToolPermissionMode::Confirm => &mut tool_rules.always_confirm,
            };

            if let Some(list) = rules_list {
                let already_exists = list.0.iter().any(|r| r.pattern == new_pattern);
                if !already_exists {
                    if let Some(rule) = list.0.iter_mut().find(|r| r.pattern == old_pattern) {
                        rule.pattern = new_pattern;
                    }
                }
            }
        }
    });

    true
}

fn delete_pattern(tool_name: &str, rule_type: ToolPermissionMode, pattern: &str, cx: &mut App) {
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
                ToolPermissionMode::Allow => &mut tool_rules.always_allow,
                ToolPermissionMode::Deny => &mut tool_rules.always_deny,
                ToolPermissionMode::Confirm => &mut tool_rules.always_confirm,
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

macro_rules! tool_config_page_fn {
    ($fn_name:ident, $tool_id:literal) => {
        pub fn $fn_name(
            settings_window: &SettingsWindow,
            scroll_handle: &ScrollHandle,
            window: &mut Window,
            cx: &mut Context<SettingsWindow>,
        ) -> AnyElement {
            const INDEX: usize = tool_index($tool_id);
            render_tool_config_page(&TOOLS[INDEX], settings_window, scroll_handle, window, cx)
        }
    };
}

tool_config_page_fn!(render_terminal_tool_config, "terminal");
tool_config_page_fn!(render_edit_file_tool_config, "edit_file");
tool_config_page_fn!(render_delete_path_tool_config, "delete_path");
tool_config_page_fn!(render_copy_path_tool_config, "copy_path");
tool_config_page_fn!(render_move_path_tool_config, "move_path");
tool_config_page_fn!(render_create_directory_tool_config, "create_directory");
tool_config_page_fn!(render_save_file_tool_config, "save_file");
tool_config_page_fn!(render_fetch_tool_config, "fetch");
tool_config_page_fn!(render_web_search_tool_config, "web_search");
tool_config_page_fn!(
    render_restore_file_from_disk_tool_config,
    "restore_file_from_disk"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_tools_are_in_tool_info_or_excluded() {
        // Tools that intentionally don't appear in the permissions UI.
        // If you add a new tool and this test fails, either:
        //   1. Add a ToolInfo entry to TOOLS (if the tool has permission checks), or
        //   2. Add it to this list with a comment explaining why it's excluded.
        const EXCLUDED_TOOLS: &[&str] = &[
            // Read-only / low-risk tools that don't call decide_permission_from_settings
            "diagnostics",
            "find_path",
            "grep",
            "list_directory",
            "now",
            "open",
            "read_file",
            "thinking",
            // streaming_edit_file uses "edit_file" for permission lookups,
            // so its rules are configured under the edit_file entry.
            "streaming_edit_file",
            // Subagent permission checks happen at the level of individual
            // tool calls within the subagent, not at the spawning level.
            "subagent",
        ];

        let tool_info_ids: Vec<&str> = TOOLS.iter().map(|t| t.id).collect();

        for tool_name in agent::ALL_TOOL_NAMES {
            if EXCLUDED_TOOLS.contains(tool_name) {
                assert!(
                    !tool_info_ids.contains(tool_name),
                    "Tool '{}' is in both EXCLUDED_TOOLS and TOOLS — pick one.",
                    tool_name,
                );
                continue;
            }
            assert!(
                tool_info_ids.contains(tool_name),
                "Tool '{}' is in ALL_TOOL_NAMES but has no entry in TOOLS and \
                 is not in EXCLUDED_TOOLS. Either add a ToolInfo entry (if the \
                 tool has permission checks) or add it to EXCLUDED_TOOLS with \
                 a comment explaining why.",
                tool_name,
            );
        }

        for tool_id in &tool_info_ids {
            assert!(
                agent::ALL_TOOL_NAMES.contains(tool_id),
                "TOOLS contains '{}' but it is not in ALL_TOOL_NAMES. \
                 Is this a valid built-in tool?",
                tool_id,
            );
        }
    }
}
