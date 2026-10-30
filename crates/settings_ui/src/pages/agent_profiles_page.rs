use std::sync::Arc;

use agent::ContextServerRegistry;
use agent_settings::{
    AgentProfile, AgentProfileId, AgentProfileSettings, AgentSettings, builtin_profiles,
};
use context_server::ContextServerId;
use convert_case::{Case, Casing as _};
use editor::Editor;
use fs::Fs;
use gpui::{
    Context, Entity, Focusable as _, ScrollHandle, SharedString, Subscription, Window, prelude::*,
};
use language_model::{LanguageModel, LanguageModelProviderId, LanguageModelRegistry};
use settings::{
    AgentProfileContent, LanguageModelProviderSetting, LanguageModelSelection, Settings as _,
    update_settings_file,
};
use ui::{Divider, SwitchField, ToggleState, prelude::*};
use util::ResultExt as _;

use crate::SettingsWindow;
use crate::components::{SettingsSectionHeader, render_language_model_picker};

pub(crate) const AGENT_PROFILES_JSON_PATH: &str = "agent.profiles";
#[cfg(test)]
const AGENT_PROFILES_PAGE_TITLE: &str = "Agent Profiles";
const ZED_AGENT_ONLY: &str = "Applies to the Zed Agent only.";

/// State of the active agent profile detail sub-page. Owned by
/// `SettingsWindow`: sub-page render callbacks are plain function pointers, so
/// per-page state cannot live on a view entity of the page itself.
pub(crate) struct AgentProfilePageState {
    profile_id: AgentProfileId,
    /// Display name while `awaiting_creation` and the profile is not yet
    /// visible in `AgentSettings` (its settings write is still in flight).
    pending_name: SharedString,
    /// Set for a detail page opened right after Save: the profile must not be
    /// treated as deleted until the create write lands in `AgentSettings`.
    awaiting_creation: bool,
    context_server_registry: Option<Entity<ContextServerRegistry>>,
    _registry_subscription: Option<Subscription>,
    // D5: each "… Configured" telemetry event fires at most once per detail
    // page opening, on the first write of that section.
    tools_configured_sent: bool,
    mcps_configured_sent: bool,
    default_model_configured_sent: bool,
}

/// Editor-backed state of the active add/fork profile form sub-page.
pub(crate) struct AgentProfileForm {
    base_profile_id: Option<AgentProfileId>,
    name: Entity<Editor>,
}

/// The one-line descriptions the agent panel's profile picker shows for
/// built-in profiles (`ProfileSelector::documentation`), duplicated here so
/// `settings_ui` stays independent of `agent_ui`.
fn builtin_profile_description(profile_id: &AgentProfileId) -> Option<&'static str> {
    match profile_id.as_str() {
        builtin_profiles::WRITE => Some("Get help to write anything."),
        builtin_profiles::ASK => Some("Chat about your codebase."),
        builtin_profiles::MINIMAL => Some("Chat about anything with no tools."),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// List page
// ---------------------------------------------------------------------------

pub(crate) fn render_agent_profiles_page(
    _settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    _window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let mut builtins = Vec::new();
    let mut custom = Vec::new();
    for (profile_id, name) in AgentProfile::available_profiles(cx) {
        if builtin_profiles::is_builtin(&profile_id) {
            builtins.push((profile_id, name));
        } else {
            custom.push((profile_id, name));
        }
    }

    v_flex()
        .id("agent-profiles-page")
        .track_scroll(scroll_handle)
        .size_full()
        .pt_2p5()
        .px_8()
        .pb_16()
        .gap_6()
        .overflow_y_scroll()
        .child(
            v_flex()
                .child(SettingsSectionHeader::new("Built-in Profiles").no_padding(true))
                .children(builtins.into_iter().map(|(profile_id, name)| {
                    let description = builtin_profile_description(&profile_id);
                    render_profile_list_item(profile_id, name, description, cx)
                })),
        )
        .child(
            v_flex()
                .child(SettingsSectionHeader::new("Custom Profiles").no_padding(true))
                .when(custom.is_empty(), |this| {
                    this.child(render_dashed_note(
                        "No custom profiles yet. Click \"Add Profile\" to create one.",
                        cx,
                    ))
                })
                .children(custom.into_iter().map(|(profile_id, name)| {
                    render_profile_list_item(profile_id, name, None, cx)
                })),
        )
        .into_any_element()
}

fn render_profile_list_item(
    profile_id: AgentProfileId,
    name: SharedString,
    description: Option<&'static str>,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    h_flex()
        .w_full()
        .min_w_0()
        .py_3()
        .justify_between()
        .child(
            v_flex()
                .min_w_0()
                .child(Label::new(name.clone()))
                .when_some(description, |this, description| {
                    this.child(
                        Label::new(description)
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                }),
        )
        .child(
            Button::new(format!("agent-profile-configure-{profile_id}"), "Configure")
                .tab_index(0_isize)
                .style(ButtonStyle::OutlinedGhost)
                .size(ButtonSize::Medium)
                .end_icon(
                    Icon::new(IconName::ChevronRight)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                )
                .on_click(cx.listener(move |this, _, window, cx| {
                    open_agent_profile_detail_page(
                        this,
                        profile_id.clone(),
                        false,
                        name.clone(),
                        window,
                        cx,
                    );
                })),
        )
        .into_any_element()
}

fn render_dashed_note(text: &'static str, cx: &App) -> AnyElement {
    h_flex()
        .p_4()
        .justify_center()
        .border_1()
        .border_dashed()
        .border_color(cx.theme().colors().border.opacity(0.6))
        .rounded_sm()
        .child(Label::new(text).color(Color::Muted).size(LabelSize::Small))
        .into_any_element()
}

// ---------------------------------------------------------------------------
// Detail page
// ---------------------------------------------------------------------------

fn open_agent_profile_detail_page(
    settings_window: &mut SettingsWindow,
    profile_id: AgentProfileId,
    awaiting_creation: bool,
    pending_name: SharedString,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) {
    let context_server_registry = settings_window.active_project(cx).map(|project| {
        cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx))
    });
    let registry_subscription = context_server_registry
        .as_ref()
        .map(|registry| cx.observe(registry, |_, _, cx| cx.notify()));

    let title = AgentSettings::get_global(cx)
        .profiles
        .get(&profile_id)
        .map(|profile| profile.name.clone())
        .unwrap_or_else(|| pending_name.clone());

    settings_window.agent_profile_page = Some(AgentProfilePageState {
        profile_id,
        pending_name,
        awaiting_creation,
        context_server_registry,
        _registry_subscription: registry_subscription,
        tools_configured_sent: false,
        mcps_configured_sent: false,
        default_model_configured_sent: false,
    });

    settings_window.push_dynamic_sub_page(
        title,
        "Agent Configuration",
        Some(AGENT_PROFILES_JSON_PATH),
        true,
        render_agent_profile_detail_page,
        window,
        cx,
    );
}

fn render_agent_profile_detail_page(
    settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let Some(state) = settings_window.agent_profile_page.as_ref() else {
        return div().into_any_element();
    };
    let profile_id = state.profile_id.clone();
    let awaiting_creation = state.awaiting_creation;
    let pending_name = state.pending_name.clone();
    let context_server_registry = state.context_server_registry.clone();

    let profile = AgentSettings::get_global(cx)
        .profiles
        .get(&profile_id)
        .cloned();
    let is_builtin = builtin_profiles::is_builtin(&profile_id);
    let saving = awaiting_creation && profile.is_none();
    // A deleted profile's page is popped by sync_agent_profile_page_state; the
    // disabled-controls fallback below covers the frame before the pop lands.
    let controls_disabled = saving || profile.is_none();
    let display_name = profile
        .as_ref()
        .map(|profile| profile.name.clone())
        .unwrap_or(pending_name);

    v_flex()
        .id("agent-profile-detail-page")
        .track_scroll(scroll_handle)
        .size_full()
        .pt_2p5()
        .px_8()
        .pb_16()
        .gap_6()
        .overflow_y_scroll()
        .child(render_general_section(
            &profile_id,
            display_name,
            is_builtin,
            saving,
            controls_disabled,
            cx,
        ))
        .child(Divider::horizontal())
        .child(render_default_model_section(
            settings_window,
            &profile_id,
            profile.as_ref(),
            controls_disabled,
            window,
            cx,
        ))
        .child(Divider::horizontal())
        .child(render_builtin_tools_section(
            &profile_id,
            profile.as_ref(),
            controls_disabled,
            cx,
        ))
        .child(Divider::horizontal())
        .child(render_mcp_tools_section(
            settings_window,
            &profile_id,
            profile.as_ref(),
            context_server_registry.as_ref(),
            controls_disabled,
            cx,
        ))
        .into_any_element()
}

fn render_general_section(
    profile_id: &AgentProfileId,
    display_name: SharedString,
    is_builtin: bool,
    saving: bool,
    controls_disabled: bool,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let fork_profile_id = profile_id.clone();
    let delete_profile_id = profile_id.clone();
    v_flex()
        .gap_4()
        .child(SettingsSectionHeader::new("General").no_padding(true))
        .child(
            h_flex()
                .w_full()
                .min_w_0()
                .justify_between()
                .gap_2()
                .child(
                    v_flex()
                        .min_w_0()
                        .gap_0p5()
                        .child(Label::new("Name"))
                        .child(
                            Label::new(display_name)
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                )
                .child(
                    h_flex()
                        .flex_shrink_0()
                        .gap_2()
                        .child(
                            Button::new("agent-profile-fork", "Fork")
                                .style(ButtonStyle::OutlinedGhost)
                                .disabled(controls_disabled)
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    open_agent_profile_form(
                                        this,
                                        Some(fork_profile_id.clone()),
                                        window,
                                        cx,
                                    );
                                })),
                        )
                        .child(
                            Button::new("agent-profile-delete", "Delete")
                                .style(ButtonStyle::OutlinedGhost)
                                .disabled(controls_disabled || is_builtin)
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    let fs = <dyn Fs>::global(cx);
                                    delete_agent_profile(fs, delete_profile_id.clone(), cx);
                                    this.agent_profile_page = None;
                                    this.pop_sub_page(window, cx);
                                })),
                        ),
                ),
        )
        .when(saving, |this| {
            this.child(
                Label::new("Saving profile…")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
        })
        .into_any_element()
}

fn find_selected_model(selection: &LanguageModelSelection, cx: &App) -> Option<LanguageModel> {
    let provider_id = LanguageModelProviderId(SharedString::from(selection.provider.0.clone()));
    LanguageModelRegistry::global(cx)
        .read(cx)
        .provider(&provider_id)?
        .provided_models(cx)
        .into_iter()
        .find(|model| model.id().0 == selection.model.as_str())
}

fn render_default_model_section(
    _settings_window: &SettingsWindow,
    profile_id: &AgentProfileId,
    profile: Option<&AgentProfileSettings>,
    controls_disabled: bool,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let selection = profile.and_then(|profile| profile.default_model.clone());
    let current_model = selection
        .as_ref()
        .and_then(|selection| find_selected_model(selection, cx));
    let trigger_label: SharedString = match (&selection, &current_model) {
        (Some(_), Some(model)) => model.name().0,
        (Some(selection), None) => format!("{}/{}", selection.provider.0, selection.model).into(),
        (None, _) => "Select a model…".into(),
    };

    let settings_window = cx.weak_entity();
    let select_profile_id = profile_id.clone();
    let model_picker = render_language_model_picker(
        trigger_label,
        current_model,
        controls_disabled,
        move |model, cx| {
            settings_window
                .update(cx, |this, cx| {
                    mark_default_model_configured(this, &select_profile_id);
                    let fs = <dyn Fs>::global(cx);
                    set_profile_default_model(fs, select_profile_id.clone(), model.clone(), cx);
                })
                .log_err();
        },
        window,
        cx,
    );

    let clear_profile_id = profile_id.clone();
    let clear_disabled = controls_disabled
        || profile
            .and_then(|profile| profile.default_model.as_ref())
            .is_none();
    let clear_button = Button::new("agent-profile-clear-default-model", "Clear")
        .style(ButtonStyle::OutlinedGhost)
        .disabled(clear_disabled)
        .on_click(cx.listener(move |this, _, _window, cx| {
            mark_default_model_configured(this, &clear_profile_id);
            let fs = <dyn Fs>::global(cx);
            clear_profile_default_model(fs, clear_profile_id.clone(), cx);
        }));

    v_flex()
        .gap_4()
        .child(SettingsSectionHeader::new("Default Model").no_padding(true))
        .child(
            h_flex()
                .min_w_0()
                .justify_between()
                .child(
                    Label::new(ZED_AGENT_ONLY)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .child(h_flex().gap_2().child(model_picker).child(clear_button)),
        )
        .into_any_element()
}

fn render_builtin_tools_section(
    profile_id: &AgentProfileId,
    profile: Option<&AgentProfileSettings>,
    controls_disabled: bool,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let tool_fields: Vec<AnyElement> = agent::ALL_TOOL_NAMES
        .iter()
        .copied()
        .filter(|tool_name| agent::tool_feature_flag_enabled(tool_name, cx))
        .map(|tool_name| {
            let enabled = profile
                .map(|profile| profile.is_tool_enabled(tool_name))
                .unwrap_or(false);
            let settings_window = cx.weak_entity();
            let profile_id = profile_id.clone();
            SwitchField::new(
                format!("agent-profile-tool-{tool_name}"),
                Some(tool_name),
                None,
                enabled,
                move |state, _window, cx| {
                    let enabled = *state == ToggleState::Selected;
                    settings_window
                        .update(cx, |this, cx| {
                            mark_tools_configured(this, &profile_id);
                            let fs = <dyn Fs>::global(cx);
                            set_builtin_tool_enabled(
                                fs,
                                profile_id.clone(),
                                tool_name,
                                enabled,
                                cx,
                            );
                        })
                        .log_err();
                },
            )
            .disabled(controls_disabled)
            .tab_index(0)
            .into_any_element()
        })
        .collect();

    v_flex()
        .gap_4()
        .child(SettingsSectionHeader::new("Built-in Tools").no_padding(true))
        .child(
            Label::new(ZED_AGENT_ONLY)
                .size(LabelSize::Small)
                .color(Color::Muted),
        )
        .child(v_flex().gap_2().children(tool_fields))
        .into_any_element()
}

fn render_mcp_tools_section(
    settings_window: &SettingsWindow,
    profile_id: &AgentProfileId,
    profile: Option<&AgentProfileSettings>,
    context_server_registry: Option<&Entity<ContextServerRegistry>>,
    controls_disabled: bool,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let enable_all = profile
        .map(|profile| profile.enable_all_context_servers)
        .unwrap_or(false);
    let settings_window_weak = cx.weak_entity();
    let enable_all_profile_id = profile_id.clone();
    let enable_all_switch = SwitchField::new(
        "agent-profile-enable-all-mcp",
        Some("Enable All MCP Servers"),
        None,
        enable_all,
        move |state, _window, cx| {
            let enabled = *state == ToggleState::Selected;
            settings_window_weak
                .update(cx, |this, cx| {
                    mark_mcps_configured(this, &enable_all_profile_id);
                    let fs = <dyn Fs>::global(cx);
                    set_enable_all_context_servers(fs, enable_all_profile_id.clone(), enabled, cx);
                })
                .log_err();
        },
    )
    .disabled(controls_disabled)
    .tab_index(0);

    let body: AnyElement = if settings_window.active_project(cx).is_none() {
        render_dashed_note(
            "No active project found. Open a workspace to configure MCP tools.",
            cx,
        )
    } else {
        let servers: Vec<(ContextServerId, Vec<SharedString>)> = context_server_registry
            .map(|registry| {
                registry
                    .read(cx)
                    .servers()
                    .map(|(server_id, tools)| {
                        (server_id.clone(), tools.keys().cloned().collect::<Vec<_>>())
                    })
                    .collect()
            })
            .unwrap_or_default();
        if servers.is_empty() {
            render_dashed_note("No running MCP servers with tools.", cx)
        } else {
            v_flex()
                .gap_4()
                .children(servers.into_iter().map(|(server_id, tool_names)| {
                    v_flex()
                        .gap_2()
                        .child(SettingsSectionHeader::new(server_id.0.clone()).no_padding(true))
                        .children(tool_names.into_iter().map(|tool_name| {
                            let enabled = profile
                                .map(|profile| {
                                    profile.is_context_server_tool_enabled(&server_id.0, &tool_name)
                                })
                                .unwrap_or(false);
                            let settings_window = cx.weak_entity();
                            let profile_id = profile_id.clone();
                            let server_id_value = server_id.0.clone();
                            let tool_name_value: Arc<str> = tool_name.clone().into();
                            SwitchField::new(
                                format!("agent-profile-mcp-{}-{tool_name}", server_id.0),
                                Some(tool_name),
                                None,
                                enabled,
                                move |state, _window, cx| {
                                    let enabled = *state == ToggleState::Selected;
                                    settings_window
                                        .update(cx, |this, cx| {
                                            mark_mcps_configured(this, &profile_id);
                                            let fs = <dyn Fs>::global(cx);
                                            set_context_server_tool_enabled(
                                                fs,
                                                profile_id.clone(),
                                                server_id_value.clone(),
                                                tool_name_value.clone(),
                                                enabled,
                                                cx,
                                            );
                                        })
                                        .log_err();
                                },
                            )
                            .disabled(controls_disabled)
                            .tab_index(0)
                            .into_any_element()
                        }))
                }))
                .into_any_element()
        }
    };

    v_flex()
        .gap_4()
        .child(SettingsSectionHeader::new("MCP Tools").no_padding(true))
        .child(enable_all_switch)
        .child(body)
        .into_any_element()
}

// ---------------------------------------------------------------------------
// Add/fork form
// ---------------------------------------------------------------------------

/// Opens the add-profile (`base_profile_id = None`) or fork form sub-page.
pub(crate) fn open_agent_profile_form(
    settings_window: &mut SettingsWindow,
    base_profile_id: Option<AgentProfileId>,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) {
    let title: SharedString = match &base_profile_id {
        Some(base_profile_id) => {
            let base_name = AgentSettings::get_global(cx)
                .profiles
                .get(base_profile_id)
                .map(|profile| profile.name.clone())
                .unwrap_or_else(|| SharedString::from(base_profile_id.as_str()));
            format!("Fork {base_name}").into()
        }
        None => "Add Profile".into(),
    };

    let name = cx.new(|cx| {
        let mut editor = Editor::single_line(window, cx);
        editor.set_placeholder_text("Profile name", window, cx);
        editor
    });
    // Re-render on each keystroke so the name validation and the Save button's
    // disabled state stay live.
    cx.observe(&name, |_, _, cx| cx.notify()).detach();
    settings_window.agent_profile_form = Some(AgentProfileForm {
        base_profile_id,
        name,
    });

    settings_window.push_dynamic_sub_page(
        title,
        "Agent Configuration",
        Some(AGENT_PROFILES_JSON_PATH),
        false,
        render_agent_profile_form_page,
        window,
        cx,
    );
}

/// The kebab-case id `AgentProfile::create` derives from a display name.
fn profile_id_from_name(name: &str) -> String {
    name.to_case(Case::Kebab)
}

/// Save-enable rule (D6): a name is usable when it is non-empty, its kebab id
/// contains at least one alphanumeric character (symbol-only names like `!!!`
/// kebab-case to themselves, which is unusable as a profile id) and no
/// existing profile owns that id.
fn profile_name_error(name: &str, cx: &App) -> Option<SharedString> {
    if name.trim().is_empty() {
        return Some("Name is required.".into());
    }
    let id = profile_id_from_name(name);
    if !id.chars().any(|c| c.is_alphanumeric())
        || AgentSettings::get_global(cx)
            .profiles
            .contains_key(&AgentProfileId(id.as_str().into()))
    {
        return Some("A profile with this ID already exists.".into());
    }
    None
}

fn input_box(editor: &Entity<Editor>, cx: &App) -> impl IntoElement {
    let colors = cx.theme().colors();
    let focus_handle = editor.focus_handle(cx).tab_index(0).tab_stop(true);
    h_flex()
        .min_w_64()
        .py_1()
        .px_2()
        .h_8()
        .rounded_md()
        .border_1()
        .border_color(colors.border)
        .bg(colors.editor_background)
        .track_focus(&focus_handle)
        .focus(|style| style.border_color(colors.border_focused))
        .child(editor.clone())
}

fn render_agent_profile_form_page(
    settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    _window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let Some(form) = settings_window.agent_profile_form.as_ref() else {
        return div().into_any_element();
    };

    let name = form.name.read(cx).text(cx);
    let error = profile_name_error(&name, cx);

    v_flex()
        .id("agent-profile-form-page")
        .size_full()
        .pt_2p5()
        .px_8()
        .pb_16()
        .track_scroll(scroll_handle)
        .overflow_y_scroll()
        .child(
            v_flex()
                .w_full()
                .gap_4()
                .child(crate::render_settings_item_layout(
                    settings_window,
                    "Name",
                    "Required. The profile ID is derived from this name.",
                    input_box(&form.name, cx).into_any_element(),
                    None,
                    None,
                    None,
                    false,
                    cx,
                ))
                .when_some(error.clone(), |this, error| {
                    this.child(Label::new(error).size(LabelSize::Small).color(Color::Error))
                })
                .child(
                    h_flex()
                        .w_full()
                        .gap_2()
                        .justify_end()
                        .pt_2()
                        .child(
                            Button::new("agent-profile-form-cancel", "Cancel")
                                .style(ButtonStyle::Subtle)
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.agent_profile_form = None;
                                    this.pop_sub_page(window, cx);
                                })),
                        )
                        .child(
                            Button::new("agent-profile-form-save", "Save")
                                .style(ButtonStyle::Filled)
                                .disabled(error.is_some())
                                .on_click(cx.listener(|this, _, window, cx| {
                                    save_agent_profile_form(this, window, cx);
                                })),
                        ),
                ),
        )
        .into_any_element()
}

fn save_agent_profile_form(
    settings_window: &mut SettingsWindow,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) {
    let Some(form) = settings_window.agent_profile_form.as_ref() else {
        return;
    };
    let name = form.name.read(cx).text(cx);
    if profile_name_error(&name, cx).is_some() {
        return;
    }
    let base_profile_id = form.base_profile_id.clone();
    let fs = <dyn Fs>::global(cx);
    let profile_id = AgentProfile::create(name.clone(), base_profile_id.clone(), fs, cx);
    telemetry::event!(
        "Agent Profile Created",
        profile_id = profile_id.as_str(),
        is_fork = base_profile_id.is_some(),
        base_profile_id = base_profile_id.as_ref().map(|id| id.as_str())
    );

    settings_window.agent_profile_form = None;
    settings_window.pop_sub_page(window, cx);
    open_agent_profile_detail_page(
        settings_window,
        profile_id,
        true,
        SharedString::from(name),
        window,
        cx,
    );
}

// ---------------------------------------------------------------------------
// SettingsStore synchronization
// ---------------------------------------------------------------------------

/// Called from `SettingsWindow`'s `SettingsStore` observer before re-render:
/// flips `awaiting_creation` when a just-created profile lands in
/// `AgentSettings`, and pops the detail/form sub-pages when the tracked
/// profile disappears (e.g. deleted through the modal or settings.json).
pub(crate) fn sync_agent_profile_page_state(
    settings_window: &mut SettingsWindow,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) {
    let Some(state) = settings_window.agent_profile_page.as_mut() else {
        return;
    };
    if AgentSettings::get_global(cx)
        .profiles
        .contains_key(&state.profile_id)
    {
        state.awaiting_creation = false;
        return;
    }
    if state.awaiting_creation {
        return;
    }
    settings_window.agent_profile_page = None;
    settings_window.agent_profile_form = None;
    // Registered pages carry a description; the dynamic detail/form pages of
    // this feature never do. Pop everything of ours above the list page.
    while settings_window
        .sub_page_stack
        .last()
        .is_some_and(|sub_page| {
            sub_page.link.json_path == Some(AGENT_PROFILES_JSON_PATH)
                && sub_page.link.description.is_none()
        })
    {
        settings_window.pop_sub_page(window, cx);
    }
}

// ---------------------------------------------------------------------------
// Settings writes (called by UI handlers and directly by tests)
// ---------------------------------------------------------------------------

/// Applies `write` to the profile inside the user settings file. When the
/// profile is not yet saved there (e.g. an untouched built-in), it is first
/// materialized with the same full shape `save_to_settings` produces — the
/// check runs inside the file-update closure, so writes queue serially and
/// never lose a key written by a concurrent update.
fn write_profile(
    fs: Arc<dyn Fs>,
    profile_id: AgentProfileId,
    cx: &App,
    write: impl FnOnce(&mut AgentProfileContent) + Send + 'static,
) {
    let snapshot = AgentSettings::get_global(cx)
        .profiles
        .get(&profile_id)
        .cloned();
    update_settings_file(fs, cx, move |settings, _cx| {
        let saved = settings
            .agent
            .as_ref()
            .and_then(|agent| agent.profiles.as_ref())
            .is_some_and(|profiles| profiles.contains_key(profile_id.as_str()));
        if !saved {
            let Some(snapshot) = &snapshot else {
                // The profile vanished from merged settings — the sync hook
                // pops its page; silently skipping matches modal behavior.
                return;
            };
            if snapshot
                .save_to_settings(profile_id.clone(), settings)
                .log_err()
                .is_none()
            {
                return;
            }
        }
        if let Some(profile) = settings
            .agent
            .as_mut()
            .and_then(|agent| agent.profiles.as_mut())
            .and_then(|profiles| profiles.get_mut(profile_id.as_str()))
        {
            write(profile);
        }
    });
}

fn set_builtin_tool_enabled(
    fs: Arc<dyn Fs>,
    profile_id: AgentProfileId,
    tool_name: &'static str,
    enabled: bool,
    cx: &App,
) {
    write_profile(fs, profile_id, cx, move |profile| {
        profile.tools.insert(Arc::from(tool_name), enabled);
    });
}

fn set_context_server_tool_enabled(
    fs: Arc<dyn Fs>,
    profile_id: AgentProfileId,
    server_id: Arc<str>,
    tool_name: Arc<str>,
    enabled: bool,
    cx: &App,
) {
    write_profile(fs, profile_id, cx, move |profile| {
        profile
            .context_servers
            .entry(server_id)
            .or_default()
            .tools
            .insert(tool_name, enabled);
    });
}

fn set_enable_all_context_servers(
    fs: Arc<dyn Fs>,
    profile_id: AgentProfileId,
    enabled: bool,
    cx: &App,
) {
    write_profile(fs, profile_id, cx, move |profile| {
        profile.enable_all_context_servers = Some(enabled);
    });
}

fn set_profile_default_model(
    fs: Arc<dyn Fs>,
    profile_id: AgentProfileId,
    model: LanguageModel,
    cx: &App,
) {
    let selection = LanguageModelSelection {
        provider: LanguageModelProviderSetting(model.provider_id().0.to_string()),
        model: model.id().0.to_string(),
        enable_thinking: model.supports_thinking(),
        effort: model
            .default_effort_level()
            .map(|effort| effort.value.to_string()),
        speed: None,
    };
    write_profile(fs, profile_id, cx, move |profile| {
        profile.default_model = Some(selection);
    });
}

fn clear_profile_default_model(fs: Arc<dyn Fs>, profile_id: AgentProfileId, cx: &App) {
    write_profile(fs, profile_id, cx, move |profile| {
        profile.default_model = None;
    });
}

fn delete_agent_profile(fs: Arc<dyn Fs>, profile_id: AgentProfileId, cx: &App) {
    if builtin_profiles::is_builtin(&profile_id) {
        return;
    }
    telemetry::event!("Agent Profile Deleted", profile_id = profile_id.as_str());

    update_settings_file(fs, cx, move |settings, _cx| {
        let Some(agent_settings) = settings.agent.as_mut() else {
            return;
        };
        let Some(profiles) = agent_settings.profiles.as_mut() else {
            return;
        };
        profiles.shift_remove(profile_id.0.as_ref());
        if agent_settings
            .default_profile
            .as_deref()
            .is_some_and(|default_profile| default_profile == profile_id.0.as_ref())
        {
            agent_settings.default_profile = Some(AgentProfileId::default().0);
        }
    });
}

// ---------------------------------------------------------------------------
// Telemetry (once per section per detail-page opening — D5)
// ---------------------------------------------------------------------------

fn mark_tools_configured(settings_window: &mut SettingsWindow, profile_id: &AgentProfileId) {
    let Some(state) = settings_window.agent_profile_page.as_mut() else {
        return;
    };
    if state.tools_configured_sent {
        return;
    }
    state.tools_configured_sent = true;
    telemetry::event!(
        "Agent Profile Tools Configured",
        profile_id = profile_id.as_str(),
        is_builtin = builtin_profiles::is_builtin(profile_id)
    );
}

fn mark_mcps_configured(settings_window: &mut SettingsWindow, profile_id: &AgentProfileId) {
    let Some(state) = settings_window.agent_profile_page.as_mut() else {
        return;
    };
    if state.mcps_configured_sent {
        return;
    }
    state.mcps_configured_sent = true;
    telemetry::event!(
        "Agent Profile MCPs Configured",
        profile_id = profile_id.as_str(),
        is_builtin = builtin_profiles::is_builtin(profile_id)
    );
}

fn mark_default_model_configured(
    settings_window: &mut SettingsWindow,
    profile_id: &AgentProfileId,
) {
    let Some(state) = settings_window.agent_profile_page.as_mut() else {
        return;
    };
    if state.default_model_configured_sent {
        return;
    }
    state.default_model_configured_sent = true;
    telemetry::event!(
        "Agent Profile Default Model Configured",
        profile_id = profile_id.as_str(),
        is_builtin = builtin_profiles::is_builtin(profile_id)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    use feature_flags::FeatureFlagAppExt as _;
    use fs::FakeFs;
    use gpui::{TestAppContext, UpdateGlobal as _};
    use language_model::{LanguageModelId, LanguageModelName, LanguageModelProviderName};
    use project::DisableAiSettings;
    use settings::SettingsStore;

    use crate::{SubPageLink, SubPageType, USER};

    /// The registered list page link, as `page_data` declares it — the sync
    /// hook must never pop it (it carries a description unlike dynamic pages).
    fn list_page_link() -> SubPageLink {
        SubPageLink {
            title: AGENT_PROFILES_PAGE_TITLE.into(),
            r#type: SubPageType::default(),
            description: Some("profiles".into()),
            search_aliases: &[],
            json_path: Some(AGENT_PROFILES_JSON_PATH),
            in_json: true,
            files: USER,
            render: render_agent_profiles_page,
        }
    }

    /// Mirrors `setup_backfill_test`: a fake settings file on FakeFs, a test
    /// SettingsStore as global, agent-related settings registered, the FakeFs
    /// exposed as `<dyn Fs>` global for the form/save paths.
    async fn init_agent_profile_test(
        cx: &mut TestAppContext,
        initial_settings: &str,
    ) -> Arc<FakeFs> {
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.save(
            paths::settings_file().as_path(),
            &initial_settings.into(),
            Default::default(),
        )
        .await
        .expect("failed to save initial settings");
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            menu::init();
            language_model::init(cx);
            DisableAiSettings::register(cx);
            AgentSettings::register(cx);
            <dyn Fs>::set_global(fs.clone(), cx);
            cx.set_staff(true);
            // Keep merged settings in sync with the file content so
            // AgentSettings reflects profiles seeded on disk.
            SettingsStore::update_global(cx, |store, cx| {
                store
                    .set_user_settings(initial_settings, cx)
                    .result()
                    .unwrap();
            });
        });
        fs
    }

    fn profile_id(id: &str) -> AgentProfileId {
        AgentProfileId(id.into())
    }

    async fn settings_file_json(fs: &Arc<FakeFs>) -> serde_json::Value {
        let content = fs
            .load(paths::settings_file().as_path())
            .await
            .expect("failed to load settings file");
        serde_json::from_str(&content).expect("settings file must be valid JSON")
    }

    fn test_model() -> LanguageModel {
        LanguageModel::new(
            LanguageModelId::from("test-model".to_string()),
            LanguageModelName::from("Test Model".to_string()),
            LanguageModelProviderId::from("test-provider".to_string()),
            LanguageModelProviderName::from("Test Provider".to_string()),
            "test-provider/test-model",
            1000,
        )
    }

    #[gpui::test]
    async fn create_profile_via_form_writes_create_json(cx: &mut TestAppContext) {
        let fs = init_agent_profile_test(cx, "{}").await;

        let cx = cx.add_empty_window();
        let settings_window = cx.update(|window, cx| cx.new(|cx| SettingsWindow::test(window, cx)));

        settings_window.update_in(cx, |this, window, cx| {
            open_agent_profile_form(this, None, window, cx);
            let form = this.agent_profile_form.as_ref().expect("form state");
            form.name.update(cx, |editor, cx| {
                editor.set_text("My Profile", window, cx);
            });
            save_agent_profile_form(this, window, cx);

            // The detail page opens immediately, while the create write is
            // still in flight — the new profile is not yet in AgentSettings.
            let state = this.agent_profile_page.as_ref().expect("page state");
            assert!(state.awaiting_creation);
            assert_eq!(state.profile_id, profile_id("my-profile"));
        });

        cx.run_until_parked();

        let json = settings_file_json(&fs).await;
        let expected = serde_json::to_value(&AgentProfileContent {
            name: "My Profile".into(),
            tools: Default::default(),
            enable_all_context_servers: Some(false),
            context_servers: Default::default(),
            default_model: None,
        })
        .expect("serialize expected profile content");
        assert_eq!(json["agent"]["profiles"]["my-profile"], expected);

        // After the write lands, the sync hook flips awaiting_creation.
        settings_window.update_in(cx, |this, window, cx| {
            sync_agent_profile_page_state(this, window, cx);
            let state = this.agent_profile_page.as_ref().expect("page state");
            assert!(!state.awaiting_creation);
        });
    }

    #[gpui::test]
    async fn fork_preserves_tools_mcp_and_model(cx: &mut TestAppContext) {
        let seed = r#"{
            "agent": {
                "profiles": {
                    "base": {
                        "name": "Base",
                        "tools": {"fetch": true, "terminal": false},
                        "enable_all_context_servers": true,
                        "context_servers": {"srv": {"tools": {"t1": false}}},
                        "default_model": {
                            "provider": "zed.dev",
                            "model": "m1",
                            "enable_thinking": true
                        }
                    }
                }
            }
        }"#;
        let fs = init_agent_profile_test(cx, seed).await;

        let cx = cx.add_empty_window();
        let settings_window = cx.update(|window, cx| cx.new(|cx| SettingsWindow::test(window, cx)));

        settings_window.update_in(cx, |this, window, cx| {
            open_agent_profile_form(this, Some(profile_id("base")), window, cx);
            let form = this.agent_profile_form.as_ref().expect("form state");
            form.name.update(cx, |editor, cx| {
                editor.set_text("Forked", window, cx);
            });
            save_agent_profile_form(this, window, cx);
        });

        cx.run_until_parked();

        let json = settings_file_json(&fs).await;
        let base = json["agent"]["profiles"]["base"].clone();
        let forked = &json["agent"]["profiles"]["forked"];
        let mut expected = base;
        expected["name"] = "Forked".into();
        assert_eq!(forked, &expected);
    }

    #[gpui::test]
    async fn delete_default_profile_resets_to_write(cx: &mut TestAppContext) {
        let seed = r#"{
            "agent": {
                "default_profile": "my-profile",
                "profiles": {"my-profile": {"name": "My Profile"}}
            }
        }"#;
        let fs = init_agent_profile_test(cx, seed).await;

        cx.update(|cx| {
            delete_agent_profile(fs.clone(), profile_id("my-profile"), cx);
        });
        cx.run_until_parked();

        let json = settings_file_json(&fs).await;
        assert!(json["agent"]["profiles"]["my-profile"].is_null());
        assert_eq!(json["agent"]["default_profile"], "write");
    }

    #[gpui::test]
    async fn delete_builtin_writes_nothing(cx: &mut TestAppContext) {
        let fs = init_agent_profile_test(cx, "{}").await;

        cx.update(|cx| {
            delete_agent_profile(fs.clone(), profile_id("write"), cx);
        });
        cx.run_until_parked();

        let content = fs
            .load(paths::settings_file().as_path())
            .await
            .expect("failed to load settings file");
        assert_eq!(content.trim(), "{}");
    }

    #[gpui::test]
    async fn toggle_builtin_tool_materializes_profile(cx: &mut TestAppContext) {
        let fs = init_agent_profile_test(cx, "{}").await;

        cx.update(|cx| {
            set_builtin_tool_enabled(fs.clone(), profile_id("write"), "fetch", false, cx);
        });
        cx.run_until_parked();

        let json = settings_file_json(&fs).await;
        let profile = &json["agent"]["profiles"]["write"];
        assert_eq!(profile["name"], "Write");
        assert_eq!(profile["tools"]["fetch"], false);
        // The rest of the merged profile must have been materialized too.
        assert_eq!(profile["tools"]["terminal"], true);
        assert_eq!(profile["enable_all_context_servers"], true);
    }

    #[gpui::test]
    async fn mcp_tool_disable_under_enable_all_writes_false(cx: &mut TestAppContext) {
        let fs = init_agent_profile_test(cx, "{}").await;

        // On `write` enable_all_context_servers=true: disabling a tool that
        // has no explicit value must write `false` (not `!false`).
        cx.update(|cx| {
            set_context_server_tool_enabled(
                fs.clone(),
                profile_id("write"),
                "my-server".into(),
                "some_tool".into(),
                false,
                cx,
            );
        });
        cx.run_until_parked();

        let json = settings_file_json(&fs).await;
        let profile = &json["agent"]["profiles"]["write"];
        assert_eq!(profile["enable_all_context_servers"], true);
        assert_eq!(
            profile["context_servers"]["my-server"]["tools"]["some_tool"],
            false
        );

        // Enabling the same tool again writes `true` in place.
        cx.update(|cx| {
            set_context_server_tool_enabled(
                fs.clone(),
                profile_id("write"),
                "my-server".into(),
                "some_tool".into(),
                true,
                cx,
            );
        });
        cx.run_until_parked();

        let json = settings_file_json(&fs).await;
        assert_eq!(
            json["agent"]["profiles"]["write"]["context_servers"]["my-server"]["tools"]["some_tool"],
            true
        );
    }

    #[gpui::test]
    async fn default_model_select_and_clear_on_builtin(cx: &mut TestAppContext) {
        let fs = init_agent_profile_test(cx, "{}").await;

        cx.update(|cx| {
            set_profile_default_model(fs.clone(), profile_id("write"), test_model(), cx);
        });
        cx.run_until_parked();

        let json = settings_file_json(&fs).await;
        let profile = &json["agent"]["profiles"]["write"];
        assert_eq!(profile["name"], "Write");
        // None fields are not serialized into the settings file.
        assert_eq!(
            profile["default_model"],
            serde_json::json!({
                "provider": "test-provider",
                "model": "test-model",
                "enable_thinking": false,
            })
        );

        cx.update(|cx| {
            clear_profile_default_model(fs.clone(), profile_id("write"), cx);
        });
        cx.run_until_parked();

        let json = settings_file_json(&fs).await;
        assert!(json["agent"]["profiles"]["write"]["default_model"].is_null());
    }

    #[gpui::test]
    async fn profile_name_validation(cx: &mut TestAppContext) {
        init_agent_profile_test(cx, "{}").await;

        cx.update(|cx| {
            assert_eq!(
                profile_name_error("", cx).as_deref(),
                Some("Name is required.")
            );
            assert_eq!(
                profile_name_error("   ", cx).as_deref(),
                Some("Name is required.")
            );
            // A name that kebab-cases to an empty id.
            assert_eq!(
                profile_name_error("!!!", cx).as_deref(),
                Some("A profile with this ID already exists.")
            );
            // "Write" kebab-cases to the existing built-in id.
            assert_eq!(
                profile_name_error("Write", cx).as_deref(),
                Some("A profile with this ID already exists.")
            );
            assert!(profile_name_error("My Profile", cx).is_none());
        });
    }

    #[gpui::test]
    async fn detail_page_pops_when_profile_removed(cx: &mut TestAppContext) {
        let fs = init_agent_profile_test(
            cx,
            r#"{"agent": {"profiles": {"my-profile": {"name": "My Profile"}}}}"#,
        )
        .await;

        let cx = cx.add_empty_window();
        let settings_window = cx.update(|window, cx| cx.new(|cx| SettingsWindow::test(window, cx)));

        settings_window.update_in(cx, |this, window, cx| {
            this.push_sub_page(list_page_link(), "Agent Configuration".into(), window, cx);
            open_agent_profile_detail_page(
                this,
                profile_id("my-profile"),
                false,
                "My Profile".into(),
                window,
                cx,
            );
            assert_eq!(this.sub_page_stack.len(), 2);
        });

        // The profile disappears outside of this window (modal, settings.json).
        cx.update(|_window, cx| {
            update_settings_file(fs.clone(), cx, |settings, _cx| {
                if let Some(profiles) = settings
                    .agent
                    .as_mut()
                    .and_then(|agent| agent.profiles.as_mut())
                {
                    profiles.shift_remove("my-profile");
                }
            });
        });
        cx.run_until_parked();

        settings_window.update_in(cx, |this, window, cx| {
            sync_agent_profile_page_state(this, window, cx);
            assert!(this.agent_profile_page.is_none());
            let titles: Vec<String> = this
                .sub_page_stack
                .iter()
                .map(|sub_page| sub_page.link.title.to_string())
                .collect();
            assert_eq!(titles, ["Agent Profiles"]);
        });
    }

    #[gpui::test]
    async fn detail_page_stays_open_while_awaiting_creation(cx: &mut TestAppContext) {
        init_agent_profile_test(cx, "{}").await;

        let cx = cx.add_empty_window();
        let settings_window = cx.update(|window, cx| cx.new(|cx| SettingsWindow::test(window, cx)));

        settings_window.update_in(cx, |this, window, cx| {
            this.push_sub_page(list_page_link(), "Agent Configuration".into(), window, cx);
            open_agent_profile_detail_page(
                this,
                profile_id("my-profile"),
                true,
                "My Profile".into(),
                window,
                cx,
            );
            sync_agent_profile_page_state(this, window, cx);

            // The write has not landed yet: the detail page must not be
            // treated as "profile deleted" and popped.
            assert!(this.agent_profile_page.is_some());
            assert_eq!(this.sub_page_stack.len(), 2);
        });
    }
}
